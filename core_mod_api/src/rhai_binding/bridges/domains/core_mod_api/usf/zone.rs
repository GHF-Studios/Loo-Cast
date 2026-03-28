use rhai::FuncRegistration;

use crate::rhai_binding::engine::statics::{ScriptZoneDensityProfileDefinition, USF_ZONE_DENSITY_PROFILE_BY_TYPE, USF_ZONE_KIND_BY_TYPE, USF_ZONE_TYPES};

core_mod_macros::reflect_extern_sub_module!(
    id = core_mod_api::usf::zone,
    sub_modules = [],
    traits = [],
    types = [],
    module_associated_functions = [
        time_scale_for_levels_above,
        time_scale_for_scale_indices,
        set_phenomenon_kind,
        clear_phenomenon_kinds,
        set_density_profile,
        clear_density_profiles
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
    id = core_mod_api::usf::zone::set_phenomenon_kind,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |zone_type: &str, phenomenon_kind: &str| -> Result<(), Box<rhai::EvalAltResult>> {
                let normalized_zone_type = normalize_zone_type(zone_type)?;
                let normalized_kind = phenomenon_kind.trim().to_ascii_lowercase();
                if normalized_kind.is_empty() {
                    return Err("phenomenon_kind must not be empty".into());
                }

                USF_ZONE_TYPES().lock().unwrap().insert(normalized_zone_type.clone());
                USF_ZONE_KIND_BY_TYPE().lock().unwrap().insert(normalized_zone_type, normalized_kind);
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::zone::clear_phenomenon_kinds,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> Result<(), Box<rhai::EvalAltResult>> {
            USF_ZONE_KIND_BY_TYPE().lock().unwrap().clear();
            Ok(())
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
