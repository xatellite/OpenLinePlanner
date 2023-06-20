use std::{
    env, fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use admin_area::AdminArea;
use geo::Polygon;
use serde::{Serialize, Serializer};

use anyhow::{anyhow, bail, Result};

mod admin_area;
mod overpass;

fn main() {
    let admin_levels: [u16; 1] = [4]; //, 6, 8, 9, 10];
    for level in admin_levels {
        let mut level_path = env::current_dir().expect("failed to get current directory");
        level_path.push(&level.to_string());
        fs::create_dir_all(&level_path).expect("failed to create directory");
        let files: Vec<PathBuf> = fs::read_dir(env::current_dir().unwrap())
            .unwrap()
            .into_iter()
            .map(|file| file.unwrap().path())
            .filter(|file| fs::metadata(file).unwrap().is_file())
            .collect();
        for file in files {
            split_for_level(&file, level, &level_path).unwrap();
        }
        env::set_current_dir(level_path).unwrap();
    }
}

fn split_for_level(pbf: &Path, admin_level: u16, target_dir: &Path) -> Result<()> {
    let area_id = pbf
        .file_stem()
        .ok_or(anyhow!("failed to get file stem"))?
        .to_string_lossy()
        .parse()?;
    let areas = admin_area::find_admin_boundaries(admin_level, area_id)?;
    println!("Splitting {:?} for areas {:?}", pbf, areas);
    if areas.is_empty() {
        return Ok(())
    }
    let config = generate_osmium_config(pbf, target_dir, areas)?;
    split_pbf(&config, pbf)?;
    Ok(())
}

#[derive(Serialize)]
struct OsmiumExtractConfig {
    extracts: Vec<OsmiumExtract>,
    directory: PathBuf,
}

#[derive(Serialize)]
struct OsmiumExtract {
    output: String,
    #[serde(
        serialize_with = "serialize_polygon_list",
    )]
    polygon: Polygon,
}

fn generate_osmium_config(pbf: &Path, target_dir: &Path, areas: Vec<AdminArea>) -> Result<PathBuf> {
    let config_file_path = PathBuf::from(pbf).with_extension("json");

    let config = OsmiumExtractConfig {
        directory: target_dir.to_owned(),
        extracts: areas
            .into_iter()
            .map(|area| OsmiumExtract {
                output: area.id.to_string() + ".pbf",
                polygon: area.geometry,
            })
            .collect(),
    };

    let mut config_file = fs::File::create(&config_file_path)?;
    let config_string = serde_json::to_string(&config)?;
    config_file.write_all(config_string.as_bytes())?;

    Ok(config_file_path)
}

fn split_pbf(config_file: &Path, pbf: &Path) -> Result<()> {
    if !Command::new("osmium")
        .arg("extract")
        .arg("--overwrite")
        .arg("-c")
        .arg(config_file)
        .arg(pbf)
        .status()
        .expect("failed to spawn child process")
        .success()
    {
        bail!("execution of osmium failed")
    }
    Ok(())
}

fn serialize_polygon_list<S>(polygon: &Polygon, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer{
    let inner_list: Vec<[f64; 2]> = polygon.exterior().coords().map(|coord| [coord.x, coord.y]).collect();
    serializer.collect_seq([inner_list])
}
