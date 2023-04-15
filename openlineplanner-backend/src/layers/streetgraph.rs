use std::collections::HashMap;

use geo::{HaversineDistance, Point};
use osmpbfreader::{NodeId, OsmObj, OsmPbfReader};
use petgraph::prelude::UnGraphMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Streets {
    pub nodes: HashMap<NodeId, Point>,
    #[serde(
        serialize_with = "Streets::serialize_streetgraph",
        deserialize_with = "Streets::deserialize_streetgraph"
    )]
    pub streetgraph: UnGraphMap<NodeId, f64>,
}

impl Streets {
    fn serialize_streetgraph<S>(
        graph: &UnGraphMap<NodeId, f64>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let edges: Vec<(NodeId, NodeId, f64)> = graph
            .all_edges()
            .into_iter()
            .map(|edge| (edge.0, edge.1, *edge.2))
            .collect();

        Serialize::serialize(&edges, serializer)
    }

    fn deserialize_streetgraph<'de, D>(deserializer: D) -> Result<UnGraphMap<NodeId, f64>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let edges: Vec<(NodeId, NodeId, f64)> = Deserialize::deserialize(deserializer)?;

        let streetgraph = UnGraphMap::from_edges(edges);
        Ok(streetgraph)
    }
}

/// Generates a street graph for a given area and maps new houses
pub fn generate_streetgraph<T: std::io::Read + std::io::Seek>(
    pbf: &mut OsmPbfReader<T>,
) -> Streets {
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

    let streetgraph = UnGraphMap::from_edges(edges);

    Streets { streetgraph, nodes }
}
