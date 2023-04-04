use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::HttpResponse;
use actix_web::Responder;
use serde::Serialize;

use crate::coverage::CoverageMap;
use crate::layers::LayerType;

use std::collections::HashMap;

#[derive(Serialize)]
pub struct InhabitantsInfo {
    layer_type: LayerType,
    value: u32,
}

#[derive(Serialize)]
pub struct InhabitantsMap(HashMap<String, Vec<InhabitantsInfo>>);

impl From<&[(LayerType, CoverageMap<'_, '_>)]> for InhabitantsMap {
    fn from(value: &[(LayerType, CoverageMap<'_, '_>)]) -> Self {
        let mut map: HashMap<String, Vec<InhabitantsInfo>> = HashMap::new();
        for (layer_type, coverage_map) in value {
            for (station, coverage) in &coverage_map.0 {
                map.entry(station.to_string())
                    .or_default()
                    .push(InhabitantsInfo {
                        layer_type: layer_type.clone(),
                        value: coverage.inhabitants,
                    });
            }
        }
        InhabitantsMap(map)
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
