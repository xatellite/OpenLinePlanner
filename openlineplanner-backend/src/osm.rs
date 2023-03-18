use std::{collections::HashMap, path::Path};

use actix_web::Responder;
use geo::{Point, HaversineDistance};
use osmpbfreader::{NodeId, OsmObj};
use petgraph::prelude::UnGraphMap;

use crate::layers::PopulatedCentroid;

async fn get_admin_bounds(lat: f64, lon: f64) -> impl Responder {
    ""
}

pub struct Streets {
    pub nodes: HashMap<NodeId, Point>,
    pub streetgraph: UnGraphMap<NodeId, f64>,
}

pub fn read_osm_nodes(
    file: &Path,
    mut houses: Vec<PopulatedCentroid>,
) ->(UnGraphMap<NodeId, f64>, Vec<PopulatedCentroid>, HashMap<NodeId, Point>) {
    let r = std::fs::File::open(file).unwrap();
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

    (graph, houses, nodes)
}