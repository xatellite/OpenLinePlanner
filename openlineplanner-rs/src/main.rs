use actix_web::{dev::Response, web, App, HttpServer, Responder};
use geo::Point;
use serde::Deserialize;

mod overlay;
mod population;

use overlay::{OverlayName, Overlays};

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

#[derive(Deserialize)]
struct Station {
    id: String,
    location: Point,
    coverage: Option<i32>,
}

#[derive(Deserialize)]
enum Method {
    #[serde(rename = "relative")]
    Relative,
    #[serde(rename = "absolute")]
    Absolute,
}

async fn station_info(request: web::Query<StationInfoRequest>) -> impl Responder {
    Response::ok()
}

async fn coverage_info(stations: web::Query<Vec<Station>>) -> impl Responder {
    Response::ok()
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
    let overlays = overlay::load_overlay_files();

    HttpServer::new(|| {
        App::new()
            .route("/station-info", web::get().to(station_info))
            .route("/coverage-info", web::get().to(coverage_info))
            .route("/find-station", web::get().to(find_station))
            .route("/overlay/{layer_name}", web::get().to(overlay))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
