use std::fmt::Display;

use actix_web::{body::BoxBody, HttpResponse, Responder};

pub enum OLPError {
    OverpassError(anyhow::Error),
    GeometryError
}

impl Display for OLPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "an olp error occured")
    }
}

impl Responder for OLPError {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        HttpResponse::InternalServerError().body(self.to_string())
    }
}
