//! Administrative boundary lookup via Nominatim reverse geocoding.

use std::{
    collections::HashMap,
    sync::{LazyLock, Mutex},
    time::{Duration, Instant},
};

use geo::Point;
use geojson::{feature::Id, Feature, Geometry, JsonObject, JsonValue};
use serde::Deserialize;

use super::http;
use crate::error::OLPError;

const DEFAULT_NOMINATIM_URL: &str = "https://nominatim.openstreetmap.org";
const NOMINATIM_URL_ENV: &str = "NOMINATIM_API_URL";
const NOMINATIM_INTERVAL_ENV: &str = "NOMINATIM_MIN_INTERVAL_MS";

/// Smallest area first. 12 lands on the municipality or city district (OSM
/// `admin_level` 8 or 9), 10 on the enclosing city. Deeper than 12 returns
/// cadastral communities and neighbourhoods, which are not import targets.
const ZOOM_LEVELS: [u8; 2] = [12, 10];

/// The public instance caps use at one request per second and blocks abusers.
/// Set `NOMINATIM_MIN_INTERVAL_MS=0` for an instance without that limit.
const DEFAULT_MIN_REQUEST_INTERVAL: Duration = Duration::from_millis(1100);

/// Falls back to the default when unset or blank, so an empty
/// `NOMINATIM_API_URL=` in a compose file still works.
fn resolve_nominatim_url(configured: Option<String>) -> String {
    configured
        .filter(|url| !url.trim().is_empty())
        .map(|url| url.trim().trim_end_matches('/').to_owned())
        .unwrap_or_else(|| DEFAULT_NOMINATIM_URL.to_owned())
}

fn resolve_min_interval(configured: Option<String>) -> Duration {
    configured
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .map(Duration::from_millis)
        .unwrap_or(DEFAULT_MIN_REQUEST_INTERVAL)
}

static NOMINATIM_URL: LazyLock<String> = LazyLock::new(|| {
    let url = resolve_nominatim_url(std::env::var(NOMINATIM_URL_ENV).ok());
    log::info!("reverse geocoding via {}", url);
    url
});

static MIN_REQUEST_INTERVAL: LazyLock<Duration> =
    LazyLock::new(|| resolve_min_interval(std::env::var(NOMINATIM_INTERVAL_ENV).ok()));

/// Earliest instant at which the next request may be sent.
static NEXT_SLOT: LazyLock<Mutex<Instant>> = LazyLock::new(|| Mutex::new(Instant::now()));

/// Advances `next` past this caller's slot and reports how long to wait.
/// Split from the global so the rule is testable without shared state.
fn reserve(next: &mut Instant, now: Instant, interval: Duration) -> Duration {
    if interval.is_zero() {
        return Duration::ZERO;
    }

    let scheduled = (*next).max(now);
    *next = scheduled + interval;
    scheduled.saturating_duration_since(now)
}

/// Claimed under the lock so concurrent lookups queue instead of all deciding
/// they may send at once.
fn claim_slot(now: Instant, interval: Duration) -> Duration {
    // Skip the process-wide mutex entirely when the wait is disabled.
    if interval.is_zero() {
        return Duration::ZERO;
    }

    let mut next = NEXT_SLOT
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());

    reserve(&mut next, now, interval)
}

/// The subset of a Nominatim `jsonv2` reverse response we act on.
#[derive(Deserialize)]
struct ReverseResponse {
    /// Present instead of a result when the point resolves to nothing.
    error: Option<JsonValue>,
    osm_type: Option<String>,
    osm_id: Option<u64>,
    /// `boundary` for an administrative area, otherwise not importable.
    category: Option<String>,
    name: Option<String>,
    display_name: Option<String>,
    extratags: Option<HashMap<String, String>>,
    geojson: Option<JsonValue>,
}

