use std::{
    env,
    ffi::OsStr,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process,
};

use admin_area::AdminArea;
use clap::{Parser, Subcommand};
use geo::{MultiPolygon, Polygon};
use serde::{Serialize, Serializer};

use anyhow::{anyhow, bail, Result};

mod admin_area;
mod overpass;
mod processing;

/// This program can be used to split .pbf files along administrative boundaries
/// and preprocess the resulting files for use with OpenLinePlanner
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Split a file or all files in the current directory along the
    /// administrative boundaries of one or multiple admin_levels
    Split {
        levels: Option<Vec<u16>>,
        #[clap(last(true))]
        file: Option<PathBuf>,
    },
    /// Remove all files with the same name as the directory above
    Deduplicate,
    /// Preprocess all files in the current directory for use with OpenLinePlanner
    Preprocess,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Split { levels, file } => {
            let admin_levels: Vec<u16> = levels.unwrap_or(Vec::from([4, 6, 8, 9, 10]));
            for level in admin_levels {
                let mut level_path = env::current_dir().expect("failed to get current directory");
                level_path.push(&level.to_string());
                fs::create_dir_all(&level_path).expect("failed to create directory");
                if let Some(file) = file {
                    split_for_level(&file, level, &level_path).unwrap();
                    break;
                }
                let files: Vec<PathBuf> = fs::read_dir(env::current_dir().unwrap())
                    .unwrap()
                    .into_iter()
                    .map(|file| file.unwrap().path())
                    .filter(|file| fs::metadata(file).unwrap().is_file())
                    .filter(|file| file.extension().unwrap_or_default() == "pbf")
                    .collect();
                for file in files {
                    split_for_level(&file, level, &level_path).unwrap();
                }
                env::set_current_dir(level_path).unwrap();
            }
        }
        Commands::Deduplicate => {
            let path = env::current_dir().unwrap();
            let mut files_here: Vec<PathBuf> = fs::read_dir(&path)
                .unwrap()
                .into_iter()
                .filter_map(|file| file.ok().and_then(|f| Some(f.path())))
                .filter(|file| file.is_file() && file.extension() == Some(OsStr::new("pbf")))
                .collect();
            let files_above: Vec<String> =
                fs::read_dir(path.parent().expect("directory has to have parent"))
                    .unwrap()
                    .into_iter()
                    .filter_map(|file| file.ok().and_then(|f| Some(f.path())))
                    .filter(|file| file.is_file() && file.extension() == Some(OsStr::new("pbf")))
                    .map(|file| file.file_name().unwrap().to_string_lossy().into_owned())
                    .collect();

            files_here.retain(|elem| {
                files_above.contains(&elem.file_name().unwrap().to_string_lossy().into_owned())
            });

            for file in files_here {
                fs::remove_file(file).expect("failed to delete file");
            }
        }
        Commands::Preprocess => processing::process_data(&env::current_dir().unwrap()),
    }
}

fn split_for_level(pbf: &Path, admin_level: u16, target_dir: &Path) -> Result<()> {
    println!("splitting pbf {:?}", pbf.file_stem().unwrap());
    let area_id = pbf
        .file_stem()
        .ok_or(anyhow!("failed to get file stem"))?
        .to_string_lossy()
        .parse()?;
    println!("getting areas");
    let areas = admin_area::find_admin_boundaries(admin_level, area_id)?;
    println!(
        "Splitting {:?} for areas {:?}",
        pbf,
        areas
            .iter()
            .map(|area| (&area.name, area.id))
            .collect::<Vec<_>>()
    );
    if areas.is_empty() {
        if admin_level != 10 {
            fs::copy(
                pbf,
                target_dir
                    .with_file_name(area_id.to_string())
                    .with_extension("pbf"),
            )?;
        }
        return Ok(());
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
    #[serde(serialize_with = "serialize_polygon_list")]
    polygon: MultiPolygon,
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
    if !process::Command::new("osmium")
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

fn serialize_polygon_list<S>(polygon: &MultiPolygon, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let polygons: Vec<Vec<Vec<[f64; 2]>>> = polygon
        .0
        .iter()
        .map(|polygon| {
            let mut res = Vec::new();
            res.push(
                polygon
                    .exterior()
                    .coords()
                    .map(|coord| [coord.x, coord.y])
                    .collect(),
            );
            res.extend(polygon.interiors().into_iter().map(|linestring| {
                linestring
                    .coords()
                    .map(|coord| [coord.x, coord.y])
                    .collect()
            }));
            res
        })
        .collect();
    serializer.collect_seq(polygons)
}
