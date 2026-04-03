use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use serde::{Deserialize, Serialize};

use crate::bevy::prelude::*;
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::types::GridXyz;
use crate::usf::scale::Scale;

use super::components::{
    MonolithicPhenomenonModel, PartialPhenomenonModel, PhenomenonModelProjectionContract, PhenomenonModelState, PhenomenonModelSupport, PhenomenonModelTopology,
};
use super::types::PhenomenonId;

pub const PHENOMENON_SCHEMA_VERSION: u16 = 2;
pub const PHENOMENON_MODEL_SCHEMA_VERSION: u16 = 2;
pub const PARTIAL_PHENOMENON_MODEL_SCHEMA_VERSION: u16 = 2;

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhenomenonPersistenceDurability {
    AtomicReplace,
    AtomicReplaceAndFsync,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PersistedGridCoord {
    pub scale_index: u8,
    pub digits: Vec<[i8; 3]>,
}
impl PersistedGridCoord {
    pub fn from_grid(coord: &GridVec) -> Self {
        let mut canonical = coord.clone();
        canonical.normalize();
        Self {
            scale_index: canonical.scale.index_from_top(),
            digits: canonical
                .to_raw_vec_3d()
                .into_iter()
                .map(|xyz| [xyz.x as i8, xyz.y as i8, xyz.z as i8])
                .collect(),
        }
    }

    pub fn to_grid(&self) -> Result<GridVec, String> {
        let Some(scale) = Scale::from_index_from_top(self.scale_index) else {
            return Err(format!("invalid scale_index {}", self.scale_index));
        };
        if self.digits.is_empty() {
            return Err("persisted grid coord must contain at least one digit".to_string());
        }
        let expected_depth = scale.index_from_top() as usize + 1;
        if self.digits.len() != expected_depth {
            return Err(format!(
                "persisted grid coord depth mismatch: scale_index={} expects {} digits, got {}",
                self.scale_index,
                expected_depth,
                self.digits.len()
            ));
        }
        let mut stack = Vec::with_capacity(self.digits.len());
        for [x, y, z] in &self.digits {
            let xyz = GridXyz::new_raw(*x as i32, *y as i32, *z as i32);
            xyz.try_assert_local()?;
            stack.push(xyz);
        }
        GridVec::try_from(stack).map_err(|error| error.to_string())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PersistedPhenomenonRecord {
    pub schema_version: u16,
    pub phenomenon_id: u64,
    pub kind: String,
    pub script_id: String,
    #[serde(default)]
    pub metadata: Vec<(String, String)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersistedPhenomenonModelRecord {
    pub schema_version: u16,
    pub phenomenon_id: u64,
    pub model_id: String,
    pub scale_index: u8,
    pub topology: String,
    pub support_anchor_chunk: PersistedGridCoord,
    pub support_chunk_radius: u16,
    pub projection_metric_name: String,
    pub projection_bias: f32,
    pub projection_gain: f32,
    #[serde(default)]
    pub scalar_channels: Vec<(String, f32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PersistedPartialPhenomenonModelRecord {
    pub schema_version: u16,
    pub phenomenon_id: u64,
    pub model_id: String,
    pub scale_index: u8,
    pub chunk_coord: PersistedGridCoord,
    pub partition_key: u64,
    #[serde(default)]
    pub scalar_channels: Vec<(String, f32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
struct PersistedPhenomenonRecordV1 {
    pub phenomenon_id: u64,
    pub kind: String,
    pub script_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PersistedPhenomenonModelRecordV1 {
    pub phenomenon_id: u64,
    pub model_id: String,
    pub scale_index: u8,
    pub support_anchor_chunk: PersistedGridCoord,
    pub support_chunk_radius: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
struct PersistedPartialPhenomenonModelRecordV1 {
    pub phenomenon_id: u64,
    pub model_id: String,
    pub scale_index: u8,
    pub chunk_coord: PersistedGridCoord,
    pub partition_key: u64,
}

pub fn migrate_phenomenon_record(value: serde_json::Value) -> Result<PersistedPhenomenonRecord, String> {
    match value.get("schema_version").and_then(|version| version.as_u64()).unwrap_or(1) {
        1 => {
            let legacy = serde_json::from_value::<PersistedPhenomenonRecordV1>(value).map_err(|error| format!("phenomenon v1 parse failed: {error}"))?;
            Ok(PersistedPhenomenonRecord {
                schema_version: PHENOMENON_SCHEMA_VERSION,
                phenomenon_id: legacy.phenomenon_id,
                kind: legacy.kind,
                script_id: legacy.script_id,
                metadata: Vec::new(),
            })
        }
        2 => serde_json::from_value::<PersistedPhenomenonRecord>(value).map_err(|error| format!("phenomenon v2 parse failed: {error}")),
        unknown => Err(format!("unsupported phenomenon schema version {unknown}")),
    }
}

pub fn migrate_phenomenon_model_record(value: serde_json::Value) -> Result<PersistedPhenomenonModelRecord, String> {
    match value.get("schema_version").and_then(|version| version.as_u64()).unwrap_or(1) {
        1 => {
            let legacy =
                serde_json::from_value::<PersistedPhenomenonModelRecordV1>(value).map_err(|error| format!("phenomena_model v1 parse failed: {error}"))?;
            Ok(PersistedPhenomenonModelRecord {
                schema_version: PHENOMENON_MODEL_SCHEMA_VERSION,
                phenomenon_id: legacy.phenomenon_id,
                model_id: legacy.model_id,
                scale_index: legacy.scale_index,
                topology: "monolithic_chunk".to_string(),
                support_anchor_chunk: legacy.support_anchor_chunk,
                support_chunk_radius: legacy.support_chunk_radius,
                projection_metric_name: "demo_mass_density".to_string(),
                projection_bias: 0.0,
                projection_gain: 1.0,
                scalar_channels: vec![("demo_mass_density.base".to_string(), 0.5)],
            })
        }
        2 => serde_json::from_value::<PersistedPhenomenonModelRecord>(value).map_err(|error| format!("phenomena_model v2 parse failed: {error}")),
        unknown => Err(format!("unsupported phenomena_model schema version {unknown}")),
    }
}

pub fn migrate_partial_phenomenon_model_record(value: serde_json::Value) -> Result<PersistedPartialPhenomenonModelRecord, String> {
    match value.get("schema_version").and_then(|version| version.as_u64()).unwrap_or(1) {
        1 => {
            let legacy = serde_json::from_value::<PersistedPartialPhenomenonModelRecordV1>(value)
                .map_err(|error| format!("partial_phenomena_model v1 parse failed: {error}"))?;
            Ok(PersistedPartialPhenomenonModelRecord {
                schema_version: PARTIAL_PHENOMENON_MODEL_SCHEMA_VERSION,
                phenomenon_id: legacy.phenomenon_id,
                model_id: legacy.model_id,
                scale_index: legacy.scale_index,
                chunk_coord: legacy.chunk_coord,
                partition_key: legacy.partition_key,
                scalar_channels: vec![("demo_mass_density.base".to_string(), 0.5)],
            })
        }
        2 => {
            serde_json::from_value::<PersistedPartialPhenomenonModelRecord>(value).map_err(|error| format!("partial_phenomena_model v2 parse failed: {error}"))
        }
        unknown => Err(format!("unsupported partial_phenomena_model schema version {unknown}")),
    }
}

pub fn load_phenomenon_record(path: &Path) -> Result<Option<PersistedPhenomenonRecord>, String> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("read phenomenon record failed: {error}")),
    };
    let raw_value = serde_json::from_slice::<serde_json::Value>(&bytes).map_err(|error| format!("parse json failed: {error}"))?;
    let migrated = migrate_phenomenon_record(raw_value)?;
    Ok(Some(migrated))
}

pub fn load_phenomenon_model_record(path: &Path) -> Result<Option<PersistedPhenomenonModelRecord>, String> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("read phenomena_model record failed: {error}")),
    };
    let raw_value = serde_json::from_slice::<serde_json::Value>(&bytes).map_err(|error| format!("parse json failed: {error}"))?;
    let migrated = migrate_phenomenon_model_record(raw_value)?;
    Ok(Some(migrated))
}

pub fn load_partial_phenomenon_model_record(path: &Path) -> Result<Option<PersistedPartialPhenomenonModelRecord>, String> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(format!("read partial_phenomena_model record failed: {error}")),
    };
    let raw_value = serde_json::from_slice::<serde_json::Value>(&bytes).map_err(|error| format!("parse json failed: {error}"))?;
    let migrated = migrate_partial_phenomenon_model_record(raw_value)?;
    Ok(Some(migrated))
}

pub fn save_phenomenon_record(path: &Path, record: &PersistedPhenomenonRecord) -> Result<(), String> {
    save_phenomenon_record_with_durability(path, record, PhenomenonPersistenceDurability::AtomicReplace)
}

pub fn save_phenomenon_model_record(path: &Path, record: &PersistedPhenomenonModelRecord) -> Result<(), String> {
    save_phenomenon_model_record_with_durability(path, record, PhenomenonPersistenceDurability::AtomicReplace)
}

pub fn save_partial_phenomenon_model_record(path: &Path, record: &PersistedPartialPhenomenonModelRecord) -> Result<(), String> {
    save_partial_phenomenon_model_record_with_durability(path, record, PhenomenonPersistenceDurability::AtomicReplace)
}

pub fn save_phenomenon_record_with_durability(
    path: &Path,
    record: &PersistedPhenomenonRecord,
    durability: PhenomenonPersistenceDurability,
) -> Result<(), String> {
    save_json(path, record, durability)
}

pub fn save_phenomenon_model_record_with_durability(
    path: &Path,
    record: &PersistedPhenomenonModelRecord,
    durability: PhenomenonPersistenceDurability,
) -> Result<(), String> {
    save_json(path, record, durability)
}

pub fn save_partial_phenomenon_model_record_with_durability(
    path: &Path,
    record: &PersistedPartialPhenomenonModelRecord,
    durability: PhenomenonPersistenceDurability,
) -> Result<(), String> {
    save_json(path, record, durability)
}

pub fn topology_to_tag(topology: PhenomenonModelTopology) -> &'static str {
    match topology {
        PhenomenonModelTopology::MonolithicChunk => "monolithic_chunk",
        PhenomenonModelTopology::PartitionedByChunk => "partitioned_by_chunk",
    }
}

pub fn topology_from_tag(tag: &str) -> Result<PhenomenonModelTopology, String> {
    match tag.trim().to_ascii_lowercase().as_str() {
        "monolithic_chunk" | "monolithic-chunk" | "monolithic" => Ok(PhenomenonModelTopology::MonolithicChunk),
        "partitioned_by_chunk" | "partitioned-by-chunk" | "partitioned" => Ok(PhenomenonModelTopology::PartitionedByChunk),
        unknown => Err(format!("unsupported topology tag '{unknown}'")),
    }
}

pub fn phenomenon_record_from_runtime(phenomenon_id: PhenomenonId, kind_id: &str, script_id: &str) -> PersistedPhenomenonRecord {
    PersistedPhenomenonRecord {
        schema_version: PHENOMENON_SCHEMA_VERSION,
        phenomenon_id: phenomenon_id.0,
        kind: kind_id.to_ascii_lowercase(),
        script_id: script_id.to_ascii_lowercase(),
        metadata: Vec::new(),
    }
}

pub fn model_record_from_runtime(
    phenomenon_id: PhenomenonId,
    model_id: &str,
    scale: Scale,
    topology: PhenomenonModelTopology,
    support: &PhenomenonModelSupport,
    projection: &PhenomenonModelProjectionContract,
    state: &PhenomenonModelState,
) -> PersistedPhenomenonModelRecord {
    PersistedPhenomenonModelRecord {
        schema_version: PHENOMENON_MODEL_SCHEMA_VERSION,
        phenomenon_id: phenomenon_id.0,
        model_id: model_id.to_ascii_lowercase(),
        scale_index: scale.index_from_top(),
        topology: topology_to_tag(topology).to_string(),
        support_anchor_chunk: PersistedGridCoord::from_grid(&support.support.anchor_chunk),
        support_chunk_radius: support.support.chunk_radius,
        projection_metric_name: projection.contract.metric_name.to_ascii_lowercase(),
        projection_bias: projection.contract.projection_bias,
        projection_gain: projection.contract.projection_gain,
        scalar_channels: state.scalar_channels.clone(),
    }
}

pub fn partial_model_record_from_runtime(
    model_id: &str,
    partial_model: &PartialPhenomenonModel,
    state: &PhenomenonModelState,
) -> PersistedPartialPhenomenonModelRecord {
    PersistedPartialPhenomenonModelRecord {
        schema_version: PARTIAL_PHENOMENON_MODEL_SCHEMA_VERSION,
        phenomenon_id: partial_model.phenomenon_id.0,
        model_id: model_id.to_ascii_lowercase(),
        scale_index: partial_model.scale.index_from_top(),
        chunk_coord: PersistedGridCoord::from_grid(&partial_model.chunk_coord),
        partition_key: partial_model.partition_key,
        scalar_channels: state.scalar_channels.clone(),
    }
}

pub fn monolithic_model_record_from_runtime(
    model_id: &str,
    monolithic_model: &MonolithicPhenomenonModel,
    support: &PhenomenonModelSupport,
    projection: &PhenomenonModelProjectionContract,
    state: &PhenomenonModelState,
) -> PersistedPhenomenonModelRecord {
    model_record_from_runtime(
        monolithic_model.phenomenon_id,
        model_id,
        monolithic_model.scale,
        PhenomenonModelTopology::MonolithicChunk,
        support,
        projection,
        state,
    )
}

pub fn phenomenon_record_path(root: &str, phenomenon_id: PhenomenonId) -> std::path::PathBuf {
    Path::new(root).join(format!("phenomenon_{:016x}.json", phenomenon_id.0))
}

pub fn model_record_path(root: &str, phenomenon_id: PhenomenonId, scale: Scale, model_id: &str) -> std::path::PathBuf {
    Path::new(root).join(format!(
        "model_{:016x}_scale_{:02}_{}.json",
        phenomenon_id.0,
        scale.index_from_top(),
        sanitize_id_for_path(model_id),
    ))
}

pub fn partial_record_path(root: &str, phenomenon_id: PhenomenonId, scale: Scale, model_id: &str, partition_key: u64) -> std::path::PathBuf {
    Path::new(root).join(format!(
        "partial_{:016x}_scale_{:02}_{}_part_{:016x}.json",
        phenomenon_id.0,
        scale.index_from_top(),
        sanitize_id_for_path(model_id),
        partition_key,
    ))
}

fn sanitize_id_for_path(id: &str) -> String {
    let normalized = id.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return "_".to_string();
    }
    normalized
        .chars()
        .map(|value| {
            if value.is_ascii_alphanumeric() || value == '_' || value == '-' || value == '.' {
                value
            } else {
                '_'
            }
        })
        .collect::<String>()
}

