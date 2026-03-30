use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::bevy::ecs::schedule::IntoScheduleConfigs;
use crate::bevy::prelude::{App, First, Last, PostStartup, PostUpdate, PreStartup, PreUpdate, Startup, Update};
use crate::config::{statics::CONFIG, types::ConfigValue};
use crate::core::functions::asset_root;
use crate::rhai_binding::bind::engine_ext::EngineExt;
use crate::rhai_binding::engine::hook::{new_hook_runner_system, register_hook_param_types};
use crate::rhai_binding::engine::preprocess::preprocess_script_source;
use crate::rhai_binding::engine::resources::MainScriptEngineHandle;
use crate::rhai_binding::engine::statics::{
    SCHEDULE_HOOKS, ScriptSingletonConflictPolicy, ScriptUsfContentPackageDefinition, ScriptUsfContentPackageManifestDefinition, ScriptUsfPackageContribution,
    USF_CONTENT_PACKAGE_MANIFESTS_BY_ID, USF_CONTENT_PACKAGES_BY_ID, USF_CONTENT_PROFILES_BY_ID, USF_DPT_SCHEMAS_BY_SCALE, USF_METRIC_SETS_BY_ID,
    USF_METRICS_BY_NAME, USF_PACKAGE_CONTRIBUTIONS_BY_ID, USF_PHENOMENA_BY_ID, USF_PHENOMENON_MODELS_BY_ID, USF_PRIMARY_PHENOMENON_MODEL_BY_PHENOMENON_ID,
    USF_SCALE_BINDINGS_BY_SCALE, USF_ZLM_SCALES_BY_SCALE, USF_ZONE_DENSITY_PROFILE_BY_TYPE, USF_ZONE_PHENOMENON_SUPPORT_BY_ZONE_TYPE,
    USF_ZONE_SELECTION_POLICY_BY_ZONE_TYPE, USF_ZONE_TYPES,
};
use crate::rhai_binding::runtime::ecs::message::bindings::types::ScriptProbeMessage;
use crate::usf::content::PLACEHOLDER_GAMEPLAY_CONTENT_PACKAGE_ID;
use crate::usf::schedule::{UsfPhenomenonSet, UsfSubstrateSet, UsfZoneSet};
use rhai::Engine;

#[derive(Clone, Copy)]
struct UsfScriptTypeSpec {
    relative_dir: &'static str,
    suffix: &'static str,
    entrypoint: &'static str,
}

const ACTIVE_MODPACK_CONFIG_KEY: &str = "usf_content/active_modpack_id";

#[derive(Debug, Clone, Copy)]
enum SingletonDomain {
    ScaleBinding,
    DptSchema,
    Zlm,
}

#[derive(Debug, Clone)]
struct SingletonEntryOrigin {
    package_id: String,
    priority: i32,
    load_order_index: usize,
}

#[derive(Debug, Clone, Default)]
struct CompositionSingletonOrigins {
    scale_binding_by_scale: HashMap<u8, SingletonEntryOrigin>,
    dpt_schema_by_scale: HashMap<u8, SingletonEntryOrigin>,
    zlm_by_scale: HashMap<u8, SingletonEntryOrigin>,
}

const USF_GLOBAL_SCRIPT_TYPE_SPECS: [UsfScriptTypeSpec; 2] = [
    UsfScriptTypeSpec {
        relative_dir: "mods",
        suffix: ".mod.rhai",
        entrypoint: "register_mod",
    },
    UsfScriptTypeSpec {
        relative_dir: "modpacks",
        suffix: ".modpack.rhai",
        entrypoint: "register_modpack",
    },
];

