use std::collections::HashMap;

use datatypes::Streets;
use geo::{
    CoordFloat, HaversineDistance, HaversineLength, Line, LineInterpolatePoint, LineString, Point,
};
use osmpbfreader::NodeId;
use petgraph::algo::dijkstra;

use crate::layers::PopulatedCentroid;

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

pub trait DistanceCalculator {
    type FixedPoint: DistanceFromPoint + Sync;
    fn distance(&self, a: &PopulatedCentroid, b: &Point) -> f64;
    fn fix_point(&self, point: &Point) -> Option<Self::FixedPoint>;
}

pub trait DistanceFromPoint {
    fn distance(&self, other: &PopulatedCentroid) -> f64;
}

pub struct HaversineDistanceCalculator;

pub struct HaversineFixedPoint {
    point: Point,
}

impl DistanceFromPoint for HaversineFixedPoint {
    fn distance(&self, other: &PopulatedCentroid) -> f64 {
        self.point.haversine_distance(&other.geometry)
    }
}

impl DistanceCalculator for HaversineDistanceCalculator {
    type FixedPoint = HaversineFixedPoint;

    fn distance(&self, a: &PopulatedCentroid, b: &Point) -> f64 {
        a.haversine_distance(b)
    }
    fn fix_point(&self, point: &Point) -> Option<Self::FixedPoint> {
        Some(HaversineFixedPoint {
            point: point.clone(),
        })
    }
}

impl HaversineDistanceCalculator {
    pub fn new() -> Self {
        Self
    }
}

pub struct OsmDistanceCalculator<'a> {
    streets: &'a Streets,
}

pub struct OsmFixedPoint {
    diff_distance: f64,
    distance_matrix: HashMap<NodeId, f64>,
}

impl DistanceFromPoint for OsmFixedPoint {
    fn distance(&self, other: &PopulatedCentroid) -> f64 {
        self.distance_matrix
            .get(&other.street_graph_id.unwrap())
            .unwrap_or(&f64::MAX)
            + self.diff_distance
    }
}

impl<'a> DistanceCalculator for OsmDistanceCalculator<'a> {
    type FixedPoint = OsmFixedPoint;

    fn distance(&self, a: &PopulatedCentroid, b: &Point) -> f64 {
        let Some((origin_node, diff_distance)) = self.find_closest_node_to_point(b) else {
            return f64::MAX
        };
        let osm_distance_matrix = dijkstra(&self.streets.streetgraph, origin_node, None, |e| *e.2);
        osm_distance_matrix
            .get(&a.street_graph_id.unwrap())
            .unwrap_or(&f64::MAX)
            + diff_distance
    }
    fn fix_point(&self, point: &Point) -> Option<Self::FixedPoint> {
        let Some((origin_node, diff_distance)) = self.find_closest_node_to_point(point) else {
            return None
        };
        let distance_matrix = dijkstra(&self.streets.streetgraph, origin_node, None, |e| *e.2);
        Some(OsmFixedPoint {
            diff_distance,
            distance_matrix,
        })
    }
}

impl<'a> OsmDistanceCalculator<'a> {
    pub fn new(streets: &'a Streets) -> Self {
        Self { streets }
    }

    fn find_closest_node_to_point(&self, origin: &Point) -> Option<(NodeId, f64)> {
        self.streets
            .nodes
            .iter()
            .min_by_key(|(_, node)| node.haversine_distance(&origin) as u32)
            .map(|(id, node)| (id.clone(), node.haversine_distance(&origin)))
    }
}
