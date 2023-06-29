use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use anyhow::Result;
use datatypes::Streets;
use openhousepopulator::Buildings;
use serde::{Deserialize, Serialize};

use crate::error::OLPError;
use crate::layers::Layers;

#[derive(Serialize, Deserialize)]
pub(crate) struct PreProcessingData {
    pub buildings: Buildings,
    pub streets: Streets,
}

pub(crate) fn load_preprocessed_data(path: &Path) -> Result<PreProcessingData, OLPError> {
    let mut file = File::open(path).map_err(OLPError::from_error)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data).map_err(OLPError::from_error)?;
    let result = postcard::from_bytes(&data).map_err(OLPError::from_error)?;
    Ok(result)
}

pub(crate) fn save_layers(layers: &Layers, path: &Path) -> Result<(), OLPError> {
    let mut file = File::create(path).map_err(OLPError::from_error)?;
    file.write_all(
        serde_json::to_vec(layers)
            .map_err(OLPError::from_error)?
            .as_slice(),
    )
    .map_err(OLPError::from_error)
}