const USF_PACKAGE_SCOPED_SCRIPT_TYPE_SPECS: [UsfScriptTypeSpec; 7] = [
    UsfScriptTypeSpec {
        relative_dir: "metrics",
        suffix: ".metric.rhai",
        entrypoint: "register_metric",
    },
    UsfScriptTypeSpec {
        relative_dir: "zones",
        suffix: ".zone.rhai",
        entrypoint: "register_zone",
    },
    UsfScriptTypeSpec {
        relative_dir: "metric_sets",
        suffix: ".metric_set.rhai",
        entrypoint: "register_metric_set",
    },
    UsfScriptTypeSpec {
        relative_dir: "zlms",
        suffix: ".zlm.rhai",
        entrypoint: "register_zlm",
    },
    UsfScriptTypeSpec {
        relative_dir: "scales",
        suffix: ".scale.rhai",
        entrypoint: "register_scale",
    },
    UsfScriptTypeSpec {
        relative_dir: "phenomena",
        suffix: ".phenomenon.rhai",
        entrypoint: "register_phenomenon",
    },
    UsfScriptTypeSpec {
        relative_dir: "phenomenon_models",
        suffix: ".phenomenon_model.rhai",
        entrypoint: "register_phenomenon_model",
    },
];

pub fn build(app: &mut App) {
    app.init_resource::<MainScriptEngineHandle>();
    app.add_message::<ScriptProbeMessage>();

    let path = "core_mod/scripts/ecs/schedule_hooks/";
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

fn clear_usf_bootstrap_statics() {
    clear_usf_domain_bootstrap_statics();
    USF_CONTENT_PACKAGES_BY_ID().lock().unwrap().clear();
    USF_CONTENT_PACKAGE_MANIFESTS_BY_ID().lock().unwrap().clear();
    USF_CONTENT_PROFILES_BY_ID().lock().unwrap().clear();
    USF_PACKAGE_CONTRIBUTIONS_BY_ID().lock().unwrap().clear();
}

fn clear_usf_domain_bootstrap_statics() {
    USF_ZONE_TYPES().lock().unwrap().clear();
    USF_ZONE_DENSITY_PROFILE_BY_TYPE().lock().unwrap().clear();
    USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap().clear();
    USF_ZLM_SCALES_BY_SCALE().lock().unwrap().clear();
    USF_SCALE_BINDINGS_BY_SCALE().lock().unwrap().clear();
    USF_METRICS_BY_NAME().lock().unwrap().clear();
    USF_METRIC_SETS_BY_ID().lock().unwrap().clear();
    USF_PHENOMENA_BY_ID().lock().unwrap().clear();
    USF_ZONE_PHENOMENON_SUPPORT_BY_ZONE_TYPE().lock().unwrap().clear();
    USF_ZONE_SELECTION_POLICY_BY_ZONE_TYPE().lock().unwrap().clear();
    USF_PHENOMENON_MODELS_BY_ID().lock().unwrap().clear();
    USF_PRIMARY_PHENOMENON_MODEL_BY_PHENOMENON_ID().lock().unwrap().clear();
}

fn collect_usf_registration_scripts(dir: &Path, suffix: &str, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_usf_registration_scripts(&path, suffix, out);
            continue;
        }
        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        if file_name.ends_with(suffix) {
            out.push(path);
        }
    }
}

fn run_usf_script_file(engine: &Engine, file: &Path, entrypoint: &str) {
    let file_path = file.display().to_string();
    let source = std::fs::read_to_string(file).unwrap_or_else(|error| panic!("Failed to read USF script '{}': {error}", file.display()));
    let source = preprocess_script_source(&source, &file_path);
    let ast = engine
        .compile(source)
        .unwrap_or_else(|error| panic!("Failed to compile USF script '{}': {error}", file.display()));
    let mut scope = rhai::Scope::new();
    if let Err(error) = engine.call_fn::<()>(&mut scope, &ast, entrypoint, ()) {
        panic!("USF script '{}' failed calling entrypoint '{}': {}", file.display(), entrypoint, error);
    }
}

fn script_owner_package_id(script_relative_path: &Path) -> Option<String> {
    let mut components = script_relative_path.components();
    let Some(first) = components.next() else {
        return None;
    };
    if components.next().is_none() {
        return Some(PLACEHOLDER_GAMEPLAY_CONTENT_PACKAGE_ID.to_string());
    }
    Some(first.as_os_str().to_string_lossy().to_string())
}

