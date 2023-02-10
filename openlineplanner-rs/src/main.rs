use actix_web::{dev::Response, web, App, HttpServer, Responder};
use geo::Point;
use serde::Deserialize;

mod coverage;
mod datalayer;
mod population;
mod station;

use coverage::Method;
use datalayer::{DataLayerName, DataLayers};
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
    datalayers: web::Data<DataLayers>,
) -> impl Responder {
    let houses = &datalayers.residence;
    let coverage_info = coverage::houses_for_stations(
        &request.stations,
        houses.get_houses(),
        &request.method.as_ref().unwrap_or(&Method::Relative),
    );
    population::InhabitantsMap::from(coverage_info)
}

async fn coverage_info(
    stations: web::Query<Vec<Station>>,
    datalayers: web::Data<DataLayers>,
) -> impl Responder {
    let houses = &datalayers.residence;
    let coverage_info =
        coverage::houses_for_stations(&stations, houses.get_houses(), &Method::Relative);
    datalayer::HouseCoverageDataLayer::from(coverage_info)
}

async fn find_station(request: web::Query<FindStationRequest>) -> impl Responder {
    Response::ok()
}

async fn overlay(
    layer_name: web::Path<DataLayerName>,
    datalayers: web::Data<DataLayers>,
) -> impl Responder {
    datalayers.get_by_name(layer_name.as_ref())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(
                datalayer::load_data_layer_files().expect("Failed to read data layer data"),
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
