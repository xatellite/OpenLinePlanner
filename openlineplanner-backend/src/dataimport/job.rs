use std::{
    collections::HashMap,
    sync::RwLock,
    time::{SystemTime, UNIX_EPOCH},
};

use actix_web::web;
use serde::Serialize;
use uuid::Uuid;

/// Registry of all import jobs known to this process. Jobs are intentionally
/// ephemeral: the durable output of an import is the `.map` file, so a restart
/// simply forgets the bookkeeping, not the result.
pub type JobRegistry = RwLock<HashMap<Uuid, ImportJob>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ImportPhase {
    Queued,
    Resolving,
    Downloading,
    Parsing,
    Populating,
    Clipping,
    Saving,
    Done,
    Failed,
}

impl ImportPhase {
    pub fn is_terminal(&self) -> bool {
        matches!(self, ImportPhase::Done | ImportPhase::Failed)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ImportJob {
    pub id: Uuid,
    pub area_id: u64,
    pub area_name: String,
    pub phase: ImportPhase,
    pub downloaded_bytes: u64,
    pub total_bytes: Option<u64>,
    pub extract_name: Option<String>,
    pub error: Option<String>,
    pub started_at: u64,
}

impl ImportJob {
    pub fn new(area_id: u64, area_name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            area_id,
            area_name,
            phase: ImportPhase::Queued,
            downloaded_bytes: 0,
            total_bytes: None,
            extract_name: None,
            error: None,
            started_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or_default(),
        }
    }
}

/// Cloneable handle used by the background task to report progress back into
/// the registry. Every method takes the write lock for as short a time as
/// possible and never panics on a poisoned lock -- losing progress updates must
/// not take the import down.
#[derive(Clone)]
pub struct JobHandle {
    registry: web::Data<JobRegistry>,
    id: Uuid,
}

impl JobHandle {
    pub fn new(registry: web::Data<JobRegistry>, id: Uuid) -> Self {
        Self { registry, id }
    }

    fn update<F: FnOnce(&mut ImportJob)>(&self, apply: F) {
        match self.registry.write() {
            Ok(mut jobs) => {
                if let Some(job) = jobs.get_mut(&self.id) {
                    apply(job);
                }
            }
            Err(err) => log::error!("import job registry lock poisoned: {}", err),
        }
    }

    pub fn set_phase(&self, phase: ImportPhase) {
        log::info!("import job {} entering phase {:?}", self.id, phase);
        self.update(|job| job.phase = phase);
    }

    pub fn set_extract(&self, name: String, total_bytes: Option<u64>) {
        self.update(|job| {
            job.extract_name = Some(name);
            job.total_bytes = total_bytes;
        });
    }

    pub fn set_downloaded(&self, bytes: u64) {
        self.update(|job| job.downloaded_bytes = bytes);
    }

    pub fn finish(&self) {
        self.set_phase(ImportPhase::Done);
    }

    pub fn fail<T: std::fmt::Display>(&self, error: T) {
        let message = error.to_string();
        log::error!("import job {} failed: {}", self.id, message);
        self.update(|job| {
            job.phase = ImportPhase::Failed;
            job.error = Some(message);
        });
    }
}
