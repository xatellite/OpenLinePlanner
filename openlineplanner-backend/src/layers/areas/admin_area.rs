use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::{HttpResponse, Responder};
use geo::{Point, Polygon};
use geojson::feature::Id;
use geojson::ser::{serialize_geometry, to_feature_collection_string};
use geojson::{Feature, GeoJson};
use serde::Serialize;
use tinytemplate::TinyTemplate;

use super::overpass::query_overpass;
use crate::error::OLPError;

#[derive(Serialize)]
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
    type Error = OLPError;

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
            return Err(OLPError::GeometryError)
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
is_in({lat}, {lon}) -> .a;
(
  relation[\"boundary\" = \"administrative\"][\"admin_level\"=\"8\"](pivot.a);
  relation[\"boundary\" = \"administrative\"][\"admin_level\"=\"9\"](pivot.a);
);

out geom;";

pub struct AdminAreas(Vec<AdminArea>);

impl TryFrom<GeoJson> for AdminAreas {
    type Error = OLPError;

    fn try_from(value: GeoJson) -> Result<Self, Self::Error> {
        match value {
            GeoJson::FeatureCollection(feature_collection) => Ok(AdminAreas(
                feature_collection
                    .into_iter()
                    .filter_map(|feature| feature.try_into().ok())
                    .collect(),
            )),
            _ => Err(OLPError::GeometryError),
        }
    }
}

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

#[derive(Serialize)]
struct Context {
    lat: f64,
    lon: f64,
}

pub fn render_ovp_query_template(point: Point) -> Result<String, OLPError> {
    let mut tt = TinyTemplate::new();
    tt.add_template("query", OVP_QUERY_TEMPLATE)
        .map_err(OLPError::from_error)?;

    let context = Context {
        lon: point.x(),
        lat: point.y(),
    };

    Ok(tt.render("query", &context).map_err(OLPError::from_error)?)
}

pub async fn find_admin_boundaries_for_point(point: Point) -> Result<AdminAreas, OLPError> {
    let ovp_query = render_ovp_query_template(point)?;

    let ovp_response = query_overpass(ovp_query)
        .await
        .map_err(OLPError::from_error)?;

    Ok(ovp_response.try_into().map_err(OLPError::from_error)?)
}
