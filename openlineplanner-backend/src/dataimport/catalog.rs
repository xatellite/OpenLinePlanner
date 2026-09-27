use std::{
    fs,
    path::PathBuf,
    sync::RwLock,
    time::{Duration, SystemTime},
};

use config::Config;
use geo::{BoundingRect, Contains, Centroid, MultiPolygon, Point, Polygon, Rect};
use geojson::GeoJson;
use serde::Serialize;

use crate::error::OLPError;

/// Geofabrik publishes a GeoJSON index of every downloadable extract. Each
/// feature carries an id, a display name, the parent region and the download
/// urls, with the region outline as geometry.
const DEFAULT_CATALOG_URL: &str = "https://download.geofabrik.de/index-v1.json";
const CATALOG_CACHE_FILE: &str = "region-catalog.json";
const CATALOG_TTL: Duration = Duration::from_secs(60 * 60 * 24 * 7);

/// One downloadable region. `geometry` and `pbf_url` are only needed on the
/// server side, so they are kept out of the serialized form -- the raw index is
/// roughly a megabyte of polygons and must not be shipped to the browser.
#[derive(Debug, Clone, Serialize)]
pub struct CatalogEntry {
    pub id: String,
    pub name: String,
    pub parent: Option<String>,
    pub bbox: [f64; 4],
    #[serde(skip)]
    pub pbf_url: String,
    #[serde(skip)]
    pub geometry: MultiPolygon,
}

impl CatalogEntry {
    fn bbox_rect(&self) -> Rect {
        Rect::new(
            (self.bbox[0], self.bbox[1]),
            (self.bbox[2], self.bbox[3]),
        )
    }

    fn bbox_area(&self) -> f64 {
        (self.bbox[2] - self.bbox[0]) * (self.bbox[3] - self.bbox[1])
    }

    fn contains_point(&self, point: &Point) -> bool {
        // Contains<Point> is implemented for Polygon, so test the members
        // individually rather than relying on a MultiPolygon impl.
        self.geometry.0.iter().any(|polygon| polygon.contains(point))
    }
}

#[derive(Debug, Clone, Default)]
pub struct Catalog {
    pub entries: Vec<CatalogEntry>,
}

impl Catalog {
    fn parse(raw: &str) -> Result<Self, OLPError> {
        let geojson = raw.parse::<GeoJson>().map_err(OLPError::from_error)?;
        let GeoJson::FeatureCollection(collection) = geojson else {
            return Err(OLPError::GenericError(
                "region catalog is not a feature collection".to_owned(),
            ));
        };

        let entries = collection
            .features
            .into_iter()
            .filter_map(|feature| {
                let properties = feature.properties?;
                let id = properties.get("id")?.as_str()?.to_owned();
                let name = properties
                    .get("name")
                    .and_then(|name| name.as_str())
                    .unwrap_or(&id)
                    .to_owned();
                let parent = properties
                    .get("parent")
                    .and_then(|parent| parent.as_str())
                    .map(|parent| parent.to_owned());
                let pbf_url = properties
                    .get("urls")
                    .and_then(|urls| urls.get("pbf"))
                    .and_then(|pbf| pbf.as_str())?
                    .to_owned();

                let geometry = match feature.geometry?.value {
                    value @ geojson::Value::MultiPolygon(_) => {
                        TryInto::<MultiPolygon>::try_into(value).ok()?
                    }
                    value @ geojson::Value::Polygon(_) => {
                        MultiPolygon::new(vec![TryInto::<Polygon>::try_into(value).ok()?])
                    }
                    _ => return None,
                };

                let bounds = geometry.bounding_rect()?;
                Some(CatalogEntry {
                    id,
                    name,
                    parent,
                    bbox: [
                        bounds.min().x,
                        bounds.min().y,
                        bounds.max().x,
                        bounds.max().y,
                    ],
                    pbf_url,
                    geometry,
                })
            })
            .collect::<Vec<_>>();

        if entries.is_empty() {
            return Err(OLPError::GenericError(
                "region catalog contained no usable entries".to_owned(),
            ));
        }

        Ok(Self { entries })
    }

    pub fn by_id(&self, id: &str) -> Option<&CatalogEntry> {
        self.entries.iter().find(|entry| entry.id == id)
    }

