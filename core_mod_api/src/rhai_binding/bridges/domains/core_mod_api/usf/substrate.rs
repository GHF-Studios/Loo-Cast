use rhai::FuncRegistration;

use crate::rhai_binding::engine::statics::{
    ScriptDptMetricDefinition, ScriptDptSchemaDefinition, ScriptMetricDefinition, ScriptScaleBindingDefinition, ScriptSingletonConflictPolicy,
    ScriptUsfContentPackageDefinition, ScriptUsfContentPackageManifestDefinition, ScriptUsfContentProfileDefinition, ScriptZlmMetricBandDefinition,
    ScriptZlmRuleDefinition, ScriptZlmScaleDefinition, USF_CONTENT_PACKAGE_MANIFESTS_BY_ID, USF_CONTENT_PACKAGES_BY_ID, USF_CONTENT_PROFILES_BY_ID,
    USF_DPT_SCHEMAS_BY_SCALE, USF_METRIC_SETS_BY_ID, USF_METRICS_BY_NAME, USF_SCALE_BINDINGS_BY_SCALE, USF_ZLM_SCALES_BY_SCALE, USF_ZONE_TYPES,
};
use crate::usf::content::{DPT_CATEGORIZER_KERNEL_ZLM_LOOKUP_ID, DPT_SAMPLER_KERNEL_DEFAULT_ID};
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
        add_dpt_metric_typed,
        clear_metrics,
        add_metric,
        add_metric_typed,
        clear_metric_sets,
        set_metric_set,
        add_metric_set_metric,
        build_metric_set_from_all_metrics,
        set_dpt_schema_from_metric_set,
        default_dpt_sampler_kernel_id,
        default_dpt_categorizer_kernel_id,
        clear_usf_mods,
        set_usf_mod,
        set_usf_mod_priority,
        add_usf_mod_dependency,
        add_usf_mod_load_after,
        add_usf_mod_conflict,
        set_usf_mod_singleton_conflict_policy,
        clear_usf_mod_manifests,
        declare_mod_metric,
        declare_mod_metric_set,
        declare_mod_zone,
        declare_mod_phenomenon,
        declare_mod_phenomenon_model,
        declare_mod_scale_binding,
        declare_mod_dpt_schema,
        declare_mod_zlm,
        require_mod_all_scale_bindings,
        require_mod_all_dpt_schemas,
        require_mod_all_zlms,
        clear_usf_modpacks,
        set_usf_modpack,
        add_usf_modpack_mod,
        clear_zlm_maps,
        set_zlm_scale,
        add_zlm_rule,
        add_zlm_metric_band,
        clear_scale_bindings,
        set_scale_binding,
        set_scale_binding_with_usf_modpack
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
                let mut schemas = USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap();
                let Some(schema) = schemas.get_mut(&scale_index) else {
                    return Err(format!("no DPT schema defined for scale_index={scale_index}; call set_dpt_schema first").into());
                };
                let definition = build_legacy_metric_definition(metric_id, metric_name, primitive)?;
                schema.metrics.push(ScriptDptMetricDefinition {
                    id: definition.id,
                    name: definition.name,
                    value_type: definition.value_type,
                    semantics_tag: definition.semantics_tag,
                    storage_class: definition.storage_class,
                    derived: definition.derived,
                    min_scale_index: definition.min_scale_index,
                    max_scale_index: definition.max_scale_index,
                });
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::add_dpt_metric_typed,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |scale_index: i64,
             metric_id: i64,
             metric_name: &str,
             value_type: &str,
             semantics_tag: &str,
             storage_class: &str,
             derived: bool,
             min_scale_index: i64,
             max_scale_index: i64|
             -> Result<(), Box<rhai::EvalAltResult>> {
                let scale_index = parse_scale_index(scale_index)?;
                let metric_id = parse_u16_value("metric_id", metric_id)?;
                let metric_name = normalize_identifier("metric_name", metric_name)?;
                let value_type = parse_metric_value_type(value_type)?;
                let semantics_tag = normalize_identifier("semantics_tag", semantics_tag)?;
                let storage_class = parse_metric_storage_class(storage_class)?;
                let min_scale_index = parse_scale_index_with_name("min_scale_index", min_scale_index)?;
                let max_scale_index = parse_scale_index_with_name("max_scale_index", max_scale_index)?;
                if min_scale_index > max_scale_index {
                    return Err(format!("invalid metric scale range [{min_scale_index}..{max_scale_index}] for metric '{}'", metric_name).into());
                }
                let mut schemas = USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap();
                let Some(schema) = schemas.get_mut(&scale_index) else {
                    return Err(format!("no DPT schema defined for scale_index={scale_index}; call set_dpt_schema first").into());
                };
                schema.metrics.push(ScriptDptMetricDefinition {
                    id: metric_id,
                    name: metric_name,
                    value_type,
                    semantics_tag,
                    storage_class,
                    derived,
                    min_scale_index,
                    max_scale_index,
                });
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::clear_metrics,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> Result<(), Box<rhai::EvalAltResult>> {
            USF_METRICS_BY_NAME().lock().unwrap().clear();
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::add_metric,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |metric_id: i64, metric_name: &str, primitive: bool| -> Result<(), Box<rhai::EvalAltResult>> {
                let metric_id = parse_u16_value("metric_id", metric_id)?;
                let mut metrics = USF_METRICS_BY_NAME().lock().unwrap();
                let definition = build_legacy_metric_definition(metric_id, metric_name, primitive)?;
                let metric_name = definition.name.clone();

                if let Some(existing) = metrics.get(&metric_name) {
                    if existing != &definition {
                        return Err(format!("metric '{}' already exists with a different definition", metric_name).into());
                    }
                    return Ok(());
                }

                if let Some(conflict) = metrics.values().find(|def| def.id == metric_id) {
                    return Err(format!("metric_id {} is already assigned to metric '{}'", metric_id, conflict.name).into());
                }

                metrics.insert(metric_name, definition);
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::add_metric_typed,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |metric_id: i64,
             metric_name: &str,
             value_type: &str,
             semantics_tag: &str,
             storage_class: &str,
             derived: bool,
             min_scale_index: i64,
             max_scale_index: i64|
             -> Result<(), Box<rhai::EvalAltResult>> {
                let metric_id = parse_u16_value("metric_id", metric_id)?;
                let metric_name = normalize_identifier("metric_name", metric_name)?;
                let value_type = parse_metric_value_type(value_type)?;
                let semantics_tag = normalize_identifier("semantics_tag", semantics_tag)?;
                let storage_class = parse_metric_storage_class(storage_class)?;
                let min_scale_index = parse_scale_index_with_name("min_scale_index", min_scale_index)?;
                let max_scale_index = parse_scale_index_with_name("max_scale_index", max_scale_index)?;
                if min_scale_index > max_scale_index {
                    return Err(format!("invalid metric scale range [{min_scale_index}..{max_scale_index}] for metric '{}'", metric_name).into());
                }
                let mut metrics = USF_METRICS_BY_NAME().lock().unwrap();

                let definition = ScriptMetricDefinition {
                    id: metric_id,
                    name: metric_name.clone(),
                    value_type,
                    semantics_tag,
                    storage_class,
                    derived,
                    min_scale_index,
                    max_scale_index,
                };

                if let Some(existing) = metrics.get(&metric_name) {
                    if existing != &definition {
                        return Err(format!("metric '{}' already exists with a different definition", metric_name).into());
                    }
                    return Ok(());
                }

                if let Some(conflict) = metrics.values().find(|def| def.id == metric_id) {
                    return Err(format!("metric_id {} is already assigned to metric '{}'", metric_id, conflict.name).into());
                }

                metrics.insert(metric_name, definition);
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::clear_metric_sets,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> Result<(), Box<rhai::EvalAltResult>> {
            USF_METRIC_SETS_BY_ID().lock().unwrap().clear();
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::set_metric_set,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |metric_set_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
            let metric_set_id = normalize_identifier("metric_set_id", metric_set_id)?;
            USF_METRIC_SETS_BY_ID().lock().unwrap().insert(metric_set_id, Vec::new());
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::add_metric_set_metric,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |metric_set_id: &str, metric_name: &str| -> Result<(), Box<rhai::EvalAltResult>> {
                let metric_set_id = normalize_identifier("metric_set_id", metric_set_id)?;
                let metric_name = normalize_identifier("metric_name", metric_name)?;
                let metrics = USF_METRICS_BY_NAME().lock().unwrap();
                if !metrics.contains_key(&metric_name) {
                    return Err(format!("metric '{}' is not registered", metric_name).into());
                }
                drop(metrics);

                let mut metric_sets = USF_METRIC_SETS_BY_ID().lock().unwrap();
                let Some(metric_set) = metric_sets.get_mut(&metric_set_id) else {
                    return Err(format!("metric_set '{}' is not registered; call set_metric_set first", metric_set_id).into());
                };
                if !metric_set.iter().any(|entry| entry == &metric_name) {
                    metric_set.push(metric_name);
                }
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::build_metric_set_from_all_metrics,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |metric_set_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
            let metric_set_id = normalize_identifier("metric_set_id", metric_set_id)?;
            let metrics = USF_METRICS_BY_NAME().lock().unwrap();
            if metrics.is_empty() {
                return Err("cannot build metric set: no metrics are registered".into());
            }

            let mut ordered = metrics.values().cloned().collect::<Vec<_>>();
            ordered.sort_by(|lhs, rhs| lhs.id.cmp(&rhs.id).then_with(|| lhs.name.cmp(&rhs.name)));
            let ordered_metric_names = ordered.into_iter().map(|metric| metric.name).collect::<Vec<_>>();
            drop(metrics);

            USF_METRIC_SETS_BY_ID().lock().unwrap().insert(metric_set_id, ordered_metric_names);
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::set_dpt_schema_from_metric_set,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |scale_index: i64, revision: i64, fallback_zone: &str, metric_set_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
                let scale_index = parse_scale_index(scale_index)?;
                let revision = parse_positive_revision(revision)?;
                let fallback_zone = normalize_zone_type(fallback_zone)?;
                let metric_set_id = normalize_identifier("metric_set_id", metric_set_id)?;

                let metric_set = {
                    let metric_sets = USF_METRIC_SETS_BY_ID().lock().unwrap();
                    let Some(metric_set) = metric_sets.get(&metric_set_id) else {
                        return Err(format!("metric_set '{}' is not registered", metric_set_id).into());
                    };
                    if metric_set.is_empty() {
                        return Err(format!("metric_set '{}' must contain at least one metric", metric_set_id).into());
                    }
                    metric_set.clone()
                };

                let metrics = USF_METRICS_BY_NAME().lock().unwrap();
                let mut compiled_metrics = Vec::<ScriptDptMetricDefinition>::with_capacity(metric_set.len());
                for metric_name in metric_set {
                    let Some(metric) = metrics.get(&metric_name) else {
                        return Err(format!("metric_set '{}' references unknown metric '{}'", metric_set_id, metric_name).into());
                    };
                    compiled_metrics.push(ScriptDptMetricDefinition {
                        id: metric.id,
                        name: metric.name.clone(),
                        value_type: metric.value_type.clone(),
                        semantics_tag: metric.semantics_tag.clone(),
                        storage_class: metric.storage_class.clone(),
                        derived: metric.derived,
                        min_scale_index: metric.min_scale_index,
                        max_scale_index: metric.max_scale_index,
                    });
                }
                drop(metrics);

                let mut schemas = USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap();
                schemas.insert(
                    scale_index,
                    ScriptDptSchemaDefinition {
                        revision,
                        fallback_zone,
                        metrics: compiled_metrics,
                    },
                );
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::default_dpt_sampler_kernel_id,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> String { DPT_SAMPLER_KERNEL_DEFAULT_ID.to_string() });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::default_dpt_categorizer_kernel_id,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> String { DPT_CATEGORIZER_KERNEL_ZLM_LOOKUP_ID.to_string() });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::clear_usf_mods,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> Result<(), Box<rhai::EvalAltResult>> {
            USF_CONTENT_PACKAGES_BY_ID().lock().unwrap().clear();
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::set_usf_mod,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |content_package_id: &str, default_enabled: bool, config_enabled_key: &str| -> Result<(), Box<rhai::EvalAltResult>> {
                let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
                let config_enabled_key = normalize_config_key("config_enabled_key", config_enabled_key)?;
                USF_CONTENT_PACKAGES_BY_ID().lock().unwrap().insert(
                    content_package_id,
                    ScriptUsfContentPackageDefinition {
                        default_enabled,
                        config_enabled_key,
                        ..ScriptUsfContentPackageDefinition::default()
                    },
                );
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::set_usf_mod_priority,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |content_package_id: &str, priority: i64| -> Result<(), Box<rhai::EvalAltResult>> {
                let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
                if priority < i32::MIN as i64 || priority > i32::MAX as i64 {
                    return Err(format!("priority must be in [{}..={}], got {}", i32::MIN, i32::MAX, priority).into());
                }
                let mut packages = USF_CONTENT_PACKAGES_BY_ID().lock().unwrap();
                let Some(package) = packages.get_mut(content_package_id.as_str()) else {
                    return Err(format!("mod '{}' is not registered; call set_usf_mod first", content_package_id).into());
                };
                package.priority = priority as i32;
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::add_usf_mod_dependency,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |content_package_id: &str, dependency_package_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
                let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
                let dependency_package_id = normalize_identifier("dependency_package_id", dependency_package_id)?;
                if content_package_id == dependency_package_id {
                    return Err(format!("mod '{}' cannot depend on itself", content_package_id).into());
                }
                let mut packages = USF_CONTENT_PACKAGES_BY_ID().lock().unwrap();
                let Some(package) = packages.get_mut(content_package_id.as_str()) else {
                    return Err(format!("mod '{}' is not registered; call set_usf_mod first", content_package_id).into());
                };
                package.dependencies.insert(dependency_package_id);
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::add_usf_mod_load_after,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |content_package_id: &str, other_package_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
                let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
                let other_package_id = normalize_identifier("other_package_id", other_package_id)?;
                if content_package_id == other_package_id {
                    return Err(format!("mod '{}' cannot load_after itself", content_package_id).into());
                }
                let mut packages = USF_CONTENT_PACKAGES_BY_ID().lock().unwrap();
                let Some(package) = packages.get_mut(content_package_id.as_str()) else {
                    return Err(format!("mod '{}' is not registered; call set_usf_mod first", content_package_id).into());
                };
                package.load_after.insert(other_package_id);
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::add_usf_mod_conflict,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |content_package_id: &str, conflicting_package_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
                let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
                let conflicting_package_id = normalize_identifier("conflicting_package_id", conflicting_package_id)?;
                if content_package_id == conflicting_package_id {
                    return Err(format!("mod '{}' cannot conflict with itself", content_package_id).into());
                }
                let mut packages = USF_CONTENT_PACKAGES_BY_ID().lock().unwrap();
                let Some(package) = packages.get_mut(content_package_id.as_str()) else {
                    return Err(format!("mod '{}' is not registered; call set_usf_mod first", content_package_id).into());
                };
                package.conflicts_with.insert(conflicting_package_id);
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::set_usf_mod_singleton_conflict_policy,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |content_package_id: &str, singleton_domain: &str, policy_tag: &str| -> Result<(), Box<rhai::EvalAltResult>> {
                let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
                let singleton_domain = normalize_identifier("singleton_domain", singleton_domain)?;
                let policy = parse_singleton_conflict_policy(policy_tag)?;
                let mut packages = USF_CONTENT_PACKAGES_BY_ID().lock().unwrap();
                let Some(package) = packages.get_mut(content_package_id.as_str()) else {
                    return Err(format!("mod '{}' is not registered; call set_usf_mod first", content_package_id).into());
                };

                match singleton_domain.as_str() {
                    "scale" | "scale_binding" => package.scale_binding_conflict_policy = policy,
                    "dpt_schema" => package.dpt_schema_conflict_policy = policy,
                    "zlm" | "zlm_scale" => package.zlm_conflict_policy = policy,
                    _ => {
                        return Err(format!("singleton_domain '{}' is invalid; expected one of: scale, dpt_schema, zlm", singleton_domain).into());
                    }
                }

                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::clear_usf_mod_manifests,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> Result<(), Box<rhai::EvalAltResult>> {
            USF_CONTENT_PACKAGE_MANIFESTS_BY_ID().lock().unwrap().clear();
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::declare_mod_metric,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |content_package_id: &str, metric_name: &str| -> Result<(), Box<rhai::EvalAltResult>> {
                let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
                let metric_name = normalize_identifier("metric_name", metric_name)?;
                with_content_package_manifest_mut(content_package_id.as_str(), |manifest| {
                    manifest.required_metrics.insert(metric_name);
                });
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::declare_mod_metric_set,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |content_package_id: &str, metric_set_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
                let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
                let metric_set_id = normalize_identifier("metric_set_id", metric_set_id)?;
                with_content_package_manifest_mut(content_package_id.as_str(), |manifest| {
                    manifest.required_metric_sets.insert(metric_set_id);
                });
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::declare_mod_zone,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |content_package_id: &str, zone_type: &str| -> Result<(), Box<rhai::EvalAltResult>> {
                let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
                let zone_type = normalize_zone_type(zone_type)?;
                with_content_package_manifest_mut(content_package_id.as_str(), |manifest| {
                    manifest.required_zone_types.insert(zone_type);
                });
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::declare_mod_phenomenon,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |content_package_id: &str, phenomenon_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
                let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
                let phenomenon_id = normalize_identifier("phenomenon_id", phenomenon_id)?;
                with_content_package_manifest_mut(content_package_id.as_str(), |manifest| {
                    manifest.required_phenomena.insert(phenomenon_id);
                });
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::declare_mod_phenomenon_model,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |content_package_id: &str, model_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
                let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
                let model_id = normalize_identifier("model_id", model_id)?;
                with_content_package_manifest_mut(content_package_id.as_str(), |manifest| {
                    manifest.required_phenomenon_models.insert(model_id);
                });
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::declare_mod_scale_binding,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |content_package_id: &str, scale_index: i64| -> Result<(), Box<rhai::EvalAltResult>> {
                let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
                let scale_index = parse_scale_index(scale_index)?;
                with_content_package_manifest_mut(content_package_id.as_str(), |manifest| {
                    manifest.required_scale_binding_scales.insert(scale_index);
                });
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::declare_mod_dpt_schema,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |content_package_id: &str, scale_index: i64| -> Result<(), Box<rhai::EvalAltResult>> {
                let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
                let scale_index = parse_scale_index(scale_index)?;
                with_content_package_manifest_mut(content_package_id.as_str(), |manifest| {
                    manifest.required_dpt_schema_scales.insert(scale_index);
                });
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::declare_mod_zlm,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |content_package_id: &str, scale_index: i64| -> Result<(), Box<rhai::EvalAltResult>> {
                let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
                let scale_index = parse_scale_index(scale_index)?;
                with_content_package_manifest_mut(content_package_id.as_str(), |manifest| {
                    manifest.required_zlm_scales.insert(scale_index);
                });
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::require_mod_all_scale_bindings,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |content_package_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
            let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
            with_content_package_manifest_mut(content_package_id.as_str(), |manifest| {
                manifest.required_scale_binding_scales.extend(0..(Scale::SCALE_LEVEL_COUNT as u8));
            });
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::require_mod_all_dpt_schemas,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |content_package_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
            let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
            with_content_package_manifest_mut(content_package_id.as_str(), |manifest| {
                manifest.required_dpt_schema_scales.extend(0..(Scale::SCALE_LEVEL_COUNT as u8));
            });
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::require_mod_all_zlms,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |content_package_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
            let content_package_id = normalize_identifier("content_package_id", content_package_id)?;
            with_content_package_manifest_mut(content_package_id.as_str(), |manifest| {
                manifest.required_zlm_scales.extend(0..(Scale::SCALE_LEVEL_COUNT as u8));
            });
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::clear_usf_modpacks,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> Result<(), Box<rhai::EvalAltResult>> {
            USF_CONTENT_PROFILES_BY_ID().lock().unwrap().clear();
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::set_usf_modpack,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |usf_modpack_id: &str, mod_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
            let usf_modpack_id = normalize_identifier("usf_modpack_id", usf_modpack_id)?;
            let mod_id = normalize_identifier("mod_id", mod_id)?;
            USF_CONTENT_PROFILES_BY_ID().lock().unwrap().insert(
                usf_modpack_id,
                ScriptUsfContentProfileDefinition {
                    content_package_ids: vec![mod_id],
                },
            );
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::add_usf_modpack_mod,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, |usf_modpack_id: &str, mod_id: &str| -> Result<(), Box<rhai::EvalAltResult>> {
            let usf_modpack_id = normalize_identifier("usf_modpack_id", usf_modpack_id)?;
            let mod_id = normalize_identifier("mod_id", mod_id)?;
            let mut profiles = USF_CONTENT_PROFILES_BY_ID().lock().unwrap();
            let Some(profile) = profiles.get_mut(&usf_modpack_id) else {
                return Err(format!("modpack '{}' is not registered; call set_usf_modpack first", usf_modpack_id).into());
            };
            if !profile.content_package_ids.iter().any(|existing| existing == &mod_id) {
                profile.content_package_ids.push(mod_id);
            }
            Ok(())
        });
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

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::clear_scale_bindings,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(parent_module, || -> Result<(), Box<rhai::EvalAltResult>> {
            USF_SCALE_BINDINGS_BY_SCALE().lock().unwrap().clear();
            Ok(())
        });
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::set_scale_binding,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |scale_index: i64, dpt_sampler_id: &str, dpt_categorizer_id: &str, chunk_store_key: &str| -> Result<(), Box<rhai::EvalAltResult>> {
                let scale_index = parse_scale_index(scale_index)?;
                let dpt_sampler_id = normalize_identifier("dpt_sampler_id", dpt_sampler_id)?;
                let dpt_categorizer_id = normalize_identifier("dpt_categorizer_id", dpt_categorizer_id)?;
                let chunk_store_key = normalize_identifier("chunk_store_key", chunk_store_key)?;
                USF_SCALE_BINDINGS_BY_SCALE().lock().unwrap().insert(
                    scale_index,
                    ScriptScaleBindingDefinition {
                        dpt_sampler_id,
                        dpt_categorizer_id,
                        chunk_store_key,
                    },
                );
                Ok(())
            },
        );
    },
);

core_mod_macros::reflect_extern_module_associated_function!(
    id = core_mod_api::usf::substrate::set_scale_binding_with_usf_modpack,
    registrator = |name: rhai::ImmutableString, parent_module: &mut rhai::Module| {
        FuncRegistration::new(name).set_into_module(
            parent_module,
            |scale_index: i64,
             dpt_sampler_id: &str,
             dpt_categorizer_id: &str,
             chunk_store_key: &str,
             _usf_modpack_id: &str|
             -> Result<(), Box<rhai::EvalAltResult>> {
                let scale_index = parse_scale_index(scale_index)?;
                let dpt_sampler_id = normalize_identifier("dpt_sampler_id", dpt_sampler_id)?;
                let dpt_categorizer_id = normalize_identifier("dpt_categorizer_id", dpt_categorizer_id)?;
                let chunk_store_key = normalize_identifier("chunk_store_key", chunk_store_key)?;
                USF_SCALE_BINDINGS_BY_SCALE().lock().unwrap().insert(
                    scale_index,
                    ScriptScaleBindingDefinition {
                        dpt_sampler_id,
                        dpt_categorizer_id,
                        chunk_store_key,
                    },
                );
                Ok(())
            },
        );
    },
);

#[inline]
fn with_content_package_manifest_mut(content_package_id: &str, f: impl FnOnce(&mut ScriptUsfContentPackageManifestDefinition)) {
    let mut manifests = USF_CONTENT_PACKAGE_MANIFESTS_BY_ID().lock().unwrap();
    let manifest = manifests
        .entry(content_package_id.to_string())
        .or_insert_with(ScriptUsfContentPackageManifestDefinition::default);
    f(manifest);
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
    parse_scale_index_with_name("scale_index", scale_index)
}

#[inline]
fn parse_scale_index_with_name(name: &str, scale_index: i64) -> Result<u8, Box<rhai::EvalAltResult>> {
    if scale_index < 0 {
        return Err(format!("{name} must be >= 0, got {scale_index}").into());
    }
    let max_index = (Scale::SCALE_LEVEL_COUNT.saturating_sub(1)) as i64;
    if scale_index > max_index {
        return Err(format!("{name} must be <= {max_index}, got {scale_index}").into());
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

#[inline]
fn parse_metric_value_type(value: &str) -> Result<String, Box<rhai::EvalAltResult>> {
    let normalized = value.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return Err("metric value_type must not be empty".into());
    }
    match normalized.as_str() {
        "u8" | "u16" | "i32" | "f32" | "f64" => Ok(normalized),
        _ => Err(format!("unsupported metric value_type '{normalized}'").into()),
    }
}

#[inline]
fn parse_metric_storage_class(value: &str) -> Result<String, Box<rhai::EvalAltResult>> {
    let normalized = value.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return Err("metric storage_class must not be empty".into());
    }
    match normalized.as_str() {
        "uniform" | "brick" => Ok(normalized),
        _ => Err(format!("unsupported metric storage_class '{normalized}'").into()),
    }
}

#[inline]
fn parse_singleton_conflict_policy(value: &str) -> Result<ScriptSingletonConflictPolicy, Box<rhai::EvalAltResult>> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "hard_error" => Ok(ScriptSingletonConflictPolicy::HardError),
        "replace" => Ok(ScriptSingletonConflictPolicy::Replace),
        "replace_if_higher_priority" => Ok(ScriptSingletonConflictPolicy::ReplaceIfHigherPriority),
        _ => Err(format!(
            "invalid singleton conflict policy '{}'; expected one of: hard_error, replace, replace_if_higher_priority",
            value
        )
        .into()),
    }
}

#[inline]
fn build_legacy_metric_definition(metric_id: u16, metric_name: &str, primitive: bool) -> Result<ScriptMetricDefinition, Box<rhai::EvalAltResult>> {
    let metric_name = normalize_identifier("metric_name", metric_name)?;
    let max_scale = Scale::SCALE_LEVEL_COUNT.saturating_sub(1);
    Ok(ScriptMetricDefinition {
        id: metric_id,
        name: metric_name.clone(),
        value_type: "f32".to_string(),
        semantics_tag: format!("legacy.{metric_name}"),
        storage_class: "brick".to_string(),
        derived: !primitive,
        min_scale_index: 0,
        max_scale_index: max_scale,
    })
}

#[inline]
fn normalize_config_key(value_name: &str, value: &str) -> Result<String, Box<rhai::EvalAltResult>> {
    let normalized = value.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return Err(format!("{value_name} must not be empty").into());
    }
    if normalized.chars().any(char::is_whitespace) {
        return Err(format!("{value_name} must not contain whitespace").into());
    }
    Ok(normalized)
}

#[inline]
fn normalize_identifier(value_name: &str, value: &str) -> Result<String, Box<rhai::EvalAltResult>> {
    let normalized = value.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return Err(format!("{value_name} must not be empty").into());
    }
    Ok(normalized)
}
