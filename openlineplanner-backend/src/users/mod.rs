use std::time::Duration;

use actix_web::{web, Scope};
use jwtk::{jwk::Jwk, HeaderAndClaims};
use uuid::Uuid;

use crate::error::OLPError;

pub fn users(jwk_key: web::Data<Jwk>) -> Scope {
    web::scope("/user")
        .app_data(jwk_key)
        .route("/anon", web::get().to(get_anonymous_user_token))
}

async fn get_anonymous_user_token(jwk_key: web::Data<Jwk>) -> Result<String, OLPError> {
    let mut token = HeaderAndClaims::new_dynamic();
    token
        .set_exp_from_now(Duration::from_secs(86400))
        .set_iss("openlineplanner")
        .set_sub(Uuid::new_v4().to_string())
        .set_kid(jwk_key.kid.as_ref().cloned().unwrap_or_default())
        .insert("name", "Anonymous User");
    jwtk::sign(
        &mut token,
        &jwk_key
            .to_signing_key(jwtk::rsa::RsaAlgorithm::RS256)
            .unwrap(),
    )
    .map_err(OLPError::from_error)
}