fn run_usf_script_type_bootstrap_global(engine: &Engine, usf_root: &Path, spec: UsfScriptTypeSpec) {
    let script_dir = usf_root.join(spec.relative_dir);
    if !script_dir.is_dir() {
        return;
    }

    let mut files = Vec::new();
    collect_usf_registration_scripts(&script_dir, spec.suffix, &mut files);
    files.sort_by(|a, b| a.to_string_lossy().cmp(&b.to_string_lossy()));

    for file in files {
        run_usf_script_file(engine, &file, spec.entrypoint);
    }
}

fn run_usf_script_type_bootstrap_for_package(engine: &Engine, usf_root: &Path, spec: UsfScriptTypeSpec, content_package_id: &str) {
    let script_dir = usf_root.join(spec.relative_dir);
    if !script_dir.is_dir() {
        return;
    }

    let mut files = Vec::new();
    collect_usf_registration_scripts(&script_dir, spec.suffix, &mut files);
    files.sort_by(|a, b| a.to_string_lossy().cmp(&b.to_string_lossy()));

    for file in files {
        let Ok(relative_path) = file.strip_prefix(&script_dir) else {
            continue;
        };
        let Some(owner_package_id) = script_owner_package_id(relative_path) else {
            continue;
        };
        if owner_package_id != content_package_id {
            continue;
        }
        run_usf_script_file(engine, &file, spec.entrypoint);
    }
}

fn snapshot_usf_domain_statics() -> ScriptUsfPackageContribution {
    ScriptUsfPackageContribution {
        zone_types: USF_ZONE_TYPES().lock().unwrap().clone(),
        dpt_schemas_by_scale: USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap().clone(),
        zlm_scales_by_scale: USF_ZLM_SCALES_BY_SCALE().lock().unwrap().clone(),
        zone_density_profile_by_type: USF_ZONE_DENSITY_PROFILE_BY_TYPE().lock().unwrap().clone(),
        scale_bindings_by_scale: USF_SCALE_BINDINGS_BY_SCALE().lock().unwrap().clone(),
        metrics_by_name: USF_METRICS_BY_NAME().lock().unwrap().clone(),
        metric_sets_by_id: USF_METRIC_SETS_BY_ID().lock().unwrap().clone(),
        phenomena_by_id: USF_PHENOMENA_BY_ID().lock().unwrap().clone(),
        zone_phenomenon_support_by_zone_type: USF_ZONE_PHENOMENON_SUPPORT_BY_ZONE_TYPE().lock().unwrap().clone(),
        zone_selection_policy_by_zone_type: USF_ZONE_SELECTION_POLICY_BY_ZONE_TYPE().lock().unwrap().clone(),
        phenomenon_models_by_id: USF_PHENOMENON_MODELS_BY_ID().lock().unwrap().clone(),
        primary_phenomenon_model_by_phenomenon_id: USF_PRIMARY_PHENOMENON_MODEL_BY_PHENOMENON_ID().lock().unwrap().clone(),
    }
}

fn apply_usf_domain_snapshot(snapshot: ScriptUsfPackageContribution) {
    *USF_ZONE_TYPES().lock().unwrap() = snapshot.zone_types;
    *USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap() = snapshot.dpt_schemas_by_scale;
    *USF_ZLM_SCALES_BY_SCALE().lock().unwrap() = snapshot.zlm_scales_by_scale;
    *USF_ZONE_DENSITY_PROFILE_BY_TYPE().lock().unwrap() = snapshot.zone_density_profile_by_type;
    *USF_SCALE_BINDINGS_BY_SCALE().lock().unwrap() = snapshot.scale_bindings_by_scale;
    *USF_METRICS_BY_NAME().lock().unwrap() = snapshot.metrics_by_name;
    *USF_METRIC_SETS_BY_ID().lock().unwrap() = snapshot.metric_sets_by_id;
    *USF_PHENOMENA_BY_ID().lock().unwrap() = snapshot.phenomena_by_id;
    *USF_ZONE_PHENOMENON_SUPPORT_BY_ZONE_TYPE().lock().unwrap() = snapshot.zone_phenomenon_support_by_zone_type;
    *USF_ZONE_SELECTION_POLICY_BY_ZONE_TYPE().lock().unwrap() = snapshot.zone_selection_policy_by_zone_type;
    *USF_PHENOMENON_MODELS_BY_ID().lock().unwrap() = snapshot.phenomenon_models_by_id;
    *USF_PRIMARY_PHENOMENON_MODEL_BY_PHENOMENON_ID().lock().unwrap() = snapshot.primary_phenomenon_model_by_phenomenon_id;
}

