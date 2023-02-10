use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web::Responder;
use serde::Serialize;

use crate::coverage::CoverageMap;

use std::collections::HashMap;

#[derive(Serialize)]
pub struct InhabitantsInfo {
    ped: u32,
    bike: u32,
    total: u32,
    residential: u32,
    work: u32,
    school: u32,
}

#[derive(Serialize)]
pub struct InhabitantsMap(HashMap<String, InhabitantsInfo>);

impl From<CoverageMap<'_, '_>> for InhabitantsMap {
    fn from(value: CoverageMap<'_, '_>) -> Self {
        InhabitantsMap(
            value
                .0
                .into_iter()
                .map(|(key, coverage)| {
                    (
                        key.to_owned(),
                        InhabitantsInfo {
                            ped: coverage.inhabitants,
                            bike: 0,
                            total: coverage.inhabitants,
                            residential: coverage.inhabitants,
                            work: 0,
                            school: 0,
                        },
                    )
                })
                .collect(),
        )
    }
}

impl Responder for InhabitantsMap {
    type Body = BoxBody;

    fn respond_to(self, _req: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .json(self)
    }
}
