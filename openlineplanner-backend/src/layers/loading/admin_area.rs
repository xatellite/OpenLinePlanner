use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use geo::{MultiPolygon, Point, Polygon};
use geojson::{
    feature::Id,
    ser::{serialize_geometry, to_feature_collection_string},
    Feature,
};
use serde::Serialize;

use crate::error::OLPError;

use super::nominatim;

#[derive(Serialize)]
pub struct AdminArea {
    pub name: String,
    pub id: u64,
    pub admin_level: u16,
    #[serde(
        serialize_with = "serialize_geometry",
        deserialize_with = "deserialize_geometry"
    )]
    /// A MultiPolygon rather than a Polygon: OSM boundary relations regularly
    /// convert to multiple rings (exclaves, islands), and those were previously
    /// dropped outright.
    pub geometry: MultiPolygon,
}

impl TryFrom<Feature> for AdminArea {
    type Error = OLPError;

    fn try_from(value: Feature) -> Result<Self, Self::Error> {
        let properties = value.properties.unwrap_or_default();
        let id: u64 = match value.id.unwrap() {
            Id::String(id) => id.split('/').nth(1).unwrap().parse().unwrap(),
            Id::Number(id) => id.as_u64().unwrap(),
        };
        let Some(geometry) = value.geometry.and_then(|geometry| match geometry.value {
            value @ geojson::Value::MultiPolygon(_) => {
                TryInto::<MultiPolygon>::try_into(value).ok()
            }
            value @ geojson::Value::Polygon(_) => TryInto::<Polygon>::try_into(value)
                .ok()
                .map(|polygon| MultiPolygon::new(vec![polygon])),
            _ => None,
        }) else {
            log::info!("Area dropped due to wrong geometry: {:?}", properties);
            return Err(OLPError::GeometryError)
        };
        Ok(AdminArea {
            // Trimmed because most areas carry no name:prefix, and the empty
            // prefix would otherwise leave a leading space in the UI label.
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
            )
            .trim()
            .to_owned(),
            id,
            admin_level: properties
                .get("admin_level")
                .and_then(|id| id.as_str())
                .and_then(|id| id.parse().ok())
                .unwrap_or_default(),
            geometry,
        })
    }
}

pub struct AdminAreas(Vec<AdminArea>);

impl Responder for AdminAreas {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match to_feature_collection_string(&self.0) {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(error) => HttpResponse::InternalServerError()
                .body(format!("failed to convert to geojson: {}", error)),
        }
    }
}

/// Administrative areas containing `point`, smallest first.
pub async fn find_admin_boundaries_for_point(point: Point) -> Result<AdminAreas, OLPError> {
    let features = nominatim::admin_areas_for_point(point).await?;

    Ok(AdminAreas(
        features
            .into_iter()
            .filter_map(|feature| feature.try_into().ok())
            .collect(),
    ))
}
