use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

use datatypes::Streets;
use geo::{Centroid, Contains, MultiPolygon, Point};
use openhousepopulator::{Buildings, GenericGeometry};
use osmpbfreader::{NodeId, OsmPbfReader};
use petgraph::prelude::UnGraphMap;

use crate::{
    dataimport::job::{ImportPhase, JobHandle},
    error::OLPError,
    layers::AdminArea,
};

/// Streams a pbf to disk, reporting progress as it goes.
pub async fn download_pbf(url: &str, dest: &Path, job: &JobHandle) -> Result<(), OLPError> {
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).map_err(OLPError::from_error)?;
    }

    let mut response = reqwest::Client::new()
        .get(url)
        .send()
        .await
        .and_then(|response| response.error_for_status())
        .map_err(OLPError::from_error)?;

    let total = response.content_length();
    job.set_extract(url.to_owned(), total);
    job.set_phase(ImportPhase::Downloading);

    let file = File::create(dest).map_err(OLPError::from_error)?;
    let mut writer = BufWriter::new(file);
    let mut downloaded: u64 = 0;
    let mut last_reported: u64 = 0;

    while let Some(chunk) = response.chunk().await.map_err(OLPError::from_error)? {
        writer.write_all(&chunk).map_err(OLPError::from_error)?;
        downloaded += chunk.len() as u64;
        // Reporting every chunk would hammer the registry lock for no benefit.
        if downloaded - last_reported > 4 * 1024 * 1024 {
            job.set_downloaded(downloaded);
            last_reported = downloaded;
        }
    }

    writer.flush().map_err(OLPError::from_error)?;
    job.set_downloaded(downloaded);
    log::info!("downloaded {} bytes to {:?}", downloaded, dest);
    Ok(())
}

/// Turns a downloaded pbf into the `.map` artifact the layer pipeline consumes.
///
/// This is the in-process equivalent of `regionalextracts split && preprocess`:
/// the street graph and buildings are read from the extract, then clipped to the
/// admin area outline instead of shelling out to `osmium extract`.
///
/// Blocking and CPU bound -- run it off the request workers.
pub fn preprocess_extract(
    pbf_path: &Path,
    area: &AdminArea,
    out_path: &Path,
    job: &JobHandle,
) -> Result<(), OLPError> {
    job.set_phase(ImportPhase::Parsing);
    let file = File::open(pbf_path).map_err(OLPError::from_error)?;
    let mut pbf = OsmPbfReader::new(file);

    // Both readers rewind internally via get_objs_and_deps, so one reader
    // serves both passes -- same order as the offline tool.
    let streets = Streets::from_pbf(&mut pbf);

    job.set_phase(ImportPhase::Populating);
    let buildings = openhousepopulator::calculate_buildings(
        &mut pbf,
        true,
        &openhousepopulator::Config::builder().build(),
    )
    .map_err(OLPError::from_error)?;

    job.set_phase(ImportPhase::Clipping);
    let buildings = clip_buildings(buildings, &area.geometry);
    let streets = clip_streets(streets, &area.geometry);

    if buildings.clone().into_iter().next().is_none() {
        return Err(OLPError::GenericError(format!(
            "no buildings fall inside {} -- the downloaded extract does not cover this area",
            area.name
        )));
    }

    job.set_phase(ImportPhase::Saving);
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent).map_err(OLPError::from_error)?;
    }

    // Write beside the target and rename in, so a crash can never leave a
    // truncated .map that load_preprocessed_data would choke on later.
    let staging_path = out_path.with_extension("map.partial");
    crate::persistence::save_preprocessed_data(buildings, streets, &staging_path)?;
    fs::rename(&staging_path, out_path).map_err(OLPError::from_error)?;

    log::info!("wrote preprocessed data to {:?}", out_path);
    Ok(())
}

// Contains<Point> is implemented for Polygon, so the members are tested
// individually rather than relying on a MultiPolygon impl.
fn covers(area: &MultiPolygon, point: &Point) -> bool {
    area.0.iter().any(|polygon| polygon.contains(point))
}

fn clip_buildings(buildings: Buildings, area: &MultiPolygon) -> Buildings {
    buildings
        .into_iter()
        .filter(|building| match &building.geometry {
            GenericGeometry::GenericPoint(point) => covers(area, point),
            GenericGeometry::GenericPolygon(polygon) => polygon
                .centroid()
                .map(|centroid| covers(area, &centroid))
                .unwrap_or(false),
        })
        .collect()
}

/// Keeps an edge when *either* endpoint lies inside the area, so streets
/// crossing the boundary stay connected, then rebuilds the node map from the
/// surviving edges only.
fn clip_streets(streets: Streets, area: &MultiPolygon) -> Streets {
    let inside = |id: &NodeId| {
        streets
            .nodes
            .get(id)
            .map(|point: &Point| covers(area, point))
            .unwrap_or(false)
    };

    let edges: Vec<(NodeId, NodeId, f64)> = streets
        .streetgraph
        .all_edges()
        .filter(|(from, to, _)| inside(from) || inside(to))
        .map(|(from, to, weight)| (from, to, *weight))
        .collect();

    let nodes: HashMap<NodeId, Point> = edges
        .iter()
        .flat_map(|(from, to, _)| [from, to])
        .filter_map(|id| streets.nodes.get(id).map(|point| (*id, *point)))
        .collect();

    Streets {
        streetgraph: UnGraphMap::from_edges(edges),
        nodes,
    }
}

/// Path of the artifact for an area, i.e. what `POST /layer/calculate` reads.
pub fn map_path(data_dir: &str, area_id: u64) -> PathBuf {
    let mut path = PathBuf::from(data_dir);
    path.push(area_id.to_string());
    path.set_extension("map");
    path
}
