use std::sync::{Arc, RwLock};

use actix_web::{web, Scope};
use anyhow::Result;
use coverage::{CoverageMap, Method, Routing};
use geo::Point;
use population::InhabitantsMap;
use serde::Deserialize;
use station::{OptimalStationResult, Station};

use self::coverage::{houses_for_stations, PopulatedCentroidCoverageLayer};
use crate::error::OLPError;
use crate::layers::{LayerType, Layers};

mod coverage;
mod geometry;
mod population;
mod station;

pub fn calculation() -> Scope {
    web::scope("/calculate")
        .route("/station-info", web::post().to(station_info))
        .route("/coverage-info/{router}", web::post().to(coverage_info))
        .route("/find-station", web::post().to(find_station))
}

#[derive(Deserialize)]
pub struct StationInfoRequest {
    stations: Vec<Station>,
    _separation_distance: Option<i32>,
    method: Option<Method>,
    routing: Option<Routing>,
}

pub async fn station_info(
    request: web::Json<StationInfoRequest>,
    layers: web::ReqData<Arc<RwLock<Layers>>>,
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

#[derive(Deserialize)]
pub struct FindStationRequest {
    stations: Vec<Station>,
    route: Vec<Point>,
    method: Option<Method>,
    routing: Option<Routing>,
}

pub async fn find_station(
    request: web::Json<FindStationRequest>,
    layers: web::ReqData<Arc<RwLock<Layers>>>,
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

pub async fn coverage_info(
    stations: web::Json<Vec<Station>>,
    routing: web::Path<Routing>,
    layers: web::ReqData<Arc<RwLock<Layers>>>,
) -> Result<PopulatedCentroidCoverageLayer, OLPError> {
    let layer = layers.read().map_err(OLPError::from_error)?.all_merged();
    let coverage_info = houses_for_stations(
        &stations,
        layer.get_centroids(),
        &Method::Absolute,
        &routing,
        layer.get_streets(),
    );
    Ok(PopulatedCentroidCoverageLayer::from(coverage_info))
}
