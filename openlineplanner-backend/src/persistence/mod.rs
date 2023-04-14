use std::{path::Path, fs::File, io::{Write, Read}};
use serde::{Serialize, Deserialize};

use openhousepopulator::Buildings;
use crate::{layers::streetgraph::Streets};

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
