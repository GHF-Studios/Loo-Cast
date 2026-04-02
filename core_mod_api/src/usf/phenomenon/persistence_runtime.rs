use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::JoinHandle;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

use crate::bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::usf::authority::{
    USF_DOMAIN_PARTIAL_PHENOMENON_MODEL, USF_DOMAIN_PHENOMENON, USF_DOMAIN_PHENOMENON_MODEL, UsfAuthorityDiagnostics, UsfWorldAuthorityContract,
    guard_canonical_domain_with_diagnostics,
};

use super::components::{
    MonolithicPhenomenonModel, PartialPhenomenonModel, PartitionedPhenomenonModelMember, PartitionedPhenomenonModelRoot, Phenomenon, PhenomenonModel,
    PhenomenonModelProjectionContract, PhenomenonModelScriptDefinitionRef, PhenomenonModelState, PhenomenonModelSupport, PhenomenonScriptDefinitionRef,
};
use super::persistence::{
    PersistedPartialPhenomenonModelRecord, PersistedPhenomenonModelRecord, PersistedPhenomenonRecord, PhenomenonPersistenceDurability,
    model_record_from_runtime, model_record_path, monolithic_model_record_from_runtime, partial_model_record_from_runtime, partial_record_path,
    phenomenon_record_from_runtime, phenomenon_record_path, save_partial_phenomenon_model_record_with_durability, save_phenomenon_model_record_with_durability,
    save_phenomenon_record_with_durability,
};
use super::systems::PhenomenonPersistenceRuntimeSettings;

const PHENOMENON_PERSISTENCE_BATCH_JOURNAL_SCHEMA_VERSION: u16 = 1;
static NEXT_PERSISTENCE_BATCH_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone)]
enum PersistenceWriteRequest {
    Phenomenon {
        path: PathBuf,
        script_id: String,
        record: PersistedPhenomenonRecord,
    },
    Model {
        path: PathBuf,
        script_id: String,
        model_id: String,
        record: PersistedPhenomenonModelRecord,
    },
    Partial {
        path: PathBuf,
        script_id: String,
        model_id: String,
        record: PersistedPartialPhenomenonModelRecord,
    },
}

#[derive(Debug, Clone)]
struct PersistenceWriteResult {
    context: String,
    path: PathBuf,
    error: Option<String>,
}

#[derive(Debug, Clone)]
struct PersistenceWriteBatchResult {
    outcomes: Vec<PersistenceWriteResult>,
    record_count: usize,
    elapsed_ms: f64,
    journal_batch_written: bool,
}

#[derive(Debug, Clone)]
struct PersistenceWriteBatchPolicy {
    durability: PhenomenonPersistenceDurability,
    journal_enabled: bool,
    journal_dir: PathBuf,
    retain_successful_journal_batches: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum PersistedBatchJournalRequest {
    Phenomenon {
        path: String,
        script_id: String,
        record: PersistedPhenomenonRecord,
    },
    Model {
        path: String,
        script_id: String,
        model_id: String,
        record: PersistedPhenomenonModelRecord,
    },
    Partial {
        path: String,
        script_id: String,
        model_id: String,
        record: PersistedPartialPhenomenonModelRecord,
    },
}

impl PersistedBatchJournalRequest {
    fn from_runtime(request: &PersistenceWriteRequest) -> Self {
        match request {
            PersistenceWriteRequest::Phenomenon { path, script_id, record } => Self::Phenomenon {
                path: path.to_string_lossy().to_string(),
                script_id: script_id.clone(),
                record: record.clone(),
            },
            PersistenceWriteRequest::Model {
                path,
                script_id,
                model_id,
                record,
            } => Self::Model {
                path: path.to_string_lossy().to_string(),
                script_id: script_id.clone(),
                model_id: model_id.clone(),
                record: record.clone(),
            },
            PersistenceWriteRequest::Partial {
                path,
                script_id,
                model_id,
                record,
            } => Self::Partial {
                path: path.to_string_lossy().to_string(),
                script_id: script_id.clone(),
                model_id: model_id.clone(),
                record: record.clone(),
            },
        }
    }

