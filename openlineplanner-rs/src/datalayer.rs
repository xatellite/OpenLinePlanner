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

use crate::coverage::CoverageMap;

#[derive(Deserialize)]
pub enum DataLayerName {
    #[serde(rename = "residence")]
    Residence,
    #[serde(rename = "schools")]
    Schools,
    #[serde(rename = "jobs")]
    Jobs,
}

pub struct DataLayers {
    pub residence: DataLayer,
    pub schools: DataLayer,
    pub jobs: DataLayer,
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
    pub pop: u32,
}

impl House {
    pub fn haversine_distance(&self, rhs: &Point) -> f64 {
        self.geometry.haversine_distance(rhs)
    }
}

#[derive(Serialize)]
pub struct HouseCoverageDataLayer(Vec<HouseCoverage>);

impl From<CoverageMap<'_, '_>> for HouseCoverageDataLayer {
    fn from(value: CoverageMap<'_, '_>) -> Self {
        Self(
            value
                .0
                .into_iter()
                .flat_map(|(station, sci)| {
                    sci.houses.into_iter().map(|hi| HouseCoverage {
                        geometry: hi.house.geometry,
                        data_layer: "dl".to_owned(), // TODO: maybe remove this
                        distance: hi.distance,
                        closest_station: station.to_owned(),
                    })
                })
                .collect(),
        )
    }
}

impl Responder for HouseCoverageDataLayer {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match to_feature_collection_string(&self.0) {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(error) => HttpResponse::InternalServerError()
                .body(format!("failed to get coverage collection: {}", error)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HouseCoverage {
    #[serde(
        serialize_with = "serialize_geometry",
        deserialize_with = "deserialize_geometry"
    )]
    geometry: Point,
    data_layer: String,
    distance: f64,
    closest_station: String,
}

impl DataLayers {
    pub fn get_by_name(&self, name: &DataLayerName) -> DataLayer {
        match name {
            DataLayerName::Residence => self.residence.clone(),
            DataLayerName::Schools => self.schools.clone(),
            DataLayerName::Jobs => self.jobs.clone(),
        }
    }
}

#[derive(Clone)]
pub struct DataLayer(Vec<House>);

impl DataLayer {
    pub fn get_houses(&self) -> &Vec<House> {
        &self.0
    }
}

impl Responder for DataLayer {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match to_feature_collection_string(&self.0) {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(error) => HttpResponse::InternalServerError()
                .body(format!("failed to get data layers: {}", error)),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct DataFilePaths {
    residence: String,
    schools: String,
    jobs: String,
}

pub fn load_data_layer_files(paths: DataFilePaths) -> Result<DataLayers> {
    let residence_file = File::open(paths.residence)?;
    let residence = DataLayer(deserialize_feature_collection_to_vec(residence_file)?);

    let schools_file = File::open(paths.schools)?;
    let schools = DataLayer(deserialize_feature_collection_to_vec(schools_file)?);

    let jobs_file = File::open(paths.jobs)?;
    let jobs = DataLayer(deserialize_feature_collection_to_vec(jobs_file)?);

    Ok(DataLayers {
        residence,
        schools,
        jobs,
    })
}
