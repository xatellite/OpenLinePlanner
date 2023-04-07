use std::{
    io::Write,
    process::{Command, Stdio},
};

use anyhow::Result;
use geojson::GeoJson;

use crate::error::OLPError;

pub async fn query_overpass(query: String) -> Result<GeoJson> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://overpass-api.de/api/interpreter")
        .body(query)
        .send()
        .await?
        .text()
        .await?;

    let mut child = Command::new("osmtogeojson")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child
        .stdin
        .take()
        .ok_or(OLPError::GenericError("failed to take pipe".to_owned()))?;
    std::thread::spawn(move || {
        stdin
            .write_all(response.as_bytes())
            .expect("failed to write to pipe");
    });

    let output = child.wait_with_output()?;
    let output_data = String::from_utf8_lossy(&output.stdout);
    let geometry = output_data.parse::<GeoJson>()?;

    Ok(geometry)
}