    fn into_runtime(self) -> Result<PersistenceWriteRequest, String> {
        match self {
            Self::Phenomenon { path, script_id, record } => {
                let path = parse_non_empty_path(path.as_str())?;
                Ok(PersistenceWriteRequest::Phenomenon { path, script_id, record })
            }
            Self::Model {
                path,
                script_id,
                model_id,
                record,
            } => {
                let path = parse_non_empty_path(path.as_str())?;
                Ok(PersistenceWriteRequest::Model {
                    path,
                    script_id,
                    model_id,
                    record,
                })
            }
            Self::Partial {
                path,
                script_id,
                model_id,
                record,
            } => {
                let path = parse_non_empty_path(path.as_str())?;
                Ok(PersistenceWriteRequest::Partial {
                    path,
                    script_id,
                    model_id,
                    record,
                })
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PersistedBatchJournalRecord {
    schema_version: u16,
    batch_id: u64,
    created_unix_ms: u64,
    request_count: u32,
    requests: Vec<PersistedBatchJournalRequest>,
}

impl PersistedBatchJournalRecord {
    fn from_runtime(batch_id: u64, batch: &[PersistenceWriteRequest]) -> Self {
        let requests = batch.iter().map(PersistedBatchJournalRequest::from_runtime).collect::<Vec<_>>();
        Self {
            schema_version: PHENOMENON_PERSISTENCE_BATCH_JOURNAL_SCHEMA_VERSION,
            batch_id,
            created_unix_ms: unix_timestamp_ms(),
            request_count: requests.len().min(u32::MAX as usize) as u32,
            requests,
        }
    }

    fn into_runtime_batch(self) -> Result<Vec<PersistenceWriteRequest>, String> {
        if self.schema_version != PHENOMENON_PERSISTENCE_BATCH_JOURNAL_SCHEMA_VERSION {
            return Err(format!("unsupported persistence batch journal schema version {}", self.schema_version));
        }
        self.requests
            .into_iter()
            .map(PersistedBatchJournalRequest::into_runtime)
            .collect::<Result<Vec<_>, _>>()
    }
}

#[derive(Resource, Debug, Default)]
pub struct PhenomenonPersistenceWriteRuntimeState {
    queued_by_path: HashMap<PathBuf, PersistenceWriteRequest>,
    in_flight: Option<JoinHandle<PersistenceWriteBatchResult>>,
}

#[derive(Resource, Reflect, Debug, Clone, Copy, Default, PartialEq)]
#[reflect(Resource)]
pub struct PhenomenonPersistenceWriteStats {
    pub queued_records: u32,
    pub in_flight_batches: u32,
    pub max_queued_records_seen: u32,
    pub enqueue_events: u64,
    pub soft_cap_exceeded_events: u64,
    pub flushed_records_total: u64,
    pub flushed_batches_total: u64,
    pub last_flushed_records: u32,
    pub last_flush_duration_ms: f32,
    pub journal_batches_written_total: u64,
    pub journal_batches_replayed_total: u64,
    pub journal_records_replayed_total: u64,
}

#[derive(Resource, Reflect, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[reflect(Resource)]
pub struct PhenomenonPersistenceJournalRecoveryState {
    pub completed: bool,
}

pub(super) fn recover_authoritative_phenomena_persistence_journal_system(
    settings: Res<PhenomenonPersistenceRuntimeSettings>,
    mut recovery_state: ResMut<PhenomenonPersistenceJournalRecoveryState>,
    mut stats: ResMut<PhenomenonPersistenceWriteStats>,
) {
    if recovery_state.completed {
        return;
    }

    if !settings.enabled {
        return;
    }
    if !settings.journal_enabled {
        recovery_state.completed = true;
        return;
    }
    let journal_dir = Path::new(settings.journal_dir.as_str());
    let pending_paths = collect_pending_journal_paths(journal_dir).unwrap_or_else(|error| {
        panic!(
            "USF phenomenon persistence journal recovery failed: could not enumerate pending journals in '{}': {}",
            journal_dir.display(),
            error
        )
    });
    if pending_paths.is_empty() {
        return;
    }

    for pending_path in pending_paths {
        let journal = load_batch_journal_record(&pending_path).unwrap_or_else(|error| {
            panic!(
                "USF phenomenon persistence journal recovery failed: could not load journal '{}': {}",
                pending_path.display(),
                error
            )
        });
        let batch = journal.into_runtime_batch().unwrap_or_else(|error| {
            panic!(
                "USF phenomenon persistence journal recovery failed: could not decode runtime batch from '{}': {}",
                pending_path.display(),
                error
            )
        });
        let result = run_persistence_write_batch(
            batch,
            PersistenceWriteBatchPolicy {
                durability: settings.durability,
                journal_enabled: false,
                journal_dir: journal_dir.to_path_buf(),
                retain_successful_journal_batches: false,
            },
        );
        apply_persistence_write_results(&result.outcomes);
        stats.journal_batches_replayed_total = stats.journal_batches_replayed_total.saturating_add(1);
        stats.journal_records_replayed_total = stats.journal_records_replayed_total.saturating_add(result.record_count as u64);

        if settings.retain_successful_journal_batches {
            archive_pending_journal(&pending_path).unwrap_or_else(|error| {
                panic!(
                    "USF phenomenon persistence journal recovery failed: could not archive journal '{}': {}",
                    pending_path.display(),
                    error
                )
            });
        } else {
            remove_journal_file(&pending_path).unwrap_or_else(|error| {
                panic!(
                    "USF phenomenon persistence journal recovery failed: could not delete replayed journal '{}': {}",
                    pending_path.display(),
                    error
                )
            });
        }
    }

    recovery_state.completed = true;
    stats.queued_records = 0;
}

pub(super) fn enqueue_authoritative_phenomena_persistence_writes_system(
    authority_contract: Res<UsfWorldAuthorityContract>,
    mut authority_diagnostics: Option<ResMut<UsfAuthorityDiagnostics>>,
    settings: Res<PhenomenonPersistenceRuntimeSettings>,
    dirty_phenomena_query: Query<
        Entity,
        (
            With<Phenomenon>,
            Or<(
                Added<Phenomenon>,
                Changed<Phenomenon>,
                Added<PhenomenonScriptDefinitionRef>,
                Changed<PhenomenonScriptDefinitionRef>,
            )>,
        ),
    >,
    dirty_model_query: Query<
        Entity,
        (
            With<PhenomenonModel>,
            Or<(
                Added<PhenomenonModel>,
                Changed<PhenomenonModel>,
                Added<PhenomenonModelScriptDefinitionRef>,
                Changed<PhenomenonModelScriptDefinitionRef>,
                Added<PhenomenonModelSupport>,
                Changed<PhenomenonModelSupport>,
                Added<PhenomenonModelProjectionContract>,
                Changed<PhenomenonModelProjectionContract>,
                Added<PhenomenonModelState>,
                Changed<PhenomenonModelState>,
                Added<PartialPhenomenonModel>,
                Changed<PartialPhenomenonModel>,
            )>,
        ),
    >,
    phenomenon_query: Query<(Entity, &Phenomenon, &PhenomenonScriptDefinitionRef)>,
    model_query: Query<(
        Entity,
        &PhenomenonModel,
        &PhenomenonModelScriptDefinitionRef,
        &PhenomenonModelSupport,
        &PhenomenonModelProjectionContract,
        &PhenomenonModelState,
        Option<&MonolithicPhenomenonModel>,
        Option<&PartialPhenomenonModel>,
        Option<&PartitionedPhenomenonModelRoot>,
        Option<&PartitionedPhenomenonModelMember>,
    )>,
    mut runtime_state: ResMut<PhenomenonPersistenceWriteRuntimeState>,
    mut stats: ResMut<PhenomenonPersistenceWriteStats>,
) {
    if !guard_canonical_domain_with_diagnostics(authority_contract.as_ref(), authority_diagnostics.as_deref_mut(), USF_DOMAIN_PHENOMENON) {
        return;
    }
    if !guard_canonical_domain_with_diagnostics(authority_contract.as_ref(), authority_diagnostics.as_deref_mut(), USF_DOMAIN_PHENOMENON_MODEL) {
        return;
    }
    if !guard_canonical_domain_with_diagnostics(
        authority_contract.as_ref(),
        authority_diagnostics.as_deref_mut(),
        USF_DOMAIN_PARTIAL_PHENOMENON_MODEL,
    ) {
        return;
    }

    if !settings.enabled {
        stats.queued_records = queued_records_u32(runtime_state.queued_by_path.len());
        stats.in_flight_batches = runtime_state.in_flight.as_ref().map(|_| 1).unwrap_or(0);
        return;
    }
    let dirty_phenomena = dirty_phenomena_query.iter().collect::<HashSet<_>>();
    let dirty_models = dirty_model_query.iter().collect::<HashSet<_>>();
    if dirty_phenomena.is_empty() && dirty_models.is_empty() {
        stats.queued_records = queued_records_u32(runtime_state.queued_by_path.len());
        stats.in_flight_batches = runtime_state.in_flight.as_ref().map(|_| 1).unwrap_or(0);
        return;
    }
    let queued_before = runtime_state.queued_by_path.len();
    stats.enqueue_events = stats.enqueue_events.saturating_add(1);

    let mut script_id_by_entity = HashMap::<Entity, String>::new();
    for (entity, phenomenon, script_ref) in phenomenon_query.iter() {
        if dirty_phenomena.contains(&entity) {
            let record = phenomenon_record_from_runtime(phenomenon.id, phenomenon.kind, script_ref.phenomenon_id.as_str());
            let path = phenomenon_record_path(settings.persistence_dir.as_str(), phenomenon.id);
            runtime_state.queued_by_path.insert(
                path.clone(),
                PersistenceWriteRequest::Phenomenon {
                    path,
                    script_id: script_ref.phenomenon_id.clone(),
                    record,
                },
            );
        }
        script_id_by_entity.insert(entity, script_ref.phenomenon_id.clone());
    }

    for (model_entity, model, model_script_ref, support, projection, state, monolithic, partial, _partition_root, partition_member) in model_query.iter() {
        let model_or_parent_dirty = dirty_models.contains(&model_entity) || dirty_phenomena.contains(&model.phenomenon_entity);
        if !model_or_parent_dirty {
            continue;
        }
        let Some(script_id) = script_id_by_entity.get(&model.phenomenon_entity) else {
            continue;
        };
        if partition_member.is_none() {
            let model_record = if let Some(monolithic) = monolithic {
                monolithic_model_record_from_runtime(model_script_ref.model_id.as_str(), monolithic, support, projection, state)
            } else {
                model_record_from_runtime(
                    model.phenomenon_id,
                    model_script_ref.model_id.as_str(),
                    model.scale,
                    model.topology,
                    support,
                    projection,
                    state,
                )
            };
            let path = model_record_path(
                settings.persistence_dir.as_str(),
                model.phenomenon_id,
                model.scale,
                model_script_ref.model_id.as_str(),
            );
            runtime_state.queued_by_path.insert(
                path.clone(),
                PersistenceWriteRequest::Model {
                    path,
                    script_id: script_id.clone(),
                    model_id: model_script_ref.model_id.clone(),
                    record: model_record,
                },
            );
        }

        if let Some(partial) = partial {
            let partial_record = partial_model_record_from_runtime(model_script_ref.model_id.as_str(), partial, state);
            let path = partial_record_path(
                settings.persistence_dir.as_str(),
                partial.phenomenon_id,
                partial.scale,
                model_script_ref.model_id.as_str(),
                partial.partition_key,
            );
            runtime_state.queued_by_path.insert(
                path.clone(),
                PersistenceWriteRequest::Partial {
                    path,
                    script_id: script_id.clone(),
                    model_id: model_script_ref.model_id.clone(),
                    record: partial_record,
                },
            );
        }
    }

    let queued_after = runtime_state.queued_by_path.len();
    let queued_after_u32 = queued_records_u32(queued_after);
    stats.queued_records = queued_after_u32;
    stats.max_queued_records_seen = stats.max_queued_records_seen.max(queued_after_u32);
    stats.in_flight_batches = runtime_state.in_flight.as_ref().map(|_| 1).unwrap_or(0);

    if queued_after > settings.max_queued_records_soft {
        stats.soft_cap_exceeded_events = stats.soft_cap_exceeded_events.saturating_add(1);
        if queued_before <= settings.max_queued_records_soft {
            warn!(
                "USF phenomenon persistence queue exceeded soft cap: queued_records={} soft_cap={} batch_size={} async_enabled={} durability={:?} journal_enabled={}",
                queued_after,
                settings.max_queued_records_soft,
                settings.async_write_batch_size,
                settings.async_write_enabled,
                settings.durability,
                settings.journal_enabled
            );
        }
    }
}

pub(super) fn flush_authoritative_phenomena_persistence_writes_system(
    settings: Res<PhenomenonPersistenceRuntimeSettings>,
    mut runtime_state: ResMut<PhenomenonPersistenceWriteRuntimeState>,
    mut stats: ResMut<PhenomenonPersistenceWriteStats>,
) {
    if runtime_state.in_flight.as_ref().is_some_and(|handle| handle.is_finished()) {
        let handle = runtime_state.in_flight.take().expect("in_flight should exist when is_finished() is true");
        let batch_result = handle
            .join()
            .unwrap_or_else(|_| panic!("USF phenomenon persistence worker panicked while writing authoritative records."));
        apply_persistence_write_results(&batch_result.outcomes);
        record_flush_stats(&mut stats, &batch_result);
    }

    if !settings.enabled {
        runtime_state.queued_by_path.clear();
        stats.queued_records = 0;
        stats.in_flight_batches = runtime_state.in_flight.as_ref().map(|_| 1).unwrap_or(0);
        return;
    }

    if runtime_state.in_flight.is_some() || runtime_state.queued_by_path.is_empty() {
        stats.queued_records = queued_records_u32(runtime_state.queued_by_path.len());
        stats.in_flight_batches = runtime_state.in_flight.as_ref().map(|_| 1).unwrap_or(0);
        return;
    }

    let batch_size = settings.async_write_batch_size.max(1);
    let mut sorted_paths = runtime_state.queued_by_path.keys().cloned().collect::<Vec<_>>();
    sorted_paths.sort();
    let batch = sorted_paths
        .into_iter()
        .take(batch_size)
        .filter_map(|path| runtime_state.queued_by_path.remove(&path))
        .collect::<Vec<_>>();
    if batch.is_empty() {
        stats.queued_records = queued_records_u32(runtime_state.queued_by_path.len());
        stats.in_flight_batches = runtime_state.in_flight.as_ref().map(|_| 1).unwrap_or(0);
        return;
    }

    let batch_policy = PersistenceWriteBatchPolicy {
        durability: settings.durability,
        journal_enabled: settings.journal_enabled,
        journal_dir: PathBuf::from(settings.journal_dir.clone()),
        retain_successful_journal_batches: settings.retain_successful_journal_batches,
    };

    if !settings.async_write_enabled {
        let batch_result = run_persistence_write_batch(batch, batch_policy);
        apply_persistence_write_results(&batch_result.outcomes);
        record_flush_stats(&mut stats, &batch_result);
        stats.queued_records = queued_records_u32(runtime_state.queued_by_path.len());
        stats.in_flight_batches = runtime_state.in_flight.as_ref().map(|_| 1).unwrap_or(0);
        return;
    }

    let handle = std::thread::Builder::new()
        .name("usf-phenomena-persistence".to_string())
        .spawn(move || run_persistence_write_batch(batch, batch_policy))
        .unwrap_or_else(|error| panic!("USF phenomenon persistence failed: could not spawn persistence worker: {}", error));
    runtime_state.in_flight = Some(handle);
    stats.queued_records = queued_records_u32(runtime_state.queued_by_path.len());
    stats.in_flight_batches = 1;
}

fn run_persistence_write_batch(batch: Vec<PersistenceWriteRequest>, policy: PersistenceWriteBatchPolicy) -> PersistenceWriteBatchResult {
    let started = Instant::now();
    let record_count = batch.len();
    let mut journal_batch_written = false;
    let mut pending_journal_path = None::<PathBuf>;
    if policy.journal_enabled {
        let batch_id = NEXT_PERSISTENCE_BATCH_ID.fetch_add(1, Ordering::Relaxed);
        let pending_path = pending_journal_path_for(policy.journal_dir.as_path(), batch_id);
        let journal = PersistedBatchJournalRecord::from_runtime(batch_id, &batch);
        write_batch_journal(&pending_path, &journal).unwrap_or_else(|error| {
            panic!(
                "USF phenomenon persistence failed: could not write batch journal '{}': {}",
                pending_path.display(),
                error
            )
        });
        journal_batch_written = true;
        pending_journal_path = Some(pending_path);
    }

    let outcomes = batch
        .into_iter()
        .map(|request| match request {
            PersistenceWriteRequest::Phenomenon { path, script_id, record } => PersistenceWriteResult {
                context: format!("phenomenon '{}'", script_id),
                path: path.clone(),
                error: save_phenomenon_record_with_durability(&path, &record, policy.durability).err(),
            },
            PersistenceWriteRequest::Model {
                path,
                script_id,
                model_id,
                record,
            } => PersistenceWriteResult {
                context: format!("model '{}' for phenomenon '{}'", model_id, script_id),
                path: path.clone(),
                error: save_phenomenon_model_record_with_durability(&path, &record, policy.durability).err(),
            },
            PersistenceWriteRequest::Partial {
                path,
                script_id,
                model_id,
                record,
            } => PersistenceWriteResult {
                context: format!("partial model '{}' for phenomenon '{}'", model_id, script_id),
                path: path.clone(),
                error: save_partial_phenomenon_model_record_with_durability(&path, &record, policy.durability).err(),
            },
        })
        .collect::<Vec<_>>();
    let has_write_errors = outcomes.iter().any(|outcome| outcome.error.is_some());
    if !has_write_errors {
        if let Some(pending_path) = pending_journal_path.as_ref() {
            if policy.retain_successful_journal_batches {
                archive_pending_journal(pending_path).unwrap_or_else(|error| {
                    panic!(
                        "USF phenomenon persistence failed: could not archive successful batch journal '{}': {}",
                        pending_path.display(),
                        error
                    )
                });
            } else {
                remove_journal_file(pending_path).unwrap_or_else(|error| {
                    panic!(
                        "USF phenomenon persistence failed: could not remove successful batch journal '{}': {}",
                        pending_path.display(),
                        error
                    )
                });
            }
        }
    }
    PersistenceWriteBatchResult {
        outcomes,
        record_count,
        elapsed_ms: started.elapsed().as_secs_f64() * 1000.0,
        journal_batch_written,
    }
}

fn apply_persistence_write_results(outcomes: &[PersistenceWriteResult]) {
    for outcome in outcomes {
        if let Some(error) = &outcome.error {
            panic!(
                "USF phenomenon persistence failed: could not save {} ({:?}): {}",
                outcome.context, outcome.path, error
            );
        }
    }
}

fn pending_journal_path_for(journal_dir: &Path, batch_id: u64) -> PathBuf {
    journal_dir.join(format!("batch_{:020}.pending.json", batch_id))
}

fn applied_journal_path_for(pending_path: &Path) -> PathBuf {
    let file_name = pending_path.file_name().and_then(|name| name.to_str()).unwrap_or("batch.pending.json");
    let applied_name = file_name.replace(".pending.json", ".applied.json");
    pending_path.with_file_name(applied_name)
}

fn write_batch_journal(path: &Path, journal: &PersistedBatchJournalRecord) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("create_dir_all failed: {error}"))?;
    }
    let bytes = serde_json::to_vec_pretty(journal).map_err(|error| format!("serialize journal failed: {error}"))?;
    write_bytes_atomically(path, bytes.as_slice())
}

fn load_batch_journal_record(path: &Path) -> Result<PersistedBatchJournalRecord, String> {
    let bytes = fs::read(path).map_err(|error| format!("read journal failed: {error}"))?;
    serde_json::from_slice::<PersistedBatchJournalRecord>(&bytes).map_err(|error| format!("parse journal failed: {error}"))
}

fn collect_pending_journal_paths(journal_dir: &Path) -> Result<Vec<PathBuf>, String> {
    if !journal_dir.exists() {
        return Ok(Vec::new());
    }
    let mut pending_paths = Vec::<PathBuf>::new();
    let entries = fs::read_dir(journal_dir).map_err(|error| format!("read_dir failed: {error}"))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("read_dir entry failed: {error}"))?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if name.ends_with(".pending.json") {
            pending_paths.push(path);
        }
    }
    pending_paths.sort();
    Ok(pending_paths)
}

