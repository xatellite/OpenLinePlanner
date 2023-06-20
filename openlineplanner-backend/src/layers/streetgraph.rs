use std::collections::HashMap;

use geo::Point;
use osmpbfreader::{NodeId};
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
    pub fn new() -> Streets {
        Streets {
            streetgraph: UnGraphMap::new(),
            nodes: HashMap::new(),
        }
    }

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