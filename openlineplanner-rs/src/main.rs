use actix_cors::Cors;
use actix_web::{web, App, HttpServer, Responder};
use config::Config;
use geo::Point;
use serde::Deserialize;

mod coverage;
mod datalayer;
mod population;
mod station;

use coverage::Method;
use datalayer::{DataFilePaths, DataLayerName, DataLayers};
use station::Station;

#[derive(Deserialize)]
struct StationInfoRequest {
    stations: Vec<Station>,
    _separation_distance: Option<i32>,
    method: Option<Method>,
}

#[derive(Deserialize)]
struct CoverageInfoRequest {
    stations: Vec<Station>,
    method: Option<Method>,
}

#[derive(Deserialize)]
struct FindStationRequest {
    stations: Vec<Station>,
    route: Vec<Point>,
    method: Option<Method>,
}

async fn station_info(
    request: web::Json<StationInfoRequest>,
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
    request: web::Json<CoverageInfoRequest>,
    datalayers: web::Data<DataLayers>,
) -> impl Responder {
    let houses = &datalayers.residence;
    let coverage_info = coverage::houses_for_stations(
        &request.stations,
        houses.get_houses(),
        &request.method.as_ref().unwrap_or(&Method::Relative),
    );
    datalayer::HouseCoverageDataLayer::from(coverage_info)
}

async fn find_station(
    request: web::Json<FindStationRequest>,
    datalayers: web::Data<DataLayers>,
) -> impl Responder {
    let houses = &datalayers.residence;
    station::find_optimal_station(
        request.route.clone(),
        300f64,
        &houses.get_houses(),
        &request.stations,
        &request.method.as_ref().unwrap_or(&Method::Relative),
    )
}

async fn overlay(
    layer_name: web::Path<DataLayerName>,
    datalayers: web::Data<DataLayers>,
) -> impl Responder {
    datalayers.get_by_name(layer_name.as_ref())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = Config::builder()
        .add_source(config::File::with_name("Settings"))
        .build()
        .expect("failed to load config file");
    let data_file_paths: DataFilePaths = settings
        .get("data")
        .expect("missing data file paths in config");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .app_data(web::Data::new(
                datalayer::load_data_layer_files(data_file_paths.clone())
                    .expect("Failed to read data layer data"),
            ))
            .route("/station-info", web::post().to(station_info))
            .route("/coverage-info", web::post().to(coverage_info))
            .route("/find-station", web::post().to(find_station))
            .route("/overlay/{layer_name}", web::get().to(overlay))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
