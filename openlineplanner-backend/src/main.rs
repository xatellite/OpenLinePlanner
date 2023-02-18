use actix_cors::Cors;
use actix_web::{web, http, App, HttpServer, Responder};
use anyhow::Result;
use config::Config;
use geo::Point;
use log::info;
use serde::Deserialize;

mod coverage;
mod datalayer;
mod geometry;
mod population;
mod station;

use coverage::{Method, Routing};
use datalayer::{DataFilePaths, DataLayerName, DataLayers};
use station::Station;

#[derive(Deserialize)]
struct StationInfoRequest {
    stations: Vec<Station>,
    _separation_distance: Option<i32>,
    method: Option<Method>,
    routing: Option<Routing>,
}

#[derive(Deserialize)]
struct FindStationRequest {
    stations: Vec<Station>,
    route: Vec<Point>,
    method: Option<Method>,
    routing: Option<Routing>,
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
        &request.routing.as_ref().unwrap_or(&Routing::Osm),
        &datalayers.nodes,
        &datalayers.streetgraph,
    );
    population::InhabitantsMap::from(coverage_info)
}

async fn coverage_info(
    stations: web::Json<Vec<Station>>,
    routing: web::Path<Routing>,
    datalayers: web::Data<DataLayers>,
) -> impl Responder {
    let houses = &datalayers.residence;
    let coverage_info = coverage::houses_for_stations(
        &stations,
        houses.get_houses(),
        &Method::Absolute,
        &routing,
        &datalayers.nodes,
        &datalayers.streetgraph,
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
        &request.routing.as_ref().unwrap_or(&Routing::Osm),
        &datalayers.nodes,
        &datalayers.streetgraph,
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
    setup_logger().expect("failed to initialize logger");

    info!("starting openlineplanner backend");

    let settings = Config::builder()
        .add_source(config::File::with_name("settings/Settings"))
        .build()
        .expect("failed to read config");
    let data_file_paths: DataFilePaths = settings
        .get("data")
        .expect("missing data file paths in config");

    let data_layer = datalayer::load_data_layer_files(data_file_paths.clone())
        .expect("Failed to read data layer data");

    HttpServer::new(move || {
        let cors = Cors::default()
              .allowed_origin("https://openlineplanner.xatellite.io")
              .allowed_origin_fn(|origin, _req_head| {
                  origin.as_bytes().ends_with(b".openlineplanner.xatellite.io")
              })
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(data_layer.clone()))
            .route("/station-info", web::post().to(station_info))
            .route("/coverage-info/{router}", web::post().to(coverage_info))
            .route("/find-station", web::post().to(find_station))
            .route("/overlay/{layer_name}", web::get().to(overlay))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

fn setup_logger() -> Result<()> {
    let colors = fern::colors::ColoredLevelConfig::new();

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .chain(std::io::stdout())
        .chain(fern::log_file("log/output.log")?)
        .apply()?;
    Ok(())
}
