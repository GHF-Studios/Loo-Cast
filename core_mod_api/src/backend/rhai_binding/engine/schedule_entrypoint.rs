use crate::bevy::prelude::{Mut, Time, Virtual, World as BevyWorld};
use crate::usf::chunk::realization::output_channels::ChunkRealizationChannelTelemetry;
use crate::rhai_binding::engine::preprocess::preprocess_script_source;
use crate::rhai_binding::engine::resources::MainScriptEngineHandle;
use crate::rhai_binding::engine::statics::USF_BOOTSTRAP_REPORT;
use crate::rhai_binding::runtime::ecs::world::bindings::types::World;
use crate::rhai_binding::value_semantics::access_cell::{AccessCell, Scoped};
use crate::usf::phenomenon::PhenomenonDebugStats;
use crate::usf::zone::{ZoneRealizationState, ZoneRuntimeState, ZoneTemporalContext};
use rhai::{Dynamic, Engine};
use std::path::{Path, PathBuf};

fn collect_rhai_files_recursive(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = std::fs::read_dir(dir).unwrap_or_else(|e| panic!("Failed to read companion schedule entrypoint dir '{}': {e}", dir.display()));

    for entry in entries {
        let Ok(entry) = entry else {
            continue;
        };
        let path = entry.path();
        if path.is_dir() {
            collect_rhai_files_recursive(&path, out);
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) == Some("rhai") {
            out.push(path);
        }
    }
}

#[inline]
fn companion_file_type_priority(path: &Path) -> u8 {
    let name = path.file_name().and_then(|name| name.to_str()).unwrap_or_default();
    if name.ends_with(".lib.rhai") {
        return 0;
    }
    if name.ends_with(".entrypoint.rhai") {
        return 1;
    }
    if name.ends_with(".substrate.rhai") {
        return 2;
    }
    if name.ends_with(".zone.rhai") {
        return 3;
    }
    if name.ends_with(".phenomenon.rhai") {
        return 4;
    }
    5
}

fn compose_schedule_entrypoint_source(path: &str) -> String {
    let script_path = PathBuf::from(path);
    let companion_dir = script_path.with_extension("");

    let mut source_parts: Vec<String> = Vec::new();

    if companion_dir.is_dir() {
        let mut companion_files = Vec::new();
        collect_rhai_files_recursive(&companion_dir, &mut companion_files);
        companion_files.sort_by(|a, b| {
            companion_file_type_priority(a)
                .cmp(&companion_file_type_priority(b))
                .then_with(|| a.to_string_lossy().cmp(&b.to_string_lossy()))
        });

        for file in companion_files {
            let code = std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("Failed to read companion schedule entrypoint file '{}': {e}", file.display()));
            let code = preprocess_script_source(&code, &file.display().to_string());
            source_parts.push(code);
        }
    }

    let main_entrypoint_code =
        std::fs::read_to_string(&script_path).unwrap_or_else(|e| panic!("Failed to read schedule entrypoint file '{}': {e}", script_path.display()));
    let main_entrypoint_code = preprocess_script_source(&main_entrypoint_code, &script_path.display().to_string());
    source_parts.push(main_entrypoint_code);

    source_parts.join("\n\n")
}

#[inline]
fn schedule_entrypoint_name_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("unknown_schedule_entrypoint")
        .to_string()
}

#[inline]
fn schedule_entrypoint_domain_and_stage(entrypoint_name: &str) -> (&str, &str) {
    if let Some(rest) = entrypoint_name.strip_prefix("substrate_") {
        return ("substrate", rest);
    }
    if let Some(rest) = entrypoint_name.strip_prefix("zone_") {
        return ("zone", rest);
    }
    if let Some(rest) = entrypoint_name.strip_prefix("phenomenon_") {
        return ("phenomenon", rest);
    }
    ("global", entrypoint_name)
}

