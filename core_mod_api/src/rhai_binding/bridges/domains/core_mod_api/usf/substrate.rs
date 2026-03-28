use rhai::FuncRegistration;

use crate::rhai_binding::engine::statics::{
    ScriptDptMetricDefinition, ScriptDptSchemaDefinition, ScriptZlmMetricBandDefinition, ScriptZlmRuleDefinition, ScriptZlmScaleDefinition,
    USF_DPT_SCHEMAS_BY_SCALE, USF_ZLM_SCALES_BY_SCALE, USF_ZONE_TYPES,
};
use crate::usf::scale::Scale;

core_mod_macros::reflect_extern_sub_module!(
    id = core_mod_api::usf::substrate,
    sub_modules = [],
    traits = [],
    types = [],
    module_associated_functions = [
        scale_level_count,
        clear_zone_types,
        add_zone_type,
        clear_dpt_schemas,
        set_dpt_schema,
        add_dpt_metric,
        clear_zlm_maps,
        set_zlm_scale,
        add_zlm_rule,
        add_zlm_metric_band
    ],
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::scale_level_count,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> rhai::INT { Scale::SCALE_LEVEL_COUNT as rhai::INT });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::clear_zone_types,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> Result<(), Box<rhai::EvalAltResult>> {
            USF_ZONE_TYPES().lock().unwrap().clear();
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::add_zone_type,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |zone_type: &str| -> Result<(), Box<rhai::EvalAltResult>> {
            let normalized_zone_type = normalize_zone_type(zone_type)?;
            USF_ZONE_TYPES().lock().unwrap().insert(normalized_zone_type);
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::clear_dpt_schemas,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> Result<(), Box<rhai::EvalAltResult>> {
            USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap().clear();
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::set_dpt_schema,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
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
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::add_dpt_metric,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |scale_index: i64, metric_id: i64, metric_name: &str, primitive: bool| -> Result<(), Box<rhai::EvalAltResult>> {
                let scale_index = parse_scale_index(scale_index)?;
                let metric_id = parse_u16_value("metric_id", metric_id)?;
                let metric_name = metric_name.trim();
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
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::clear_zlm_maps,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> Result<(), Box<rhai::EvalAltResult>> {
            USF_ZLM_SCALES_BY_SCALE().lock().unwrap().clear();
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::set_zlm_scale,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
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
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::add_zlm_rule,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
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
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::add_zlm_metric_band,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
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
    },
);

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
