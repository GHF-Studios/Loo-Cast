//! Project-file loading and hot reload.

use std::{
    env, fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

use bevy::prelude::*;

use super::{EngineConfig, EngineConfigOverrides};

const CONFIG_RELOAD_INTERVAL_SECONDS: f32 = 0.5;

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

/// Installs typed project configuration and hot reload.
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
        source_changed = reload_project_file_if_changed(&mut source);
    }

    if source_changed || overrides_changed {
        rebuild_effective_config(&source.file_config, &overrides, &mut effective);
    }
}

fn reload_project_file_if_changed(source: &mut EngineConfigSource) -> bool {
    let modified = modified_time(&source.path);
    if modified == source.last_modified {
        return false;
    }

    source.last_modified = modified;
    match load_project_config(&source.path) {
        Ok(config) => {
            source.file_config = config;
            info!(path = %source.path.display(), "hot-reloaded engine runtime configuration");
            true
        }
        Err(error) => {
            warn!(
                path = %source.path.display(),
                %error,
                "engine config reload rejected; retaining last valid configuration"
            );
            false
        }
    }
}

fn rebuild_effective_config(
    file_config: &EngineConfig,
    overrides: &EngineConfigOverrides,
    effective: &mut EngineConfig,
) {
    let mut candidate = file_config.clone();
    candidate.apply_overrides(overrides);

    if let Err(error) = candidate.validate() {
        warn!(%error, "engine runtime overrides rejected; retaining last valid configuration");
        return;
    }

    if *effective != candidate {
        *effective = candidate;
    }
}

fn load_project_config(path: &Path) -> Result<EngineConfig, String> {
    let source =
        fs::read_to_string(path).map_err(|error| format!("could not read config: {error}"))?;
    let config = ron::from_str::<EngineConfig>(&source)
        .map_err(|error| format!("could not parse RON config: {error}"))?;
    config.validate()?;
    Ok(config)
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

    [
        PathBuf::from("assets/config/engine.ron"),
        PathBuf::from("spacetime-engine/assets/config/engine.ron"),
    ]
    .into_iter()
    .find(|candidate| candidate.exists())
    .unwrap_or_else(|| PathBuf::from("assets/config/engine.ron"))
}
