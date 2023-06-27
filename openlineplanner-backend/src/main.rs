use std::path::PathBuf;
use std::sync::RwLock;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use anyhow::Result;
use config::Config;
use error::OLPError;
use geo::Point;
use log::info;
use population::InhabitantsMap;
use serde::Deserialize;

mod coverage;
mod error;
mod geometry;
mod layers;
mod persistence;
mod population;
mod station;

use coverage::{CoverageMap, Method, Routing};
use layers::{LayerType, Layers};
use station::{OptimalStationResult, Station};

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
    layers: web::Data<RwLock<Layers>>,
) -> Result<InhabitantsMap, OLPError> {
    let merged_layers = layers
        .read()
        .map_err(OLPError::from_error)?
        .all_merged_by_type();
    let coverage_info: Vec<(LayerType, CoverageMap)> = merged_layers
        .iter()
        .map(|layer| {
            log::debug!("calculating for layer type: {}", layer.get_type());
            (
                layer.get_type().clone(),
                coverage::houses_for_stations(
                    &request.stations,
                    layer.get_centroids(),
                    &request.method.as_ref().unwrap_or(&Method::Relative),
                    &request.routing.as_ref().unwrap_or(&Routing::Osm),
                    layer.get_streets(),
                ),
            )
        })
        .collect();
    let coverage_slice: &[(LayerType, CoverageMap)] = &coverage_info;
    Ok(population::InhabitantsMap::from(coverage_slice))
}

async fn find_station(
    request: web::Json<FindStationRequest>,
    layers: web::Data<RwLock<Layers>>,
) -> Result<OptimalStationResult, OLPError> {
    let layer = layers.read().map_err(OLPError::from_error)?.all_merged();
    Ok(station::find_optimal_station(
        request.route.clone(),
        300f64,
        layer.get_centroids(),
        &request.stations,
        &request.method.as_ref().unwrap_or(&Method::Relative),
        &request.routing.as_ref().unwrap_or(&Routing::Osm),
        layer.get_streets(),
    ))
}

async fn health() -> &'static str {
    "ok"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logger().expect("failed to initialize logger");

    info!("starting openlineplanner backend");

    #[rustfmt::skip]
    let config = Config::builder()
        .set_default("cache.dir", "./cache/").unwrap()
        .set_default("data.dir", "./data/").unwrap()
        .add_source(config::File::with_name("Config.toml").required(false))
        .build()
        .unwrap();

    let layers = load_layers(&config);
    let config = web::Data::new(config);

    log::info!("loading data done");

    HttpServer::new(move || {
        #[cfg(debug_assertions)]
        let cors = Cors::permissive();
        #[cfg(not(debug_assertions))]
        let cors = Cors::default();

        App::new()
            .wrap(cors)
            .app_data(layers.clone())
            .app_data(config.clone())
            .route("/station-info", web::post().to(station_info))
            .route(
                "/coverage-info/{router}",
                web::post().to(coverage::coverage_info),
            )
            .route("/find-station", web::post().to(find_station))
            .route("/health", web::get().to(health))
            .service(layers::layers())
            .service(layers::osm())
    })
    .bind(("0.0.0.0", 8080))?
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
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

fn load_layers(config: &Config) -> web::Data<RwLock<Layers>> {
    let mut path = PathBuf::from(config.get_string("cache.dir").unwrap());
    path.push("layers");
    let layers = persistence::load_layers(&path).unwrap_or_default();
    web::Data::new(RwLock::new(layers))
}
