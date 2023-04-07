use actix_web::web;

use super::*;

pub async fn get_merged_layers_by_type(
    layer_type: web::Path<LayerType>,
    layers: web::Data<RwLock<Layers>>,
) -> Result<Layer, OLPError> {
    Ok(layers
        .read()
        .map_err(OLPError::from_error)?
        .by_type(layer_type.into_inner()))
}
