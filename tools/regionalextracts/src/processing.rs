use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use openhousepopulator::Buildings;
use osmpbfreader::OsmPbfReader;

use datatypes::{persistence, Streets};

fn load_buildings<T: std::io::Read + std::io::Seek>(pbf: &mut OsmPbfReader<T>) -> Buildings {
    openhousepopulator::calculate_buildings(
        pbf,
        true,
        &openhousepopulator::Config::builder().build(),
    )
    .unwrap()
}

pub fn process_data(path: &Path) {
    let paths = fs::read_dir(path).unwrap();
    let pbf_files: Vec<PathBuf> = paths
        .into_iter()
        .filter_map(|direntry| direntry.map(|de| de.path()).ok())
        .filter(|path| path.extension().map(|e| e.eq_ignore_ascii_case("pbf")) == Some(true))
        .collect();

    for pbf_file in pbf_files {
        println!("prepocessing file {:?}", pbf_file.file_name());
        let mut path = PathBuf::from("./out");
        fs::create_dir_all(&path).expect("failed to create cache dir");
        path.push(&pbf_file.file_stem().unwrap());
        path.set_extension("map");

        let mut pbf = OsmPbfReader::new(File::open(pbf_file).unwrap());

        let streets = load_streetgraph(&mut pbf);
        let buildings = load_buildings(&mut pbf);

        persistence::save_preprocessed_data(buildings, streets, &path).unwrap();
    }
}

fn load_streetgraph<T: std::io::Read + std::io::Seek>(pbf: &mut OsmPbfReader<T>) -> Streets {
    Streets::from_pbf(pbf)
}