fn merge_set_unique<T: Eq + Hash + Clone + std::fmt::Debug>(target: &mut HashSet<T>, source: HashSet<T>, domain: &str, package_id: &str) {
    for value in source {
        if !target.insert(value.clone()) {
            panic!(
                "USF content composition hard error: duplicate {} entry {:?} while merging package '{}'",
                domain, value, package_id
            );
        }
    }
}

fn merge_map_unique<K: Eq + Hash + Clone + std::fmt::Debug, V>(target: &mut HashMap<K, V>, source: HashMap<K, V>, domain: &str, package_id: &str) {
    for (key, value) in source {
        if target.contains_key(&key) {
            panic!(
                "USF content composition hard error: duplicate {} entry {:?} while merging package '{}'",
                domain, key, package_id
            );
        }
        target.insert(key, value);
    }
}

fn singleton_conflict_policy_for_domain(package: &ScriptUsfContentPackageDefinition, domain: SingletonDomain) -> ScriptSingletonConflictPolicy {
    match domain {
        SingletonDomain::ScaleBinding => package.scale_binding_conflict_policy,
        SingletonDomain::DptSchema => package.dpt_schema_conflict_policy,
        SingletonDomain::Zlm => package.zlm_conflict_policy,
    }
}

fn should_replace_singleton_value(
    existing: &SingletonEntryOrigin,
    incoming_priority: i32,
    incoming_load_order_index: usize,
    policy: ScriptSingletonConflictPolicy,
) -> bool {
    match policy {
        ScriptSingletonConflictPolicy::HardError => false,
        ScriptSingletonConflictPolicy::Replace => true,
        ScriptSingletonConflictPolicy::ReplaceIfHigherPriority => {
            (incoming_priority, incoming_load_order_index) > (existing.priority, existing.load_order_index)
        }
    }
}

fn merge_singleton_map<K: Eq + Hash + Clone + std::fmt::Debug, V>(
    target: &mut HashMap<K, V>,
    source: HashMap<K, V>,
    origins: &mut HashMap<K, SingletonEntryOrigin>,
    package_id: &str,
    package_definitions: &HashMap<String, ScriptUsfContentPackageDefinition>,
    incoming_load_order_index: usize,
    domain: SingletonDomain,
    domain_name: &str,
) {
    let incoming_definition = package_definitions.get(package_id).unwrap_or_else(|| {
        panic!(
            "USF content composition hard error: selected package '{}' has no package definition",
            package_id
        )
    });
    let incoming_origin = SingletonEntryOrigin {
        package_id: package_id.to_string(),
        priority: incoming_definition.priority,
        load_order_index: incoming_load_order_index,
    };

    for (key, value) in source {
        let Some(existing_origin) = origins.get(&key).cloned() else {
            target.insert(key.clone(), value);
            origins.insert(key, incoming_origin.clone());
            continue;
        };

        let existing_definition = package_definitions.get(existing_origin.package_id.as_str()).unwrap_or_else(|| {
            panic!(
                "USF content composition hard error: source package '{}' for singleton {} key {:?} is missing package definition",
                existing_origin.package_id, domain_name, key
            )
        });
        let policy = singleton_conflict_policy_for_domain(existing_definition, domain);
        if policy == ScriptSingletonConflictPolicy::HardError {
            panic!(
                "USF content composition hard error: singleton {} key {:?} from package '{}' conflicts with '{}' and policy is hard_error",
                domain_name, key, package_id, existing_origin.package_id
            );
        }
        if should_replace_singleton_value(&existing_origin, incoming_origin.priority, incoming_origin.load_order_index, policy) {
            target.insert(key.clone(), value);
            origins.insert(key, incoming_origin.clone());
        }
    }
}

