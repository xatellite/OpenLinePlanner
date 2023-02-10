use actix_web::{dev::Response, web, App, HttpServer, Responder};
use geo::Point;
use serde::Deserialize;

mod coverage;
mod overlay;
mod population;
mod station;

use coverage::Method;
use overlay::{OverlayName, Overlays};
use station::Station;

#[derive(Deserialize)]
struct StationInfoRequest {
    stations: Vec<Station>,
    separation_distance: Option<i32>,
    method: Option<Method>,
}

#[derive(Deserialize)]
struct FindStationRequest {
    stations: Vec<Station>,
    route: Vec<Point>,
    method: Option<Method>,
}

async fn station_info(
    request: web::Query<StationInfoRequest>,
    overlays: web::Data<Overlays>,
) -> impl Responder {
    let houses = &overlays.residence;
    let coverage_info = coverage::houses_for_stations(
        &request.stations,
        houses.get_houses(),
        &request.method.as_ref().unwrap_or(&Method::Relative),
    );
    population::InhabitantsMap::from(coverage_info)
}

async fn coverage_info(
    stations: web::Query<Vec<Station>>,
    overlays: web::Data<Overlays>,
) -> impl Responder {
    let houses = &overlays.residence;
    let coverage_info =
        coverage::houses_for_stations(&stations, houses.get_houses(), &Method::Relative);
    overlay::HouseCoverageCollection::from(coverage_info)
}

async fn find_station(request: web::Query<FindStationRequest>) -> impl Responder {
    Response::ok()
}

async fn overlay(
    layer_name: web::Path<OverlayName>,
    overlays: web::Data<Overlays>,
) -> impl Responder {
    overlays.get_by_name(layer_name.as_ref())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(
                overlay::load_overlay_files().expect("Failed to read overlay data"),
            ))
            .route("/station-info", web::get().to(station_info))
            .route("/coverage-info", web::get().to(coverage_info))
            .route("/find-station", web::get().to(find_station))
            .route("/overlay/{layer_name}", web::get().to(overlay))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
