use geo::{CoordFloat, HaversineLength, Line, LineInterpolatePoint, LineString, Point};

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
