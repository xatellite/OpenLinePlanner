use std::borrow::Borrow;

use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use geo::{Densify, HaversineDistance, LineString, Point};
use serde::Deserialize;

use crate::{coverage::get_houses_in_coverage, coverage::StationCoverageInfo, datalayer::House};

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
    houses: &[House],
    other_stations: &[Station],
) -> OptimalStationResult {
    let linestring = Into::<LineString>::into(line).densify(10f64);
    let others: Vec<&Station> = other_stations.iter().map(|x| x.borrow()).collect();
    OptimalStationResult(linestring.points().max_by_key(|point| {
        // TODO is max_by_key use like this efficient?
        StationCoverageInfo::from(get_houses_in_coverage(&point, coverage, houses, &others))
            .inhabitants
    }))
}

pub struct OptimalStationResult(Option<Point>);

impl Responder for OptimalStationResult {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(self.0)
    }
}