#[derive(Clone, Debug)]
struct ScheduleEntrypointCommonParams {
    entrypoint_name: String,
    entrypoint_file: String,
    domain: String,
    stage: String,
    delta_seconds: rhai::FLOAT,
    elapsed_seconds: rhai::FLOAT,
    has_virtual_time: bool,
    chunk_realization_mesh_instances: rhai::INT,
    chunk_realization_material_instances: rhai::INT,
    chunk_realization_collider_instances: rhai::INT,
    chunk_realization_audio_emitters: rhai::INT,
    chunk_realization_particle_emitters: rhai::INT,
    chunk_realization_interaction_triggers: rhai::INT,
    chunk_realization_simulation_services: rhai::INT,
    bootstrap_global_script_count: rhai::INT,
    bootstrap_package_script_count: rhai::INT,
    bootstrap_selected_mod_count: rhai::INT,
    bootstrap_executed_entrypoint_count: rhai::INT,
}

#[derive(Clone, Debug)]
struct GlobalScheduleEntrypointParams {
    common: ScheduleEntrypointCommonParams,
}

#[derive(Clone, Debug)]
struct SubstrateScheduleEntrypointParams {
    common: ScheduleEntrypointCommonParams,
    active_scale_index: rhai::INT,
    has_active_scale: bool,
}

#[derive(Clone, Debug)]
struct ZoneScheduleEntrypointParams {
    common: ScheduleEntrypointCommonParams,
    active_scale_index: rhai::INT,
    has_active_scale: bool,
    loaded_zone_count: rhai::INT,
    realized_zone_count: rhai::INT,
}

#[derive(Clone, Debug)]
struct PhenomenonScheduleEntrypointParams {
    common: ScheduleEntrypointCommonParams,
    active_scale_index: rhai::INT,
    has_active_scale: bool,
    active_node_count: rhai::INT,
    active_frontier_proxy_count: rhai::INT,
    partitioned_root_model_count: rhai::INT,
    partitioned_member_model_count: rhai::INT,
}

#[inline]
fn active_scale_context(world: &BevyWorld) -> (rhai::INT, bool) {
    let Some(temporal_context) = world.get_resource::<ZoneTemporalContext>() else {
        return (0, false);
    };
    (temporal_context.active_scale.index_from_top() as rhai::INT, true)
}

