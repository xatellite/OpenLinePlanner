use std::{collections::HashMap, path::Path};

use actix_web::Responder;
use geo::{Point, HaversineDistance};
use geojson::FeatureCollection;
use osmpbfreader::{NodeId, OsmObj};
use petgraph::prelude::UnGraphMap;
use serde::{Serialize, Deserialize};
use tinytemplate::TinyTemplate;

use anyhow::Result;

use crate::layers::PopulatedCentroid;

static OVP_QUERY_TEMPLATE: &'static str = "[out:json][timeout:25];
is_in({}, {}) -> .a;
(
  relation[\"boundary\" = \"administrative\"][\"admin_level\"=\"7\"](pivot.a);
  relation[\"boundary\" = \"administrative\"][\"admin_level\"=\"8\"](pivot.a);
  relation[\"boundary\" = \"administrative\"][\"admin_level\"=\"9\"](pivot.a);
  relation[\"boundary\" = \"administrative\"][\"admin_level\"=\"10\"](pivot.a);
);

out body;
>;
out skel qt;";


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

struct AdminBoundary {

}

#[derive(Serialize)]
struct Context {
    lat: f64,
    lon: f64
}

pub fn render_ovp_query_template(point: Point) -> Result<String> {
    let mut tt = TinyTemplate::new();
    tt.add_template("query", OVP_QUERY_TEMPLATE)?;

    let context = Context {
        lat: point.x(),
        lon: point.y()
    };

    Ok(tt.render("query", &context)?)
}

#[derive(Deserialize)]
struct OverpassResponse {
    version: f32,
    generator: String,
    elements: FeatureCollection,
}

pub async fn find_admin_boundaries_for_point(point: Point) -> Result<Vec<AdminBoundary>> {
    let ovp_query = render_ovp_query_template(point).expect("Failed to render overpass template");

    let client = reqwest::Client::new();
    let ovp_response: OverpassResponse = client.post("https://overpass-api.de/api/interpreter").body(ovp_query).send().await?.json().await?;


    Ok(vec![])
}