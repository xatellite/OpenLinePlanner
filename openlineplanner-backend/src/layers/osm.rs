use std::{collections::HashMap, path::Path};

use actix_web::{
    web::{self, Json},
    Responder, Scope,
};
use geo::{HaversineDistance, Point, Polygon};
use osmpbfreader::{NodeId, OsmObj};
use petgraph::prelude::UnGraphMap;
use serde::{Deserialize, Serialize};
use tinytemplate::TinyTemplate;

use anyhow::Result;

use super::overpass::{query_overpass, OverpassResponseElement};
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

/// Defining /osm endpoint for arcix-web router
pub fn osm() -> Scope {
    web::scope("osm").route("/admin_bounds/{lat}/{lon}", web::get().to(get_admin_bounds))
}

/// Handler for admin_bounds endpoint
async fn get_admin_bounds(lat: web::Path<f64>, lon: web::Path<f64>) -> impl Responder {
    let admin_areas =
        find_admin_boundaries_for_point(Point::new(lon.into_inner(), lat.into_inner())).await;
    Json(admin_areas.unwrap())
}

pub struct Streets {
    pub nodes: HashMap<NodeId, Point>,
    pub streetgraph: UnGraphMap<NodeId, f64>,
}

/// Generates a street graph for a given area and maps new houses
pub fn read_osm_nodes(
    file: &Path,
    mut houses: Vec<PopulatedCentroid>,
) -> (
    UnGraphMap<NodeId, f64>,
    Vec<PopulatedCentroid>,
    HashMap<NodeId, Point>,
) {
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

#[derive(Serialize, Deserialize)]
pub struct AdminArea {
    name: String,
    id: u64,
    level: u16,
    pub bounding_box: Vec<f64>,
    pub geometry: Polygon,
}

impl From<OverpassResponseElement> for AdminArea {
    fn from(value: OverpassResponseElement) -> Self {
        Self {
            name: value.tags.get("name").cloned().unwrap_or_default(),
            id: value.id,
            bounding_box: value.bounds.into(),
            level: value
                .tags
                .get("level")
                .and_then(|v| str::parse::<u16>(v).ok())
                .unwrap_or_default(),
            geometry: Polygon::new(
                value
                    .members
                    .into_iter()
                    .filter(|elem| elem.ovp_type == "way")
                    .filter(|elem| elem.role == "outer")
                    .flat_map(|elem| elem.geometry)
                    .map(Into::<Point>::into)
                    .collect(),
                vec![],
            ),
        }
    }
}

#[derive(Serialize)]
struct Context {
    lat: f64,
    lon: f64,
}

pub fn render_ovp_query_template(point: Point) -> Result<String> {
    let mut tt = TinyTemplate::new();
    tt.add_template("query", OVP_QUERY_TEMPLATE)?;

    let context = Context {
        lat: point.x(),
        lon: point.y(),
    };

    Ok(tt.render("query", &context)?)
}

pub async fn find_admin_boundaries_for_point(point: Point) -> Result<Vec<AdminArea>> {
    let ovp_query = render_ovp_query_template(point).expect("Failed to render overpass template");

    let ovp_response = query_overpass(ovp_query).await?;

    Ok(ovp_response
        .elements
        .into_iter()
        .map(Into::<AdminArea>::into)
        .collect())
}
