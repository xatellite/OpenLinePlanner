use actix_web::HttpMessage;
use actix_web::web::Data;
use actix_web::dev::ServiceRequest;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use serde::{Deserialize, Serialize};
use alcoholic_jwt::{JWKS, Validation, validate, token_kid};
use anyhow::anyhow;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
   pub sub: String,
   pub name: String
}

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let config = req.app_data::<Config>().cloned().unwrap_or_default();
    let jwks = req.app_data::<Data<JWKS>>().unwrap();
    match validate_token(credentials.token(), jwks) {
        Ok(claims) => {
            log::debug!("Validated token with claims {:?}", claims);
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(e) => {
            log::error!("Failed to validate token: {}", e);
            Err((AuthenticationError::from(config).into(), req))
        },
    }
}

fn validate_token(token: &str, jwks: &JWKS) -> Result<Claims, anyhow::Error> {
    let validations = vec![
        Validation::Issuer("https://dex.prod.k8s.xatellite.space".into()),
        Validation::Audience("example-app".into()),
        Validation::NotExpired,
        Validation::SubjectPresent,
    ];
    let kid = token_kid(&token)?.ok_or(anyhow!("KID is empty"))?;
    let jwk = jwks.find(&kid).ok_or(anyhow!("KID not found"))?;
    let decoded_token = validate(token, jwk, validations)?;
    Ok(serde_json::from_value(decoded_token.claims)?)
}
