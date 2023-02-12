use std::collections::HashMap;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;

use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web::Responder;
use geo::Closest;
use geo::ClosestPoint;
use geo::HaversineDistance;
use geo::HaversineLength;
use geo::Line;
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
    #[serde(rename = "schools")]
    Schools,
    #[serde(rename = "jobs")]
    Jobs,
}

#[derive(Clone)]
pub struct DataLayers {
    pub residence: DataLayer,
    pub schools: DataLayer,
    pub jobs: DataLayer,
    pub streetgraph: UnGraphMap<NodeId, f64>,
    pub nodes: HashMap<NodeId, Point>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct House {
    #[serde(
        serialize_with = "serialize_geometry",
        deserialize_with = "deserialize_geometry"
    )]
    geometry: Point,
    flats: u32,
    housenumbers: u32,
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
            DataLayerName::Schools => self.schools.clone(),
            DataLayerName::Jobs => self.jobs.clone(),
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
    schools: String,
    jobs: String,
    osm: String,
}

pub fn load_data_layer_files(paths: DataFilePaths) -> Result<DataLayers> {
    let residence_file = File::open(paths.residence)?;
    let residence_raw = deserialize_feature_collection_to_vec(residence_file)?;

    let (streetgraph, residence_with_streetconnection, nodes) =
        read_osm_nodes(&PathBuf::from(paths.osm), residence_raw)?;

    let residence = DataLayer(residence_with_streetconnection);

    let schools_file = File::open(paths.schools)?;
    let schools = DataLayer(deserialize_feature_collection_to_vec(schools_file)?);

    let jobs_file = File::open(paths.jobs)?;
    let jobs = DataLayer(deserialize_feature_collection_to_vec(jobs_file)?);

    Ok(DataLayers {
        residence,
        schools,
        jobs,
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

    let mut edges: Vec<(NodeId, NodeId, (Line, f64))> = osm_ways
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
            let line = Line::new(
                nodes.get(&n1).unwrap().to_owned(),
                nodes.get(&n2).unwrap().to_owned(),
            );
            (n1, n2, (line, line.haversine_length()))
        })
        .collect();

    let nodes: HashMap<NodeId, Point> = edges
        .iter()
        .map(|edge| vec![nodes.get_key_value(&edge.0), nodes.get_key_value(&edge.1)])
        .flatten()
        .flatten()
        .map(|(a, b)| (a.clone(), b.clone()))
        .collect();

    println!("edges length: {}", edges.len());

    for mut house in &mut houses {
        let closest_street_point = edges
            .iter()
            .filter_map(|edge| {
                let closest_point = edge.2 .0.closest_point(&house.geometry);
                match closest_point {
                    Closest::Intersection(point) | Closest::SinglePoint(point) => Some((
                        point.haversine_distance(&house.geometry),
                        point,
                        edge.clone(),
                    )),
                    _ => None,
                }
            })
            .min_by_key(|(distance, _, _)| *distance as u32);

        if let Some((distance, point_n, old_edge)) = closest_street_point {
            let (node_a, node_b, _) = old_edge;
            let index = edges.iter().position(|elem| elem == &old_edge).unwrap();
            edges.remove(index);
            let node_n = get_new_node_id(&nodes);
            if nodes.get(&node_a).is_none() {
                continue;
            };
            let line_a_n = Line::new(nodes.get(&node_a).unwrap().to_owned(), point_n);
            edges.push((node_a, node_n, (line_a_n, line_a_n.haversine_length())));
            if nodes.get(&node_b).is_none() {
                continue;
            };
            let line_n_b = Line::new(point_n, nodes.get(&node_b).unwrap().to_owned());
            edges.push((node_n, node_b, (line_n_b, line_n_b.haversine_length())));
            let node_h = get_new_node_id(&nodes);
            let line_h_n = Line::new(house.geometry, point_n);
            edges.push((node_h, node_n, (line_h_n, distance)));

            house.street_graph_id = Some(node_h);
        }
    }

    let edges: Vec<(NodeId, NodeId, f64)> = edges
        .into_iter()
        .map(|(a, b, (_, distance))| (a, b, distance))
        .collect();

    println!("edges length: {}", edges.len());

    let graph = UnGraphMap::from_edges(edges);

    Ok((graph, houses, nodes))
}

fn get_new_node_id(nodes: &HashMap<NodeId, Point>) -> NodeId {
    let mut random_id = NodeId(rand::random());
    while nodes.contains_key(&random_id) {
        random_id = NodeId(rand::random());
    }
    random_id
}