fn build_schedule_entrypoint_params(world: &BevyWorld, entrypoint_name: &str, entrypoint_file: &str) -> Dynamic {
    let (domain, stage) = schedule_entrypoint_domain_and_stage(entrypoint_name);
    let (delta_seconds, elapsed_seconds, has_virtual_time) = if let Some(time) = world.get_resource::<Time<Virtual>>() {
        (time.delta_secs() as rhai::FLOAT, time.elapsed_secs() as rhai::FLOAT, true)
    } else {
        (0.0, 0.0, false)
    };
    let (
        chunk_realization_mesh_instances,
        chunk_realization_material_instances,
        chunk_realization_collider_instances,
        chunk_realization_audio_emitters,
        chunk_realization_particle_emitters,
        chunk_realization_interaction_triggers,
        chunk_realization_simulation_services,
    ) = world
        .get_resource::<ChunkRealizationChannelTelemetry>()
        .map(|telemetry| {
            (
                telemetry.mesh_instances as rhai::INT,
                telemetry.material_instances as rhai::INT,
                telemetry.collider_instances as rhai::INT,
                telemetry.audio_emitters as rhai::INT,
                telemetry.particle_emitters as rhai::INT,
                telemetry.interaction_triggers as rhai::INT,
                telemetry.simulation_services as rhai::INT,
            )
        })
        .unwrap_or((0, 0, 0, 0, 0, 0, 0));
    let (bootstrap_global_script_count, bootstrap_package_script_count, bootstrap_selected_mod_count, bootstrap_executed_entrypoint_count) = {
        let report = USF_BOOTSTRAP_REPORT().lock().unwrap();
        (
            report.discovered_global_scripts.len() as rhai::INT,
            report.discovered_package_scripts.len() as rhai::INT,
            report.selected_mod_ids.len() as rhai::INT,
            report.executed_entrypoints.len() as rhai::INT,
        )
    };
    let common = ScheduleEntrypointCommonParams {
        entrypoint_name: entrypoint_name.to_string(),
        entrypoint_file: entrypoint_file.to_string(),
        domain: domain.to_string(),
        stage: stage.to_string(),
        delta_seconds,
        elapsed_seconds,
        has_virtual_time,
        chunk_realization_mesh_instances,
        chunk_realization_material_instances,
        chunk_realization_collider_instances,
        chunk_realization_audio_emitters,
        chunk_realization_particle_emitters,
        chunk_realization_interaction_triggers,
        chunk_realization_simulation_services,
        bootstrap_global_script_count,
        bootstrap_package_script_count,
        bootstrap_selected_mod_count,
        bootstrap_executed_entrypoint_count,
    };
    let (active_scale_index, has_active_scale) = active_scale_context(world);

    match domain {
        "substrate" => Dynamic::from(SubstrateScheduleEntrypointParams {
            common,
            active_scale_index,
            has_active_scale,
        }),
        "zone" => {
            let loaded_zone_count = world
                .get_resource::<ZoneRuntimeState>()
                .map(|state| state.records.len() as rhai::INT)
                .unwrap_or_default();
            let realized_zone_count = world
                .get_resource::<ZoneRealizationState>()
                .map(|state| state.zone_to_phenomenon.len() as rhai::INT)
                .unwrap_or_default();
            Dynamic::from(ZoneScheduleEntrypointParams {
                common,
                active_scale_index,
                has_active_scale,
                loaded_zone_count,
                realized_zone_count,
            })
        }
        "phenomenon" => {
            let (active_node_count, active_frontier_proxy_count, partitioned_root_model_count, partitioned_member_model_count) = world
                .get_resource::<PhenomenonDebugStats>()
                .map(|stats| {
                    (
                        stats.active_nodes as rhai::INT,
                        stats.active_frontier_proxies as rhai::INT,
                        stats.partitioned_root_models as rhai::INT,
                        stats.partitioned_member_models as rhai::INT,
                    )
                })
                .unwrap_or((0, 0, 0, 0));
            Dynamic::from(PhenomenonScheduleEntrypointParams {
                common,
                active_scale_index,
                has_active_scale,
                active_node_count,
                active_frontier_proxy_count,
                partitioned_root_model_count,
                partitioned_member_model_count,
            })
        }
        _ => Dynamic::from(GlobalScheduleEntrypointParams { common }),
    }
}

