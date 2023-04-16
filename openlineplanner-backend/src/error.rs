use std::{error::Error, fmt::Display};

use actix_web::{body::BoxBody, HttpResponse, Responder, ResponseError};

#[derive(Debug)]
pub enum OLPError {
    GeometryError,
    GenericError(String),
}

impl OLPError {
    pub fn from_error<T: Display>(error: T) -> Self {
        Self::GenericError(error.to_string())
    }
}

impl Error for OLPError {}

impl Display for OLPError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OLPError::GeometryError => write!(f, "an error occurred when converting geometries"),
            OLPError::GenericError(err) => write!(f, "{}", err.to_string()),
        }
    }
}

impl Responder for OLPError {
    type Body = BoxBody;

    fn respond_to(self, req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        log::error!(
            "failed to process {} query to {} with {}",
            req.method(),
            req.path(),
            self
        );
        HttpResponse::InternalServerError().body(self.to_string())
    }
}

impl ResponseError for OLPError {
    fn status_code(&self) -> reqwest::StatusCode {
        reqwest::StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        log::error!("failed to process query: {}", self);
        HttpResponse::new(self.status_code()).set_body(BoxBody::new(self.to_string()))
    }
}