fn merge_package_contribution_into_composed(
    package_id: &str,
    contribution: ScriptUsfPackageContribution,
    composed: &mut ScriptUsfPackageContribution,
    singleton_origins: &mut CompositionSingletonOrigins,
    package_definitions: &HashMap<String, ScriptUsfContentPackageDefinition>,
    incoming_load_order_index: usize,
) {
    merge_set_unique(&mut composed.zone_types, contribution.zone_types, "zone_type", package_id);

    merge_singleton_map(
        &mut composed.dpt_schemas_by_scale,
        contribution.dpt_schemas_by_scale,
        &mut singleton_origins.dpt_schema_by_scale,
        package_id,
        package_definitions,
        incoming_load_order_index,
        SingletonDomain::DptSchema,
        "dpt_schema_scale_index",
    );
    merge_singleton_map(
        &mut composed.zlm_scales_by_scale,
        contribution.zlm_scales_by_scale,
        &mut singleton_origins.zlm_by_scale,
        package_id,
        package_definitions,
        incoming_load_order_index,
        SingletonDomain::Zlm,
        "zlm_scale_index",
    );
    merge_map_unique(
        &mut composed.zone_density_profile_by_type,
        contribution.zone_density_profile_by_type,
        "zone_density_profile",
        package_id,
    );
    merge_singleton_map(
        &mut composed.scale_bindings_by_scale,
        contribution.scale_bindings_by_scale,
        &mut singleton_origins.scale_binding_by_scale,
        package_id,
        package_definitions,
        incoming_load_order_index,
        SingletonDomain::ScaleBinding,
        "scale_binding_scale_index",
    );
    merge_map_unique(&mut composed.metrics_by_name, contribution.metrics_by_name, "metric_name", package_id);
    merge_map_unique(&mut composed.metric_sets_by_id, contribution.metric_sets_by_id, "metric_set_id", package_id);
    merge_map_unique(&mut composed.phenomena_by_id, contribution.phenomena_by_id, "phenomenon_id", package_id);
    merge_map_unique(
        &mut composed.zone_phenomenon_support_by_zone_type,
        contribution.zone_phenomenon_support_by_zone_type,
        "zone_support_zone_type",
        package_id,
    );
    merge_map_unique(
        &mut composed.zone_selection_policy_by_zone_type,
        contribution.zone_selection_policy_by_zone_type,
        "zone_selection_policy_zone_type",
        package_id,
    );
    merge_map_unique(
        &mut composed.phenomenon_models_by_id,
        contribution.phenomenon_models_by_id,
        "phenomenon_model_id",
        package_id,
    );
    merge_map_unique(
        &mut composed.primary_phenomenon_model_by_phenomenon_id,
        contribution.primary_phenomenon_model_by_phenomenon_id,
        "primary_model_phenomenon_id",
        package_id,
    );
}

