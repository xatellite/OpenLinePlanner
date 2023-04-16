use std::{collections::HashMap, fmt::Display, fs, path::PathBuf, sync::RwLock, thread};

use actix_web::{
    body::BoxBody,
    http::header::ContentType,
    web::{self, Data, Json},
    HttpResponse, Responder, Scope,
};

use config::Config;
use geo::{
    point, BooleanOps, Centroid, Contains, HaversineDistance, LineString, MultiPolygon, Point,
    Polygon,
};
use geojson::{
    de::deserialize_geometry,
    ser::{serialize_geometry, to_feature_collection_string},
    Feature,
};
use osmpbfreader::NodeId;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

mod loading;
mod merge;
pub mod streetgraph;
pub use loading::osm;
pub use merge::*;
use uuid::Uuid;

use self::{loading::AdminArea, streetgraph::Streets};
use crate::{error::OLPError, persistence::save_layers};
use openhousepopulator::{Building, Buildings, GenericGeometry};

pub fn layers() -> Scope {
    web::scope("/layer")
        .app_data(Data::new(Layers::new()))
        .route("/center", web::get().to(find_center))
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

pub async fn find_center(layers: web::Data<RwLock<Layers>>) -> Result<Json<Point<f64>>, OLPError> {
    let point = layers
        .read()
        .map_err(OLPError::from_error)?
        .all_merged()
        .bbox
        .centroid()
        .unwrap_or(point!((16.37159586889, 48.20750456628)));
    Ok(Json(point))
}

pub async fn summarize_layers(
    layers: web::Data<RwLock<Layers>>,
) -> Result<Json<Vec<Value>>, OLPError> {
    log::info!("getting layer summary {:?}", thread::current().id());
    let layer_summary = layers
        .read()
        .map_err(OLPError::from_error)?
        .0
        .iter()
        .map(|(_, layer)| layer.serialize_info())
        .collect::<Vec<_>>();
    Ok(Json(layer_summary))
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Layers(HashMap<Uuid, Layer>);

impl Layers {
    pub fn by_type(&self, layer_type: LayerType) -> Layer {
        let centroids: Vec<PopulatedCentroid> = self
            .0
            .iter()
            .filter(|(_, layer)| layer.layer_type == layer_type)
            .flat_map(|(_, layer)| layer.centroids.clone())
            .collect();
        let bbox = self
            .0
            .iter()
            .filter(|(_, layer)| layer.layer_type == layer_type)
            .map(|(_, layer)| layer.bbox.clone())
            .reduce(|acc, bbox| acc.union(&bbox))
            .unwrap_or(MultiPolygon::new(vec![Polygon::new(
                LineString::from(vec![(0., 0.)]),
                vec![],
            )]));
        Layer {
            id: Uuid::nil(),
            bbox,
            centroids,
            layer_type,
            layer_name: layer_type.to_string(),
        }
    }

    pub fn all_merged_by_type(&self) -> Vec<Layer> {
        let mut layers_map: HashMap<LayerType, Layer> = HashMap::new();
        for (_, layer) in &self.0 {
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
                id: Uuid::nil(),
                bbox: MultiPolygon::new(vec![]),
                centroids: Vec::new(),
                layer_type: LayerType::Residential,
                layer_name: "Residential".to_string(),
            };
        }
        let centroids: Vec<PopulatedCentroid> = self
            .0
            .iter()
            .flat_map(|(_, layer)| layer.centroids.clone())
            .collect();
        let bbox = self
            .0
            .iter()
            .map(|(_, layer)| layer.bbox.clone())
            .reduce(|acc, bbox| acc.union(&bbox))
            .unwrap();

        Layer {
            id: Uuid::nil(),
            bbox,
            centroids,
            layer_type: LayerType::Residential,
            layer_name: "Residential".to_string(),
        }
    }

    pub fn contains_key(&self, key: &Uuid) -> bool {
        self.0.contains_key(key)
    }

    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn push(&mut self, layer: Layer) {
        self.0.insert(layer.id, layer);
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Layer {
    id: Uuid,
    bbox: MultiPolygon,
    centroids: Vec<PopulatedCentroid>,
    layer_type: LayerType,
    layer_name: String,
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
            "layer_type": self.layer_type,
            "name": self.layer_name,
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

#[derive(PartialEq, Debug, Serialize, Deserialize, Eq, Hash, Clone, Copy)]
pub enum LayerType {
    #[serde(alias = "residential")]
    Residential,
    #[serde(alias = "workplace")]
    Workplace,
    #[serde(alias = "shopping")]
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

#[derive(Serialize, Deserialize, Hash, Debug)]
struct Answer {
    name: String,
    value: u64,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
enum AnswerValue {
    IntAnswer(u64),
    BoolAnswer(bool),
}

#[derive(Deserialize)]
struct CalculateLayerRequest {
    name: String,
    area: Feature,
    layer_type: LayerType,
    method: String,
    answers: Vec<Answer>,
}

impl CalculateLayerRequest {
    fn admin_area(&self) -> Result<AdminArea, OLPError> {
        self.area.clone().try_into()
    }
}

async fn calculate_new_layer(
    request: web::Json<CalculateLayerRequest>,
    layers: web::Data<RwLock<Layers>>,
    buildings: web::Data<Buildings>,
    streets: web::Data<Streets>,
    config: web::Data<Config>,
) -> Result<Json<Uuid>, OLPError> {
    let request = request.into_inner();
    let admin_area = request.admin_area()?;
    let layer_type = request.layer_type;
    let method = request.method;
    let answers = request.answers;
    let layer_name = request.name;

    let new_layer_id = uuid::Uuid::new_v5(
        &uuid::Uuid::NAMESPACE_URL,
        format!("{}{}{}{:?}", admin_area.id, layer_type, method, answers).as_bytes(),
    );

    if layers
        .read()
        .map_err(OLPError::from_error)?
        .contains_key(&new_layer_id)
    {
        log::info!("layer {} is already calculated, reusing", new_layer_id);
        return Ok(Json(new_layer_id));
    }

    let mut filtered_buildings: Buildings = buildings
        .iter()
        .filter(|building| match &building.geometry {
            GenericGeometry::GenericPoint(point) => admin_area.geometry.contains(point),
            GenericGeometry::GenericPolygon(polygon) => admin_area.geometry.contains(polygon),
        })
        .cloned()
        .collect();

    if let Some(answer) = answers.get(0).map(|ans| ans.value) {
        filtered_buildings
            .distribute_population(answer, &openhousepopulator::Config::builder().build());
    } else {
        filtered_buildings.estimate_population();
    }

    let mut centroids: Vec<PopulatedCentroid> = filtered_buildings
        .into_iter()
        .filter_map(|building| building.try_into().ok())
        .collect();

    let nodes = &streets.nodes;

    for centroid in &mut centroids {
        let closest_street_node = nodes
            .iter()
            .min_by_key(|(_, node)| node.haversine_distance(&centroid.geometry) as u32)
            .map(|(id, _)| id)
            .copied();

        centroid.street_graph_id = closest_street_node;
    }

    layers.write().map_err(OLPError::from_error)?.push(Layer {
        id: new_layer_id.clone(),
        bbox: MultiPolygon::new(vec![admin_area.geometry]),
        centroids,
        layer_type,
        layer_name,
    });

    let mut path = PathBuf::from(config.get_string("cache.dir").unwrap());
    match fs::create_dir_all(&path) {
        Ok(_) => {
            path.push("layers");
            if let Err(e) =
                save_layers(layers.read().as_ref().map_err(OLPError::from_error)?, &path)
            {
                log::error!("failed to save layers to cache: {}", e)
            }
        }
        Err(e) => log::error!("failed to create directory {}: {}", path.display(), e),
    }

    Ok(Json(new_layer_id))
}

async fn get_layer(
    id: web::Path<Uuid>,
    layers: web::Data<RwLock<Layers>>,
) -> Result<Option<Layer>, OLPError> {
    Ok(layers
        .read()
        .map_err(OLPError::from_error)?
        .0
        .get(&id)
        .cloned())
}

async fn delete_layer(
    id: web::Path<Uuid>,
    layers: web::Data<RwLock<Layers>>,
) -> Result<HttpResponse, OLPError> {
    match layers.write().map_err(OLPError::from_error)?.0.remove(&id) {
        Some(_) => Ok(HttpResponse::Ok().finish()),
        None => Ok(HttpResponse::NotFound().finish()),
    }
}
