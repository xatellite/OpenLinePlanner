use std::fs::File;
use std::path::Path;
use std::sync::RwLock;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use anyhow::Result;
use error::OLPError;
use geo::Point;
use log::info;
use openhousepopulator::Buildings;
use osmpbfreader::OsmPbfReader;
use population::InhabitantsMap;
use serde::Deserialize;

mod coverage;
mod error;
mod geometry;
mod layers;
mod population;
mod station;

use coverage::{CoverageMap, Method, Routing};
use layers::streetgraph::generate_streetgraph;
use layers::streetgraph::Streets;
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
    streets: web::Data<Streets>,
) -> Result<InhabitantsMap, OLPError> {
    let merged_layers = layers
        .read()
        .map_err(OLPError::from_error)?
        .all_merged_by_type();
    let coverage_info: Vec<(LayerType, CoverageMap)> = merged_layers
        .iter()
        .map(|layer| {
            (
                layer.get_type().clone(),
                coverage::houses_for_stations(
                    &request.stations,
                    &layer.get_centroids(),
                    &request.method.as_ref().unwrap_or(&Method::Relative),
                    &request.routing.as_ref().unwrap_or(&Routing::Osm),
                    &streets,
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
    streets: web::Data<Streets>,
) -> Result<OptimalStationResult, OLPError> {
    let layer = layers.read().map_err(OLPError::from_error)?.all_merged();
    Ok(station::find_optimal_station(
        request.route.clone(),
        300f64,
        &layer.get_centroids(),
        &request.stations,
        &request.method.as_ref().unwrap_or(&Method::Relative),
        &request.routing.as_ref().unwrap_or(&Routing::Osm),
        &streets,
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logger().expect("failed to initialize logger");

    info!("starting openlineplanner backend");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://openlineplanner.xatellite.io")
            .allowed_origin("http://localhost:3000")
            .allowed_origin_fn(|origin, _req_head| {
                origin
                    .as_bytes()
                    .ends_with(b".openlineplanner.xatellite.io")
            })
            .allowed_origin_fn(|origin, _req_head| origin.as_bytes().ends_with(b"localhost:3000"))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        let mut pbf = OsmPbfReader::new(File::open(Path::new("./pbf/LHGZ.osm.pbf")).unwrap());

        let streets = web::Data::new(load_streetgraph(&mut pbf));
        let buildings = web::Data::new(load_buildings(&mut pbf));

        log::info!("loading data done");

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(RwLock::new(Layers::new())))
            .app_data(streets)
            .app_data(buildings)
            .route("/station-info", web::post().to(station_info))
            .route(
                "/coverage-info/{router}",
                web::post().to(coverage::coverage_info),
            )
            .route("/find-station", web::post().to(find_station))
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

fn load_buildings<T: std::io::Read + std::io::Seek>(pbf: &mut OsmPbfReader<T>) -> Buildings {
    openhousepopulator::calculate_buildings(
        pbf,
        true,
        &openhousepopulator::Config::builder().build(),
    )
    .unwrap()
}

fn load_streetgraph<T: std::io::Read + std::io::Seek>(pbf: &mut OsmPbfReader<T>) -> Streets {
    generate_streetgraph(pbf)
}