fn validate_package_contribution_against_manifest(
    package_id: &str,
    contribution: &ScriptUsfPackageContribution,
    manifest: &ScriptUsfContentPackageManifestDefinition,
) {
    for metric_name in &manifest.required_metrics {
        if !contribution.metrics_by_name.contains_key(metric_name) {
            panic!(
                "USF content composition hard error: package '{}' manifest requires metric '{}' but contribution did not define it",
                package_id, metric_name
            );
        }
    }
    for metric_set_id in &manifest.required_metric_sets {
        if !contribution.metric_sets_by_id.contains_key(metric_set_id) {
            panic!(
                "USF content composition hard error: package '{}' manifest requires metric set '{}' but contribution did not define it",
                package_id, metric_set_id
            );
        }
    }
    for zone_type in &manifest.required_zone_types {
        if !contribution.zone_types.contains(zone_type) {
            panic!(
                "USF content composition hard error: package '{}' manifest requires zone '{}' but contribution did not define it",
                package_id, zone_type
            );
        }
    }
    for phenomenon_id in &manifest.required_phenomena {
        if !contribution.phenomena_by_id.contains_key(phenomenon_id) {
            panic!(
                "USF content composition hard error: package '{}' manifest requires phenomenon '{}' but contribution did not define it",
                package_id, phenomenon_id
            );
        }
    }
    for model_id in &manifest.required_phenomenon_models {
        if !contribution.phenomenon_models_by_id.contains_key(model_id) {
            panic!(
                "USF content composition hard error: package '{}' manifest requires phenomenon model '{}' but contribution did not define it",
                package_id, model_id
            );
        }
    }
    for scale_index in &manifest.required_scale_binding_scales {
        if !contribution.scale_bindings_by_scale.contains_key(scale_index) {
            panic!(
                "USF content composition hard error: package '{}' manifest requires scale binding at scale {}, but contribution did not define it",
                package_id, scale_index
            );
        }
    }
    for scale_index in &manifest.required_dpt_schema_scales {
        if !contribution.dpt_schemas_by_scale.contains_key(scale_index) {
            panic!(
                "USF content composition hard error: package '{}' manifest requires DPT schema at scale {}, but contribution did not define it",
                package_id, scale_index
            );
        }
    }
    for scale_index in &manifest.required_zlm_scales {
        if !contribution.zlm_scales_by_scale.contains_key(scale_index) {
            panic!(
                "USF content composition hard error: package '{}' manifest requires ZLM map at scale {}, but contribution did not define it",
                package_id, scale_index
            );
        }
    }
}

fn active_usf_modpack_id_from_config() -> String {
    match CONFIG().data.get(ACTIVE_MODPACK_CONFIG_KEY) {
        Some(ConfigValue::String(value)) => {
            let normalized = value.trim().to_ascii_lowercase();
            if normalized.is_empty() {
                panic!("USF bootstrap failed: '{}' must not be empty", ACTIVE_MODPACK_CONFIG_KEY);
            }
            normalized
        }
        Some(other) => panic!("USF bootstrap failed: '{}' must be a string, got {:?}", ACTIVE_MODPACK_CONFIG_KEY, other),
        None => panic!(
            "USF bootstrap failed: '{}' must be configured explicitly; no default modpack fallback exists",
            ACTIVE_MODPACK_CONFIG_KEY
        ),
    }
}

fn content_package_enabled_from_config(content_package_id: &str, config_enabled_key: &str, default_enabled: bool) -> bool {
    match CONFIG().data.get(config_enabled_key) {
        Some(ConfigValue::Boolean(enabled)) => *enabled,
        Some(other) => panic!(
            "USF bootstrap failed: package '{}' expected boolean config at '{}', got {:?}",
            content_package_id, config_enabled_key, other
        ),
        None => default_enabled,
    }
}

