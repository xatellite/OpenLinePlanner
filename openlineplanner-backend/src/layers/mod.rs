use std::{collections::HashMap, fmt::Display, sync::RwLock};

use actix_web::{
    body::BoxBody,
    http::header::ContentType,
    web::{self, Data, Json},
    HttpResponse, Responder, Scope,
};

use geo::{BooleanOps, HaversineDistance, LineString, MultiPolygon, Point, Polygon};
use geojson::{
    de::deserialize_geometry,
    ser::{serialize_geometry, to_feature_collection_string},
};
use osmpbfreader::NodeId;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

mod loading;
mod merge;
pub mod osm;
pub use loading::osm;
pub use merge::*;

use self::loading::AdminArea;
use crate::error::OLPError;
use openhousepopulator::{Building, GenericGeometry};

pub fn layers() -> Scope {
    web::scope("/layer")
        .app_data(Data::new(Layers::new()))
        .route("/calculate", web::post().to(calculate_new_layer))
        .route("/methods", web::get().to(get_layer_methods))
        .route(
            "/by_type/{layer_type}",
            web::get().to(get_merged_layers_by_type),
        )
        .route("/{layer_id}", web::get().to(get_layer))
        .route("/{layer_id}", web::delete().to(delete_layer))
        .route("", web::get().to(summarize_layers))
}

pub async fn summarize_layers(
    layers: web::Data<RwLock<Layers>>,
) -> Result<Json<Vec<Value>>, OLPError> {
    Ok(Json(
        layers
            .read()
            .map_err(OLPError::from_error)?
            .0
            .iter()
            .map(|layer| layer.serialize_info())
            .collect::<Vec<_>>(),
    ))
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
                street_graph_id: None,
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
            .unwrap_or(MultiPolygon::new(vec![Polygon::new(
                LineString::from(vec![(0., 0.)]),
                vec![],
            )]));
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
        if self.0.is_empty() {
            // Return empty layer to stay restful
            return Layer {
                id: "all".to_string(),
                bbox: MultiPolygon::new(vec![]),
                centroids: Vec::new(),
                layer_type: LayerType::Residential,
            };
        }
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

    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, layer: Layer) {
        self.0.push(layer)
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

    pub fn serialize_info(&self) -> Value {
        json! ({
            "id": self.id,
            "layer_type": self.layer_type
        })
    }
}

impl Responder for Layer {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match to_feature_collection_string(&self.centroids) {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_error) => HttpResponse::InternalServerError().finish(),
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

async fn get_layer_methods() -> impl Responder {
    // ToDo: Replace this with actual method list
    let tmp_json = json! ([{
        "title": "OpenHousePopulator",
        "description": "Using OSM Data and population count",
        "layer_type": "residential",
        "method": "population",
        "questions": [
            {
            "id": "population",
            "type": "number",
            "text": "Total population in this area",
            },
        ],
    }]);
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(tmp_json)
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
    method: String,
    answers: Vec<Answer>,
}

async fn calculate_new_layer(
    request: web::Json<CalculateLayerRequest>,
    layers: web::Data<RwLock<Layers>>,
) -> Result<HttpResponse, OLPError> {
    let request = request.into_inner();
    let layer_type = request.layer_type;
    let _method = request.method;
    let _answers = request.answers;
    /*let populated_buildings = openhousepopulator::populate_houses(
        &mut pbf_reader,
        &None,
        true,
        &openhousepopulator::Config::builder().build(),
    );*/
    layers.write().map_err(OLPError::from_error)?.push(Layer {
        id: String::new(),
        bbox: MultiPolygon::new(vec![request.area.geometry]),
        centroids: vec![],
        layer_type: layer_type,
    });
    Ok(HttpResponse::Ok().finish())
}

async fn get_layer(
    id: web::Path<String>,
    layers: web::Data<RwLock<Layers>>,
) -> Result<Option<Layer>, OLPError> {
    Ok(layers
        .read()
        .map_err(OLPError::from_error)?
        .0
        .iter()
        .find(|layer| &layer.id == id.as_ref())
        .cloned())
}

async fn delete_layer(id: web::Path<String>, layers: web::Data<RwLock<Layers>>) -> impl Responder {
    let index = layers.read().unwrap()
        .0
        .iter()
        .position(|layer| &layer.id == id.as_ref());

    match index {
        Some(index) => {
            layers.write().unwrap().0.remove(index);
            HttpResponse::Ok().finish()
        }
        None => HttpResponse::NotFound().finish(),
    }
}
