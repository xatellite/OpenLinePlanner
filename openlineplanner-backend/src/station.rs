use std::borrow::Borrow;

use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use geo::{HaversineDistance, LineString, Point};
use serde::{Deserialize, Serialize};
use datatypes::Streets;

use crate::{
    coverage::StationCoverageInfo,
    coverage::{get_houses_in_coverage, houses_for_stations, Method, Routing},
    geometry::{DensifyHaversine, OsmDistanceCalculator},
    layers::PopulatedCentroid,
};

static DEFAULT_COVERAGE: f64 = 300f64;

#[derive(Deserialize, PartialEq)]
pub struct Station {
    pub id: String,
    pub location: Point,
    coverage: Option<f64>,
}

impl Station {
    pub fn haversine_distance(&self, rhs: &Station) -> f64 {
        self.location.haversine_distance(&rhs.location)
    }

    pub fn coverage(&self) -> f64 {
        self.coverage.unwrap_or(DEFAULT_COVERAGE)
    }
}

pub fn find_optimal_station(
    line: Vec<Point>,
    coverage: f64,
    houses: &[PopulatedCentroid],
    other_stations: &[Station],
    method: &Method,
    routing: &Routing,
    streets: &Streets,
) -> OptimalStationResult {
    let linestring = Into::<LineString>::into(line.clone()).densify_haversine(10.0);
    let others: Vec<&Station> = other_stations.iter().map(|x| x.borrow()).collect();
    let original_coverage: Vec<&PopulatedCentroid> =
        houses_for_stations(other_stations, houses, method, routing, streets)
            .0
            .values()
            .into_iter()
            .flat_map(|elem| elem.houses.clone())
            .map(|elem| elem.centroid)
            .collect();
    let leftover_houses: Vec<PopulatedCentroid> = houses
        .iter()
        .filter(|house| !original_coverage.contains(house))
        .cloned()
        .collect();
    let location = linestring
        .points()
        .max_by_key(|point| {
            StationCoverageInfo::from_houses_with_method(
                get_houses_in_coverage(
                    &point,
                    coverage,
                    &leftover_houses,
                    OsmDistanceCalculator::new(streets),
                    &others,
                ),
                method,
            )
            .inhabitants
        })
        .expect("could not find ideal station");
    // find the position of the newly found point inside the original line
    let points = linestring.into_points();
    let location_index = points
        .iter()
        .position(|elem| elem == &location)
        .expect("could not find location of new point in line");
    let index = points[0..location_index]
        .iter()
        .filter(|elem| line.contains(elem))
        .count();
    OptimalStationResult { location, index }
}

#[derive(Serialize)]
pub struct OptimalStationResult {
    location: Point,
    index: usize,
}

impl Responder for OptimalStationResult {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(self)
    }
}
