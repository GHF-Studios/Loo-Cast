use std::{path::PathBuf, sync::Arc};

use crate::bevy::ecs::schedule::IntoScheduleConfigs;
use crate::bevy::prelude::{App, First, Last, PostStartup, PostUpdate, PreStartup, PreUpdate, Startup, Update};
use crate::config::statics::CONFIG;
use crate::core::functions::asset_root;
use crate::rhai_binding::bind::engine_ext::EngineExt;
use crate::rhai_binding::engine::hook::{new_hook_runner_system, register_hook_param_types};
use crate::rhai_binding::engine::preprocess::preprocess_script_source;
use crate::rhai_binding::engine::resources::MainScriptEngineHandle;
use crate::rhai_binding::engine::statics::{
    SCHEDULE_HOOKS, ScriptDptMetricDefinition, ScriptDptSchemaDefinition, ScriptZlmMetricBandDefinition, ScriptZlmRuleDefinition, ScriptZlmScaleDefinition,
    USF_DPT_SCHEMAS_BY_SCALE, USF_ZLM_SCALES_BY_SCALE, USF_ZONE_KIND_BY_TYPE, USF_ZONE_TYPES,
};
use crate::rhai_binding::runtime::ecs::message::bindings::types::ScriptProbeMessage;
use crate::usf::scale::Scale;
use crate::usf::schedule::{UsfPhenomenonSet, UsfSubstrateSet, UsfZoneSet};
use rhai::Engine;

pub fn build(app: &mut App) {
    app.init_resource::<MainScriptEngineHandle>();
    app.add_message::<ScriptProbeMessage>();

    let path = "core_mod/scripts/core/schedule_hooks/";
    let mut abs_path = PathBuf::from(path);
    if abs_path.is_relative() {
        abs_path = asset_root().join(path);
    }
    let path = abs_path;

    for name in SCHEDULE_HOOKS().lock().unwrap().drain(..) {
        match name.as_str() {
            "pre_startup" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(PreStartup, new_hook_runner_system(file_path.display().to_string()));
            }
            "startup" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(Startup, new_hook_runner_system(file_path.display().to_string()));
            }
            "post_startup" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(PostStartup, new_hook_runner_system(file_path.display().to_string()));
            }
            "first" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(First, new_hook_runner_system(file_path.display().to_string()));
            }
            "pre_update" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(PreUpdate, new_hook_runner_system(file_path.display().to_string()));
            }
            "substrate_pre_update" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(Update, new_hook_runner_system(file_path.display().to_string()).in_set(UsfSubstrateSet::Pre));
            }
            "zone_pre_update" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(Update, new_hook_runner_system(file_path.display().to_string()).in_set(UsfZoneSet::Pre));
            }
            "phenomenon_pre_update" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(Update, new_hook_runner_system(file_path.display().to_string()).in_set(UsfPhenomenonSet::Pre));
            }
            "update" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(Update, new_hook_runner_system(file_path.display().to_string()));
            }
            "substrate_update" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(Update, new_hook_runner_system(file_path.display().to_string()).in_set(UsfSubstrateSet::Post));
            }
            "zone_update" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(Update, new_hook_runner_system(file_path.display().to_string()).in_set(UsfZoneSet::Post));
            }
            "phenomenon_update" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(Update, new_hook_runner_system(file_path.display().to_string()).in_set(UsfPhenomenonSet::Post));
            }
            "post_update" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(PostUpdate, new_hook_runner_system(file_path.display().to_string()));
            }
            "last" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(Last, new_hook_runner_system(file_path.display().to_string()));
            }
            unknown => {
                panic!("Schedule name '{unknown}' is not known!");
            }
        }
    }
}

