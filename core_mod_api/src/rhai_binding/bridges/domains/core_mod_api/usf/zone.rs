use rhai::FuncRegistration;

use crate::rhai_binding::engine::statics::{
    ScriptZoneDensityProfileDefinition, ScriptZonePhenomenonSupportDefinition, ScriptZoneSelectionPolicyDefinition, USF_ZONE_DENSITY_PROFILE_BY_TYPE,
    USF_ZONE_PHENOMENON_SUPPORT_BY_ZONE_TYPE, USF_ZONE_SELECTION_POLICY_BY_ZONE_TYPE, USF_ZONE_TYPES,
};

core_mod_macros::reflect_extern_sub_module!(
    id = core_mod_api::usf::zone,
    sub_modules = [],
    traits = [],
    types = [],
    module_associated_functions = [
        time_scale_for_levels_above,
        time_scale_for_scale_indices,
        set_density_profile,
        clear_density_profiles,
        clear_supported_phenomena,
        add_supported_phenomenon,
        clear_selection_policies,
        set_selection_policy
    ],
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::zone::time_scale_for_levels_above,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |levels_above: i64| -> rhai::FLOAT {
            crate::usf::zone::time_scale_for_levels_above(levels_above) as rhai::FLOAT
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::zone::time_scale_for_scale_indices,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |active_scale_index: i64, target_scale_index: i64| -> rhai::FLOAT {
            crate::usf::zone::time_scale_for_scale_indices(active_scale_index, target_scale_index) as rhai::FLOAT
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::zone::set_density_profile,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |zone_type: &str,
             density_multiplier: rhai::FLOAT,
             density_offset: rhai::FLOAT,
             density_floor: rhai::FLOAT,
             density_ceil: rhai::FLOAT|
             -> Result<(), Box<rhai::EvalAltResult>> {
                let normalized_zone_type = normalize_zone_type(zone_type)?;
                let density_multiplier = parse_finite_f32("density_multiplier", density_multiplier)?;
                let density_offset = parse_finite_f32("density_offset", density_offset)?;
                let density_floor = parse_finite_f32("density_floor", density_floor)?;
                let density_ceil = parse_finite_f32("density_ceil", density_ceil)?;
                if density_floor > density_ceil {
                    return Err(format!("density_floor ({density_floor}) must be <= density_ceil ({density_ceil})").into());
                }

                USF_ZONE_TYPES().lock().unwrap().insert(normalized_zone_type.clone());
                USF_ZONE_DENSITY_PROFILE_BY_TYPE().lock().unwrap().insert(
                    normalized_zone_type,
                    ScriptZoneDensityProfileDefinition {
                        density_multiplier,
                        density_offset,
                        density_floor,
                        density_ceil,
                    },
                );
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::zone::clear_density_profiles,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> Result<(), Box<rhai::EvalAltResult>> {
            USF_ZONE_DENSITY_PROFILE_BY_TYPE().lock().unwrap().clear();
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::zone::clear_supported_phenomena,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> Result<(), Box<rhai::EvalAltResult>> {
            USF_ZONE_PHENOMENON_SUPPORT_BY_ZONE_TYPE().lock().unwrap().clear();
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::zone::add_supported_phenomenon,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |zone_type: &str,
             phenomenon_id: &str,
             priority: i64,
             weight: rhai::FLOAT,
             spawn_policy: &str,
             max_active: i64|
             -> Result<(), Box<rhai::EvalAltResult>> {
                let normalized_zone_type = normalize_zone_type(zone_type)?;
                if !USF_ZONE_TYPES().lock().unwrap().contains(&normalized_zone_type) {
                    return Err(format!(
                        "zone_type '{}' is not registered; define it via UsfSubstrate::add_zone_type(...) first",
                        normalized_zone_type
                    )
                    .into());
                }
                let phenomenon_id = normalize_identifier("phenomenon_id", phenomenon_id)?;
                let priority = i32::try_from(priority).map_err(|_| format!("priority must fit in i32, got {priority}"))?;
                let weight = parse_finite_f32("weight", weight)?;
                if weight <= 0.0 {
                    return Err(format!("weight must be > 0, got {weight}").into());
                }
                let spawn_policy = normalize_spawn_policy(spawn_policy)?;
                if max_active < 1 {
                    return Err(format!("max_active must be >= 1, got {max_active}").into());
                }
                let max_active = max_active as u32;

                let mut supports_by_zone = USF_ZONE_PHENOMENON_SUPPORT_BY_ZONE_TYPE().lock().unwrap();
                let supports = supports_by_zone.entry(normalized_zone_type).or_default();
                if let Some(existing) = supports.iter_mut().find(|support| support.phenomenon_id == phenomenon_id) {
                    existing.priority = priority;
                    existing.weight = weight;
                    existing.spawn_policy = spawn_policy;
                    existing.max_active = max_active;
                    return Ok(());
                }

                supports.push(ScriptZonePhenomenonSupportDefinition {
                    phenomenon_id,
                    priority,
                    weight,
                    spawn_policy,
                    max_active,
                });
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::zone::clear_selection_policies,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> Result<(), Box<rhai::EvalAltResult>> {
            USF_ZONE_SELECTION_POLICY_BY_ZONE_TYPE().lock().unwrap().clear();
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::zone::set_selection_policy,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |zone_type: &str, strategy: &str| -> Result<(), Box<rhai::EvalAltResult>> {
            let normalized_zone_type = normalize_zone_type(zone_type)?;
            if !USF_ZONE_TYPES().lock().unwrap().contains(&normalized_zone_type) {
                return Err(format!(
                    "zone_type '{}' is not registered; define it via UsfSubstrate::add_zone_type(...) first",
                    normalized_zone_type
                )
                .into());
            }

            let strategy = normalize_selection_strategy(strategy)?;
            USF_ZONE_SELECTION_POLICY_BY_ZONE_TYPE()
                .lock()
                .unwrap()
                .insert(normalized_zone_type, ScriptZoneSelectionPolicyDefinition { strategy });
            Ok(())
        });
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
fn parse_finite_f32(value_name: &str, value: rhai::FLOAT) -> Result<f32, Box<rhai::EvalAltResult>> {
    if !value.is_finite() {
        return Err(format!("{value_name} must be finite, got {value}").into());
    }
    Ok(value as f32)
}

#[inline]
fn normalize_identifier(name: &str, value: &str) -> Result<String, Box<rhai::EvalAltResult>> {
    let normalized = value.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return Err(format!("{name} must not be empty").into());
    }
    Ok(normalized)
}

#[inline]
fn normalize_spawn_policy(value: &str) -> Result<String, Box<rhai::EvalAltResult>> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "single_primary" | "single-primary" | "single" => Ok("single_primary".to_string()),
        _ => Err(format!("unsupported spawn_policy '{normalized}'; currently supported: single_primary").into()),
    }
}

#[inline]
fn normalize_selection_strategy(value: &str) -> Result<String, Box<rhai::EvalAltResult>> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "weighted_top_priority" | "weighted-top-priority" | "weighted" => Ok("weighted_top_priority".to_string()),
        "highest_weight_top_priority" | "highest-weight-top-priority" | "highest_weight" => Ok("highest_weight_top_priority".to_string()),
        "round_robin_top_priority" | "round-robin-top-priority" | "round_robin" => Ok("round_robin_top_priority".to_string()),
        _ => Err(format!(
            "unsupported selection strategy '{normalized}'; supported: weighted_top_priority, highest_weight_top_priority, round_robin_top_priority"
        )
        .into()),
    }
}
