use std::{fs::File, io::Write, path::Path};

use anyhow::Result;
use openhousepopulator::Buildings;
use serde::{Deserialize, Serialize};
use crate::Streets;

#[derive(Serialize, Deserialize)]
pub struct PreProcessingData {
    pub buildings: Buildings,
    pub streets: Streets,
}

pub fn save_preprocessed_data(
    buildings: Buildings,
    streets: Streets,
    path: &Path,
) -> Result<()> {
    let data = PreProcessingData { buildings, streets };
    let mut file = File::create(path)?;
    file.write_all(postcard::to_allocvec(&data)?.as_slice())?;
    Ok(())
}
