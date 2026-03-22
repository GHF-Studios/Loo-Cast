use crate::bevy::prelude::{Mut, Time, Virtual, World as BevyWorld};
use crate::rhai_binding::engine::preprocess::preprocess_script_source;
use crate::rhai_binding::engine::resources::MainScriptEngineHandle;
use crate::rhai_binding::runtime::ecs::world::bindings::types::World;
use crate::rhai_binding::value_semantics::access_cell::{AccessCell, Scoped};
use crate::usf::phenomenon::PhenomenonDebugStats;
use crate::usf::zone::{ZoneRealizationState, ZoneRuntimeState, ZoneTemporalContext};
use rhai::{Dynamic, Engine, EvalAltResult};
use std::path::{Path, PathBuf};

fn collect_rhai_files_recursive(dir: &Path, out: &mut Vec<PathBuf>) {
    let entries = std::fs::read_dir(dir).unwrap_or_else(|e| panic!("Failed to read companion hook dir '{}': {e}", dir.display()));

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
    if name.ends_with(".hook.rhai") {
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

fn compose_hook_source(path: &str) -> String {
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
            let code = std::fs::read_to_string(&file).unwrap_or_else(|e| panic!("Failed to read companion hook file '{}': {e}", file.display()));
            let code = preprocess_script_source(&code, &file.display().to_string());
            source_parts.push(code);
        }
    }

    let main_hook_code = std::fs::read_to_string(&script_path).unwrap_or_else(|e| panic!("Failed to read hook file '{}': {e}", script_path.display()));
    let main_hook_code = preprocess_script_source(&main_hook_code, &script_path.display().to_string());
    source_parts.push(main_hook_code);

    source_parts.join("\n\n")
}

#[inline]
fn hook_name_from_path(path: &Path) -> String {
    path.file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("unknown_hook")
        .to_string()
}

#[inline]
fn hook_domain_and_stage(hook_name: &str) -> (&str, &str) {
    if let Some(rest) = hook_name.strip_prefix("substrate_") {
        return ("substrate", rest);
    }
    if let Some(rest) = hook_name.strip_prefix("zone_") {
        return ("zone", rest);
    }
    if let Some(rest) = hook_name.strip_prefix("phenomenon_") {
        return ("phenomenon", rest);
    }
    ("global", hook_name)
}

#[derive(Clone, Debug)]
struct HookCommonParams {
    hook_name: String,
    hook_file: String,
    domain: String,
    stage: String,
    delta_seconds: rhai::FLOAT,
    elapsed_seconds: rhai::FLOAT,
    has_virtual_time: bool,
}

#[derive(Clone, Debug)]
struct GlobalHookParams {
    common: HookCommonParams,
}

#[derive(Clone, Debug)]
struct SubstrateHookParams {
    common: HookCommonParams,
    active_scale_index: rhai::INT,
    has_active_scale: bool,
}

#[derive(Clone, Debug)]
struct ZoneHookParams {
    common: HookCommonParams,
    active_scale_index: rhai::INT,
    has_active_scale: bool,
    loaded_zone_count: rhai::INT,
    realized_zone_count: rhai::INT,
}

#[derive(Clone, Debug)]
struct PhenomenonHookParams {
    common: HookCommonParams,
    active_scale_index: rhai::INT,
    has_active_scale: bool,
    active_node_count: rhai::INT,
    active_frontier_proxy_count: rhai::INT,
}

#[inline]
fn active_scale_context(world: &BevyWorld) -> (rhai::INT, bool) {
    let Some(temporal_context) = world.get_resource::<ZoneTemporalContext>() else {
        return (0, false);
    };
    (temporal_context.active_scale.index_from_top() as rhai::INT, true)
}

