use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::web;
use actix_web::HttpResponse;
use actix_web::Responder;
use geo::Point;
use geojson::de::deserialize_geometry;
use geojson::ser::serialize_geometry;
use geojson::ser::to_feature_collection_string;
use serde::Deserialize;
use serde::Serialize;

use crate::geometry::HaversinePopulatedCentroidDistanceCalculator;
use crate::geometry::OsmPopulatedCentroidDistanceCalculator;
use crate::geometry::PopulatedCentroidDistanceCalculator;
use crate::layers::Layers;
use crate::layers::PopulatedCentroid;
use crate::Station;
use crate::osm::Streets;

use std::collections::HashMap;

#[derive(Serialize)]
pub struct CoverageMap<'a, 'b>(pub HashMap<&'a str, StationCoverageInfo<'b>>);

#[derive(Serialize)]
pub struct StationCoverageInfo<'a> {
    pub houses: Vec<PopulatedCentroidInfo<'a>>,
    pub inhabitants: u32,
}

#[derive(Deserialize)]
pub enum Routing {
    #[serde(rename = "naive")]
    Naive,
    #[serde(rename = "osm")]
    Osm,
}

impl<'a> StationCoverageInfo<'a> {
    pub fn from_houses_with_method(value: Vec<PopulatedCentroidInfo<'a>>, method: &Method) -> Self {
        StationCoverageInfo {
            inhabitants: value
                .iter()
                .map(|hi| match method {
                    Method::Absolute => hi.centroid.pop,
                    Method::Relative => {
                        (hi.centroid.pop as f64 * (1f64 / hi.distance.sqrt())) as u32
                    }
                })
                .sum(),
            houses: value,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct PopulatedCentroidInfo<'a> {
    pub centroid: &'a PopulatedCentroid,
    pub distance: f64,
}

#[derive(Deserialize)]
pub enum Method {
    #[serde(rename = "relative")]
    Relative,
    #[serde(rename = "absolute")]
    Absolute,
}

/// Gets all houses which are in the coverage area of a station and which are not closer to another station
pub fn get_houses_in_coverage<'a, D: PopulatedCentroidDistanceCalculator>(
    origin: &Point,
    coverage: f64,
    houses: &'a [PopulatedCentroid],
    distance_calculator: D,
    possible_collision_stations: &[&Station],
) -> Vec<PopulatedCentroidInfo<'a>> {
    houses
        .iter()
        .filter_map(|house| {
            let distance = distance_calculator.distance(house, origin);
            if distance < coverage {
                Some(PopulatedCentroidInfo{centroid: house, distance})
            } else {
                None
            }
        }) // PopulatedCentroid is in the radius of our station
        .filter(|hi| {
            possible_collision_stations.iter().all(|other| {
                distance_calculator.distance(hi.centroid, &other.location) > other.coverage() // PopulatedCentroid is not in the coverage area of the other station or
                    || distance_calculator.distance(hi.centroid, &origin) < distance_calculator.distance(hi.centroid, &other.location)
                // PopulatedCentroid is closer to the current station
            })
        })
        .collect()
}

pub fn houses_for_stations<'a, 'b>(
    stations: &'a [Station],
    houses: &'b [PopulatedCentroid],
    method: &Method,
    routing: &Routing,
    streets: &Streets
) -> CoverageMap<'a, 'b> {
    let mut inhabitants_map = HashMap::new();

    for station in stations {
        let possible_collision_stations: Vec<&Station> = stations
            .iter()
            .filter(|other| *other != station)
            .filter(|other| {
                other.haversine_distance(station) < (other.coverage() + station.coverage())
            })
            .collect();
        let houses = match routing {
            Routing::Naive => get_houses_in_coverage(
                &station.location,
                station.coverage(),
                houses,
                HaversinePopulatedCentroidDistanceCalculator::new(),
                &possible_collision_stations,
            ),
            Routing::Osm => get_houses_in_coverage(
                &station.location,
                station.coverage(),
                houses,
                OsmPopulatedCentroidDistanceCalculator::new(streets),
                &possible_collision_stations,
            ),
        };
        inhabitants_map.insert(
            station.id.as_str(),
            StationCoverageInfo::from_houses_with_method(houses, method),
        );
    }

    CoverageMap(inhabitants_map)
}

#[derive(Serialize)]
pub struct PopulatedCentroidCoverageLayer(Vec<PopulatedCentroidCoverage>);

impl From<CoverageMap<'_, '_>> for PopulatedCentroidCoverageLayer {
    fn from(value: CoverageMap<'_, '_>) -> Self {
        Self(
            value
                .0
                .into_iter()
                .flat_map(|(station, sci)| {
                    sci.houses.into_iter().map(|hi| PopulatedCentroidCoverage {
                        geometry: hi.centroid.geometry,
                        data_layer: "dl".to_owned(), // TODO: maybe remove this
                        distance: hi.distance,
                        closest_station: station.to_owned(),
                    })
                })
                .collect(),
        )
    }
}

impl Responder for PopulatedCentroidCoverageLayer {
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
pub struct PopulatedCentroidCoverage {
    #[serde(
        serialize_with = "serialize_geometry",
        deserialize_with = "deserialize_geometry"
    )]
    geometry: Point,
    data_layer: String,
    distance: f64,
    closest_station: String,
}

pub async fn coverage_info(
    stations: web::Json<Vec<Station>>,
    routing: web::Path<Routing>,
    layers: web::Data<Layers>,
    streets: web::Data<Streets>,
) -> impl Responder {
    let layer = layers.all_merged();
    let coverage_info = houses_for_stations(
        &stations,
        layer.get_centroids(),
        &Method::Absolute,
        &routing,
        &streets,
    );
    PopulatedCentroidCoverageLayer::from(coverage_info)
}