pub(super) fn new_main_script_engine() -> Engine {
    let mut engine = Engine::new();
    let testing_enabled = CONFIG().get::<bool>("rhai_binding/testing_enabled");

    engine.register_binding_graph_with_testing(testing_enabled);
    register_runtime_bindings(&mut engine);

    let boot_script_path = "core_mod/scripts/core/boot.rhai";
    let mut abs_boot_script_path = PathBuf::from(boot_script_path);
    if abs_boot_script_path.is_relative() {
        abs_boot_script_path = asset_root().join(boot_script_path);
    }
    let boot_script_path = abs_boot_script_path.to_string_lossy().to_string();

    let boot_script = std::fs::read_to_string(&boot_script_path).unwrap();
    let boot_script = preprocess_script_source(&boot_script, &boot_script_path);
    let boot_script = engine.compile(boot_script).unwrap();
    engine.eval_ast::<()>(&boot_script).unwrap();

    engine
}

fn register_runtime_bindings(engine: &mut rhai::Engine) {
    register_hook_param_types(engine);
    register_schedule_hooks_runtime_module(engine);
    register_usf_substrate_runtime_module(engine);
    register_usf_zone_runtime_module(engine);
    register_testing_runtime_module(engine);
}

fn register_schedule_hooks_runtime_module(engine: &mut rhai::Engine) {
    let mut schedule_hooks_module = rhai::Module::new();
    schedule_hooks_module.set_native_fn("add", |hook: &str| -> Result<(), Box<rhai::EvalAltResult>> {
        let hook = hook.trim();
        if hook.is_empty() {
            return Err("schedule hook must not be empty".into());
        }
        let mut hooks = SCHEDULE_HOOKS().lock().unwrap();
        if !hooks.iter().any(|registered| registered == hook) {
            hooks.push(hook.to_string());
        }
        Ok(())
    });
    engine.register_static_module("rhai_binding::schedule_hooks", Arc::new(schedule_hooks_module));
}

fn register_testing_runtime_module(engine: &mut rhai::Engine) {
    let mut testing_module = rhai::Module::new();
    testing_module.set_native_fn("enabled", || -> Result<bool, Box<rhai::EvalAltResult>> {
        Ok(CONFIG().get::<bool>("rhai_binding/testing_enabled"))
    });
    engine.register_static_module("rhai_binding::testing", Arc::new(testing_module));
}

fn register_usf_zone_runtime_module(engine: &mut rhai::Engine) {
    let mut usf_zone_module = rhai::Module::new();
    usf_zone_module.set_native_fn(
        "set_phenomenon_kind",
        |zone_type: &str, phenomenon_kind: &str| -> Result<(), Box<rhai::EvalAltResult>> {
            let normalized_zone_type = zone_type.trim().to_ascii_lowercase();
            if normalized_zone_type.is_empty() {
                return Err("zone_type must not be empty".into());
            }
            let normalized_kind = phenomenon_kind.trim().to_ascii_lowercase();
            if normalized_kind.is_empty() {
                return Err("phenomenon_kind must not be empty".into());
            }
            USF_ZONE_TYPES().lock().unwrap().insert(normalized_zone_type.clone());
            USF_ZONE_KIND_BY_TYPE().lock().unwrap().insert(normalized_zone_type, normalized_kind);
            Ok(())
        },
    );
    usf_zone_module.set_native_fn("clear_phenomenon_kinds", || -> Result<(), Box<rhai::EvalAltResult>> {
        USF_ZONE_KIND_BY_TYPE().lock().unwrap().clear();
        Ok(())
    });
    engine.register_static_module("rhai_binding::usf_zone", Arc::new(usf_zone_module));
}

#[inline]
fn normalize_zone_type(zone_type: &str) -> Result<String, Box<rhai::EvalAltResult>> {
    let normalized_zone_type = zone_type.trim().to_ascii_lowercase();
    if normalized_zone_type.is_empty() {
        return Err("zone_type must not be empty".into());
    }
    Ok(normalized_zone_type)
}

