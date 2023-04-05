use std::{collections::HashMap, fmt::Display};

use actix_web::{body::BoxBody, http::header::ContentType, web, HttpResponse, Responder, Scope};

use geo::{BooleanOps, HaversineDistance, MultiPolygon, Point};
use geojson::{
    de::deserialize_geometry,
    ser::{serialize_geometry, to_feature_collection_string},
};
use osmgraphing::multi_ch_constructor::build;
use osmpbfreader::{NodeId, OsmId};
use serde::{Deserialize, Serialize};

mod merge;
pub mod osm;
mod overpass;
pub mod protomaps;
pub use merge::*;

use self::osm::AdminArea;
use crate::error::OLPError;
use openhousepopulator::{Building, GenericGeometry};

pub fn layers() -> Scope {
    web::scope("layer")
        .route("/{layer_id}", web::get().to(get_layer))
        .route("/calculate", web::post().to(calculate_new_layer))
        .route("/types", web::get().to(get_layer_types))
        .route(
            "/by_type/{layer_type}",
            web::get().to(get_merged_layers_by_type),
        )
        .route("", web::get().to(get_merged_layers))
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct PopulatedCentroid {
    #[serde(
        serialize_with = "serialize_geometry",
        deserialize_with = "deserialize_geometry"
    )]
    pub geometry: Point,
    flats: u32,
    pub pop: u32,
    pub street_graph_id: Option<NodeId>,
}

impl TryFrom<Building> for PopulatedCentroid {
    type Error = OLPError;
    fn try_from(value: Building) -> Result<Self, Self::Error> {
        if let GenericGeometry::GenericPoint(point) = value.geometry {
            return Ok(Self {
                geometry: point,
                flats: value.flats as u32,
                pop: value.pop as u32,
                street_graph_id: None
            });
        }
        return Err(OLPError::GeometryError);
    }
}


impl PopulatedCentroid {
    pub fn haversine_distance(&self, rhs: &Point) -> f64 {
        self.geometry.haversine_distance(rhs)
    }
}

pub struct Layers(Vec<Layer>);

impl Layers {
    pub fn by_type(&self, layer_type: LayerType) -> Layer {
        let centroids: Vec<PopulatedCentroid> = self
            .0
            .iter()
            .filter(|layer| layer.layer_type == layer_type)
            .flat_map(|layer| layer.centroids.clone())
            .collect();
        let bbox = self
            .0
            .iter()
            .filter(|layer| layer.layer_type == layer_type)
            .map(|layer| layer.bbox.clone())
            .reduce(|acc, bbox| acc.union(&bbox))
            .unwrap();
        Layer {
            id: layer_type.to_string(),
            bbox,
            centroids,
            layer_type: layer_type,
        }
    }

    pub fn all_merged_by_type(&self) -> Vec<Layer> {
        let mut layers_map: HashMap<LayerType, Layer> = HashMap::new();
        for layer in &self.0 {
            layers_map
                .entry(layer.layer_type.clone())
                .and_modify(|elem| {
                    elem.centroids.append(&mut layer.centroids.clone());
                    elem.bbox.union(&layer.bbox);
                })
                .or_insert(layer.clone());
        }
        layers_map.into_iter().map(|(_, elem)| elem).collect()
    }

    pub fn all_merged(&self) -> Layer {
        let centroids: Vec<PopulatedCentroid> = self
            .0
            .iter()
            .flat_map(|layer| layer.centroids.clone())
            .collect();
        let bbox = self
            .0
            .iter()
            .map(|layer| layer.bbox.clone())
            .reduce(|acc, bbox| acc.union(&bbox))
            .unwrap();

        Layer {
            id: "all".to_string(),
            bbox,
            centroids,
            layer_type: LayerType::Residential,
        }
    }
}

#[derive(Clone)]
pub struct Layer {
    id: String,
    bbox: MultiPolygon,
    centroids: Vec<PopulatedCentroid>,
    layer_type: LayerType,
}

impl Layer {
    pub fn get_centroids(&self) -> &Vec<PopulatedCentroid> {
        &self.centroids
    }
    pub fn get_type(&self) -> &LayerType {
        &self.layer_type
    }
}

impl Responder for Layer {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match to_feature_collection_string(&self.centroids) {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(error) => HttpResponse::InternalServerError()
                .body(format!("failed to get data layers: {}", error)),
        }
    }
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Eq, Hash, Clone)]
pub enum LayerType {
    Residential,
    Workplace,
    Shopping,
}

impl Display for LayerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

async fn get_layer_types() -> impl Responder {
    ""
}

#[derive(Serialize, Deserialize)]
struct Answer {
    name: String,
    value: AnswerValue,
}

#[derive(Serialize, Deserialize)]
enum AnswerValue {
    IntAnswer(u64),
    BoolAnswer(bool),
}

#[derive(Deserialize)]
struct CalculateLayerRequest {
    area: AdminArea,
    layer_type: LayerType,
    answers: Vec<Answer>,
}

async fn calculate_new_layer(
    request: web::Json<CalculateLayerRequest>,
    layers: web::Data<Layers>,
) -> impl Responder {
    let request = request.into_inner();
    let layer_type = request.layer_type;
    let answers = request.answers;
    let pbf_reader = protomaps::download_pbf(request.area).await.unwrap();
    //let populated_buildings = openhousepopulator::populate_houses(&mut pbf_reader, &None, true, &openhousepopulator::Config::builder().build());

    ""
}

async fn get_layer(id: web::Path<String>, layers: web::Data<Layers>) -> impl Responder {
    layers
        .0
        .iter()
        .find(|layer| &layer.id == id.as_ref())
        .cloned()
}
