use std::{
    io::Write,
    process::{Command, Stdio},
};

use anyhow::{anyhow, Result};
use geojson::GeoJson;

pub fn query_overpass(query: String) -> Result<GeoJson> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("https://overpass-api.de/api/interpreter")
        .body(query)
        .send()?
        .text()?;

    let mut child = Command::new("osmtogeojson")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to spawn child process");

    let mut stdin = child.stdin.take().ok_or(anyhow!("failed to take pipe"))?;
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
