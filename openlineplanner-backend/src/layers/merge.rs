use actix_web::{web, Responder};

use super::*;

pub async fn get_merged_layers(layers: web::Data<Layers>) -> impl Responder {
    layers.all_merged()
}

pub async fn get_merged_layers_by_type(
    layer_type: web::Path<LayerType>,
    layers: web::Data<Layers>,
) -> impl Responder {
    layers.by_type(layer_type.into_inner())
}
