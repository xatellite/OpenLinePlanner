use std::{
    io::Write,
    process::{Command, Stdio},
    sync::LazyLock,
};

use anyhow::{anyhow, Result};
use geojson::GeoJson;

/// Public Overpass instances go down or start rate-limiting regularly, so the
/// one we query is overridable without a rebuild.
const DEFAULT_OVERPASS_URL: &str = "https://overpass-api.de/api/interpreter";
const OVERPASS_URL_ENV: &str = "OVERPASS_API_URL";

/// Falls back to the default when the variable is unset or blank, so an empty
/// `OVERPASS_API_URL=` in the environment does not produce an unusable URL.
fn resolve_overpass_url(configured: Option<String>) -> String {
    configured
        .filter(|url| !url.trim().is_empty())
        .map(|url| url.trim().to_owned())
        .unwrap_or_else(|| DEFAULT_OVERPASS_URL.to_owned())
}

static OVERPASS_URL: LazyLock<String> = LazyLock::new(|| {
    let url = resolve_overpass_url(std::env::var(OVERPASS_URL_ENV).ok());
    log::info!("querying overpass at {}", url);
    url
});

pub fn query_overpass(query: String) -> Result<GeoJson> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(OVERPASS_URL.as_str())
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
