use geo::HaversineDistance;
use geo::Point;
use osmpbfreader::NodeId;
use petgraph::algo::dijkstra;
use petgraph::prelude::UnGraphMap;
use serde::Deserialize;
use serde::Serialize;

use crate::datalayer::House;
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
pub enum Distance {
    Naive,
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

#[derive(Serialize)]
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
pub fn get_houses_in_coverage<'a>(
    origin: &Point,
    coverage: f64,
    houses: &'a [House],
    possible_collision_stations: &[&Station],
) -> Vec<HouseInfo<'a>> {
    houses
        .iter()
        .filter_map(|house| {
            let distance = house.haversine_distance(&origin);
            if distance < coverage {
                Some(HouseInfo{house, distance})
            } else {
                None
            }
        }) // House is in the radius of our station
        .filter(|hi| {
            possible_collision_stations.iter().all(|other| {
                hi.house.haversine_distance(&other.location) > other.coverage() // House is not in the coverage area of the other station or
                    || hi.house.haversine_distance(&origin) < hi.house.haversine_distance(&other.location)
                // House is closer to the current station
            })
        })
        .collect()
}

pub fn get_houses_in_coverage_with_real_distance<'a>(
    origin: &Point,
    coverage: f64,
    houses: &'a [House],
    possible_collision_stations: &[&Station],
    nodes: &HashMap<NodeId, Point>,
    streetgraph: &UnGraphMap<NodeId, f64>,
) -> Vec<HouseInfo<'a>> {
    let (origin_node, diff_distance) = find_closest_node_to_point(nodes, origin);
    let osm_distance_matrix = dijkstra(streetgraph, origin_node, None, |e| *e.2);
    houses
        .iter() //TODO: Restructure that filter_map
        .filter_map(|house| {
            if house.street_graph_id.is_none() || osm_distance_matrix.get(&house.street_graph_id.unwrap()).unwrap_or(&f64::MAX) + diff_distance > coverage {
                None
            } else {
                Some(HouseInfo{house, distance: osm_distance_matrix.get(&house.street_graph_id.unwrap()).unwrap() + diff_distance})
            }
        }) // House is in the radius of our station
        .filter(|hi| {
            possible_collision_stations.iter().all(|other| {
                hi.house.haversine_distance(&other.location) > other.coverage() // House is not in the coverage area of the other station or
                    || hi.house.haversine_distance(&origin) < hi.house.haversine_distance(&other.location)
                // House is closer to the current station
            })
        })
        .collect()
}

fn find_closest_node_to_point(nodes: &HashMap<NodeId, Point>, origin: &Point) -> (NodeId, f64) {
    nodes
        .iter()
        .min_by_key(|(_, node)| node.haversine_distance(&origin) as u32)
        .map(|(id, node)| (id.clone(), node.haversine_distance(&origin)))
        .unwrap()
}

pub fn houses_for_stations<'a, 'b>(
    stations: &'a [Station],
    houses: &'b [House],
    method: &Method,
    distance: &Distance,
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
        let houses = match distance {
            Distance::Naive => get_houses_in_coverage(
                &station.location,
                station.coverage(),
                houses,
                &possible_collision_stations,
            ),
            Distance::Osm => get_houses_in_coverage_with_real_distance(
                &station.location,
                station.coverage(),
                houses,
                &possible_collision_stations,
                nodes,
                streetgraph,
            ),
        };
        inhabitants_map.insert(
            station.id.as_str(),
            StationCoverageInfo::from_houses_with_method(houses, method),
        );
    }

    CoverageMap(inhabitants_map)
}
