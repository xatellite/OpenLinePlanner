mod admin_area;
mod overpass;

use actix_web::{web, Responder, Scope};
use geo::Point;

use crate::error::OLPError;

use self::admin_area::find_admin_boundaries_for_point;

pub use self::admin_area::AdminArea;

/// Defining /osm endpoint for arcix-web router
pub fn osm() -> Scope {
    web::scope("/osm").route("/admin_bounds/{lat}/{lon}", web::get().to(get_admin_bounds))
}

/// Handler for admin_bounds endpoint
async fn get_admin_bounds(coords: web::Path<(f64, f64)>) -> impl Responder {
    let (lat, lon) = coords.into_inner();
    let admin_areas = find_admin_boundaries_for_point(Point::new(lon, lat)).await;
    admin_areas.map_err(|err| OLPError::GenericError(err.to_string()))
}
