//! Typed layered runtime configuration for Spacetime Engine.
//!
//! Configuration has three layers, applied in order:
//! 1. compiled Rust defaults,
//! 2. a hot-reloaded RON project file,
//! 3. typed runtime/developer overrides.
//!
//! Systems consume only the effective [`EngineConfig`] resource. This keeps
//! policy out of semantic world state and lets expensive realization choices be
//! tuned while profiling without recompiling the universe.

use std::{
    env, fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

use bevy::prelude::*;
use serde::Deserialize;

const CONFIG_RELOAD_INTERVAL_SECONDS: f32 = 0.5;

#[derive(Resource, Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct EngineConfig {
    pub voxel: VoxelConfig,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            voxel: VoxelConfig::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct VoxelConfig {
    pub streaming: VoxelStreamingConfig,
    pub manifestation: VoxelManifestationConfig,
}

impl Default for VoxelConfig {
    fn default() -> Self {
        Self {
            streaming: VoxelStreamingConfig::default(),
            manifestation: VoxelManifestationConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct VoxelStreamingConfig {
    pub default_load_budget_per_frame: usize,
    pub generation_publish_budget_per_frame: usize,
    pub warm_inactive_materialization_limit: usize,
    pub generation_group_base_chunks_per_axis: i32,
    pub max_chunks_per_generation_task: usize,
}

impl Default for VoxelStreamingConfig {
    fn default() -> Self {
        Self {
            default_load_budget_per_frame: 24,
            generation_publish_budget_per_frame: 16,
            warm_inactive_materialization_limit: 4096,
            generation_group_base_chunks_per_axis: 10,
            max_chunks_per_generation_task: 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(default)]
pub struct VoxelManifestationConfig {
    pub grouping: VoxelManifestationGroupingConfig,
    pub rebuild_budget_per_frame: usize,
    pub physics_interaction_radius_native: f32,
}

impl Default for VoxelManifestationConfig {
    fn default() -> Self {
        Self {
            grouping: VoxelManifestationGroupingConfig::default(),
            rebuild_budget_per_frame: 8,
            physics_interaction_radius_native: 32.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum VoxelGroupingStrategy {
    AlignedRegions,
}

impl Default for VoxelGroupingStrategy {
    fn default() -> Self {
        Self::AlignedRegions
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(default)]
pub struct VoxelManifestationGroupingConfig {
    pub strategy: VoxelGroupingStrategy,
    pub base_chunks_per_axis: i32,
}

impl Default for VoxelManifestationGroupingConfig {
    fn default() -> Self {
        Self {
            strategy: VoxelGroupingStrategy::AlignedRegions,
            base_chunks_per_axis: 10,
        }
    }
}

#[derive(Resource, Debug, Clone, Default)]
pub struct EngineConfigOverrides {
    pub voxel: VoxelConfigOverrides,
}

#[derive(Debug, Clone, Default)]
pub struct VoxelConfigOverrides {
    pub streaming: VoxelStreamingConfigOverrides,
    pub manifestation: VoxelManifestationConfigOverrides,
}

#[derive(Debug, Clone, Default)]
pub struct VoxelStreamingConfigOverrides {
    pub default_load_budget_per_frame: Option<usize>,
    pub generation_publish_budget_per_frame: Option<usize>,
    pub warm_inactive_materialization_limit: Option<usize>,
    pub generation_group_base_chunks_per_axis: Option<i32>,
    pub max_chunks_per_generation_task: Option<usize>,
}

#[derive(Debug, Clone, Default)]
pub struct VoxelManifestationConfigOverrides {
    pub grouping: VoxelManifestationGroupingConfigOverrides,
    pub rebuild_budget_per_frame: Option<usize>,
    pub physics_interaction_radius_native: Option<f32>,
}

#[derive(Debug, Clone, Default)]
pub struct VoxelManifestationGroupingConfigOverrides {
    pub strategy: Option<VoxelGroupingStrategy>,
    pub base_chunks_per_axis: Option<i32>,
}

#[derive(Resource, Debug)]
struct EngineConfigSource {
    path: PathBuf,
    file_config: EngineConfig,
    last_modified: Option<SystemTime>,
    reload_elapsed: f32,
}

impl EngineConfigSource {
    fn new() -> Self {
        Self {
            path: configured_path(),
            file_config: EngineConfig::default(),
            last_modified: None,
            reload_elapsed: 0.0,
        }
    }
}

pub struct EngineConfigPlugin;

impl Plugin for EngineConfigPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EngineConfig>()
            .init_resource::<EngineConfigOverrides>()
            .insert_resource(EngineConfigSource::new())
            .add_systems(Startup, initialize_engine_config)
            .add_systems(PreUpdate, reload_engine_config);
    }
}

fn initialize_engine_config(
    mut source: ResMut<EngineConfigSource>,
    overrides: Res<EngineConfigOverrides>,
    mut effective: ResMut<EngineConfig>,
) {
    source.last_modified = modified_time(&source.path);

    match load_project_config(&source.path) {
        Ok(config) => {
            source.file_config = config;
            info!(path = %source.path.display(), "loaded engine runtime configuration");
        }
        Err(error) => {
            warn!(
                path = %source.path.display(),
                %error,
                "using compiled engine configuration defaults"
            );
        }
    }

    rebuild_effective_config(&source.file_config, &overrides, &mut effective);
}

fn reload_engine_config(
    time: Res<Time>,
    overrides: Res<EngineConfigOverrides>,
    mut source: ResMut<EngineConfigSource>,
    mut effective: ResMut<EngineConfig>,
) {
    source.reload_elapsed += time.delta_secs();
    let overrides_changed = overrides.is_changed();
    let mut source_changed = false;

    if source.reload_elapsed >= CONFIG_RELOAD_INTERVAL_SECONDS {
        source.reload_elapsed %= CONFIG_RELOAD_INTERVAL_SECONDS;

        let modified = modified_time(&source.path);
        if modified != source.last_modified {
            source.last_modified = modified;
            match load_project_config(&source.path) {
                Ok(config) => {
                    source.file_config = config;
                    source_changed = true;
                    info!(path = %source.path.display(), "hot-reloaded engine runtime configuration");
                }
                Err(error) => {
                    warn!(
                        path = %source.path.display(),
                        %error,
                        "engine config reload rejected; retaining last valid configuration"
                    );
                }
            }
        }
    }

    if source_changed || overrides_changed {
        rebuild_effective_config(&source.file_config, &overrides, &mut effective);
    }
}

fn rebuild_effective_config(
    file_config: &EngineConfig,
    overrides: &EngineConfigOverrides,
    effective: &mut EngineConfig,
) {
    let mut candidate = file_config.clone();
    apply_runtime_overrides(&mut candidate, overrides);

    if let Err(error) = validate_engine_config(&candidate) {
        warn!(%error, "engine runtime overrides rejected; retaining last valid configuration");
        return;
    }

    if *effective != candidate {
        *effective = candidate;
    }
}

fn apply_runtime_overrides(config: &mut EngineConfig, overrides: &EngineConfigOverrides) {
    let streaming = &overrides.voxel.streaming;
    if let Some(value) = streaming.default_load_budget_per_frame {
        config.voxel.streaming.default_load_budget_per_frame = value;
    }
    if let Some(value) = streaming.generation_publish_budget_per_frame {
        config.voxel.streaming.generation_publish_budget_per_frame = value;
    }
    if let Some(value) = streaming.warm_inactive_materialization_limit {
        config.voxel.streaming.warm_inactive_materialization_limit = value;
    }
    if let Some(value) = streaming.generation_group_base_chunks_per_axis {
        config.voxel.streaming.generation_group_base_chunks_per_axis = value;
    }
    if let Some(value) = streaming.max_chunks_per_generation_task {
        config.voxel.streaming.max_chunks_per_generation_task = value;
    }

    let manifestation = &overrides.voxel.manifestation;
    if let Some(value) = manifestation.grouping.strategy {
        config.voxel.manifestation.grouping.strategy = value;
    }
    if let Some(value) = manifestation.grouping.base_chunks_per_axis {
        config.voxel.manifestation.grouping.base_chunks_per_axis = value;
    }
    if let Some(value) = manifestation.rebuild_budget_per_frame {
        config.voxel.manifestation.rebuild_budget_per_frame = value;
    }
    if let Some(value) = manifestation.physics_interaction_radius_native {
        config.voxel.manifestation.physics_interaction_radius_native = value;
    }
}

fn load_project_config(path: &Path) -> Result<EngineConfig, String> {
    let source =
        fs::read_to_string(path).map_err(|error| format!("could not read config: {error}"))?;
    let config = ron::from_str::<EngineConfig>(&source)
        .map_err(|error| format!("could not parse RON config: {error}"))?;
    validate_engine_config(&config)?;
    Ok(config)
}

fn validate_engine_config(config: &EngineConfig) -> Result<(), String> {
    let streaming = config.voxel.streaming;
    if streaming.default_load_budget_per_frame == 0 {
        return Err("voxel.streaming.default_load_budget_per_frame must be > 0".into());
    }
    if streaming.generation_publish_budget_per_frame == 0 {
        return Err("voxel.streaming.generation_publish_budget_per_frame must be > 0".into());
    }
    if streaming.max_chunks_per_generation_task == 0 {
        return Err("voxel.streaming.max_chunks_per_generation_task must be > 0".into());
    }
    validate_aligned_group_edge(
        streaming.generation_group_base_chunks_per_axis,
        "voxel.streaming.generation_group_base_chunks_per_axis",
    )?;

    let manifestation = config.voxel.manifestation;
    if manifestation.rebuild_budget_per_frame == 0 {
        return Err("voxel.manifestation.rebuild_budget_per_frame must be > 0".into());
    }
    if !manifestation.physics_interaction_radius_native.is_finite()
        || manifestation.physics_interaction_radius_native <= 0.0
    {
        return Err(
            "voxel.manifestation.physics_interaction_radius_native must be finite and > 0".into(),
        );
    }
    validate_aligned_group_edge(
        manifestation.grouping.base_chunks_per_axis,
        "voxel.manifestation.grouping.base_chunks_per_axis",
    )?;

    Ok(())
}

fn validate_aligned_group_edge(value: i32, field: &str) -> Result<(), String> {
    const MATERIALIZATION_ATOMS_PER_USF_DIGIT: i32 = 100;

    if value <= 0
        || value > MATERIALIZATION_ATOMS_PER_USF_DIGIT
        || MATERIALIZATION_ATOMS_PER_USF_DIGIT % value != 0
    {
        return Err(format!(
            "{field} must be one of the positive divisors of 100 (for example 1, 2, 4, 5, 10, 20, 25, 50, 100); got {value}"
        ));
    }
    Ok(())
}

fn modified_time(path: &Path) -> Option<SystemTime> {
    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
}

fn configured_path() -> PathBuf {
    if let Some(path) = env::var_os("SPACETIME_ENGINE_CONFIG") {
        return PathBuf::from(path);
    }

    for candidate in [
        PathBuf::from("assets/config/engine.ron"),
        PathBuf::from("spacetime-engine/assets/config/engine.ron"),
    ] {
        if candidate.exists() {
            return candidate;
        }
    }

    PathBuf::from("assets/config/engine.ron")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_valid() {
        validate_engine_config(&EngineConfig::default()).unwrap();
    }

    #[test]
    fn partial_ron_inherits_structured_defaults() {
        let config: EngineConfig = ron::from_str(
            r#"(
                voxel: (
                    manifestation: (
                        grouping: (
                            base_chunks_per_axis: 20,
                        ),
                    ),
                ),
            )"#,
        )
        .unwrap();

        assert_eq!(config.voxel.manifestation.grouping.base_chunks_per_axis, 20);
        assert_eq!(config.voxel.manifestation.rebuild_budget_per_frame, 8);
        assert_eq!(config.voxel.streaming.default_load_budget_per_frame, 24);
        validate_engine_config(&config).unwrap();
    }

    #[test]
    fn invalid_alignment_is_rejected() {
        let mut config = EngineConfig::default();
        config.voxel.manifestation.grouping.base_chunks_per_axis = 3;
        assert!(validate_engine_config(&config).is_err());
    }

    #[test]
    fn runtime_overrides_are_highest_priority() {
        let mut config = EngineConfig::default();
        let mut overrides = EngineConfigOverrides::default();
        overrides.voxel.manifestation.grouping.base_chunks_per_axis = Some(20);

        apply_runtime_overrides(&mut config, &overrides);
        assert_eq!(config.voxel.manifestation.grouping.base_chunks_per_axis, 20);
    }
}