pub(in super::super) fn register_schedule_entrypoint_param_types(engine: &mut Engine) {
    engine.register_type_with_name::<ScheduleEntrypointCommonParams>("ScheduleEntrypointCommonParams");
    engine.register_get("entrypoint_name", |params: &mut ScheduleEntrypointCommonParams| params.entrypoint_name.clone());
    engine.register_get("entrypoint_file", |params: &mut ScheduleEntrypointCommonParams| params.entrypoint_file.clone());
    engine.register_get("domain", |params: &mut ScheduleEntrypointCommonParams| params.domain.clone());
    engine.register_get("stage", |params: &mut ScheduleEntrypointCommonParams| params.stage.clone());
    engine.register_get("delta_seconds", |params: &mut ScheduleEntrypointCommonParams| params.delta_seconds);
    engine.register_get("elapsed_seconds", |params: &mut ScheduleEntrypointCommonParams| params.elapsed_seconds);
    engine.register_get("has_virtual_time", |params: &mut ScheduleEntrypointCommonParams| params.has_virtual_time);
    engine.register_get("chunk_realization_mesh_instances", |params: &mut ScheduleEntrypointCommonParams| {
        params.chunk_realization_mesh_instances
    });
    engine.register_get("chunk_realization_material_instances", |params: &mut ScheduleEntrypointCommonParams| {
        params.chunk_realization_material_instances
    });
    engine.register_get("chunk_realization_collider_instances", |params: &mut ScheduleEntrypointCommonParams| {
        params.chunk_realization_collider_instances
    });
    engine.register_get("chunk_realization_audio_emitters", |params: &mut ScheduleEntrypointCommonParams| {
        params.chunk_realization_audio_emitters
    });
    engine.register_get("chunk_realization_particle_emitters", |params: &mut ScheduleEntrypointCommonParams| {
        params.chunk_realization_particle_emitters
    });
    engine.register_get("chunk_realization_interaction_triggers", |params: &mut ScheduleEntrypointCommonParams| {
        params.chunk_realization_interaction_triggers
    });
    engine.register_get("chunk_realization_simulation_services", |params: &mut ScheduleEntrypointCommonParams| {
        params.chunk_realization_simulation_services
    });
    engine.register_get("bootstrap_global_script_count", |params: &mut ScheduleEntrypointCommonParams| {
        params.bootstrap_global_script_count
    });
    engine.register_get("bootstrap_package_script_count", |params: &mut ScheduleEntrypointCommonParams| {
        params.bootstrap_package_script_count
    });
    engine.register_get("bootstrap_selected_mod_count", |params: &mut ScheduleEntrypointCommonParams| {
        params.bootstrap_selected_mod_count
    });
    engine.register_get("bootstrap_executed_entrypoint_count", |params: &mut ScheduleEntrypointCommonParams| {
        params.bootstrap_executed_entrypoint_count
    });

    engine.register_type_with_name::<GlobalScheduleEntrypointParams>("GlobalScheduleEntrypointParams");
    engine.register_get("common", |params: &mut GlobalScheduleEntrypointParams| params.common.clone());
    engine.register_get("entrypoint_name", |params: &mut GlobalScheduleEntrypointParams| {
        params.common.entrypoint_name.clone()
    });
    engine.register_get("entrypoint_file", |params: &mut GlobalScheduleEntrypointParams| {
        params.common.entrypoint_file.clone()
    });
    engine.register_get("domain", |params: &mut GlobalScheduleEntrypointParams| params.common.domain.clone());
    engine.register_get("stage", |params: &mut GlobalScheduleEntrypointParams| params.common.stage.clone());
    engine.register_get("chunk_realization_mesh_instances", |params: &mut GlobalScheduleEntrypointParams| {
        params.common.chunk_realization_mesh_instances
    });
    engine.register_get("chunk_realization_material_instances", |params: &mut GlobalScheduleEntrypointParams| {
        params.common.chunk_realization_material_instances
    });
    engine.register_get("chunk_realization_collider_instances", |params: &mut GlobalScheduleEntrypointParams| {
        params.common.chunk_realization_collider_instances
    });
    engine.register_get("chunk_realization_audio_emitters", |params: &mut GlobalScheduleEntrypointParams| {
        params.common.chunk_realization_audio_emitters
    });
    engine.register_get("chunk_realization_particle_emitters", |params: &mut GlobalScheduleEntrypointParams| {
        params.common.chunk_realization_particle_emitters
    });
    engine.register_get("chunk_realization_interaction_triggers", |params: &mut GlobalScheduleEntrypointParams| {
        params.common.chunk_realization_interaction_triggers
    });
    engine.register_get("chunk_realization_simulation_services", |params: &mut GlobalScheduleEntrypointParams| {
        params.common.chunk_realization_simulation_services
    });
    engine.register_get("bootstrap_global_script_count", |params: &mut GlobalScheduleEntrypointParams| {
        params.common.bootstrap_global_script_count
    });
    engine.register_get("bootstrap_package_script_count", |params: &mut GlobalScheduleEntrypointParams| {
        params.common.bootstrap_package_script_count
    });
    engine.register_get("bootstrap_selected_mod_count", |params: &mut GlobalScheduleEntrypointParams| {
        params.common.bootstrap_selected_mod_count
    });
    engine.register_get("bootstrap_executed_entrypoint_count", |params: &mut GlobalScheduleEntrypointParams| {
        params.common.bootstrap_executed_entrypoint_count
    });

    engine.register_type_with_name::<SubstrateScheduleEntrypointParams>("SubstrateScheduleEntrypointParams");
    engine.register_get("common", |params: &mut SubstrateScheduleEntrypointParams| params.common.clone());
    engine.register_get("entrypoint_name", |params: &mut SubstrateScheduleEntrypointParams| {
        params.common.entrypoint_name.clone()
    });
    engine.register_get("entrypoint_file", |params: &mut SubstrateScheduleEntrypointParams| {
        params.common.entrypoint_file.clone()
    });
    engine.register_get("domain", |params: &mut SubstrateScheduleEntrypointParams| params.common.domain.clone());
    engine.register_get("stage", |params: &mut SubstrateScheduleEntrypointParams| params.common.stage.clone());
    engine.register_get("delta_seconds", |params: &mut SubstrateScheduleEntrypointParams| params.common.delta_seconds);
    engine.register_get("elapsed_seconds", |params: &mut SubstrateScheduleEntrypointParams| {
        params.common.elapsed_seconds
    });
    engine.register_get("has_virtual_time", |params: &mut SubstrateScheduleEntrypointParams| {
        params.common.has_virtual_time
    });
    engine.register_get("chunk_realization_mesh_instances", |params: &mut SubstrateScheduleEntrypointParams| {
        params.common.chunk_realization_mesh_instances
    });
    engine.register_get("chunk_realization_material_instances", |params: &mut SubstrateScheduleEntrypointParams| {
        params.common.chunk_realization_material_instances
    });
    engine.register_get("chunk_realization_collider_instances", |params: &mut SubstrateScheduleEntrypointParams| {
        params.common.chunk_realization_collider_instances
    });
    engine.register_get("chunk_realization_audio_emitters", |params: &mut SubstrateScheduleEntrypointParams| {
        params.common.chunk_realization_audio_emitters
    });
    engine.register_get("chunk_realization_particle_emitters", |params: &mut SubstrateScheduleEntrypointParams| {
        params.common.chunk_realization_particle_emitters
    });
    engine.register_get("chunk_realization_interaction_triggers", |params: &mut SubstrateScheduleEntrypointParams| {
        params.common.chunk_realization_interaction_triggers
    });
    engine.register_get("chunk_realization_simulation_services", |params: &mut SubstrateScheduleEntrypointParams| {
        params.common.chunk_realization_simulation_services
    });
    engine.register_get("bootstrap_global_script_count", |params: &mut SubstrateScheduleEntrypointParams| {
        params.common.bootstrap_global_script_count
    });
    engine.register_get("bootstrap_package_script_count", |params: &mut SubstrateScheduleEntrypointParams| {
        params.common.bootstrap_package_script_count
    });
    engine.register_get("bootstrap_selected_mod_count", |params: &mut SubstrateScheduleEntrypointParams| {
        params.common.bootstrap_selected_mod_count
    });
    engine.register_get("bootstrap_executed_entrypoint_count", |params: &mut SubstrateScheduleEntrypointParams| {
        params.common.bootstrap_executed_entrypoint_count
    });
    engine.register_get("active_scale_index", |params: &mut SubstrateScheduleEntrypointParams| params.active_scale_index);
    engine.register_get("has_active_scale", |params: &mut SubstrateScheduleEntrypointParams| params.has_active_scale);

    engine.register_type_with_name::<ZoneScheduleEntrypointParams>("ZoneScheduleEntrypointParams");
    engine.register_get("common", |params: &mut ZoneScheduleEntrypointParams| params.common.clone());
    engine.register_get("entrypoint_name", |params: &mut ZoneScheduleEntrypointParams| {
        params.common.entrypoint_name.clone()
    });
    engine.register_get("entrypoint_file", |params: &mut ZoneScheduleEntrypointParams| {
        params.common.entrypoint_file.clone()
    });
    engine.register_get("domain", |params: &mut ZoneScheduleEntrypointParams| params.common.domain.clone());
    engine.register_get("stage", |params: &mut ZoneScheduleEntrypointParams| params.common.stage.clone());
    engine.register_get("delta_seconds", |params: &mut ZoneScheduleEntrypointParams| params.common.delta_seconds);
    engine.register_get("elapsed_seconds", |params: &mut ZoneScheduleEntrypointParams| params.common.elapsed_seconds);
    engine.register_get("has_virtual_time", |params: &mut ZoneScheduleEntrypointParams| params.common.has_virtual_time);
    engine.register_get("chunk_realization_mesh_instances", |params: &mut ZoneScheduleEntrypointParams| {
        params.common.chunk_realization_mesh_instances
    });
    engine.register_get("chunk_realization_material_instances", |params: &mut ZoneScheduleEntrypointParams| {
        params.common.chunk_realization_material_instances
    });
    engine.register_get("chunk_realization_collider_instances", |params: &mut ZoneScheduleEntrypointParams| {
        params.common.chunk_realization_collider_instances
    });
    engine.register_get("chunk_realization_audio_emitters", |params: &mut ZoneScheduleEntrypointParams| {
        params.common.chunk_realization_audio_emitters
    });
    engine.register_get("chunk_realization_particle_emitters", |params: &mut ZoneScheduleEntrypointParams| {
        params.common.chunk_realization_particle_emitters
    });
    engine.register_get("chunk_realization_interaction_triggers", |params: &mut ZoneScheduleEntrypointParams| {
        params.common.chunk_realization_interaction_triggers
    });
    engine.register_get("chunk_realization_simulation_services", |params: &mut ZoneScheduleEntrypointParams| {
        params.common.chunk_realization_simulation_services
    });
    engine.register_get("bootstrap_global_script_count", |params: &mut ZoneScheduleEntrypointParams| {
        params.common.bootstrap_global_script_count
    });
    engine.register_get("bootstrap_package_script_count", |params: &mut ZoneScheduleEntrypointParams| {
        params.common.bootstrap_package_script_count
    });
    engine.register_get("bootstrap_selected_mod_count", |params: &mut ZoneScheduleEntrypointParams| {
        params.common.bootstrap_selected_mod_count
    });
    engine.register_get("bootstrap_executed_entrypoint_count", |params: &mut ZoneScheduleEntrypointParams| {
        params.common.bootstrap_executed_entrypoint_count
    });
    engine.register_get("active_scale_index", |params: &mut ZoneScheduleEntrypointParams| params.active_scale_index);
    engine.register_get("has_active_scale", |params: &mut ZoneScheduleEntrypointParams| params.has_active_scale);
    engine.register_get("loaded_zone_count", |params: &mut ZoneScheduleEntrypointParams| params.loaded_zone_count);
    engine.register_get("realized_zone_count", |params: &mut ZoneScheduleEntrypointParams| params.realized_zone_count);

    engine.register_type_with_name::<PhenomenonScheduleEntrypointParams>("PhenomenonScheduleEntrypointParams");
    engine.register_get("common", |params: &mut PhenomenonScheduleEntrypointParams| params.common.clone());
    engine.register_get("entrypoint_name", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.common.entrypoint_name.clone()
    });
    engine.register_get("entrypoint_file", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.common.entrypoint_file.clone()
    });
    engine.register_get("domain", |params: &mut PhenomenonScheduleEntrypointParams| params.common.domain.clone());
    engine.register_get("stage", |params: &mut PhenomenonScheduleEntrypointParams| params.common.stage.clone());
    engine.register_get("delta_seconds", |params: &mut PhenomenonScheduleEntrypointParams| params.common.delta_seconds);
    engine.register_get("elapsed_seconds", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.common.elapsed_seconds
    });
    engine.register_get("has_virtual_time", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.common.has_virtual_time
    });
    engine.register_get("chunk_realization_mesh_instances", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.common.chunk_realization_mesh_instances
    });
    engine.register_get("chunk_realization_material_instances", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.common.chunk_realization_material_instances
    });
    engine.register_get("chunk_realization_collider_instances", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.common.chunk_realization_collider_instances
    });
    engine.register_get("chunk_realization_audio_emitters", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.common.chunk_realization_audio_emitters
    });
    engine.register_get("chunk_realization_particle_emitters", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.common.chunk_realization_particle_emitters
    });
    engine.register_get("chunk_realization_interaction_triggers", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.common.chunk_realization_interaction_triggers
    });
    engine.register_get("chunk_realization_simulation_services", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.common.chunk_realization_simulation_services
    });
    engine.register_get("bootstrap_global_script_count", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.common.bootstrap_global_script_count
    });
    engine.register_get("bootstrap_package_script_count", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.common.bootstrap_package_script_count
    });
    engine.register_get("bootstrap_selected_mod_count", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.common.bootstrap_selected_mod_count
    });
    engine.register_get("bootstrap_executed_entrypoint_count", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.common.bootstrap_executed_entrypoint_count
    });
    engine.register_get("active_scale_index", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.active_scale_index
    });
    engine.register_get("has_active_scale", |params: &mut PhenomenonScheduleEntrypointParams| params.has_active_scale);
    engine.register_get("active_node_count", |params: &mut PhenomenonScheduleEntrypointParams| params.active_node_count);
    engine.register_get("active_frontier_proxy_count", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.active_frontier_proxy_count
    });
    engine.register_get("partitioned_root_model_count", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.partitioned_root_model_count
    });
    engine.register_get("partitioned_member_model_count", |params: &mut PhenomenonScheduleEntrypointParams| {
        params.partitioned_member_model_count
    });
}

