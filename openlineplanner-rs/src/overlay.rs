use std::fs::File;

use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web::Responder;
use geo::HaversineDistance;
use geo::Point;

use anyhow::Result;
use geojson::de::deserialize_feature_collection_to_vec;
use geojson::de::deserialize_geometry;
use geojson::ser::serialize_geometry;
use geojson::ser::to_feature_collection_string;
use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
pub enum OverlayName {
    Residence,
    Schools,
    Jobs,
}

pub struct Overlays {
    pub residence: Overlay,
    pub schools: Overlay,
    pub jobs: Overlay,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct House {
    #[serde(
        serialize_with = "serialize_geometry",
        deserialize_with = "deserialize_geometry"
    )]
    geometry: Point,
    flats: u32,
    housenumbers: u32,
    pop: u32,
}

impl House {
    pub fn haversine_distance(&self, rhs: &Point) -> f64 {
        self.geometry.haversine_distance(rhs)
    }
}

impl Overlays {
    pub fn get_by_name(&self, name: &OverlayName) -> Overlay {
        match name {
            OverlayName::Residence => self.residence.clone(),
            OverlayName::Schools => self.schools.clone(),
            OverlayName::Jobs => self.jobs.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Overlay(Vec<House>);

impl Overlay {
    pub fn get_houses(&self) -> &Vec<House> {
        &self.0
    }
}

impl Responder for Overlay {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match to_feature_collection_string(&self.0) {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(error) => HttpResponse::InternalServerError()
                .body(format!("failed to get overlay: {}", error)),
        }
    }
}

static RESIDENCE_PATH: &str = "../residents.geojson";
static SCHOOLS_PATH: &str = "../residents.geojson";
static JOBS_PATH: &str = "../residents.geojson";

pub fn load_overlay_files() -> Result<Overlays> {
    let residence_file = File::open(RESIDENCE_PATH)?;
    let residence = Overlay(deserialize_feature_collection_to_vec(residence_file)?);

    let schools_file = File::open(SCHOOLS_PATH)?;
    let schools = Overlay(deserialize_feature_collection_to_vec(schools_file)?);

    let jobs_file = File::open(JOBS_PATH)?;
    let jobs = Overlay(deserialize_feature_collection_to_vec(jobs_file)?);

    Ok(Overlays {
        residence,
        schools,
        jobs,
    })
}
