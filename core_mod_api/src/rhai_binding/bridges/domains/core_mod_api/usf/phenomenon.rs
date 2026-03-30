use rhai::FuncRegistration;

use crate::rhai_binding::engine::statics::{
    ScriptMetricSurfaceDebugDefinition, ScriptPhenomenonDefinition, ScriptPhenomenonModelDefinition, USF_PHENOMENA_BY_ID, USF_PHENOMENON_MODELS_BY_ID,
    USF_PRIMARY_PHENOMENON_MODEL_BY_PHENOMENON_ID,
};

core_mod_macros::reflect_extern_sub_module!(
    id = core_mod_api::usf::phenomenon,
    sub_modules = [],
    traits = [],
    types = [],
    module_associated_functions = [
        clear_phenomena,
        add_phenomenon,
        set_metric_surface_debug_field,
        clear_phenomenon_models,
        add_phenomenon_model,
        set_primary_model
    ],
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::phenomenon::clear_phenomena,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> Result<(), Box<rhai::EvalAltResult>> {
            USF_PHENOMENA_BY_ID().lock().unwrap().clear();
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::phenomenon::add_phenomenon,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |phenomenon_id: &str, phenomenon_kind: &str| -> Result<(), Box<rhai::EvalAltResult>> {
                let phenomenon_id = normalize_identifier("phenomenon_id", phenomenon_id)?;
                let phenomenon_kind = normalize_phenomenon_kind(phenomenon_kind)?;

                let mut phenomena = USF_PHENOMENA_BY_ID().lock().unwrap();
                if let Some(existing) = phenomena.get(&phenomenon_id) {
                    if existing.kind != phenomenon_kind {
                        return Err(format!(
                            "phenomenon '{}' already exists with kind '{}'; got '{}'",
                            phenomenon_id, existing.kind, phenomenon_kind
                        )
                        .into());
                    }
                    return Ok(());
                }
                phenomena.insert(
                    phenomenon_id.clone(),
                    ScriptPhenomenonDefinition {
                        id: phenomenon_id,
                        kind: phenomenon_kind,
                        metric_surface_debug: None,
                    },
                );
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::phenomenon::set_metric_surface_debug_field,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |phenomenon_id: &str,
             coarse_span_units: f64,
             detail_span_units: f64,
             coarse_weight: f32,
             detail_weight: f32,
             bias: f32,
             gain: f32,
             center: f32,
             seed_salt_primary: i64,
             seed_salt_detail: i64|
             -> Result<(), Box<rhai::EvalAltResult>> {
                let phenomenon_id = normalize_identifier("phenomenon_id", phenomenon_id)?;
                let mut phenomena = USF_PHENOMENA_BY_ID().lock().unwrap();
                let Some(phenomenon) = phenomena.get_mut(&phenomenon_id) else {
                    return Err(format!(
                        "phenomenon '{}' is not registered; define it in a '*.phenomenon.rhai' file first",
                        phenomenon_id
                    )
                    .into());
                };
                if phenomenon.kind != "metric_surface_debug" {
                    return Err(format!(
                        "phenomenon '{}' has kind '{}'; set_metric_surface_debug_field requires kind 'metric_surface_debug'",
                        phenomenon_id, phenomenon.kind
                    )
                    .into());
                }
                if !coarse_span_units.is_finite() || coarse_span_units <= 0.0 {
                    return Err("coarse_span_units must be finite and > 0".into());
                }
                if !detail_span_units.is_finite() || detail_span_units <= 0.0 {
                    return Err("detail_span_units must be finite and > 0".into());
                }
                if !coarse_weight.is_finite() || coarse_weight < 0.0 {
                    return Err("coarse_weight must be finite and >= 0".into());
                }
                if !detail_weight.is_finite() || detail_weight < 0.0 {
                    return Err("detail_weight must be finite and >= 0".into());
                }
                if coarse_weight + detail_weight <= 0.0 {
                    return Err("coarse_weight + detail_weight must be > 0".into());
                }
                if !bias.is_finite() {
                    return Err("bias must be finite".into());
                }
                if !gain.is_finite() || gain <= 0.0 {
                    return Err("gain must be finite and > 0".into());
                }
                if !center.is_finite() {
                    return Err("center must be finite".into());
                }
                phenomenon.metric_surface_debug = Some(ScriptMetricSurfaceDebugDefinition {
                    coarse_span_units,
                    detail_span_units,
                    coarse_weight,
                    detail_weight,
                    bias,
                    gain,
                    center,
                    seed_salt_primary: seed_salt_primary as u64,
                    seed_salt_detail: seed_salt_detail as u64,
                });
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::phenomenon::clear_phenomenon_models,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> Result<(), Box<rhai::EvalAltResult>> {
            USF_PHENOMENON_MODELS_BY_ID().lock().unwrap().clear();
            USF_PRIMARY_PHENOMENON_MODEL_BY_PHENOMENON_ID().lock().unwrap().clear();
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::phenomenon::add_phenomenon_model,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |model_id: &str, phenomenon_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
            let model_id = normalize_identifier("model_id", model_id)?;
            let phenomenon_id = normalize_identifier("phenomenon_id", phenomenon_id)?;
            if !USF_PHENOMENA_BY_ID().lock().unwrap().contains_key(&phenomenon_id) {
                return Err(format!(
                    "phenomenon '{}' is not registered; define it in a '*.phenomenon.rhai' file first",
                    phenomenon_id
                )
                .into());
            }

            let mut models = USF_PHENOMENON_MODELS_BY_ID().lock().unwrap();
            if let Some(existing) = models.get(&model_id) {
                if existing.phenomenon_id != phenomenon_id {
                    return Err(format!(
                        "phenomenon model '{}' already exists for phenomenon '{}'; got '{}'",
                        model_id, existing.phenomenon_id, phenomenon_id
                    )
                    .into());
                }
                return Ok(());
            }
            models.insert(model_id.clone(), ScriptPhenomenonModelDefinition { id: model_id, phenomenon_id });
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::phenomenon::set_primary_model,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |phenomenon_id: &str, model_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
            let phenomenon_id = normalize_identifier("phenomenon_id", phenomenon_id)?;
            let model_id = normalize_identifier("model_id", model_id)?;
            if !USF_PHENOMENA_BY_ID().lock().unwrap().contains_key(&phenomenon_id) {
                return Err(format!(
                    "phenomenon '{}' is not registered; define it in a '*.phenomenon.rhai' file first",
                    phenomenon_id
                )
                .into());
            }
            let Some(model) = USF_PHENOMENON_MODELS_BY_ID().lock().unwrap().get(&model_id).cloned() else {
                return Err(format!(
                    "phenomenon model '{}' is not registered; define it in a '*.phenomenon_model.rhai' file first",
                    model_id
                )
                .into());
            };
            if model.phenomenon_id != phenomenon_id {
                return Err(format!(
                    "phenomenon model '{}' belongs to '{}', but was assigned as primary for '{}'",
                    model_id, model.phenomenon_id, phenomenon_id
                )
                .into());
            }

            USF_PRIMARY_PHENOMENON_MODEL_BY_PHENOMENON_ID().lock().unwrap().insert(phenomenon_id, model_id);
            Ok(())
        });
    },
);

#[inline]
fn normalize_identifier(name: &str, value: &str) -> Result<String, Box<rhai::EvalAltResult>> {
    let normalized = value.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return Err(format!("{name} must not be empty").into());
    }
    Ok(normalized)
}

#[inline]
fn normalize_phenomenon_kind(kind: &str) -> Result<String, Box<rhai::EvalAltResult>> {
    let normalized = kind.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return Err("phenomenon_kind must not be empty".into());
    }
    match normalized.as_str() {
        "metric_surface_debug" | "metric-surface-debug" | "terrain_metric_surface_debug" | "terrain-metric-surface-debug" => {
            Ok("metric_surface_debug".to_string())
        }
        _ => Err(format!("unknown phenomenon_kind '{}'; expected 'metric_surface_debug'", normalized).into()),
    }
}