/// Builds the `Feature` that `AdminArea` converts from.
fn to_feature(response: ReverseResponse) -> Option<Feature> {
    if response.error.is_some() {
        return None;
    }

    // Without this a click in open country returns the nearest hamlet or road,
    // which has no polygon to clip against.
    if response.category.as_deref() != Some("boundary") {
        log::debug!(
            "ignoring non-boundary reverse result: {:?}",
            response.display_name
        );
        return None;
    }

    // Relations only: way ids would collide in the `<id>.map` namespace.
    let osm_type = response.osm_type?;
    if osm_type != "relation" {
        return None;
    }
    let osm_id = response.osm_id?;

    let geometry: Geometry = serde_json::from_value(response.geojson?).ok()?;
    let extratags = response.extratags.unwrap_or_default();

    let mut properties = JsonObject::new();
    properties.insert("name".to_owned(), JsonValue::from(response.name?));
    if let Some(prefix) = extratags.get("name:prefix") {
        properties.insert("name:prefix".to_owned(), JsonValue::from(prefix.clone()));
    }
    if let Some(level) = extratags.get("admin_level") {
        properties.insert("admin_level".to_owned(), JsonValue::from(level.clone()));
    }

    Some(Feature {
        bbox: None,
        geometry: Some(geometry),
        // AdminArea splits this on the slash.
        id: Some(Id::String(format!("{}/{}", osm_type, osm_id))),
        properties: Some(properties),
        foreign_members: None,
    })
}

async fn reverse(point: Point, zoom: u8) -> Result<Option<Feature>, OLPError> {
    let wait = claim_slot(Instant::now(), *MIN_REQUEST_INTERVAL);
    if !wait.is_zero() {
        actix_web::rt::time::sleep(wait).await;
    }

    // `RequestBuilder::query` is not in this reqwest feature set. Every value
    // is a number or a literal, so nothing needs escaping.
    let url = format!(
        "{}/reverse?lat={}&lon={}&zoom={}&format=jsonv2\
         &polygon_geojson=1&extratags=1",
        NOMINATIM_URL.as_str(),
        point.y(),
        point.x(),
        zoom,
    );

    let body = http::send_with_retry("nominatim", || http::client().get(&url))
        .await
        .map_err(OLPError::from_error)?;

    let response: ReverseResponse = serde_json::from_str(&body).map_err(OLPError::from_error)?;
    Ok(to_feature(response))
}

