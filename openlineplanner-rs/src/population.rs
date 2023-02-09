use serde::Serialize;

use crate::overlay::House;
use crate::Station;

use std::collections::HashMap;

#[derive(Serialize)]
pub struct InhabitantsMap<'a, 'b>(pub HashMap<&'a str, Vec<&'b House>>);

/// Gets all houses which are in the coverage area of a station and which are not closer to another station
fn get_houses_in_coverage<'a>(
    station: &Station,
    houses: &'a Vec<House>,
    possible_collision_stations: Vec<&Station>,
) -> Vec<&'a House> {
    let origin = station.location;
    let radius = station.coverage();

    houses
        .iter()
        .filter(|house| house.haversine_distance(&origin) < radius) // House is in the radius of our station
        .filter(|house| {
            possible_collision_stations.iter().all(|other| {
                house.haversine_distance(&other.location) > other.coverage() // House is not in the coverage area of the other station or
                    || house.haversine_distance(&origin) < house.haversine_distance(&other.location)
                // House is closer to the current station
            })
        })
        .collect()
}

pub fn inhabitants_for_stations<'a, 'b>(
    stations: &'a [Station],
    houses: &'b Vec<House>,
) -> InhabitantsMap<'a, 'b> {
    let mut inhabitants_map = HashMap::new();

    for station in stations {
        let possible_collision_stations = stations
            .iter()
            .filter(|other| {
                other.haversine_distance(station) < (other.coverage() + station.coverage())
            })
            .collect();
        let houses = get_houses_in_coverage(station, houses, possible_collision_stations);
        inhabitants_map.insert(station.id.as_str(), houses);
    }

    InhabitantsMap(inhabitants_map)
}
