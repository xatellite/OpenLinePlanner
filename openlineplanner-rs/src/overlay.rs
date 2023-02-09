use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web::Responder;
use geojson::GeoJson;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub enum OverlayName {
    Residence,
    Schools,
    Jobs,
}

pub struct Overlays {
    pub residence: Overlay,
    pub schools: Overlay,
    pub jobs: Overlay,
}

impl Overlays {
    pub fn get_by_name(&self, name: &OverlayName) -> Overlay {
        match name {
            OverlayName::Residence => self.residence.clone(),
            OverlayName::Schools => self.schools.clone(),
            OverlayName::Jobs => self.jobs.clone(),
        }
    }
}

#[derive(Clone)]
pub struct Overlay(GeoJson);

impl Responder for Overlay {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = self.0.to_string();

        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}

static RESIDENCE_PATH: &str = "";

pub fn load_overlay_files() -> Result<Overlays> {
    let residence_file = std::fs::read_to_string(RESIDENCE_PATH)?;
    let residence = Overlay(residence_file.parse::<GeoJson>()?);

    let schools_file = std::fs::read_to_string(RESIDENCE_PATH)?;
    let schools = Overlay(schools_file.parse::<GeoJson>()?);

    let jobs_file = std::fs::read_to_string(RESIDENCE_PATH)?;
    let jobs = Overlay(jobs_file.parse::<GeoJson>()?);

    Ok(Overlays {
        residence,
        schools,
        jobs,
    })
}
