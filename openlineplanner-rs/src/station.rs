use geo::{HaversineDistance, Point};
use serde::Deserialize;

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