fn archive_pending_journal(pending_path: &Path) -> Result<(), String> {
    let applied_path = applied_journal_path_for(pending_path);
    if applied_path.exists() {
        fs::remove_file(&applied_path).map_err(|error| format!("remove existing applied journal '{}' failed: {}", applied_path.display(), error))?;
    }
    fs::rename(pending_path, &applied_path).map_err(|error| {
        format!(
            "rename pending journal '{}' -> '{}' failed: {}",
            pending_path.display(),
            applied_path.display(),
            error
        )
    })
}

fn remove_journal_file(path: &Path) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!("remove file failed: {error}")),
    }
}

fn parse_non_empty_path(raw: &str) -> Result<PathBuf, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err("journal request path must not be empty".to_string());
    }
    Ok(PathBuf::from(trimmed))
}

fn unix_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .min(u64::MAX as u128) as u64
}

fn write_bytes_atomically(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let temp_path = path.with_extension(format!(
        "tmp.{}.{:016x}",
        std::process::id(),
        NEXT_PERSISTENCE_BATCH_ID.fetch_add(1, Ordering::Relaxed)
    ));
    let mut temp_file = File::create(&temp_path).map_err(|error| format!("create temp file failed: {error}"))?;
    if let Err(error) = temp_file.write_all(bytes) {
        let _ = fs::remove_file(&temp_path);
        return Err(format!("write temp file failed: {error}"));
    }
    drop(temp_file);
    if let Err(error) = fs::rename(&temp_path, path) {
        let _ = fs::remove_file(&temp_path);
        return Err(format!("rename temp file failed: {error}"));
    }
    Ok(())
}

