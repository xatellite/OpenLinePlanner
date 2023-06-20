use geo::Polygon;
use geojson::{ser::serialize_geometry, feature::Id, Feature, GeoJson};
use serde::Serialize;
use tinytemplate::TinyTemplate;

use anyhow::{anyhow, Error, Result};

use super::overpass::query_overpass;

#[derive(Serialize, Debug)]
pub struct AdminArea {
    pub name: String,
    pub id: u64,
    pub admin_level: u16,
    #[serde(
        serialize_with = "serialize_geometry",
        deserialize_with = "deserialize_geometry"
    )]
    pub geometry: Polygon,
}

impl TryFrom<Feature> for AdminArea {
    type Error = Error;

    fn try_from(value: Feature) -> Result<Self, Self::Error> {
        let properties = value.properties.unwrap_or_default();
        let id: u64 = match value.id.unwrap() {
            Id::String(id) => id.split('/').skip(1).next().unwrap().parse().unwrap(),
            Id::Number(id) => id.as_u64().unwrap(),
        };
        let Some(geometry) = value.geometry.and_then(|geometry|
            TryInto::<Polygon>::try_into(geometry.value).ok()
        ) else {
            log::info!("Area dropped due to wrong geometry: {:?}", properties);
            return Err(anyhow!("failed to parse geometry"))
        };
        Ok(AdminArea {
            name: format!(
                "{} {}",
                properties
                    .get("name:prefix")
                    .and_then(|prefix| prefix.as_str())
                    .unwrap_or_default(),
                properties
                    .get("name")
                    .and_then(|name| name.as_str())
                    .unwrap_or_default()
            ),
            id,
            admin_level: properties
                .get("admin_level")
                .and_then(|id| id.as_str())
                .and_then(|id| id.parse().ok())
                .unwrap_or_default(),
            geometry: geometry,
        })
    }
}

static OVP_QUERY_TEMPLATE: &'static str = "[out:json][timeout:25];
area({area_id});
(
  relation[\"boundary\" = \"administrative\"][\"admin_level\"={admin_level}](area);
);

out geom;";

pub struct AdminAreas(Vec<AdminArea>);

impl TryFrom<GeoJson> for AdminAreas {
    type Error = Error;

    fn try_from(value: GeoJson) -> Result<Self, Self::Error> {
        match value {
            GeoJson::FeatureCollection(feature_collection) => Ok(AdminAreas(
                feature_collection
                    .into_iter()
                    .filter_map(|feature| feature.try_into().ok())
                    .collect(),
            )),
            _ => Err(anyhow!("wrong geometry")),
        }
    }
}

#[derive(Serialize)]
struct Context {
    admin_level: u16,
    area_id: u64,
}

pub fn render_ovp_query_template(admin_level: u16, area_id: u64) -> Result<String, Error> {
    let mut tt = TinyTemplate::new();
    tt.add_template("query", OVP_QUERY_TEMPLATE)?;

    let area_id = area_id + 3600000000;

    let context = Context {
        admin_level,
        area_id,
    };

    Ok(tt.render("query", &context)?)
}

pub fn find_admin_boundaries(admin_level: u16, area_id: u64) -> Result<Vec<AdminArea>, Error> {
    let ovp_query = render_ovp_query_template(admin_level, area_id)?;

    let ovp_response = query_overpass(ovp_query)?;

    Ok(TryInto::<AdminAreas>::try_into(ovp_response)?.0)
}
