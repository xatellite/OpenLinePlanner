use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use crate::{
    error::OLPError,
    layers::{streetgraph::Streets, Layers},
};
use openhousepopulator::Buildings;

use anyhow::Result;

#[derive(Serialize, Deserialize)]
pub(crate) struct PreProcessingData {
    pub buildings: Buildings,
    pub streets: Streets,
}

pub(crate) fn load_preprocessed_data(path: &Path) -> Result<PreProcessingData> {
    let mut file = File::open(path)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data)?;
    let result = postcard::from_bytes(&data)?;
    Ok(result)
}

pub(crate) fn save_preprocessed_data(data: &PreProcessingData, path: &Path) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(postcard::to_allocvec(data)?.as_slice())?;
    Ok(())
}

pub(crate) fn save_layers(layers: &Layers, path: &Path) -> Result<(), OLPError> {
    let mut file = File::create(path).map_err(OLPError::from_error)?;
    file.write_all(
        postcard::to_allocvec(layers)
            .map_err(OLPError::from_error)?
            .as_slice(),
    )
    .map_err(OLPError::from_error)
}

pub(crate) fn load_layers(path: &Path) -> Result<Layers, OLPError> {
    let mut file = File::open(path).map_err(OLPError::from_error)?;
    let mut data: Vec<u8> = Vec::new();
    file.read_to_end(&mut data);
    postcard::from_bytes(&data).map_err(OLPError::from_error)
}