#[inline]
fn parse_scale_index(scale_index: i64) -> Result<u8, Box<rhai::EvalAltResult>> {
    if scale_index < 0 {
        return Err(format!("scale_index must be >= 0, got {scale_index}").into());
    }
    let max_index = (Scale::SCALE_LEVEL_COUNT.saturating_sub(1)) as i64;
    if scale_index > max_index {
        return Err(format!("scale_index must be <= {max_index}, got {scale_index}").into());
    }
    Ok(scale_index as u8)
}

#[inline]
fn parse_positive_revision(revision: i64) -> Result<u64, Box<rhai::EvalAltResult>> {
    if revision <= 0 {
        return Err(format!("revision must be > 0, got {revision}").into());
    }
    Ok(revision as u64)
}

#[inline]
fn parse_u16_value(value_name: &str, value: i64) -> Result<u16, Box<rhai::EvalAltResult>> {
    if value < 0 || value > u16::MAX as i64 {
        return Err(format!("{value_name} must be in 0..={}, got {value}", u16::MAX).into());
    }
    Ok(value as u16)
}

fn register_usf_substrate_runtime_module(engine: &mut rhai::Engine) {
    let mut usf_substrate_module = rhai::Module::new();
    usf_substrate_module.set_native_fn("scale_level_count", || -> Result<rhai::INT, Box<rhai::EvalAltResult>> {
        Ok(Scale::SCALE_LEVEL_COUNT as rhai::INT)
    });
    usf_substrate_module.set_native_fn("clear_zone_types", || -> Result<(), Box<rhai::EvalAltResult>> {
        USF_ZONE_TYPES().lock().unwrap().clear();
        Ok(())
    });
    usf_substrate_module.set_native_fn("add_zone_type", |zone_type: &str| -> Result<(), Box<rhai::EvalAltResult>> {
        let normalized_zone_type = normalize_zone_type(zone_type)?;
        USF_ZONE_TYPES().lock().unwrap().insert(normalized_zone_type);
        Ok(())
    });

    usf_substrate_module.set_native_fn("clear_dpt_schemas", || -> Result<(), Box<rhai::EvalAltResult>> {
        USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap().clear();
        Ok(())
    });
    usf_substrate_module.set_native_fn(
        "set_dpt_schema",
        |scale_index: i64, revision: i64, fallback_zone: &str| -> Result<(), Box<rhai::EvalAltResult>> {
            let scale_index = parse_scale_index(scale_index)?;
            let revision = parse_positive_revision(revision)?;
            let fallback_zone = normalize_zone_type(fallback_zone)?;
            let mut schemas = USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap();
            schemas.insert(
                scale_index,
                ScriptDptSchemaDefinition {
                    revision,
                    fallback_zone,
                    metrics: Vec::new(),
                },
            );
            Ok(())
        },
    );
    usf_substrate_module.set_native_fn(
        "add_dpt_metric",
        |scale_index: i64, metric_id: i64, name: &str, primitive: bool| -> Result<(), Box<rhai::EvalAltResult>> {
            let scale_index = parse_scale_index(scale_index)?;
            let metric_id = parse_u16_value("metric_id", metric_id)?;
            let metric_name = name.trim();
            if metric_name.is_empty() {
                return Err("metric name must not be empty".into());
            }
            let mut schemas = USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap();
            let Some(schema) = schemas.get_mut(&scale_index) else {
                return Err(format!("no DPT schema defined for scale_index={scale_index}; call set_dpt_schema first").into());
            };
            schema.metrics.push(ScriptDptMetricDefinition {
                id: metric_id,
                name: metric_name.to_string(),
                primitive,
            });
            Ok(())
        },
    );

    usf_substrate_module.set_native_fn("clear_zlm_maps", || -> Result<(), Box<rhai::EvalAltResult>> {
        USF_ZLM_SCALES_BY_SCALE().lock().unwrap().clear();
        Ok(())
    });
    usf_substrate_module.set_native_fn(
        "set_zlm_scale",
        |scale_index: i64, revision: i64, fallback_zone: &str| -> Result<(), Box<rhai::EvalAltResult>> {
            let scale_index = parse_scale_index(scale_index)?;
            let revision = parse_positive_revision(revision)?;
            let fallback_zone = normalize_zone_type(fallback_zone)?;
            let mut maps = USF_ZLM_SCALES_BY_SCALE().lock().unwrap();
            maps.insert(
                scale_index,
                ScriptZlmScaleDefinition {
                    revision,
                    fallback_zone,
                    rules: Vec::new(),
                },
            );
            Ok(())
        },
    );
    usf_substrate_module.set_native_fn(
        "add_zlm_rule",
        |scale_index: i64, zone_type: &str| -> Result<rhai::INT, Box<rhai::EvalAltResult>> {
            let scale_index = parse_scale_index(scale_index)?;
            let zone_type = normalize_zone_type(zone_type)?;
            let mut maps = USF_ZLM_SCALES_BY_SCALE().lock().unwrap();
            let Some(scale_map) = maps.get_mut(&scale_index) else {
                return Err(format!("no ZLM map defined for scale_index={scale_index}; call set_zlm_scale first").into());
            };
            scale_map.rules.push(ScriptZlmRuleDefinition {
                zone_type,
                metric_bands: Vec::new(),
            });
            Ok((scale_map.rules.len().saturating_sub(1)) as rhai::INT)
        },
    );
    usf_substrate_module.set_native_fn(
        "add_zlm_metric_band",
        |scale_index: i64, rule_index: i64, metric_id: i64, min: rhai::FLOAT, max: rhai::FLOAT| -> Result<(), Box<rhai::EvalAltResult>> {
            let scale_index = parse_scale_index(scale_index)?;
            if rule_index < 0 {
                return Err(format!("rule_index must be >= 0, got {rule_index}").into());
            }
            let metric_id = parse_u16_value("metric_id", metric_id)?;
            if !min.is_finite() || !max.is_finite() {
                return Err("zlm metric band bounds must be finite".into());
            }

            let mut maps = USF_ZLM_SCALES_BY_SCALE().lock().unwrap();
            let Some(scale_map) = maps.get_mut(&scale_index) else {
                return Err(format!("no ZLM map defined for scale_index={scale_index}; call set_zlm_scale first").into());
            };
            let Some(rule) = scale_map.rules.get_mut(rule_index as usize) else {
                return Err(format!(
                    "rule_index {} is out of bounds for scale_index={} (rule_count={})",
                    rule_index,
                    scale_index,
                    scale_map.rules.len()
                )
                .into());
            };
            rule.metric_bands.push(ScriptZlmMetricBandDefinition {
                metric_id,
                min: min as f32,
                max: max as f32,
            });
            Ok(())
        },
    );
    engine.register_static_module("rhai_binding::usf_substrate", Arc::new(usf_substrate_module));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_zone_type_trims_and_lowercases() {
        let normalized = normalize_zone_type("  FoReSt ").unwrap();
        assert_eq!(normalized, "forest");
    }

    #[test]
    fn parse_scale_index_enforces_usf_bounds() {
        assert!(parse_scale_index(-1).is_err());
        assert!(parse_scale_index(Scale::SCALE_LEVEL_COUNT as i64).is_err());
        assert!(parse_scale_index(0).is_ok());
    }

    #[test]
    fn parse_positive_revision_requires_non_zero_positive() {
        assert!(parse_positive_revision(0).is_err());
        assert!(parse_positive_revision(-5).is_err());
        assert_eq!(parse_positive_revision(3).unwrap(), 3);
    }

    #[test]
    fn parse_u16_value_checks_range() {
        assert!(parse_u16_value("metric_id", -1).is_err());
        assert!(parse_u16_value("metric_id", (u16::MAX as i64) + 1).is_err());
        assert_eq!(parse_u16_value("metric_id", 42).unwrap(), 42);
    }
}