fn save_json<T: Serialize>(path: &Path, value: &T, durability: PhenomenonPersistenceDurability) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("create_dir_all failed: {error}"))?;
    }
    let bytes = serde_json::to_vec_pretty(value).map_err(|error| format!("serialize json failed: {error}"))?;

    let temp_path = temp_path_for(path);
    let mut temp_file = File::create(&temp_path).map_err(|error| format!("create temp file failed: {error}"))?;
    if let Err(error) = temp_file.write_all(&bytes) {
        let _ = fs::remove_file(&temp_path);
        return Err(format!("write temp file failed: {error}"));
    }
    if durability == PhenomenonPersistenceDurability::AtomicReplaceAndFsync {
        if let Err(error) = temp_file.sync_all() {
            let _ = fs::remove_file(&temp_path);
            return Err(format!("sync temp file failed: {error}"));
        }
    }
    drop(temp_file);

    if let Err(rename_error) = fs::rename(&temp_path, path) {
        if path.exists() {
            fs::remove_file(path).map_err(|error| {
                let _ = fs::remove_file(&temp_path);
                format!("replace failed: {rename_error}; remove existing target failed: {error}")
            })?;
            fs::rename(&temp_path, path).map_err(|error| {
                let _ = fs::remove_file(&temp_path);
                format!("replace failed after removing target: {error}")
            })?;
        } else {
            let _ = fs::remove_file(&temp_path);
            return Err(format!("replace failed: {rename_error}"));
        }
    }

    if durability == PhenomenonPersistenceDurability::AtomicReplaceAndFsync {
        if let Some(parent) = path.parent() {
            sync_directory(parent)?;
        }
    }
    Ok(())
}

