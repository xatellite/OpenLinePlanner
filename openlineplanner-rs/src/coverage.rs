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
fn get_houses_in_coverage<'a>(
    station: &Station,
    houses: &'a [House],
    possible_collision_stations: Vec<&Station>,
) -> Vec<HouseInfo<'a>> {
    let origin = station.location;
    let radius = station.coverage();

    houses
        .iter()
        .filter_map(|house| {
            let distance = house.haversine_distance(&origin);
            if distance < radius {
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

pub fn houses_for_stations<'a, 'b>(
    stations: &'a [Station],
    houses: &'b [House],
    method: &Method,
) -> CoverageMap<'a, 'b> {
    let mut inhabitants_map = HashMap::new();

    for station in stations {
        let possible_collision_stations = stations
            .iter()
            .filter(|other| *other != station)
            .filter(|other| {
                other.haversine_distance(station) < (other.coverage() + station.coverage())
            })
            .collect();
        let houses = get_houses_in_coverage(station, houses, possible_collision_stations);
        inhabitants_map.insert(
            station.id.as_str(),
            StationCoverageInfo {
                inhabitants: houses
                    .iter()
                    .map(|hi| match method {
                        Method::Absolute => hi.house.pop,
                        Method::Relative => {
                            (hi.house.pop as f64 * (1f64 / hi.distance.sqrt())) as u32
                        }
                    })
                    .sum(),
                houses,
            },
        );
    }

    CoverageMap(inhabitants_map)
}