fn selected_mod_ids_for_active_modpack() -> Vec<String> {
    let active_profile_id = active_usf_modpack_id_from_config();
    let profiles = USF_CONTENT_PROFILES_BY_ID().lock().unwrap().clone();
    let Some(profile_definition) = profiles.get(&active_profile_id) else {
        panic!("USF bootstrap failed: active modpack '{}' is not registered", active_profile_id);
    };
    if profile_definition.content_package_ids.is_empty() {
        panic!("USF bootstrap failed: active modpack '{}' contains no mods", active_profile_id);
    }

    let known_packages = USF_CONTENT_PACKAGES_BY_ID().lock().unwrap().clone();
    let mut selected = Vec::<String>::new();
    let mut profile_index_by_package = HashMap::<String, usize>::new();
    let mut seen = HashSet::<String>::new();
    for (index, content_package_id) in profile_definition.content_package_ids.iter().enumerate() {
        if !seen.insert(content_package_id.clone()) {
            panic!(
                "USF bootstrap failed: active modpack '{}' contains duplicate mod '{}'",
                active_profile_id, content_package_id
            );
        }
        profile_index_by_package.insert(content_package_id.clone(), index);
        if !known_packages.contains_key(content_package_id) {
            panic!(
                "USF bootstrap failed: active modpack '{}' references unknown mod '{}'",
                active_profile_id, content_package_id
            );
        }
        let package = known_packages
            .get(content_package_id)
            .unwrap_or_else(|| unreachable!("package existence validated above"));
        let enabled = content_package_enabled_from_config(content_package_id, package.config_enabled_key.as_str(), package.default_enabled);
        if enabled {
            selected.push(content_package_id.clone());
        }
    }

    if selected.is_empty() {
        panic!(
            "USF bootstrap failed: active modpack '{}' resolved to zero enabled mods. \
             Enable at least one mod in config or choose a different modpack.",
            active_profile_id
        );
    }

    let selected_set = selected.iter().cloned().collect::<HashSet<_>>();

    for package_id in &selected {
        let package = known_packages
            .get(package_id)
            .unwrap_or_else(|| panic!("USF bootstrap failed: package '{}' definition missing unexpectedly", package_id));

        for dependency in &package.dependencies {
            if !known_packages.contains_key(dependency) {
                panic!("USF bootstrap failed: package '{}' depends_on unknown package '{}'", package_id, dependency);
            }
            if !selected_set.contains(dependency) {
                panic!(
                    "USF bootstrap failed: mod '{}' depends_on '{}' but dependency is not enabled in active modpack '{}'",
                    package_id, dependency, active_profile_id
                );
            }
        }

        for conflict in &package.conflicts_with {
            if selected_set.contains(conflict) {
                panic!(
                    "USF bootstrap failed: mod '{}' conflicts_with '{}' and both are enabled in active modpack '{}'",
                    package_id, conflict, active_profile_id
                );
            }
        }
    }

    let mut indegree = HashMap::<String, usize>::new();
    let mut edges = HashMap::<String, HashSet<String>>::new();
    for package_id in &selected {
        indegree.insert(package_id.clone(), 0);
        edges.insert(package_id.clone(), HashSet::new());
    }

    for package_id in &selected {
        let package = known_packages
            .get(package_id)
            .unwrap_or_else(|| panic!("USF bootstrap failed: package '{}' definition missing unexpectedly", package_id));

        for dependency in &package.dependencies {
            if !selected_set.contains(dependency) {
                continue;
            }
            let adjacency = edges
                .get_mut(dependency)
                .unwrap_or_else(|| panic!("USF bootstrap failed: graph missing dependency node '{}'", dependency));
            if adjacency.insert(package_id.clone()) {
                *indegree
                    .get_mut(package_id)
                    .unwrap_or_else(|| panic!("USF bootstrap failed: graph missing indegree for '{}'", package_id)) += 1;
            }
        }

        for after_package_id in &package.load_after {
            if !known_packages.contains_key(after_package_id) {
                panic!(
                    "USF bootstrap failed: package '{}' load_after unknown package '{}'",
                    package_id, after_package_id
                );
            }
            if !selected_set.contains(after_package_id) {
                continue;
            }
            let adjacency = edges
                .get_mut(after_package_id)
                .unwrap_or_else(|| panic!("USF bootstrap failed: graph missing load_after node '{}'", after_package_id));
            if adjacency.insert(package_id.clone()) {
                *indegree
                    .get_mut(package_id)
                    .unwrap_or_else(|| panic!("USF bootstrap failed: graph missing indegree for '{}'", package_id)) += 1;
            }
        }
    }

    let mut resolved = Vec::<String>::new();
    let mut ready = indegree
        .iter()
        .filter_map(|(package_id, degree)| if *degree == 0 { Some(package_id.clone()) } else { None })
        .collect::<Vec<_>>();

    while !ready.is_empty() {
        ready.sort_by(|left, right| {
            let left_package = known_packages
                .get(left)
                .unwrap_or_else(|| panic!("USF bootstrap failed: package '{}' missing during dependency resolution", left));
            let right_package = known_packages
                .get(right)
                .unwrap_or_else(|| panic!("USF bootstrap failed: package '{}' missing during dependency resolution", right));
            right_package
                .priority
                .cmp(&left_package.priority)
                .then_with(|| {
                    profile_index_by_package
                        .get(left)
                        .copied()
                        .unwrap_or(usize::MAX)
                        .cmp(&profile_index_by_package.get(right).copied().unwrap_or(usize::MAX))
                })
                .then_with(|| left.cmp(right))
        });
        let current = ready.remove(0);
        resolved.push(current.clone());

        let outgoing = edges
            .get(current.as_str())
            .cloned()
            .unwrap_or_else(|| panic!("USF bootstrap failed: graph missing adjacency for package '{}'", current));
        for downstream in outgoing {
            let degree = indegree
                .get_mut(downstream.as_str())
                .unwrap_or_else(|| panic!("USF bootstrap failed: graph missing indegree for package '{}'", downstream));
            *degree = degree.saturating_sub(1);
            if *degree == 0 {
                ready.push(downstream);
            }
        }
    }

    if resolved.len() != selected.len() {
        let unresolved = indegree
            .iter()
            .filter_map(|(package_id, degree)| if *degree > 0 { Some(package_id.clone()) } else { None })
            .collect::<Vec<_>>();
        panic!(
            "USF bootstrap failed: dependency cycle detected in active modpack '{}'; unresolved mods: {:?}",
            active_profile_id, unresolved
        );
    }

    resolved
}