#[inline]
fn queued_records_u32(len: usize) -> u32 {
    len.min(u32::MAX as usize) as u32
}

fn record_flush_stats(stats: &mut PhenomenonPersistenceWriteStats, batch: &PersistenceWriteBatchResult) {
    let flushed_records = batch.record_count as u64;
    stats.flushed_records_total = stats.flushed_records_total.saturating_add(flushed_records);
    stats.flushed_batches_total = stats.flushed_batches_total.saturating_add(1);
    stats.last_flushed_records = queued_records_u32(batch.record_count);
    stats.last_flush_duration_ms = batch.elapsed_ms as f32;
    if batch.journal_batch_written {
        stats.journal_batches_written_total = stats.journal_batches_written_total.saturating_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_record() -> PersistedPhenomenonRecord {
        PersistedPhenomenonRecord {
            schema_version: 2,
            phenomenon_id: 11,
            kind: "ManifestationDensityDebug".to_string(),
            script_id: "phenomenon.demo.manifestation_density".to_string(),
            metadata: vec![("k".to_string(), "v".to_string())],
        }
    }

    #[test]
    fn batch_journal_roundtrips_runtime_requests() {
        let request = PersistenceWriteRequest::Phenomenon {
            path: PathBuf::from("target/usf_demo/authority/phenomenon_000000000000000b.json"),
            script_id: "phenomenon.demo.manifestation_density".to_string(),
            record: sample_record(),
        };
        let journal = PersistedBatchJournalRecord::from_runtime(7, &[request.clone()]);
        let rebuilt = journal.into_runtime_batch().expect("journal should decode");
        assert_eq!(rebuilt.len(), 1);
        match &rebuilt[0] {
            PersistenceWriteRequest::Phenomenon { path, script_id, record } => {
                assert!(path.to_string_lossy().contains("phenomenon_000000000000000b.json"));
                assert_eq!(script_id, "phenomenon.demo.manifestation_density");
                assert_eq!(record, &sample_record());
            }
            _ => panic!("expected phenomenon journal request"),
        }
    }

    #[test]
    fn pending_journal_collection_is_sorted() {
        let root = std::env::temp_dir().join(format!("usf_persistence_journal_test_{:x}", unix_timestamp_ms()));
        fs::create_dir_all(&root).expect("create temp journal dir");
        fs::write(root.join("batch_00000000000000000002.pending.json"), b"{}").expect("write journal2");
        fs::write(root.join("batch_00000000000000000001.pending.json"), b"{}").expect("write journal1");
        fs::write(root.join("batch_00000000000000000001.applied.json"), b"{}").expect("write applied");

        let pending = collect_pending_journal_paths(root.as_path()).expect("collect pending journals");
        let names = pending
            .iter()
            .map(|path| path.file_name().and_then(|name| name.to_str()).unwrap_or("").to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            vec![
                "batch_00000000000000000001.pending.json".to_string(),
                "batch_00000000000000000002.pending.json".to_string(),
            ]
        );

        let _ = fs::remove_dir_all(&root);
    }
}