fn temp_path_for(path: &Path) -> std::path::PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let sequence = COUNTER.fetch_add(1, Ordering::Relaxed);
    let file_name = path.file_name().and_then(|name| name.to_str()).unwrap_or("record.json");
    path.with_file_name(format!(".{}.tmp.{}.{:016x}", file_name, std::process::id(), sequence))
}

#[cfg(unix)]
fn sync_directory(path: &Path) -> Result<(), String> {
    let directory = File::open(path).map_err(|error| format!("open directory failed: {error}"))?;
    directory.sync_all().map_err(|error| format!("sync directory failed: {error}"))
}

#[cfg(not(unix))]
fn sync_directory(_path: &Path) -> Result<(), String> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bevy::prelude::IVec3;
    use crate::usf::pos::types::GridXyz;

    fn test_coord() -> GridVec {
        GridVec::new(GridVec::new_root(GridXyz::new_local(0, 0, 0)), GridXyz::new_local(2, -1, 3))
    }

    #[test]
    fn persistence_roundtrip_for_phenomenon_and_model_records() {
        let phenomenon_record = PersistedPhenomenonRecord {
            schema_version: PHENOMENON_SCHEMA_VERSION,
            phenomenon_id: 7,
            kind: "RealizationDensityDebug".to_string(),
            script_id: "phenomenon.demo.realization_density".to_string(),
            metadata: vec![("author".to_string(), "test".to_string())],
        };
        let model_record = PersistedPhenomenonModelRecord {
            schema_version: PHENOMENON_MODEL_SCHEMA_VERSION,
            phenomenon_id: 7,
            model_id: "demo_realization_density.default".to_string(),
            scale_index: 0,
            topology: "monolithic_chunk".to_string(),
            support_anchor_chunk: PersistedGridCoord::from_grid(&test_coord()),
            support_chunk_radius: 4,
            projection_metric_name: "demo_mass_density".to_string(),
            projection_bias: 0.1,
            projection_gain: 0.9,
            scalar_channels: vec![("demo_mass_density.base".to_string(), 0.61)],
        };
        let partial_record = PersistedPartialPhenomenonModelRecord {
            schema_version: PARTIAL_PHENOMENON_MODEL_SCHEMA_VERSION,
            phenomenon_id: 7,
            model_id: "demo_realization_density.default".to_string(),
            scale_index: 1,
            chunk_coord: PersistedGridCoord::from_grid(&test_coord()),
            partition_key: 9182,
            scalar_channels: vec![("demo_mass_density.base".to_string(), 0.61)],
        };

        let temp_dir = std::env::temp_dir().join("usf_phenomenon_persistence_tests");
        let phenomenon_path = temp_dir.join("phenomenon.json");
        let model_path = temp_dir.join("model.json");
        let partial_path = temp_dir.join("partial.json");

        save_phenomenon_record(&phenomenon_path, &phenomenon_record).expect("save phenomenon");
        save_phenomenon_model_record(&model_path, &model_record).expect("save model");
        save_partial_phenomenon_model_record(&partial_path, &partial_record).expect("save partial");

        let loaded_phenomenon = load_phenomenon_record(&phenomenon_path)
            .expect("load phenomenon")
            .expect("phenomenon record exists");
        let loaded_model = load_phenomenon_model_record(&model_path).expect("load model").expect("model record exists");
        let loaded_partial = load_partial_phenomenon_model_record(&partial_path)
            .expect("load partial")
            .expect("partial record exists");

        assert_eq!(loaded_phenomenon, phenomenon_record);
        assert_eq!(loaded_model, model_record);
        assert_eq!(loaded_partial, partial_record);

        let _ = fs::remove_file(phenomenon_path);
        let _ = fs::remove_file(model_path);
        let _ = fs::remove_file(partial_path);
    }

    #[test]
    fn save_with_explicit_fsync_durability_roundtrips() {
        let phenomenon_record = PersistedPhenomenonRecord {
            schema_version: PHENOMENON_SCHEMA_VERSION,
            phenomenon_id: 17,
            kind: "RealizationDensityDebug".to_string(),
            script_id: "phenomenon.demo.realization_density".to_string(),
            metadata: vec![("durability".to_string(), "fsync".to_string())],
        };

        let temp_dir = std::env::temp_dir().join("usf_phenomenon_persistence_tests");
        let phenomenon_path = temp_dir.join("phenomenon_fsync.json");
        save_phenomenon_record_with_durability(&phenomenon_path, &phenomenon_record, PhenomenonPersistenceDurability::AtomicReplaceAndFsync)
            .expect("save phenomenon with fsync durability");

        let loaded = load_phenomenon_record(&phenomenon_path)
            .expect("load phenomenon")
            .expect("phenomenon record exists");
        assert_eq!(loaded, phenomenon_record);

        let _ = fs::remove_file(phenomenon_path);
    }

    #[test]
    fn persisted_grid_coord_roundtrips_to_gridvec() {
        let coord = test_coord();
        let persisted = PersistedGridCoord::from_grid(&coord);
        let rebuilt = persisted.to_grid().expect("roundtrip grid coordinate");
        assert_eq!(coord, rebuilt);
    }

    #[test]
    fn migration_upgrades_v1_model_records_to_v2() {
        let legacy = serde_json::json!({
            "phenomenon_id": 77_u64,
            "model_id": "demo_realization_density.default",
            "scale_index": 0_u8,
            "support_anchor_chunk": PersistedGridCoord::from_grid(&test_coord()),
            "support_chunk_radius": 3_u16
        });

        let migrated = migrate_phenomenon_model_record(legacy).expect("migration should succeed");
        assert_eq!(migrated.schema_version, PHENOMENON_MODEL_SCHEMA_VERSION);
        assert_eq!(migrated.topology, "monolithic_chunk");
        assert_eq!(migrated.projection_metric_name, "demo_mass_density");
        assert_eq!(migrated.scalar_channels.len(), 1);
    }

    #[test]
    fn partial_partition_keys_are_deterministic() {
        let coord = test_coord();
        let a = PartialPhenomenonModel::deterministic_partition_key(PhenomenonId(99), Scale::MAX.zoomed_in(), &coord);
        let b = PartialPhenomenonModel::deterministic_partition_key(PhenomenonId(99), Scale::MAX.zoomed_in(), &coord);
        let c = PartialPhenomenonModel::deterministic_partition_key(PhenomenonId(99), Scale::MAX.zoomed_in(), &(coord + IVec3::new(1, 0, 0)));

        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