pub(in super::super) fn new_schedule_entrypoint_runner_system(path: String) -> impl FnMut(&mut BevyWorld) {
    move |world: &mut BevyWorld| {
        world.resource_scope(|source_world, mut engine: Mut<MainScriptEngineHandle>| {
            let engine = &mut engine.0;
            let entrypoint_code = compose_schedule_entrypoint_source(&path);
            let ast = engine.compile(&entrypoint_code).unwrap();
            let mut scope = rhai::Scope::new();
            let script_path = PathBuf::from(&path);
            let entrypoint_name = schedule_entrypoint_name_from_path(&script_path);
            let entrypoint_params = build_schedule_entrypoint_params(source_world, &entrypoint_name, &path);

            let world = std::mem::take(source_world);
            let world_raw_handle: AccessCell<Scoped, BevyWorld> = AccessCell::new(world);
            let world_binding = World {
                world: world_raw_handle.clone(),
            };

            if let Err(error) = engine.call_fn::<()>(&mut scope, &ast, "main", (world_binding, entrypoint_params)) {
                panic!("Failed to run schedule entrypoint 'main(world, params)' for '{}': {error}", path);
            }

            let returned_world = world_raw_handle.take();
            *source_world = returned_world;
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bevy::prelude::Resource;

    #[derive(Resource, Default)]
    struct DummyTimeContext;

    #[test]
    fn schedule_entrypoint_domain_stage_mapping_handles_usf_domains() {
        assert_eq!(schedule_entrypoint_domain_and_stage("substrate_update"), ("substrate", "update"));
        assert_eq!(schedule_entrypoint_domain_and_stage("zone_pre_update"), ("zone", "pre_update"));
        assert_eq!(schedule_entrypoint_domain_and_stage("phenomenon_update"), ("phenomenon", "update"));
        assert_eq!(schedule_entrypoint_domain_and_stage("startup"), ("global", "startup"));
    }

    #[test]
    fn build_schedule_entrypoint_params_returns_domain_specific_types() {
        let mut world = BevyWorld::new();
        world.insert_resource(DummyTimeContext);

        let substrate = build_schedule_entrypoint_params(&world, "substrate_update", "substrate_update.rhai");
        let zone = build_schedule_entrypoint_params(&world, "zone_update", "zone_update.rhai");
        let phenomenon = build_schedule_entrypoint_params(&world, "phenomenon_update", "phenomenon_update.rhai");
        let global = build_schedule_entrypoint_params(&world, "startup", "startup.rhai");

        assert!(substrate.is::<SubstrateScheduleEntrypointParams>());
        assert!(zone.is::<ZoneScheduleEntrypointParams>());
        assert!(phenomenon.is::<PhenomenonScheduleEntrypointParams>());
        assert!(global.is::<GlobalScheduleEntrypointParams>());
    }
}
