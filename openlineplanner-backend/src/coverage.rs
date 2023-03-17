use geo::Point;
use osmpbfreader::NodeId;
use petgraph::prelude::UnGraphMap;
use serde::Deserialize;
use serde::Serialize;

use crate::datalayer::House;
use crate::geometry::HaversineHouseDistanceCalculator;
use crate::geometry::HouseDistanceCalculator;
use crate::geometry::OsmHouseDistanceCalculator;
use crate::Station;

use std::collections::HashMap;

#[derive(Serialize)]
pub struct CoverageMap<'a, 'b>(pub HashMap<&'a str, StationCoverageInfo<'b>>);

#[derive(Serialize)]
pub struct StationCoverageInfo<'a> {
    pub houses: Vec<HouseInfo<'a>>,
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
    pub fn from_houses_with_method(value: Vec<HouseInfo<'a>>, method: &Method) -> Self {
        StationCoverageInfo {
            inhabitants: value
                .iter()
                .map(|hi| match method {
                    Method::Absolute => hi.house.pop,
                    Method::Relative => (hi.house.pop as f64 * (1f64 / hi.distance.sqrt())) as u32,
                })
                .sum(),
            houses: value,
        }
    }
}

#[derive(Serialize, Clone)]
pub struct HouseInfo<'a> {
    pub house: &'a House,
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
pub fn get_houses_in_coverage<'a, D: HouseDistanceCalculator>(
    origin: &Point,
    coverage: f64,
    houses: &'a [House],
    distance_calculator: D,
    possible_collision_stations: &[&Station],
) -> Vec<HouseInfo<'a>> {
    houses
        .iter()
        .filter_map(|house| {
            let distance = distance_calculator.distance(house, origin);
            if distance < coverage {
                Some(HouseInfo{house, distance})
            } else {
                None
            }
        }) // House is in the radius of our station
        .filter(|hi| {
            possible_collision_stations.iter().all(|other| {
                distance_calculator.distance(hi.house, &other.location) > other.coverage() // House is not in the coverage area of the other station or
                    || distance_calculator.distance(hi.house, &origin) < distance_calculator.distance(hi.house, &other.location)
                // House is closer to the current station
            })
        })
        .collect()
}

pub fn houses_for_stations<'a, 'b>(
    stations: &'a [Station],
    houses: &'b [House],
    method: &Method,
    routing: &Routing,
    nodes: &HashMap<NodeId, Point>,
    streetgraph: &UnGraphMap<NodeId, f64>,
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
                HaversineHouseDistanceCalculator::new(),
                &possible_collision_stations,
            ),
            Routing::Osm => get_houses_in_coverage(
                &station.location,
                station.coverage(),
                houses,
                OsmHouseDistanceCalculator::new(nodes, streetgraph),
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