fn run_usf_content_bootstrap(engine: &Engine) {
    let usf_root = asset_root().join("core_mod/scripts/usf");
    if !usf_root.is_dir() {
        panic!("USF bootstrap failed: script root '{}' does not exist", usf_root.display());
    }

    clear_usf_bootstrap_statics();
    for spec in USF_GLOBAL_SCRIPT_TYPE_SPECS {
        run_usf_script_type_bootstrap_global(engine, &usf_root, spec);
    }

    let selected_package_ids = selected_mod_ids_for_active_modpack();
    let package_manifests = USF_CONTENT_PACKAGE_MANIFESTS_BY_ID().lock().unwrap().clone();
    let package_definitions = USF_CONTENT_PACKAGES_BY_ID().lock().unwrap().clone();
    let mut composed = ScriptUsfPackageContribution::default();
    let mut singleton_origins = CompositionSingletonOrigins::default();
    let mut package_contributions = HashMap::<String, ScriptUsfPackageContribution>::new();

    for (load_order_index, content_package_id) in selected_package_ids.into_iter().enumerate() {
        clear_usf_domain_bootstrap_statics();
        for spec in USF_PACKAGE_SCOPED_SCRIPT_TYPE_SPECS {
            run_usf_script_type_bootstrap_for_package(engine, &usf_root, spec, content_package_id.as_str());
        }
        let contribution = snapshot_usf_domain_statics();
        let Some(manifest) = package_manifests.get(content_package_id.as_str()) else {
            panic!(
                "USF content composition hard error: selected mod '{}' has no manifest. \
                 Declare mod requirements in '*.mod.rhai'.",
                content_package_id
            );
        };
        validate_package_contribution_against_manifest(content_package_id.as_str(), &contribution, manifest);
        merge_package_contribution_into_composed(
            content_package_id.as_str(),
            contribution.clone(),
            &mut composed,
            &mut singleton_origins,
            &package_definitions,
            load_order_index,
        );
        package_contributions.insert(content_package_id, contribution);
    }

    *USF_PACKAGE_CONTRIBUTIONS_BY_ID().lock().unwrap() = package_contributions;
    apply_usf_domain_snapshot(composed);
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
    run_usf_content_bootstrap(&engine);

    engine
}

fn register_runtime_bindings(engine: &mut rhai::Engine) {
    register_hook_param_types(engine);
    register_schedule_hooks_runtime_module(engine);
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