/// Looks up the administrative areas containing `point`, smallest first.
pub async fn admin_areas_for_point(point: Point) -> Result<Vec<Feature>, OLPError> {
    let mut features: Vec<Feature> = Vec::new();

    for zoom in ZOOM_LEVELS {
        let feature = reverse(point, zoom).await?;
        let Some(feature) = feature else { continue };

        // Zoom levels collapse onto one relation where a place has no finer
        // subdivision.
        let duplicate = features
            .iter()
            .any(|existing| existing.id.is_some() && existing.id == feature.id);
        if !duplicate {
            features.push(feature);
        }
    }

    Ok(features)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn response_from(raw: &str) -> ReverseResponse {
        serde_json::from_str(raw).expect("fixture should deserialize")
    }

    #[test]
    fn falls_back_to_default_when_unset_or_blank() {
        assert_eq!(resolve_nominatim_url(None), DEFAULT_NOMINATIM_URL);
        assert_eq!(resolve_nominatim_url(Some("".into())), DEFAULT_NOMINATIM_URL);
        assert_eq!(
            resolve_nominatim_url(Some("   ".into())),
            DEFAULT_NOMINATIM_URL
        );
    }

    #[test]
    fn normalises_configured_url() {
        assert_eq!(
            resolve_nominatim_url(Some("  https://nominatim.example.org/  ".into())),
            "https://nominatim.example.org"
        );
    }

    #[test]
    fn interval_is_configurable_and_disablable() {
        assert_eq!(resolve_min_interval(None), DEFAULT_MIN_REQUEST_INTERVAL);
        assert_eq!(resolve_min_interval(Some("0".into())), Duration::ZERO);
        assert_eq!(
            resolve_min_interval(Some("250".into())),
            Duration::from_millis(250)
        );
        assert_eq!(
            resolve_min_interval(Some("soon".into())),
            DEFAULT_MIN_REQUEST_INTERVAL
        );
    }

    #[test]
    fn slots_are_spaced_by_the_interval() {
        let interval = Duration::from_millis(1100);
        let start = Instant::now();
        let mut next = start;

        assert_eq!(reserve(&mut next, start, interval), Duration::ZERO);
        assert_eq!(reserve(&mut next, start, interval), interval);
        assert_eq!(reserve(&mut next, start, interval), interval * 2);
    }

    #[test]
    fn idle_time_is_not_banked_into_a_burst() {
        let interval = Duration::from_millis(1100);
        let start = Instant::now();
        let mut next = start;

        reserve(&mut next, start, interval);

        // Idle time must not bank up into a burst.
        let later = start + Duration::from_secs(60);
        assert_eq!(reserve(&mut next, later, interval), Duration::ZERO);
        assert_eq!(reserve(&mut next, later, interval), interval);
    }

    #[test]
    fn disabled_interval_never_waits() {
        let start = Instant::now();
        let mut next = start;
        assert_eq!(reserve(&mut next, start, Duration::ZERO), Duration::ZERO);
        assert_eq!(claim_slot(start, Duration::ZERO), Duration::ZERO);
    }

    #[test]
    fn builds_a_feature_for_an_administrative_relation() {
        let feature = to_feature(response_from(
            r#"{
                "osm_type": "relation",
                "osm_id": 1990592,
                "category": "boundary",
                "type": "administrative",
                "name": "Innere Stadt",
                "display_name": "Innere Stadt, Wien, Österreich",
                "extratags": { "admin_level": "9" },
                "geojson": {
                    "type": "Polygon",
                    "coordinates": [[[16.35,48.19],[16.38,48.19],[16.38,48.21],[16.35,48.19]]]
                }
            }"#,
        ))
        .expect("an administrative relation should convert");

        assert_eq!(feature.id, Some(Id::String("relation/1990592".to_owned())));
        let properties = feature.properties.expect("properties");
        assert_eq!(properties.get("name").unwrap(), "Innere Stadt");
        assert_eq!(properties.get("admin_level").unwrap(), "9");
        assert!(feature.geometry.is_some());
    }

    #[test]
    fn rejects_results_that_are_not_importable() {
        assert!(to_feature(response_from(r#"{"error": "Unable to geocode"}"#)).is_none());

        assert!(to_feature(response_from(
            r#"{
                "osm_type": "way", "osm_id": 42, "category": "highway",
                "name": "Ringstraße",
                "geojson": {"type": "LineString", "coordinates": [[16.3,48.2],[16.4,48.2]]}
            }"#
        ))
        .is_none());

        assert!(to_feature(response_from(
            r#"{
                "osm_type": "way", "osm_id": 1990592, "category": "boundary",
                "name": "Somewhere",
                "geojson": {"type": "Polygon", "coordinates": [[[0,0],[1,0],[1,1],[0,0]]]}
            }"#
        ))
        .is_none());
    }

    /// Run with `cargo test -- --ignored` to check the configured instance.
    #[test]
    #[ignore = "requires network access"]
    fn resolves_vienna_against_the_live_service() {
        let features = actix_web::rt::System::new()
            .block_on(admin_areas_for_point(Point::new(16.3738, 48.2082)))
            .expect("reverse geocode failed");

        assert!(
            !features.is_empty(),
            "expected at least one admin boundary around Vienna"
        );
        assert_eq!(
            features[0].id,
            Some(Id::String("relation/1990592".to_owned())),
            "expected Vienna's 1st district as the closest area"
        );
    }
}
