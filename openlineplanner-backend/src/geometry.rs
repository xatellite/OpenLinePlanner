use geo::{
    CoordFloat, HaversineDistance, HaversineLength, Line, LineInterpolatePoint, LineString, Point,
};

use crate::{layers::streetgraph::Streets, layers::PopulatedCentroid};
use osmpbfreader::NodeId;
use petgraph::algo::dijkstra;

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

pub trait PopulatedCentroidDistanceCalculator {
    fn distance(&self, a: &PopulatedCentroid, b: &Point) -> f64;
}

pub struct HaversinePopulatedCentroidDistanceCalculator;

impl PopulatedCentroidDistanceCalculator for HaversinePopulatedCentroidDistanceCalculator {
    fn distance(&self, a: &PopulatedCentroid, b: &Point) -> f64 {
        a.haversine_distance(b)
    }
}

impl HaversinePopulatedCentroidDistanceCalculator {
    pub fn new() -> Self {
        Self
    }
}

pub struct OsmPopulatedCentroidDistanceCalculator<'a> {
    streets: &'a Streets,
}

impl<'a> PopulatedCentroidDistanceCalculator for OsmPopulatedCentroidDistanceCalculator<'a> {
    fn distance(&self, a: &PopulatedCentroid, b: &Point) -> f64 {
        let (origin_node, diff_distance) = self.find_closest_node_to_point(b);
        let osm_distance_matrix = dijkstra(&self.streets.streetgraph, origin_node, None, |e| *e.2);
        osm_distance_matrix
            .get(&a.street_graph_id.unwrap())
            .unwrap_or(&f64::MAX)
            + diff_distance
    }
}

impl<'a> OsmPopulatedCentroidDistanceCalculator<'a> {
    pub fn new(streets: &'a Streets) -> Self {
        Self { streets }
    }

    fn find_closest_node_to_point(&self, origin: &Point) -> (NodeId, f64) {
        self.streets
            .nodes
            .iter()
            .min_by_key(|(_, node)| node.haversine_distance(&origin) as u32)
            .map(|(id, node)| (id.clone(), node.haversine_distance(&origin)))
            .unwrap()
    }
}
