mod catalog;
mod job;
mod preprocess;

use std::{collections::HashMap, fs, path::PathBuf};

use actix_web::{
    web::{self, Data, Json},
    HttpResponse, Scope,
};
use config::Config;
use geojson::Feature;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use catalog::CatalogStore;
pub use job::JobRegistry;

use self::{
    catalog::CatalogEntry,
    job::{ImportJob, ImportPhase, JobHandle},
    preprocess::{download_pbf, map_path, preprocess_extract},
};
use crate::{error::OLPError, layers::AdminArea};

/// Defining /data endpoint for actix-web router
pub fn data() -> Scope {
    web::scope("/data")
        .route("/catalog", web::get().to(get_catalog))
        .route("/regions", web::get().to(get_regions))
        .route("/resolve", web::post().to(resolve_extract))
        .route("/import", web::post().to(start_import))
        .route("/import", web::get().to(list_imports))
        .route("/import/{job_id}", web::get().to(get_import))
        .route("/import/{job_id}", web::delete().to(dismiss_import))
}

fn data_dir(config: &Config) -> String {
    config.get_string("data.dir").unwrap_or("./data/".into())
}

fn download_dir(config: &Config) -> PathBuf {
    PathBuf::from(
        config
            .get_string("data.download_dir")
            .unwrap_or("./data/tmp/".into()),
    )
}

async fn get_catalog(
    catalog: Data<CatalogStore>,
    config: Data<Config>,
) -> Result<Json<Vec<CatalogEntry>>, OLPError> {
    let catalog = catalog.get(&config).await?;
    Ok(Json(catalog.entries))
}

/// Area ids that already have a preprocessed `.map` on disk, so the frontend can
/// skip an import that is not needed.
async fn get_regions(config: Data<Config>) -> Result<Json<Vec<u64>>, OLPError> {
    let dir = data_dir(&config);
    let entries = match fs::read_dir(&dir) {
        Ok(entries) => entries,
        // A fresh install has no data directory yet; that is "nothing imported",
        // not an error.
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(Json(Vec::new())),
        Err(err) => return Err(OLPError::from_error(err)),
    };

    let regions = entries
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| path.extension().map(|ext| ext == "map").unwrap_or(false))
        .filter_map(|path| {
            path.file_stem()
                .and_then(|stem| stem.to_str())
                .and_then(|stem| stem.parse::<u64>().ok())
        })
        .collect();

    Ok(Json(regions))
}

#[derive(Deserialize)]
struct ImportRequest {
    area: Feature,
    /// Optional manual override when automatic extract resolution picks badly.
    catalog_id: Option<String>,
}

#[derive(Serialize)]
struct ResolvedExtract {
    id: String,
    name: String,
    /// Content-Length of the extract, when the server reports one. Lets the
    /// confirm step tell the user how large a download they are starting.
    size_bytes: Option<u64>,
    /// True when this area already has a `.map` and importing is unnecessary.
    already_available: bool,
}

/// Previews which extract would be downloaded for an area, without starting an
/// import.
async fn resolve_extract(
    request: Json<ImportRequest>,
    catalog: Data<CatalogStore>,
    config: Data<Config>,
) -> Result<Json<ResolvedExtract>, OLPError> {
    let request = request.into_inner();
    let area: AdminArea = request.area.try_into()?;
    let already_available = map_path(&data_dir(&config), area.id).exists();

    let catalog = catalog.get(&config).await?;
    let entry = select_entry(&catalog, &area, request.catalog_id.as_deref())?;

    // A HEAD request keeps this cheap; a server that declines it simply leaves
    // the size unknown rather than failing the preview. The Content-Length
    // header is read directly because `content_length()` reports the (empty)
    // HEAD body rather than the size of the eventual download.
    let size_bytes = reqwest::Client::new()
        .head(&entry.pbf_url)
        .send()
        .await
        .ok()
        .filter(|response| response.status().is_success())
        .and_then(|response| {
            response
                .headers()
                .get(reqwest::header::CONTENT_LENGTH)?
                .to_str()
                .ok()?
                .parse::<u64>()
                .ok()
        })
        .filter(|size| *size > 0);

    Ok(Json(ResolvedExtract {
        id: entry.id.clone(),
        name: entry.name.clone(),
        size_bytes,
        already_available,
    }))
}

fn select_entry<'a>(
    catalog: &'a catalog::Catalog,
    area: &AdminArea,
    catalog_id: Option<&str>,
) -> Result<&'a CatalogEntry, OLPError> {
    match catalog_id {
        Some(id) => catalog
            .by_id(id)
            .ok_or_else(|| OLPError::GenericError(format!("unknown catalog region \"{}\"", id))),
        None => catalog.resolve_for(&area.geometry).ok_or_else(|| {
            OLPError::GenericError(format!(
                "no downloadable extract covers {}; pick a region manually",
                area.name
            ))
        }),
    }
}

