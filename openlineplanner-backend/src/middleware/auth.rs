use std::fs;
use std::path::PathBuf;
use std::time::Duration;

use actix_web::dev::ServiceRequest;
use actix_web::web::Data;
use actix_web::HttpMessage;
use actix_web_httpauth::extractors::bearer::{self, BearerAuth};
use actix_web_httpauth::extractors::AuthenticationError;
use config::Config;
use jwtk::jwk::{Jwk, JwkSet, JwkSetVerifier, RemoteJwksVerifier};
use jwtk::rsa::{RsaAlgorithm, RsaPrivateKey, RsaPublicKey};
use jwtk::{PrivateKeyToJwk, PublicKeyToJwk};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ExtraClaims {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: Uuid,
    pub name: String,
}

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let config = req
        .app_data::<bearer::Config>()
        .cloned()
        .unwrap_or_default();
    let dex_jwks = req.app_data::<Data<RemoteJwksVerifier>>().unwrap();
    let local_jwks = req.app_data::<Data<JwkSetVerifier>>().unwrap();

    let verified_token = dex_jwks
        .verify(credentials.token())
        .await
        .or_else(|_| local_jwks.verify(credentials.token()));
    match verified_token {
        Ok(token) => {
            let Some(sub) = token.claims().sub.as_ref().and_then(|sub| Uuid::parse_str(sub).ok()) else {
                return Err((AuthenticationError::from(config).into(), req))
            };
            let extra_claims: &ExtraClaims = &token.claims().extra;

            log::debug!(
                "Validated token for user {} with id {:?}",
                extra_claims.name,
                sub
            );
            let claims = Claims {
                sub,
                name: extra_claims.name.clone(),
            };
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(e) => {
            log::error!("Failed to validate token: {}", e);
            Err((AuthenticationError::from(config).into(), req))
        }
    }
}

pub fn get_jwks(config: &Config) -> (RemoteJwksVerifier, JwkSetVerifier, Jwk) {
    let dex = RemoteJwksVerifier::new(
        format!("{}/keys", config.get_string("oidc.issuer").unwrap()),
        None,
        Duration::from_secs(3600),
    );

    let mut local = JwkSet { keys: Vec::new() };
    let mut cache_path = PathBuf::from(config.get_string("cache.dir").unwrap());
    cache_path.push("keys");

    fs::create_dir_all(&cache_path).unwrap();
    for file in fs::read_dir(&cache_path).unwrap() {
        let file = file.unwrap();
        log::info!("reading jwt key {:?}", file.file_name());
        let key_data = fs::read(file.path()).unwrap();
        let mut key = RsaPublicKey::from_pem(&key_data, Some(RsaAlgorithm::RS256))
            .unwrap()
            .public_key_to_jwk()
            .unwrap();
        key.kid = Some(
            file.path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string(),
        );
        local.keys.push(key);
    }

    let current_kid = Uuid::new_v4().to_string();
    let current_key = RsaPrivateKey::generate(2048, jwtk::rsa::RsaAlgorithm::RS256).unwrap();
    cache_path.push(&current_kid);
    cache_path = cache_path.with_extension("pem");
    fs::write(
        cache_path,
        current_key.public_key_to_pem().unwrap().as_bytes(),
    )
    .unwrap();

    let mut current_public = current_key.public_key_to_jwk().unwrap();
    current_public.kid = Some(current_kid.clone());
    let mut current_private = current_key.private_key_to_jwk().unwrap();
    current_private.kid = Some(current_kid);

    local.keys.push(current_public);
    let local = local.verifier();

    (dex, local, current_private)
}
