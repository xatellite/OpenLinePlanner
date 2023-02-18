use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web::Responder;
use geo::HaversineDistance;
use geo::Point;

use anyhow::Result;
use geojson::de::deserialize_feature_collection_to_vec;
use geojson::de::deserialize_geometry;
use geojson::ser::serialize_geometry;
use geojson::ser::to_feature_collection_string;
use osmpbfreader::NodeId;
use osmpbfreader::OsmObj;
use petgraph::prelude::UnGraphMap;
use serde::Deserialize;
use serde::Serialize;

use crate::coverage::CoverageMap;

#[derive(Deserialize)]
pub enum DataLayerName {
    #[serde(rename = "residence")]
    Residence,
}

#[derive(Clone)]
pub struct DataLayers {
    pub residence: DataLayer,
    pub streetgraph: UnGraphMap<NodeId, f64>,
    pub nodes: HashMap<NodeId, Point>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct House {
    #[serde(
        serialize_with = "serialize_geometry",
        deserialize_with = "deserialize_geometry"
    )]
    geometry: Point,
    flats: u32,
    pub pop: u32,
    pub street_graph_id: Option<NodeId>,
}

impl House {
    pub fn haversine_distance(&self, rhs: &Point) -> f64 {
        self.geometry.haversine_distance(rhs)
    }
}

#[derive(Serialize)]
pub struct HouseCoverageDataLayer(Vec<HouseCoverage>);

impl From<CoverageMap<'_, '_>> for HouseCoverageDataLayer {
    fn from(value: CoverageMap<'_, '_>) -> Self {
        Self(
            value
                .0
                .into_iter()
                .flat_map(|(station, sci)| {
                    sci.houses.into_iter().map(|hi| HouseCoverage {
                        geometry: hi.house.geometry,
                        data_layer: "dl".to_owned(), // TODO: maybe remove this
                        distance: hi.distance,
                        closest_station: station.to_owned(),
                    })
                })
                .collect(),
        )
    }
}

impl Responder for HouseCoverageDataLayer {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match to_feature_collection_string(&self.0) {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(error) => HttpResponse::InternalServerError()
                .body(format!("failed to get coverage collection: {}", error)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct HouseCoverage {
    #[serde(
        serialize_with = "serialize_geometry",
        deserialize_with = "deserialize_geometry"
    )]
    geometry: Point,
    data_layer: String,
    distance: f64,
    closest_station: String,
}

impl DataLayers {
    pub fn get_by_name(&self, name: &DataLayerName) -> DataLayer {
        match name {
            DataLayerName::Residence => self.residence.clone(),
        }
    }
}

#[derive(Clone)]
pub struct DataLayer(Vec<House>);

impl DataLayer {
    pub fn get_houses(&self) -> &Vec<House> {
        &self.0
    }
}

impl Responder for DataLayer {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        match to_feature_collection_string(&self.0) {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(error) => HttpResponse::InternalServerError()
                .body(format!("failed to get data layers: {}", error)),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct DataFilePaths {
    residence: String,
    osm: String,
}

pub fn load_data_layer_files(paths: DataFilePaths) -> Result<DataLayers> {
    // ToDo Load Datalayer if unchanged
    let residence_file = File::open(paths.residence)?;
    let residence_raw = deserialize_feature_collection_to_vec(residence_file)?;

    let (streetgraph, residence_with_streetconnection, nodes) =
        read_osm_nodes(&PathBuf::from(paths.osm), residence_raw)?;

    let residence = DataLayer(residence_with_streetconnection);

    Ok(DataLayers {
        residence,
        streetgraph,
        nodes,
    })
}

pub fn read_osm_nodes(
    file: &Path,
    mut houses: Vec<House>,
) -> Result<(UnGraphMap<NodeId, f64>, Vec<House>, HashMap<NodeId, Point>)> {
    let r = std::fs::File::open(file)?;
    let mut pbf = osmpbfreader::OsmPbfReader::new(r);

    let osm_nodes = pbf.get_objs_and_deps(|obj| obj.is_node()).unwrap();

    let nodes: HashMap<NodeId, Point> = osm_nodes
        .into_iter()
        .filter_map(|(_, obj)| match obj {
            OsmObj::Node(inner) => Some(inner),
            _ => None,
        })
        .map(|node| {
            (
                node.id,
                Point::new(
                    (node.decimicro_lon as f64 * 1e-7) as f64,
                    (node.decimicro_lat as f64 * 1e-7) as f64,
                ),
            )
        })
        .collect();

    let osm_ways = pbf
        .get_objs_and_deps(|obj| obj.is_way() && obj.tags().contains_key("highway"))
        .unwrap();

    let edges: Vec<(NodeId, NodeId, f64)> = osm_ways
        .into_iter()
        .filter_map(|(_, obj)| match obj {
            OsmObj::Way(inner) => Some(inner),
            _ => None,
        })
        .filter(|way| way.nodes.len() >= 2)
        .map(|way| {
            let mut other = way.nodes.clone().into_iter();
            other.next();
            way.nodes.into_iter().zip(other)
        })
        .flatten()
        .map(|(n1, n2)| {
            (
                n1,
                n2,
                nodes
                    .get(&n1)
                    .unwrap()
                    .haversine_distance(nodes.get(&n2).unwrap()),
            )
        })
        .collect();

    let nodes: HashMap<NodeId, Point> = edges
        .iter()
        .map(|edge| vec![nodes.get_key_value(&edge.0), nodes.get_key_value(&edge.1)])
        .flatten()
        .flatten()
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect();

    for mut house in &mut houses {
        let closest_street_node = nodes
            .iter()
            .min_by_key(|(_, node)| node.haversine_distance(&house.geometry) as u32)
            .map(|(id, _)| id)
            .copied();

        house.street_graph_id = closest_street_node;
    }

    let graph = UnGraphMap::from_edges(edges);

    Ok((graph, houses, nodes))
}