fn build_hook_params(world: &BevyWorld, hook_name: &str, hook_file: &str) -> Dynamic {
    let (domain, stage) = hook_domain_and_stage(hook_name);
    let (delta_seconds, elapsed_seconds, has_virtual_time) = if let Some(time) = world.get_resource::<Time<Virtual>>() {
        (time.delta_secs() as rhai::FLOAT, time.elapsed_secs() as rhai::FLOAT, true)
    } else {
        (0.0, 0.0, false)
    };
    let common = HookCommonParams {
        hook_name: hook_name.to_string(),
        hook_file: hook_file.to_string(),
        domain: domain.to_string(),
        stage: stage.to_string(),
        delta_seconds,
        elapsed_seconds,
        has_virtual_time,
    };
    let (active_scale_index, has_active_scale) = active_scale_context(world);

    match domain {
        "substrate" => Dynamic::from(SubstrateHookParams {
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
            Dynamic::from(ZoneHookParams {
                common,
                active_scale_index,
                has_active_scale,
                loaded_zone_count,
                realized_zone_count,
            })
        }
        "phenomenon" => {
            let (active_node_count, active_frontier_proxy_count) = world
                .get_resource::<PhenomenonDebugStats>()
                .map(|stats| (stats.active_nodes as rhai::INT, stats.active_frontier_proxies as rhai::INT))
                .unwrap_or((0, 0));
            Dynamic::from(PhenomenonHookParams {
                common,
                active_scale_index,
                has_active_scale,
                active_node_count,
                active_frontier_proxy_count,
            })
        }
        _ => Dynamic::from(GlobalHookParams { common }),
    }
}

pub(in super::super) fn register_hook_param_types(engine: &mut Engine) {
    engine.register_type_with_name::<HookCommonParams>("HookCommonParams");
    engine.register_get("hook_name", |params: &mut HookCommonParams| params.hook_name.clone());
    engine.register_get("hook_file", |params: &mut HookCommonParams| params.hook_file.clone());
    engine.register_get("domain", |params: &mut HookCommonParams| params.domain.clone());
    engine.register_get("stage", |params: &mut HookCommonParams| params.stage.clone());
    engine.register_get("delta_seconds", |params: &mut HookCommonParams| params.delta_seconds);
    engine.register_get("elapsed_seconds", |params: &mut HookCommonParams| params.elapsed_seconds);
    engine.register_get("has_virtual_time", |params: &mut HookCommonParams| params.has_virtual_time);

    engine.register_type_with_name::<GlobalHookParams>("GlobalHookParams");
    engine.register_get("common", |params: &mut GlobalHookParams| params.common.clone());
    engine.register_get("hook_name", |params: &mut GlobalHookParams| params.common.hook_name.clone());
    engine.register_get("hook_file", |params: &mut GlobalHookParams| params.common.hook_file.clone());
    engine.register_get("domain", |params: &mut GlobalHookParams| params.common.domain.clone());
    engine.register_get("stage", |params: &mut GlobalHookParams| params.common.stage.clone());

    engine.register_type_with_name::<SubstrateHookParams>("SubstrateHookParams");
    engine.register_get("common", |params: &mut SubstrateHookParams| params.common.clone());
    engine.register_get("hook_name", |params: &mut SubstrateHookParams| params.common.hook_name.clone());
    engine.register_get("hook_file", |params: &mut SubstrateHookParams| params.common.hook_file.clone());
    engine.register_get("domain", |params: &mut SubstrateHookParams| params.common.domain.clone());
    engine.register_get("stage", |params: &mut SubstrateHookParams| params.common.stage.clone());
    engine.register_get("delta_seconds", |params: &mut SubstrateHookParams| params.common.delta_seconds);
    engine.register_get("elapsed_seconds", |params: &mut SubstrateHookParams| params.common.elapsed_seconds);
    engine.register_get("has_virtual_time", |params: &mut SubstrateHookParams| params.common.has_virtual_time);
    engine.register_get("active_scale_index", |params: &mut SubstrateHookParams| params.active_scale_index);
    engine.register_get("has_active_scale", |params: &mut SubstrateHookParams| params.has_active_scale);

    engine.register_type_with_name::<ZoneHookParams>("ZoneHookParams");
    engine.register_get("common", |params: &mut ZoneHookParams| params.common.clone());
    engine.register_get("hook_name", |params: &mut ZoneHookParams| params.common.hook_name.clone());
    engine.register_get("hook_file", |params: &mut ZoneHookParams| params.common.hook_file.clone());
    engine.register_get("domain", |params: &mut ZoneHookParams| params.common.domain.clone());
    engine.register_get("stage", |params: &mut ZoneHookParams| params.common.stage.clone());
    engine.register_get("delta_seconds", |params: &mut ZoneHookParams| params.common.delta_seconds);
    engine.register_get("elapsed_seconds", |params: &mut ZoneHookParams| params.common.elapsed_seconds);
    engine.register_get("has_virtual_time", |params: &mut ZoneHookParams| params.common.has_virtual_time);
    engine.register_get("active_scale_index", |params: &mut ZoneHookParams| params.active_scale_index);
    engine.register_get("has_active_scale", |params: &mut ZoneHookParams| params.has_active_scale);
    engine.register_get("loaded_zone_count", |params: &mut ZoneHookParams| params.loaded_zone_count);
    engine.register_get("realized_zone_count", |params: &mut ZoneHookParams| params.realized_zone_count);

    engine.register_type_with_name::<PhenomenonHookParams>("PhenomenonHookParams");
    engine.register_get("common", |params: &mut PhenomenonHookParams| params.common.clone());
    engine.register_get("hook_name", |params: &mut PhenomenonHookParams| params.common.hook_name.clone());
    engine.register_get("hook_file", |params: &mut PhenomenonHookParams| params.common.hook_file.clone());
    engine.register_get("domain", |params: &mut PhenomenonHookParams| params.common.domain.clone());
    engine.register_get("stage", |params: &mut PhenomenonHookParams| params.common.stage.clone());
    engine.register_get("delta_seconds", |params: &mut PhenomenonHookParams| params.common.delta_seconds);
    engine.register_get("elapsed_seconds", |params: &mut PhenomenonHookParams| params.common.elapsed_seconds);
    engine.register_get("has_virtual_time", |params: &mut PhenomenonHookParams| params.common.has_virtual_time);
    engine.register_get("active_scale_index", |params: &mut PhenomenonHookParams| params.active_scale_index);
    engine.register_get("has_active_scale", |params: &mut PhenomenonHookParams| params.has_active_scale);
    engine.register_get("active_node_count", |params: &mut PhenomenonHookParams| params.active_node_count);
    engine.register_get("active_frontier_proxy_count", |params: &mut PhenomenonHookParams| {
        params.active_frontier_proxy_count
    });
}

fn is_missing_two_arg_main(error: &EvalAltResult) -> bool {
    fn signature_targets_main(signature: &str) -> bool {
        let trimmed = signature.trim();
        if trimmed == "main" {
            return true;
        }
        let function_name = trimmed.split([' ', '(']).next().unwrap_or_default();
        function_name == "main"
    }

    match error {
        EvalAltResult::ErrorFunctionNotFound(signature, _) => signature_targets_main(signature),
        EvalAltResult::ErrorInFunctionCall(_, _, inner, _) => is_missing_two_arg_main(inner),
        EvalAltResult::ErrorInModule(_, inner, _) => is_missing_two_arg_main(inner),
        _ => false,
    }
}

pub(in super::super) fn new_hook_runner_system(path: String) -> impl FnMut(&mut BevyWorld) {
    move |world: &mut BevyWorld| {
        world.resource_scope(|source_world, mut engine: Mut<MainScriptEngineHandle>| {
            let engine = &mut engine.0;
            let hook_code = compose_hook_source(&path);
            let ast = engine.compile(&hook_code).unwrap();
            let mut scope = rhai::Scope::new();
            let script_path = PathBuf::from(&path);
            let hook_name = hook_name_from_path(&script_path);
            let hook_params = build_hook_params(source_world, &hook_name, &path);

            let world = std::mem::take(source_world);
            let world_raw_handle: AccessCell<Scoped, BevyWorld> = AccessCell::new(world);
            let world_binding = World {
                world: world_raw_handle.clone(),
            };

            match engine.call_fn::<()>(&mut scope, &ast, "main", (world_binding.clone(), hook_params)) {
                Ok(()) => {}
                Err(error) if is_missing_two_arg_main(error.as_ref()) => {
                    if let Err(error) = engine.call_fn::<()>(&mut scope, &ast, "main", (world_binding,)) {
                        panic!("Failed to run legacy hook entrypoint 'main(world)' for '{}': {error}", path);
                    }
                }
                Err(error) => {
                    panic!("Failed to run hook entrypoint 'main(world, params)' for '{}': {error}", path);
                }
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
    fn hook_domain_stage_mapping_handles_usf_domains() {
        assert_eq!(hook_domain_and_stage("substrate_update"), ("substrate", "update"));
        assert_eq!(hook_domain_and_stage("zone_pre_update"), ("zone", "pre_update"));
        assert_eq!(hook_domain_and_stage("phenomenon_update"), ("phenomenon", "update"));
        assert_eq!(hook_domain_and_stage("startup"), ("global", "startup"));
    }

    #[test]
    fn missing_two_arg_main_detection_matches_only_main_signature() {
        let missing_main = EvalAltResult::ErrorFunctionNotFound("main (core_mod_api::ecs::World, map)".to_string(), rhai::Position::NONE);
        let missing_main_untyped = EvalAltResult::ErrorFunctionNotFound("main".to_string(), rhai::Position::NONE);
        let missing_other = EvalAltResult::ErrorFunctionNotFound("zone_update_tick (map)".to_string(), rhai::Position::NONE);

        assert!(is_missing_two_arg_main(&missing_main));
        assert!(is_missing_two_arg_main(&missing_main_untyped));
        assert!(!is_missing_two_arg_main(&missing_other));
    }

    #[test]
    fn build_hook_params_returns_domain_specific_types() {
        let mut world = BevyWorld::new();
        world.insert_resource(DummyTimeContext);

        let substrate = build_hook_params(&world, "substrate_update", "substrate_update.rhai");
        let zone = build_hook_params(&world, "zone_update", "zone_update.rhai");
        let phenomenon = build_hook_params(&world, "phenomenon_update", "phenomenon_update.rhai");
        let global = build_hook_params(&world, "startup", "startup.rhai");

        assert!(substrate.is::<SubstrateHookParams>());
        assert!(zone.is::<ZoneHookParams>());
        assert!(phenomenon.is::<PhenomenonHookParams>());
        assert!(global.is::<GlobalHookParams>());
    }
}
