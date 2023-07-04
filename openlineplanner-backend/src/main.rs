use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use anyhow::Result;
use config::Config;
use log::info;

mod calculation;
mod error;
mod layers;
mod middleware;
mod persistence;
mod users;

use crate::layers::LayersContainer;
use crate::middleware::auth::get_jwks;
use crate::middleware::data::UserDataExtraction;

async fn health() -> &'static str {
    "ok"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logger().expect("failed to initialize logger");

    info!("starting openlineplanner backend");

    let config = web::Data::new(build_config());
    let layers = LayersContainer::new();
    let jwks = get_jwks(&config);
    let jwks_dex = web::Data::new(jwks.0);
    let jwks_local = web::Data::new(jwks.1);
    let jwk_key = web::Data::new(jwks.2);

    log::info!("loading data done");

    HttpServer::new(move || {
        let cors = match cfg!(debug_assertions) {
            true => Cors::permissive(),
            false => Cors::default(),
        };

        let auth = HttpAuthentication::bearer(middleware::auth::validator);
        let data = UserDataExtraction::default();

        App::new()
            .wrap(cors)
            .wrap(Logger::default().exclude("/health"))
            .app_data(layers.clone())
            .app_data(config.clone())
            .app_data(jwks_dex.clone())
            .app_data(jwks_local.clone())
            .route("/health", web::get().to(health))
            .service(calculation::calculation().wrap(data).wrap(auth.clone()))
            .service(layers::layers().wrap(data).wrap(auth.clone()))
            .service(layers::osm().wrap(data).wrap(auth.clone()))
            .service(users::users(jwk_key.clone()))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}

fn setup_logger() -> Result<()> {
    let colors = fern::colors::ColoredLevelConfig::new();
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                record.target(),
                colors.color(record.level()),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;
    Ok(())
}

#[rustfmt::skip]
fn build_config() -> Config {
    Config::builder()
    .set_default("cache.dir", "./cache/").unwrap()
    .set_default("data.dir", "./data/").unwrap()
    .set_default("oidc.issuer", "https://dex.prod.k8s.xatellite.space").unwrap()
    .add_source(config::File::with_name("Config.toml").required(false))
    .build()
    .unwrap()
}