    /// Picks the smallest extract that fully covers `area`.
    ///
    /// Coverage is tested as "the extract's bounding box encloses the area's
    /// bounding box, and the area's centroid falls inside the extract outline".
    /// Testing bbox corners directly would wrongly reject coastal regions whose
    /// corners fall into the sea.
    pub fn resolve_for(&self, area: &MultiPolygon) -> Option<&CatalogEntry> {
        let area_bounds = area.bounding_rect()?;
        let centroid = area.centroid()?;

        self.entries
            .iter()
            .filter(|entry| {
                let bounds = entry.bbox_rect();
                bounds.min().x <= area_bounds.min().x
                    && bounds.min().y <= area_bounds.min().y
                    && bounds.max().x >= area_bounds.max().x
                    && bounds.max().y >= area_bounds.max().y
            })
            .filter(|entry| entry.contains_point(&centroid))
            .min_by(|a, b| {
                a.bbox_area()
                    .partial_cmp(&b.bbox_area())
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }
}

/// Caches the parsed catalog in memory and on disk. A stale on-disk copy is
/// always preferred over failing the request, so a flaky Geofabrik does not
/// break region selection.
#[derive(Default)]
pub struct CatalogStore {
    inner: RwLock<Option<Catalog>>,
}

impl CatalogStore {
    pub fn new() -> Self {
        Self::default()
    }

    fn cache_path(config: &Config) -> PathBuf {
        let mut path = PathBuf::from(config.get_string("cache.dir").unwrap_or("./cache/".into()));
        path.push(CATALOG_CACHE_FILE);
        path
    }

    fn cached_in_memory(&self) -> Option<Catalog> {
        self.inner.read().ok().and_then(|slot| slot.clone())
    }

    fn store_in_memory(&self, catalog: &Catalog) {
        match self.inner.write() {
            Ok(mut slot) => *slot = Some(catalog.clone()),
            Err(err) => log::error!("catalog lock poisoned: {}", err),
        }
    }

    /// Returns the catalog, fetching it only when neither memory nor a fresh
    /// on-disk copy can serve it.
    pub async fn get(&self, config: &Config) -> Result<Catalog, OLPError> {
        if let Some(catalog) = self.cached_in_memory() {
            return Ok(catalog);
        }

        let path = Self::cache_path(config);
        let fresh_on_disk = fs::metadata(&path)
            .and_then(|meta| meta.modified())
            .map(|modified| {
                SystemTime::now()
                    .duration_since(modified)
                    .map(|age| age < CATALOG_TTL)
                    .unwrap_or(false)
            })
            .unwrap_or(false);

        if fresh_on_disk {
            if let Ok(raw) = fs::read_to_string(&path) {
                match Catalog::parse(&raw) {
                    Ok(catalog) => {
                        self.store_in_memory(&catalog);
                        return Ok(catalog);
                    }
                    Err(err) => log::warn!("cached region catalog unusable: {}", err),
                }
            }
        }

        let url = config
            .get_string("catalog.url")
            .unwrap_or_else(|_| DEFAULT_CATALOG_URL.to_owned());
        log::info!("fetching region catalog from {}", url);

        let fetched = reqwest::Client::new()
            .get(&url)
            .send()
            .await
            .and_then(|response| response.error_for_status());

        let raw = match fetched {
            Ok(response) => response.text().await.map_err(OLPError::from_error)?,
            Err(err) => {
                // Fall back to whatever is on disk, however old it is.
                log::warn!("failed to fetch region catalog: {}", err);
                let stale = fs::read_to_string(&path).map_err(|_| {
                    OLPError::GenericError(format!("could not reach region catalog at {}", url))
                })?;
                let catalog = Catalog::parse(&stale)?;
                self.store_in_memory(&catalog);
                return Ok(catalog);
            }
        };

        let catalog = Catalog::parse(&raw)?;

        if let Some(parent) = path.parent() {
            if let Err(err) = fs::create_dir_all(parent) {
                log::warn!("failed to create cache directory: {}", err);
            }
        }
        if let Err(err) = fs::write(&path, &raw) {
            log::warn!("failed to cache region catalog: {}", err);
        }

        self.store_in_memory(&catalog);
        Ok(catalog)
    }
}
