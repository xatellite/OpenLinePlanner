use geo::{
    CoordFloat, HaversineDistance, HaversineLength, Line, LineInterpolatePoint, LineString, Point,
};

use crate::datalayer::House;
use osmpbfreader::NodeId;
use petgraph::{algo::dijkstra, prelude::UnGraphMap};
use std::collections::HashMap;

pub trait DensifyHaversine<F: CoordFloat> {
    type Output;

    fn densify_haversine(&self, max_distance: F) -> Self::Output;
}

// Helper for densification trait
fn densify_line<T>(line: Line<T>, container: &mut Vec<Point<T>>, max_distance: T)
where
    T: CoordFloat,
    Line<T>: HaversineLength<T>,
{
    assert!(max_distance > T::zero());
    container.push(line.start_point());
    let num_segments = (line.haversine_length() / max_distance)
        .ceil()
        .to_u64()
        .unwrap();
    // distance "unit" for this line segment
    let frac = T::one() / T::from(num_segments).unwrap();
    for segment_idx in 1..num_segments {
        let ratio = frac * T::from(segment_idx).unwrap();
        let interpolated_point = line
            .line_interpolate_point(ratio)
            .expect("ratio should be between 0..1");
        container.push(interpolated_point);
    }
}

impl<T> DensifyHaversine<T> for LineString<T>
where
    T: CoordFloat,
    Line<T>: HaversineLength<T>,
    LineString<T>: HaversineLength<T>,
{
    type Output = LineString<T>;

    fn densify_haversine(&self, max_distance: T) -> Self::Output {
        let mut new_line = vec![];
        self.lines()
            .for_each(|line| densify_line(line, &mut new_line, max_distance));
        // we're done, push the last coordinate on to finish
        new_line.push(self.points().last().unwrap());
        LineString::from(new_line)
    }
}

pub trait HouseDistanceCalculator {
    fn distance(&self, a: &House, b: &Point) -> f64;
}

pub struct HaversineHouseDistanceCalculator;

impl HouseDistanceCalculator for HaversineHouseDistanceCalculator {
    fn distance(&self, a: &House, b: &Point) -> f64 {
        a.haversine_distance(b)
    }
}

impl HaversineHouseDistanceCalculator {
    pub fn new() -> Self {
        Self
    }
}

pub struct OsmHouseDistanceCalculator<'a> {
    nodes: &'a HashMap<NodeId, Point>,
    osm_streetgraph: &'a UnGraphMap<NodeId, f64>,
}

impl<'a> HouseDistanceCalculator for OsmHouseDistanceCalculator<'a> {
    fn distance(&self, a: &House, b: &Point) -> f64 {
        let (origin_node, diff_distance) = self.find_closest_node_to_point(b);
        let osm_distance_matrix = dijkstra(self.osm_streetgraph, origin_node, None, |e| *e.2);
        osm_distance_matrix
            .get(&a.street_graph_id.unwrap())
            .unwrap_or(&f64::MAX)
            + diff_distance
    }
}

impl<'a> OsmHouseDistanceCalculator<'a> {
    pub fn new(
        nodes: &'a HashMap<NodeId, Point>,
        osm_streetgraph: &'a UnGraphMap<NodeId, f64>,
    ) -> Self {
        Self {
            nodes,
            osm_streetgraph,
        }
    }

    fn find_closest_node_to_point(&self, origin: &Point) -> (NodeId, f64) {
        self.nodes
            .iter()
            .min_by_key(|(_, node)| node.haversine_distance(&origin) as u32)
            .map(|(id, node)| (id.clone(), node.haversine_distance(&origin)))
            .unwrap()
    }
}