#[derive(Serialize)]
struct ImportAccepted {
    job: ImportJob,
    /// True when the area already had data and no work was started.
    already_available: bool,
}

async fn start_import(
    request: Json<ImportRequest>,
    jobs: Data<JobRegistry>,
    catalog: Data<CatalogStore>,
    config: Data<Config>,
) -> Result<Json<ImportAccepted>, OLPError> {
    let request = request.into_inner();
    let area: AdminArea = request.area.try_into()?;

    // Already imported: nothing to do.
    if map_path(&data_dir(&config), area.id).exists() {
        let mut job = ImportJob::new(area.id, area.name.clone());
        job.phase = ImportPhase::Done;
        return Ok(Json(ImportAccepted {
            job,
            already_available: true,
        }));
    }

    // Never run two imports for the same area at once -- they would race on the
    // same output path.
    if let Some(running) = jobs
        .read()
        .map_err(OLPError::from_error)?
        .values()
        .find(|job| job.area_id == area.id && !job.phase.is_terminal())
    {
        return Ok(Json(ImportAccepted {
            job: running.clone(),
            already_available: false,
        }));
    }

    // Resolve the extract before accepting the job, so an unsupported area
    // fails immediately with a useful message rather than minutes later.
    let catalog = catalog.get(&config).await?;
    let entry = select_entry(&catalog, &area, request.catalog_id.as_deref())?.clone();

    let job = ImportJob::new(area.id, area.name.clone());
    let job_id = job.id;
    jobs.write()
        .map_err(OLPError::from_error)?
        .insert(job_id, job.clone());

    let handle = JobHandle::new(jobs.clone(), job_id);
    let out_path = map_path(&data_dir(&config), area.id);
    let mut pbf_path = download_dir(&config);
    pbf_path.push(format!("{}.osm.pbf", area.id));
    let keep_download = config.get_bool("data.keep_downloads").unwrap_or(false);

    actix_web::rt::spawn(async move {
        handle.set_phase(ImportPhase::Resolving);
        log::info!(
            "importing {} ({}) from extract {}",
            area.name,
            area.id,
            entry.id
        );

        if let Err(err) = download_pbf(&entry.pbf_url, &pbf_path, &handle).await {
            handle.fail(err);
            let _ = fs::remove_file(&pbf_path);
            return;
        }

        // Parsing and populating are minutes of blocking CPU work; keep them
        // off the request workers.
        let blocking_handle = handle.clone();
        let blocking_pbf = pbf_path.clone();
        let result = actix_web::rt::task::spawn_blocking(move || {
            preprocess_extract(&blocking_pbf, &area, &out_path, &blocking_handle)
        })
        .await;

        if !keep_download {
            if let Err(err) = fs::remove_file(&pbf_path) {
                log::warn!("failed to remove downloaded extract: {}", err);
            }
        }

        match result {
            Ok(Ok(())) => handle.finish(),
            Ok(Err(err)) => handle.fail(err),
            Err(err) => handle.fail(format!("import task did not complete: {}", err)),
        }
    });

    Ok(Json(ImportAccepted {
        job,
        already_available: false,
    }))
}

async fn list_imports(jobs: Data<JobRegistry>) -> Result<Json<Vec<ImportJob>>, OLPError> {
    let jobs = jobs
        .read()
        .map_err(OLPError::from_error)?
        .values()
        .cloned()
        .collect();
    Ok(Json(jobs))
}

async fn get_import(
    id: web::Path<Uuid>,
    jobs: Data<JobRegistry>,
) -> Result<Option<Json<ImportJob>>, OLPError> {
    Ok(jobs
        .read()
        .map_err(OLPError::from_error)?
        .get(&id)
        .cloned()
        .map(Json))
}

async fn dismiss_import(
    id: web::Path<Uuid>,
    jobs: Data<JobRegistry>,
) -> Result<HttpResponse, OLPError> {
    let mut jobs = jobs.write().map_err(OLPError::from_error)?;
    match jobs.get(&id) {
        // A running import owns a download and an output path; dropping its
        // bookkeeping would let a second import race it.
        Some(job) if !job.phase.is_terminal() => Ok(HttpResponse::Conflict()
            .body("import is still running")),
        Some(_) => {
            jobs.remove(&id);
            Ok(HttpResponse::Ok().finish())
        }
        None => Ok(HttpResponse::NotFound().finish()),
    }
}

pub fn new_job_registry() -> Data<JobRegistry> {
    Data::new(JobRegistry::new(HashMap::new()))
}
