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
    SCHEDULE_HOOKS, ScriptDptMetricDefinition, ScriptDptSchemaDefinition, ScriptManifestationDensityDefinition, ScriptManifestationMaterialDefinition,
    ScriptMetricDefinition, ScriptPhenomenonDefinition, ScriptPhenomenonModelDefinition, ScriptScaleDefinition, ScriptSingletonConflictPolicy,
    ScriptUsfModContribution, ScriptUsfModDefinition, ScriptUsfModManifestDefinition, ScriptUsfModpackDefinition, ScriptZlmMetricBandDefinition,
    ScriptZlmRuleDefinition, ScriptZlmScaleDefinition, ScriptZoneDensityProfileDefinition, ScriptZonePhenomenonSupportDefinition,
    ScriptZoneSelectionPolicyDefinition, USF_DPT_SCHEMAS_BY_SCALE, USF_METRIC_SETS_BY_ID, USF_METRICS_BY_NAME, USF_MOD_CONTRIBUTIONS_BY_ID,
    USF_MOD_MANIFESTS_BY_ID, USF_MODPACKS_BY_ID, USF_MODS_BY_ID, USF_PHENOMENA_BY_ID, USF_PHENOMENON_MODEL_SELECTION_BY_PHENOMENON_SCALE,
    USF_PHENOMENON_MODELS_BY_ID, USF_SCALES_BY_INDEX, USF_ZLM_SCALES_BY_SCALE, USF_ZONE_DENSITY_PROFILE_BY_TYPE, USF_ZONE_PHENOMENON_SUPPORT_BY_ZONE_TYPE,
    USF_ZONE_SELECTION_POLICY_BY_ZONE_TYPE, USF_ZONE_TYPES,
};
use crate::rhai_binding::runtime::ecs::message::bindings::types::ScriptProbeMessage;
use crate::usf::content::{DEFAULT_DEMO_MOD_ID, DPT_CATEGORIZER_KERNEL_ZLM_LOOKUP_ID, DPT_SAMPLER_KERNEL_DEFAULT_ID};
use crate::usf::phenomenon::{PhenomenonCapability, PhenomenonKind};
use crate::usf::scale::Scale;
use crate::usf::schedule::{UsfPhenomenonSet, UsfSubstrateSet, UsfZoneSet};
use rhai::{Engine, EvalAltResult};

#[derive(Clone, Copy)]
struct UsfScriptTypeSpec {
    relative_dir: &'static str,
    suffix: &'static str,
    entrypoint: &'static str,
    single_entity_domain: Option<SingleEntityDomain>,
}

const ACTIVE_MODPACK_CONFIG_KEY: &str = "usf/active_modpack_id";

#[derive(Debug, Clone, Copy)]
enum SingletonDomain {
    Scale,
    DptSchema,
    Zlm,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SingleEntityDomain {
    Mod,
    Modpack,
    Metric,
    Zone,
    MetricSet,
    Phenomenon,
    PhenomenonModel,
}

#[derive(Debug, Clone)]
struct SingletonEntryOrigin {
    package_id: String,
    priority: i32,
    load_order_index: usize,
}

#[derive(Debug, Clone, Default)]
struct CompositionSingletonOrigins {
    scale_by_index: HashMap<u8, SingletonEntryOrigin>,
    dpt_schema_by_scale: HashMap<u8, SingletonEntryOrigin>,
    zlm_by_scale: HashMap<u8, SingletonEntryOrigin>,
}

#[derive(Clone, Debug)]
struct UsfScriptCtx {
    script_kind: String,
    script_file: String,
    script_id: String,
    owner_mod_id: String,
}

#[derive(Clone, Debug)]
struct UsfModScriptCtx {
    script_file: String,
    mod_id: String,
}

#[derive(Clone, Debug)]
struct UsfModpackScriptCtx {
    script_file: String,
    modpack_id: String,
}

#[derive(Clone, Debug)]
struct UsfMetricScriptCtx {
    script_file: String,
    owner_mod_id: String,
    metric_name: String,
}

#[derive(Clone, Debug)]
struct UsfMetricSetScriptCtx {
    script_file: String,
    owner_mod_id: String,
    metric_set_id: String,
}

#[derive(Clone, Debug)]
struct UsfZoneScriptCtx {
    script_file: String,
    owner_mod_id: String,
    zone_type: String,
}

#[derive(Clone, Debug)]
struct UsfZlmScriptCtx {
    script_file: String,
    owner_mod_id: String,
    zlm_id: String,
}

#[derive(Clone, Debug)]
struct UsfScaleScriptCtx {
    script_file: String,
    owner_mod_id: String,
    scale_script_id: String,
}

#[derive(Clone, Debug)]
struct UsfPhenomenonScriptCtx {
    script_file: String,
    owner_mod_id: String,
    phenomenon_id: String,
}

#[derive(Clone, Debug)]
struct UsfPhenomenonModelScriptCtx {
    script_file: String,
    owner_mod_id: String,
    model_id: String,
}

const USF_GLOBAL_SCRIPT_TYPE_SPECS: [UsfScriptTypeSpec; 2] = [
    UsfScriptTypeSpec {
        relative_dir: "mods",
        suffix: ".mod.rhai",
        entrypoint: "register_mod",
        single_entity_domain: Some(SingleEntityDomain::Mod),
    },
    UsfScriptTypeSpec {
        relative_dir: "modpacks",
        suffix: ".modpack.rhai",
        entrypoint: "register_modpack",
        single_entity_domain: Some(SingleEntityDomain::Modpack),
    },
];

const USF_PACKAGE_SCOPED_SCRIPT_TYPE_SPECS: [UsfScriptTypeSpec; 7] = [
    UsfScriptTypeSpec {
        relative_dir: "metrics",
        suffix: ".metric.rhai",
        entrypoint: "register_metric",
        single_entity_domain: Some(SingleEntityDomain::Metric),
    },
    UsfScriptTypeSpec {
        relative_dir: "zones",
        suffix: ".zone.rhai",
        entrypoint: "register_zone",
        single_entity_domain: Some(SingleEntityDomain::Zone),
    },
    UsfScriptTypeSpec {
        relative_dir: "metric_sets",
        suffix: ".metric_set.rhai",
        entrypoint: "register_metric_set",
        single_entity_domain: Some(SingleEntityDomain::MetricSet),
    },
    UsfScriptTypeSpec {
        relative_dir: "zlms",
        suffix: ".zlm.rhai",
        entrypoint: "register_zlm",
        single_entity_domain: None,
    },
    UsfScriptTypeSpec {
        relative_dir: "scales",
        suffix: ".scale.rhai",
        entrypoint: "register_scale",
        single_entity_domain: None,
    },
    UsfScriptTypeSpec {
        relative_dir: "phenomena",
        suffix: ".phenomenon.rhai",
        entrypoint: "register_phenomenon",
        single_entity_domain: Some(SingleEntityDomain::Phenomenon),
    },
    UsfScriptTypeSpec {
        relative_dir: "phenomenon_models",
        suffix: ".phenomenon_model.rhai",
        entrypoint: "register_phenomenon_model",
        single_entity_domain: Some(SingleEntityDomain::PhenomenonModel),
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
    USF_MODS_BY_ID().lock().unwrap().clear();
    USF_MOD_MANIFESTS_BY_ID().lock().unwrap().clear();
    USF_MODPACKS_BY_ID().lock().unwrap().clear();
    USF_MOD_CONTRIBUTIONS_BY_ID().lock().unwrap().clear();
}

fn clear_usf_domain_bootstrap_statics() {
    USF_ZONE_TYPES().lock().unwrap().clear();
    USF_ZONE_DENSITY_PROFILE_BY_TYPE().lock().unwrap().clear();
    USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap().clear();
    USF_ZLM_SCALES_BY_SCALE().lock().unwrap().clear();
    USF_SCALES_BY_INDEX().lock().unwrap().clear();
    USF_METRICS_BY_NAME().lock().unwrap().clear();
    USF_METRIC_SETS_BY_ID().lock().unwrap().clear();
    USF_PHENOMENA_BY_ID().lock().unwrap().clear();
    USF_ZONE_PHENOMENON_SUPPORT_BY_ZONE_TYPE().lock().unwrap().clear();
    USF_ZONE_SELECTION_POLICY_BY_ZONE_TYPE().lock().unwrap().clear();
    USF_PHENOMENON_MODELS_BY_ID().lock().unwrap().clear();
    USF_PHENOMENON_MODEL_SELECTION_BY_PHENOMENON_SCALE().lock().unwrap().clear();
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

#[inline]
fn normalize_script_identifier(name: &str, value: &str) -> Result<String, Box<EvalAltResult>> {
    let normalized = value.trim().to_ascii_lowercase();
    if normalized.is_empty() {
        return Err(format!("{name} must not be empty").into());
    }
    Ok(normalized)
}

fn script_id_from_path(file: &Path, suffix: &str) -> Option<String> {
    let file_name = file.file_name()?.to_str()?.to_ascii_lowercase();
    let stem = file_name.strip_suffix(suffix)?.to_string();
    if stem.trim().is_empty() {
        return None;
    }
    Some(stem)
}

fn parse_singleton_conflict_policy_tag(policy_tag: &str) -> Result<ScriptSingletonConflictPolicy, Box<EvalAltResult>> {
    match policy_tag.trim().to_ascii_lowercase().as_str() {
        "hard_error" | "hard-error" | "error" => Ok(ScriptSingletonConflictPolicy::HardError),
        "replace" => Ok(ScriptSingletonConflictPolicy::Replace),
        "replace_if_higher_priority" | "replace-if-higher-priority" | "replace_if_priority" => Ok(ScriptSingletonConflictPolicy::ReplaceIfHigherPriority),
        other => Err(format!(
            "singleton conflict policy '{}' is invalid; expected one of: hard_error, replace, replace_if_higher_priority",
            other
        )
        .into()),
    }
}

fn run_usf_script_file(engine: &Engine, file: &Path, entrypoint: &str, ctx: rhai::Dynamic) {
    let file_path = file.display().to_string();
    let source = std::fs::read_to_string(file).unwrap_or_else(|error| panic!("Failed to read USF script '{}': {error}", file.display()));
    let source = preprocess_script_source(&source, &file_path);
    let ast = engine
        .compile(source)
        .unwrap_or_else(|error| panic!("Failed to compile USF script '{}': {error}", file.display()));
    let mut scope = rhai::Scope::new();
    if let Err(error) = engine.call_fn::<()>(&mut scope, &ast, entrypoint, (ctx,)) {
        panic!(
            "USF script '{}' failed calling entrypoint '{}' with typed ctx parameter: {}",
            file.display(),
            entrypoint,
            error
        );
    }
}

fn single_entity_count(domain: SingleEntityDomain) -> usize {
    match domain {
        SingleEntityDomain::Mod => USF_MODS_BY_ID().lock().unwrap().len(),
        SingleEntityDomain::Modpack => USF_MODPACKS_BY_ID().lock().unwrap().len(),
        SingleEntityDomain::Metric => USF_METRICS_BY_NAME().lock().unwrap().len(),
        SingleEntityDomain::Zone => USF_ZONE_TYPES().lock().unwrap().len(),
        SingleEntityDomain::MetricSet => USF_METRIC_SETS_BY_ID().lock().unwrap().len(),
        SingleEntityDomain::Phenomenon => USF_PHENOMENA_BY_ID().lock().unwrap().len(),
        SingleEntityDomain::PhenomenonModel => USF_PHENOMENON_MODELS_BY_ID().lock().unwrap().len(),
    }
}

fn single_entity_domain_label(domain: SingleEntityDomain) -> &'static str {
    match domain {
        SingleEntityDomain::Mod => "mod",
        SingleEntityDomain::Modpack => "modpack",
        SingleEntityDomain::Metric => "metric",
        SingleEntityDomain::Zone => "zone",
        SingleEntityDomain::MetricSet => "metric_set",
        SingleEntityDomain::Phenomenon => "phenomenon",
        SingleEntityDomain::PhenomenonModel => "phenomenon_model",
    }
}

fn script_kind_for_spec(spec: UsfScriptTypeSpec) -> String {
    match spec.single_entity_domain {
        Some(domain) => single_entity_domain_label(domain).to_string(),
        None => spec.relative_dir.to_string(),
    }
}

fn script_ctx_for_spec(file: &Path, spec: UsfScriptTypeSpec, owner_mod_id: Option<&str>) -> rhai::Dynamic {
    let script_file = file.display().to_string();
    let script_id = script_id_from_path(file, spec.suffix).unwrap_or_else(|| {
        panic!(
            "USF bootstrap failed: script '{}' does not have a valid script id for suffix '{}'.",
            file.display(),
            spec.suffix
        )
    });
    let owner_mod_id = owner_mod_id.unwrap_or_default().to_string();

    match spec.single_entity_domain {
        Some(SingleEntityDomain::Mod) => rhai::Dynamic::from(UsfModScriptCtx {
            script_file,
            mod_id: script_id,
        }),
        Some(SingleEntityDomain::Modpack) => rhai::Dynamic::from(UsfModpackScriptCtx {
            script_file,
            modpack_id: script_id,
        }),
        Some(SingleEntityDomain::Metric) => rhai::Dynamic::from(UsfMetricScriptCtx {
            script_file,
            owner_mod_id,
            metric_name: script_id,
        }),
        Some(SingleEntityDomain::MetricSet) => rhai::Dynamic::from(UsfMetricSetScriptCtx {
            script_file,
            owner_mod_id,
            metric_set_id: script_id,
        }),
        Some(SingleEntityDomain::Zone) => rhai::Dynamic::from(UsfZoneScriptCtx {
            script_file,
            owner_mod_id,
            zone_type: script_id,
        }),
        Some(SingleEntityDomain::Phenomenon) => rhai::Dynamic::from(UsfPhenomenonScriptCtx {
            script_file,
            owner_mod_id,
            phenomenon_id: script_id,
        }),
        Some(SingleEntityDomain::PhenomenonModel) => rhai::Dynamic::from(UsfPhenomenonModelScriptCtx {
            script_file,
            owner_mod_id,
            model_id: script_id,
        }),
        None if spec.relative_dir == "zlms" => rhai::Dynamic::from(UsfZlmScriptCtx {
            script_file,
            owner_mod_id,
            zlm_id: script_id,
        }),
        None if spec.relative_dir == "scales" => rhai::Dynamic::from(UsfScaleScriptCtx {
            script_file,
            owner_mod_id,
            scale_script_id: script_id,
        }),
        _ => rhai::Dynamic::from(UsfScriptCtx {
            script_kind: script_kind_for_spec(spec),
            script_file,
            script_id,
            owner_mod_id,
        }),
    }
}

fn run_usf_script_file_for_spec(engine: &Engine, file: &Path, spec: UsfScriptTypeSpec, owner_mod_id: Option<&str>) {
    let before = spec.single_entity_domain.map(single_entity_count);
    let before_manifest = matches!(spec.single_entity_domain, Some(SingleEntityDomain::Mod)).then(|| USF_MOD_MANIFESTS_BY_ID().lock().unwrap().len());
    let script_ctx = script_ctx_for_spec(file, spec, owner_mod_id);
    run_usf_script_file(engine, file, spec.entrypoint, script_ctx);

    let Some(domain) = spec.single_entity_domain else {
        return;
    };
    let after = single_entity_count(domain);
    let expected_after = before.unwrap_or_default().saturating_add(1);
    if after != expected_after {
        panic!(
            "USF bootstrap failed: script '{}' must declare exactly one {} (expected registry count {} -> {}, got {}).",
            file.display(),
            single_entity_domain_label(domain),
            before.unwrap_or_default(),
            expected_after,
            after
        );
    }
    if domain == SingleEntityDomain::Mod {
        let manifest_after = USF_MOD_MANIFESTS_BY_ID().lock().unwrap().len();
        let expected_manifest_after = before_manifest.unwrap_or_default().saturating_add(1);
        if manifest_after != expected_manifest_after {
            panic!(
                "USF bootstrap failed: mod script '{}' must declare exactly one mod manifest (expected manifest registry count {} -> {}, got {}).",
                file.display(),
                before_manifest.unwrap_or_default(),
                expected_manifest_after,
                manifest_after
            );
        }
    }
}

fn script_owner_package_id(script_relative_path: &Path) -> Option<String> {
    let mut components = script_relative_path.components();
    let Some(first) = components.next() else {
        return None;
    };
    if components.next().is_none() {
        return Some(DEFAULT_DEMO_MOD_ID.to_string());
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
        run_usf_script_file_for_spec(engine, &file, spec, None);
    }
}

fn run_usf_script_type_bootstrap_for_package(engine: &Engine, usf_root: &Path, spec: UsfScriptTypeSpec, mod_id: &str) {
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
        if owner_package_id != mod_id {
            continue;
        }
        run_usf_script_file_for_spec(engine, &file, spec, Some(mod_id));
    }
}

fn snapshot_usf_domain_statics() -> ScriptUsfModContribution {
    ScriptUsfModContribution {
        zone_types: USF_ZONE_TYPES().lock().unwrap().clone(),
        dpt_schemas_by_scale: USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap().clone(),
        zlm_scales_by_scale: USF_ZLM_SCALES_BY_SCALE().lock().unwrap().clone(),
        zone_density_profile_by_type: USF_ZONE_DENSITY_PROFILE_BY_TYPE().lock().unwrap().clone(),
        scales_by_index: USF_SCALES_BY_INDEX().lock().unwrap().clone(),
        metrics_by_name: USF_METRICS_BY_NAME().lock().unwrap().clone(),
        metric_sets_by_id: USF_METRIC_SETS_BY_ID().lock().unwrap().clone(),
        phenomena_by_id: USF_PHENOMENA_BY_ID().lock().unwrap().clone(),
        zone_phenomenon_support_by_zone_type: USF_ZONE_PHENOMENON_SUPPORT_BY_ZONE_TYPE().lock().unwrap().clone(),
        zone_selection_policy_by_zone_type: USF_ZONE_SELECTION_POLICY_BY_ZONE_TYPE().lock().unwrap().clone(),
        phenomenon_models_by_id: USF_PHENOMENON_MODELS_BY_ID().lock().unwrap().clone(),
        phenomenon_model_selection_by_phenomenon_scale: USF_PHENOMENON_MODEL_SELECTION_BY_PHENOMENON_SCALE().lock().unwrap().clone(),
    }
}

fn apply_usf_domain_snapshot(snapshot: ScriptUsfModContribution) {
    *USF_ZONE_TYPES().lock().unwrap() = snapshot.zone_types;
    *USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap() = snapshot.dpt_schemas_by_scale;
    *USF_ZLM_SCALES_BY_SCALE().lock().unwrap() = snapshot.zlm_scales_by_scale;
    *USF_ZONE_DENSITY_PROFILE_BY_TYPE().lock().unwrap() = snapshot.zone_density_profile_by_type;
    *USF_SCALES_BY_INDEX().lock().unwrap() = snapshot.scales_by_index;
    *USF_METRICS_BY_NAME().lock().unwrap() = snapshot.metrics_by_name;
    *USF_METRIC_SETS_BY_ID().lock().unwrap() = snapshot.metric_sets_by_id;
    *USF_PHENOMENA_BY_ID().lock().unwrap() = snapshot.phenomena_by_id;
    *USF_ZONE_PHENOMENON_SUPPORT_BY_ZONE_TYPE().lock().unwrap() = snapshot.zone_phenomenon_support_by_zone_type;
    *USF_ZONE_SELECTION_POLICY_BY_ZONE_TYPE().lock().unwrap() = snapshot.zone_selection_policy_by_zone_type;
    *USF_PHENOMENON_MODELS_BY_ID().lock().unwrap() = snapshot.phenomenon_models_by_id;
    *USF_PHENOMENON_MODEL_SELECTION_BY_PHENOMENON_SCALE().lock().unwrap() = snapshot.phenomenon_model_selection_by_phenomenon_scale;
}

fn merge_set_unique<T: Eq + Hash + Clone + std::fmt::Debug>(target: &mut HashSet<T>, source: HashSet<T>, domain: &str, mod_id: &str) {
    for value in source {
        if !target.insert(value.clone()) {
            panic!(
                "USF mod composition hard error: duplicate {} entry {:?} while merging mod '{}'",
                domain, value, mod_id
            );
        }
    }
}

fn merge_map_unique<K: Eq + Hash + Clone + std::fmt::Debug, V>(target: &mut HashMap<K, V>, source: HashMap<K, V>, domain: &str, mod_id: &str) {
    for (key, value) in source {
        if target.contains_key(&key) {
            panic!(
                "USF mod composition hard error: duplicate {} entry {:?} while merging mod '{}'",
                domain, key, mod_id
            );
        }
        target.insert(key, value);
    }
}

fn singleton_conflict_policy_for_domain(package: &ScriptUsfModDefinition, domain: SingletonDomain) -> ScriptSingletonConflictPolicy {
    match domain {
        SingletonDomain::Scale => package.scale_conflict_policy,
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
    mod_id: &str,
    mod_definitions: &HashMap<String, ScriptUsfModDefinition>,
    incoming_load_order_index: usize,
    domain: SingletonDomain,
    domain_name: &str,
) {
    let incoming_definition = mod_definitions
        .get(mod_id)
        .unwrap_or_else(|| panic!("USF mod composition hard error: selected mod '{}' has no mod definition", mod_id));
    let incoming_origin = SingletonEntryOrigin {
        package_id: mod_id.to_string(),
        priority: incoming_definition.priority,
        load_order_index: incoming_load_order_index,
    };

    for (key, value) in source {
        let Some(existing_origin) = origins.get(&key).cloned() else {
            target.insert(key.clone(), value);
            origins.insert(key, incoming_origin.clone());
            continue;
        };

        let existing_definition = mod_definitions.get(existing_origin.package_id.as_str()).unwrap_or_else(|| {
            panic!(
                "USF mod composition hard error: source mod '{}' for singleton {} key {:?} is missing mod definition",
                existing_origin.package_id, domain_name, key
            )
        });
        let policy = singleton_conflict_policy_for_domain(existing_definition, domain);
        if policy == ScriptSingletonConflictPolicy::HardError {
            panic!(
                "USF mod composition hard error: singleton {} key {:?} from mod '{}' conflicts with '{}' and policy is hard_error",
                domain_name, key, mod_id, existing_origin.package_id
            );
        }
        if should_replace_singleton_value(&existing_origin, incoming_origin.priority, incoming_origin.load_order_index, policy) {
            target.insert(key.clone(), value);
            origins.insert(key, incoming_origin.clone());
        }
    }
}

fn merge_mod_contribution_into_composed(
    mod_id: &str,
    contribution: ScriptUsfModContribution,
    composed: &mut ScriptUsfModContribution,
    singleton_origins: &mut CompositionSingletonOrigins,
    mod_definitions: &HashMap<String, ScriptUsfModDefinition>,
    incoming_load_order_index: usize,
) {
    merge_set_unique(&mut composed.zone_types, contribution.zone_types, "zone_type", mod_id);

    merge_singleton_map(
        &mut composed.dpt_schemas_by_scale,
        contribution.dpt_schemas_by_scale,
        &mut singleton_origins.dpt_schema_by_scale,
        mod_id,
        mod_definitions,
        incoming_load_order_index,
        SingletonDomain::DptSchema,
        "dpt_schema_scale_index",
    );
    merge_singleton_map(
        &mut composed.zlm_scales_by_scale,
        contribution.zlm_scales_by_scale,
        &mut singleton_origins.zlm_by_scale,
        mod_id,
        mod_definitions,
        incoming_load_order_index,
        SingletonDomain::Zlm,
        "zlm_scale_index",
    );
    merge_map_unique(
        &mut composed.zone_density_profile_by_type,
        contribution.zone_density_profile_by_type,
        "zone_density_profile",
        mod_id,
    );
    merge_singleton_map(
        &mut composed.scales_by_index,
        contribution.scales_by_index,
        &mut singleton_origins.scale_by_index,
        mod_id,
        mod_definitions,
        incoming_load_order_index,
        SingletonDomain::Scale,
        "scale_index",
    );
    merge_map_unique(&mut composed.metrics_by_name, contribution.metrics_by_name, "metric_name", mod_id);
    merge_map_unique(&mut composed.metric_sets_by_id, contribution.metric_sets_by_id, "metric_set_id", mod_id);
    merge_map_unique(&mut composed.phenomena_by_id, contribution.phenomena_by_id, "phenomenon_id", mod_id);
    merge_map_unique(
        &mut composed.zone_phenomenon_support_by_zone_type,
        contribution.zone_phenomenon_support_by_zone_type,
        "zone_support_zone_type",
        mod_id,
    );
    merge_map_unique(
        &mut composed.zone_selection_policy_by_zone_type,
        contribution.zone_selection_policy_by_zone_type,
        "zone_selection_policy_zone_type",
        mod_id,
    );
    merge_map_unique(
        &mut composed.phenomenon_models_by_id,
        contribution.phenomenon_models_by_id,
        "phenomenon_model_id",
        mod_id,
    );
    merge_map_unique(
        &mut composed.phenomenon_model_selection_by_phenomenon_scale,
        contribution.phenomenon_model_selection_by_phenomenon_scale,
        "phenomenon_model_selection",
        mod_id,
    );
}

fn validate_mod_contribution_against_manifest(mod_id: &str, contribution: &ScriptUsfModContribution, manifest: &ScriptUsfModManifestDefinition) {
    for metric_name in &manifest.required_metrics {
        if !contribution.metrics_by_name.contains_key(metric_name) {
            panic!(
                "USF mod composition hard error: mod '{}' manifest requires metric '{}' but contribution did not define it",
                mod_id, metric_name
            );
        }
    }
    for metric_set_id in &manifest.required_metric_sets {
        if !contribution.metric_sets_by_id.contains_key(metric_set_id) {
            panic!(
                "USF mod composition hard error: mod '{}' manifest requires metric set '{}' but contribution did not define it",
                mod_id, metric_set_id
            );
        }
    }
    for zone_type in &manifest.required_zone_types {
        if !contribution.zone_types.contains(zone_type) {
            panic!(
                "USF mod composition hard error: mod '{}' manifest requires zone '{}' but contribution did not define it",
                mod_id, zone_type
            );
        }
    }
    for phenomenon_id in &manifest.required_phenomena {
        if !contribution.phenomena_by_id.contains_key(phenomenon_id) {
            panic!(
                "USF mod composition hard error: mod '{}' manifest requires phenomenon '{}' but contribution did not define it",
                mod_id, phenomenon_id
            );
        }
    }
    for model_id in &manifest.required_phenomenon_models {
        if !contribution.phenomenon_models_by_id.contains_key(model_id) {
            panic!(
                "USF mod composition hard error: mod '{}' manifest requires phenomenon model '{}' but contribution did not define it",
                mod_id, model_id
            );
        }
    }
    for scale_index in &manifest.required_scales {
        if !contribution.scales_by_index.contains_key(scale_index) {
            panic!(
                "USF mod composition hard error: mod '{}' manifest requires scale definition at scale {}, but contribution did not define it",
                mod_id, scale_index
            );
        }
    }
    for scale_index in &manifest.required_dpt_schema_scales {
        if !contribution.dpt_schemas_by_scale.contains_key(scale_index) {
            panic!(
                "USF mod composition hard error: mod '{}' manifest requires DPT schema at scale {}, but contribution did not define it",
                mod_id, scale_index
            );
        }
    }
    for scale_index in &manifest.required_zlm_scales {
        if !contribution.zlm_scales_by_scale.contains_key(scale_index) {
            panic!(
                "USF mod composition hard error: mod '{}' manifest requires ZLM map at scale {}, but contribution did not define it",
                mod_id, scale_index
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

fn selected_mod_ids_for_active_modpack() -> Vec<String> {
    let active_modpack_id = active_usf_modpack_id_from_config();
    let modpacks = USF_MODPACKS_BY_ID().lock().unwrap().clone();
    let Some(modpack_definition) = modpacks.get(&active_modpack_id) else {
        panic!("USF bootstrap failed: active modpack '{}' is not registered", active_modpack_id);
    };
    if modpack_definition.mod_ids.is_empty() {
        panic!("USF bootstrap failed: active modpack '{}' contains no mods", active_modpack_id);
    }

    let known_mods = USF_MODS_BY_ID().lock().unwrap().clone();
    let mut selected = Vec::<String>::new();
    let mut modpack_index_by_mod = HashMap::<String, usize>::new();
    let mut seen = HashSet::<String>::new();
    for (index, mod_id) in modpack_definition.mod_ids.iter().enumerate() {
        if !seen.insert(mod_id.clone()) {
            panic!(
                "USF bootstrap failed: active modpack '{}' contains duplicate mod '{}'",
                active_modpack_id, mod_id
            );
        }
        modpack_index_by_mod.insert(mod_id.clone(), index);
        if !known_mods.contains_key(mod_id) {
            panic!(
                "USF bootstrap failed: active modpack '{}' references unknown mod '{}'",
                active_modpack_id, mod_id
            );
        }
        selected.push(mod_id.clone());
    }

    if selected.is_empty() {
        panic!(
            "USF bootstrap failed: active modpack '{}' resolved to zero enabled mods. \
             Add at least one mod to the modpack manifest.",
            active_modpack_id
        );
    }

    let selected_set = selected.iter().cloned().collect::<HashSet<_>>();

    for mod_id in &selected {
        let mod_definition = known_mods
            .get(mod_id)
            .unwrap_or_else(|| panic!("USF bootstrap failed: mod '{}' definition missing unexpectedly", mod_id));

        for dependency in &mod_definition.dependencies {
            if !known_mods.contains_key(dependency) {
                panic!("USF bootstrap failed: mod '{}' depends_on unknown mod '{}'", mod_id, dependency);
            }
            if !selected_set.contains(dependency) {
                panic!(
                    "USF bootstrap failed: mod '{}' depends_on '{}' but dependency is not enabled in active modpack '{}'",
                    mod_id, dependency, active_modpack_id
                );
            }
        }

        for conflict in &mod_definition.conflicts_with {
            if selected_set.contains(conflict) {
                panic!(
                    "USF bootstrap failed: mod '{}' conflicts_with '{}' and both are enabled in active modpack '{}'",
                    mod_id, conflict, active_modpack_id
                );
            }
        }
    }

    let mut indegree = HashMap::<String, usize>::new();
    let mut edges = HashMap::<String, HashSet<String>>::new();
    for mod_id in &selected {
        indegree.insert(mod_id.clone(), 0);
        edges.insert(mod_id.clone(), HashSet::new());
    }

    for mod_id in &selected {
        let mod_definition = known_mods
            .get(mod_id)
            .unwrap_or_else(|| panic!("USF bootstrap failed: mod '{}' definition missing unexpectedly", mod_id));

        for dependency in &mod_definition.dependencies {
            if !selected_set.contains(dependency) {
                continue;
            }
            let adjacency = edges
                .get_mut(dependency)
                .unwrap_or_else(|| panic!("USF bootstrap failed: graph missing dependency node '{}'", dependency));
            if adjacency.insert(mod_id.clone()) {
                *indegree
                    .get_mut(mod_id)
                    .unwrap_or_else(|| panic!("USF bootstrap failed: graph missing indegree for '{}'", mod_id)) += 1;
            }
        }

        for after_mod_id in &mod_definition.load_after {
            if !known_mods.contains_key(after_mod_id) {
                panic!("USF bootstrap failed: mod '{}' load_after unknown mod '{}'", mod_id, after_mod_id);
            }
            if !selected_set.contains(after_mod_id) {
                continue;
            }
            let adjacency = edges
                .get_mut(after_mod_id)
                .unwrap_or_else(|| panic!("USF bootstrap failed: graph missing load_after node '{}'", after_mod_id));
            if adjacency.insert(mod_id.clone()) {
                *indegree
                    .get_mut(mod_id)
                    .unwrap_or_else(|| panic!("USF bootstrap failed: graph missing indegree for '{}'", mod_id)) += 1;
            }
        }
    }

    let mut resolved = Vec::<String>::new();
    let mut ready = indegree
        .iter()
        .filter_map(|(mod_id, degree)| if *degree == 0 { Some(mod_id.clone()) } else { None })
        .collect::<Vec<_>>();

    while !ready.is_empty() {
        ready.sort_by(|left, right| {
            let left_package = known_mods
                .get(left)
                .unwrap_or_else(|| panic!("USF bootstrap failed: mod '{}' missing during dependency resolution", left));
            let right_package = known_mods
                .get(right)
                .unwrap_or_else(|| panic!("USF bootstrap failed: mod '{}' missing during dependency resolution", right));
            right_package
                .priority
                .cmp(&left_package.priority)
                .then_with(|| {
                    modpack_index_by_mod
                        .get(left)
                        .copied()
                        .unwrap_or(usize::MAX)
                        .cmp(&modpack_index_by_mod.get(right).copied().unwrap_or(usize::MAX))
                })
                .then_with(|| left.cmp(right))
        });
        let current = ready.remove(0);
        resolved.push(current.clone());

        let outgoing = edges
            .get(current.as_str())
            .cloned()
            .unwrap_or_else(|| panic!("USF bootstrap failed: graph missing adjacency for mod '{}'", current));
        for downstream in outgoing {
            let degree = indegree
                .get_mut(downstream.as_str())
                .unwrap_or_else(|| panic!("USF bootstrap failed: graph missing indegree for mod '{}'", downstream));
            *degree = degree.saturating_sub(1);
            if *degree == 0 {
                ready.push(downstream);
            }
        }
    }

    if resolved.len() != selected.len() {
        let unresolved = indegree
            .iter()
            .filter_map(|(mod_id, degree)| if *degree > 0 { Some(mod_id.clone()) } else { None })
            .collect::<Vec<_>>();
        panic!(
            "USF bootstrap failed: dependency cycle detected in active modpack '{}'; unresolved mods: {:?}",
            active_modpack_id, unresolved
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

    let selected_mod_ids = selected_mod_ids_for_active_modpack();
    let mod_manifests = USF_MOD_MANIFESTS_BY_ID().lock().unwrap().clone();
    let mod_definitions = USF_MODS_BY_ID().lock().unwrap().clone();
    let mut composed = ScriptUsfModContribution::default();
    let mut singleton_origins = CompositionSingletonOrigins::default();
    let mut mod_contributions = HashMap::<String, ScriptUsfModContribution>::new();

    for (load_order_index, mod_id) in selected_mod_ids.into_iter().enumerate() {
        clear_usf_domain_bootstrap_statics();
        for spec in USF_PACKAGE_SCOPED_SCRIPT_TYPE_SPECS {
            run_usf_script_type_bootstrap_for_package(engine, &usf_root, spec, mod_id.as_str());
        }
        let contribution = snapshot_usf_domain_statics();
        let Some(manifest) = mod_manifests.get(mod_id.as_str()) else {
            panic!(
                "USF mod composition hard error: selected mod '{}' has no manifest. \
                 Declare mod requirements in '*.mod.rhai'.",
                mod_id
            );
        };
        validate_mod_contribution_against_manifest(mod_id.as_str(), &contribution, manifest);
        merge_mod_contribution_into_composed(
            mod_id.as_str(),
            contribution.clone(),
            &mut composed,
            &mut singleton_origins,
            &mod_definitions,
            load_order_index,
        );
        mod_contributions.insert(mod_id, contribution);
    }

    *USF_MOD_CONTRIBUTIONS_BY_ID().lock().unwrap() = mod_contributions;
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

fn ensure_mod_and_manifest_registered(mod_id: &str) {
    let mut mods = USF_MODS_BY_ID().lock().unwrap();
    mods.entry(mod_id.to_string()).or_insert_with(ScriptUsfModDefinition::default);
    drop(mods);

    let mut manifests = USF_MOD_MANIFESTS_BY_ID().lock().unwrap();
    manifests.entry(mod_id.to_string()).or_insert_with(ScriptUsfModManifestDefinition::default);
}

fn ensure_modpack_registered(modpack_id: &str) {
    let mut modpacks = USF_MODPACKS_BY_ID().lock().unwrap();
    modpacks.entry(modpack_id.to_string()).or_insert_with(ScriptUsfModpackDefinition::default);
}

#[inline]
fn ensure_owner_mod_for_ctx(owner_mod_id: &str) -> Result<String, Box<EvalAltResult>> {
    let owner_mod_id = normalize_script_identifier("owner_mod_id", owner_mod_id)?;
    ensure_mod_and_manifest_registered(owner_mod_id.as_str());
    Ok(owner_mod_id)
}

fn with_owner_mod_manifest_mut(owner_mod_id: &str, mutate: impl FnOnce(&mut ScriptUsfModManifestDefinition)) -> Result<(), Box<EvalAltResult>> {
    let owner_mod_id = ensure_owner_mod_for_ctx(owner_mod_id)?;
    let mut manifests = USF_MOD_MANIFESTS_BY_ID().lock().unwrap();
    let Some(manifest) = manifests.get_mut(owner_mod_id.as_str()) else {
        return Err(format!("mod '{}' manifest is not registered", owner_mod_id).into());
    };
    mutate(manifest);
    Ok(())
}

#[inline]
fn parse_u16_value(name: &str, value: i64) -> Result<u16, Box<EvalAltResult>> {
    u16::try_from(value).map_err(|_| format!("{name} must fit in u16, got {value}").into())
}

#[inline]
fn parse_scale_index_with_name(name: &str, value: i64) -> Result<u8, Box<EvalAltResult>> {
    if value < 0 || value >= Scale::SCALE_LEVEL_COUNT as i64 {
        return Err(format!("{name} must be within [0..{}], got {}", Scale::SCALE_LEVEL_COUNT.saturating_sub(1), value).into());
    }
    Ok(value as u8)
}

#[inline]
fn parse_scale_index(value: i64) -> Result<u8, Box<EvalAltResult>> {
    parse_scale_index_with_name("scale_index", value)
}

fn parse_scale_index_set(name: &str, values: rhai::Array) -> Result<Vec<u8>, Box<EvalAltResult>> {
    let mut out = Vec::<u8>::with_capacity(values.len());
    let mut seen = HashSet::<u8>::new();
    for (entry_index, value) in values.into_iter().enumerate() {
        let Some(int_value) = value.clone().try_cast::<i64>() else {
            return Err(format!("{name}[{}] must be an integer", entry_index).into());
        };
        let parsed = parse_scale_index_with_name(name, int_value)?;
        if seen.insert(parsed) {
            out.push(parsed);
        }
    }
    Ok(out)
}

#[inline]
fn parse_support_chunk_radius(value: i64) -> Result<u16, Box<EvalAltResult>> {
    if value < 0 {
        return Err(format!("support_chunk_radius must be >= 0, got {value}").into());
    }
    u16::try_from(value).map_err(|_| format!("support_chunk_radius must fit in u16, got {value}").into())
}

#[inline]
fn parse_positive_revision(value: i64) -> Result<u64, Box<EvalAltResult>> {
    if value < 1 {
        return Err(format!("revision must be >= 1, got {value}").into());
    }
    Ok(value as u64)
}

#[inline]
fn parse_finite_f32(name: &str, value: rhai::FLOAT) -> Result<f32, Box<EvalAltResult>> {
    if !value.is_finite() {
        return Err(format!("{name} must be finite, got {value}").into());
    }
    Ok(value as f32)
}

#[inline]
fn parse_metric_value_type(value: &str) -> Result<String, Box<EvalAltResult>> {
    let value = normalize_script_identifier("value_type", value)?;
    match value.as_str() {
        "u8" | "u16" | "i32" | "f32" | "f64" => Ok(value),
        _ => Err(format!("value_type '{}' is invalid; expected one of: u8, u16, i32, f32, f64", value).into()),
    }
}

#[inline]
fn parse_metric_storage_class(value: &str) -> Result<String, Box<EvalAltResult>> {
    let value = normalize_script_identifier("storage_class", value)?;
    match value.as_str() {
        "uniform" | "brick" => Ok(value),
        _ => Err(format!("storage_class '{}' is invalid; expected one of: uniform, brick", value).into()),
    }
}

#[inline]
fn normalize_zone_type(zone_type: &str) -> Result<String, Box<EvalAltResult>> {
    normalize_script_identifier("zone_type", zone_type)
}

#[inline]
fn normalize_spawn_policy(value: &str) -> Result<String, Box<EvalAltResult>> {
    let value = normalize_script_identifier("spawn_policy", value)?;
    match value.as_str() {
        "single_per_zone" | "single-per-zone" | "single" => Ok("single_per_zone".to_string()),
        _ => Err(format!("unsupported spawn_policy '{}'; currently supported: single_per_zone", value).into()),
    }
}

#[inline]
fn normalize_selection_strategy(value: &str) -> Result<String, Box<EvalAltResult>> {
    let value = normalize_script_identifier("selection_strategy", value)?;
    match value.as_str() {
        "weighted_top_priority" | "weighted-top-priority" | "weighted" => Ok("weighted_top_priority".to_string()),
        "highest_weight_top_priority" | "highest-weight-top-priority" | "highest_weight" => Ok("highest_weight_top_priority".to_string()),
        "round_robin_top_priority" | "round-robin-top-priority" | "round_robin" => Ok("round_robin_top_priority".to_string()),
        _ => Err(format!(
            "unsupported selection strategy '{}'; supported: weighted_top_priority, highest_weight_top_priority, round_robin_top_priority",
            value
        )
        .into()),
    }
}

#[inline]
fn normalize_phenomenon_kind(kind: &str) -> Result<String, Box<EvalAltResult>> {
    let kind = normalize_script_identifier("phenomenon_kind", kind)?;
    let parsed = PhenomenonKind::try_from_config_value(kind.as_str()).map_err(|error| format!("unknown phenomenon_kind '{}': {}", kind, error))?;
    Ok(parsed.canonical_id().to_string())
}

#[inline]
fn normalize_phenomenon_capability(capability: &str) -> Result<String, Box<EvalAltResult>> {
    let capability = normalize_script_identifier("capability", capability)?;
    let parsed = PhenomenonCapability::try_from_config_value(capability.as_str())
        .map_err(|error| format!("unknown phenomenon capability '{}': {}", capability, error))?;
    Ok(parsed.canonical_id().to_string())
}

#[inline]
fn normalize_phenomena_model_topology(topology: &str) -> Result<String, Box<EvalAltResult>> {
    let topology = normalize_script_identifier("topology", topology)?;
    match topology.as_str() {
        "monolithic_chunk" | "monolithic-chunk" | "monolithic" => Ok("monolithic_chunk".to_string()),
        "partitioned_by_chunk" | "partitioned-by-chunk" | "partitioned" => Ok("partitioned_by_chunk".to_string()),
        _ => Err(format!("unsupported topology '{}'; supported: monolithic_chunk, partitioned_by_chunk", topology).into()),
    }
}

#[inline]
fn phenomenon_model_selection_key(phenomenon_id: &str, scale_index: u8) -> String {
    format!("{phenomenon_id}@{scale_index}")
}

fn set_phenomenon_model_selection(phenomenon_id: &str, scale_index: u8, model_id: &str) -> Result<(), Box<EvalAltResult>> {
    if !USF_PHENOMENA_BY_ID().lock().unwrap().contains_key(phenomenon_id) {
        return Err(format!("phenomenon '{}' is not registered", phenomenon_id).into());
    }
    let Some(model) = USF_PHENOMENON_MODELS_BY_ID().lock().unwrap().get(model_id).cloned() else {
        return Err(format!("phenomenon model '{}' is not registered", model_id).into());
    };
    if model.phenomenon_id != phenomenon_id {
        return Err(format!(
            "phenomenon model '{}' belongs to '{}', but was assigned to '{}'",
            model_id, model.phenomenon_id, phenomenon_id
        )
        .into());
    }

    USF_PHENOMENON_MODEL_SELECTION_BY_PHENOMENON_SCALE()
        .lock()
        .unwrap()
        .insert(phenomenon_model_selection_key(phenomenon_id, scale_index), model_id.to_string());
    Ok(())
}

fn register_usf_script_ctx_runtime_module(engine: &mut rhai::Engine) {
    engine.register_type_with_name::<UsfScriptCtx>("UsfScriptCtx");
    engine.register_get("script_kind", |ctx: &mut UsfScriptCtx| ctx.script_kind.clone());
    engine.register_get("script_file", |ctx: &mut UsfScriptCtx| ctx.script_file.clone());
    engine.register_get("script_id", |ctx: &mut UsfScriptCtx| ctx.script_id.clone());
    engine.register_get("owner_mod_id", |ctx: &mut UsfScriptCtx| ctx.owner_mod_id.clone());

    engine.register_type_with_name::<UsfModScriptCtx>("UsfModScriptCtx");
    engine.register_get("mod_id", |ctx: &mut UsfModScriptCtx| ctx.mod_id.clone());
    engine.register_get("script_file", |ctx: &mut UsfModScriptCtx| ctx.script_file.clone());

    engine.register_fn("register", |ctx: &mut UsfModScriptCtx| -> Result<(), Box<EvalAltResult>> {
        let mod_id = normalize_script_identifier("mod_id", ctx.mod_id.as_str())?;
        ensure_mod_and_manifest_registered(mod_id.as_str());
        Ok(())
    });
    engine.register_fn("set_priority", |ctx: &mut UsfModScriptCtx, priority: i64| -> Result<(), Box<EvalAltResult>> {
        if priority < i32::MIN as i64 || priority > i32::MAX as i64 {
            return Err(format!("priority must be in [{}..={}], got {}", i32::MIN, i32::MAX, priority).into());
        }
        let mod_id = normalize_script_identifier("mod_id", ctx.mod_id.as_str())?;
        ensure_mod_and_manifest_registered(mod_id.as_str());
        let mut mods = USF_MODS_BY_ID().lock().unwrap();
        let Some(mod_definition) = mods.get_mut(mod_id.as_str()) else {
            return Err(format!("mod '{}' is not registered", mod_id).into());
        };
        mod_definition.priority = priority as i32;
        Ok(())
    });
    engine.register_fn(
        "depends_on",
        |ctx: &mut UsfModScriptCtx, dependency_mod_id: &str| -> Result<(), Box<EvalAltResult>> {
            let mod_id = normalize_script_identifier("mod_id", ctx.mod_id.as_str())?;
            let dependency_mod_id = normalize_script_identifier("dependency_mod_id", dependency_mod_id)?;
            if mod_id == dependency_mod_id {
                return Err(format!("mod '{}' cannot depend on itself", mod_id).into());
            }
            ensure_mod_and_manifest_registered(mod_id.as_str());
            let mut mods = USF_MODS_BY_ID().lock().unwrap();
            let Some(mod_definition) = mods.get_mut(mod_id.as_str()) else {
                return Err(format!("mod '{}' is not registered", mod_id).into());
            };
            mod_definition.dependencies.insert(dependency_mod_id);
            Ok(())
        },
    );
    engine.register_fn(
        "load_after",
        |ctx: &mut UsfModScriptCtx, other_mod_id: &str| -> Result<(), Box<EvalAltResult>> {
            let mod_id = normalize_script_identifier("mod_id", ctx.mod_id.as_str())?;
            let other_mod_id = normalize_script_identifier("other_mod_id", other_mod_id)?;
            if mod_id == other_mod_id {
                return Err(format!("mod '{}' cannot load_after itself", mod_id).into());
            }
            ensure_mod_and_manifest_registered(mod_id.as_str());
            let mut mods = USF_MODS_BY_ID().lock().unwrap();
            let Some(mod_definition) = mods.get_mut(mod_id.as_str()) else {
                return Err(format!("mod '{}' is not registered", mod_id).into());
            };
            mod_definition.load_after.insert(other_mod_id);
            Ok(())
        },
    );
    engine.register_fn(
        "conflicts_with",
        |ctx: &mut UsfModScriptCtx, conflicting_mod_id: &str| -> Result<(), Box<EvalAltResult>> {
            let mod_id = normalize_script_identifier("mod_id", ctx.mod_id.as_str())?;
            let conflicting_mod_id = normalize_script_identifier("conflicting_mod_id", conflicting_mod_id)?;
            if mod_id == conflicting_mod_id {
                return Err(format!("mod '{}' cannot conflict with itself", mod_id).into());
            }
            ensure_mod_and_manifest_registered(mod_id.as_str());
            let mut mods = USF_MODS_BY_ID().lock().unwrap();
            let Some(mod_definition) = mods.get_mut(mod_id.as_str()) else {
                return Err(format!("mod '{}' is not registered", mod_id).into());
            };
            mod_definition.conflicts_with.insert(conflicting_mod_id);
            Ok(())
        },
    );
    engine.register_fn(
        "set_singleton_conflict_policy",
        |ctx: &mut UsfModScriptCtx, singleton_domain: &str, policy_tag: &str| -> Result<(), Box<EvalAltResult>> {
            let mod_id = normalize_script_identifier("mod_id", ctx.mod_id.as_str())?;
            let singleton_domain = normalize_script_identifier("singleton_domain", singleton_domain)?;
            let policy = parse_singleton_conflict_policy_tag(policy_tag)?;
            ensure_mod_and_manifest_registered(mod_id.as_str());
            let mut mods = USF_MODS_BY_ID().lock().unwrap();
            let Some(mod_definition) = mods.get_mut(mod_id.as_str()) else {
                return Err(format!("mod '{}' is not registered", mod_id).into());
            };

            match singleton_domain.as_str() {
                "scale" => mod_definition.scale_conflict_policy = policy,
                "dpt_schema" => mod_definition.dpt_schema_conflict_policy = policy,
                "zlm" | "zlm_scale" => mod_definition.zlm_conflict_policy = policy,
                _ => return Err(format!("singleton_domain '{}' is invalid; expected one of: scale, dpt_schema, zlm", singleton_domain).into()),
            }
            Ok(())
        },
    );

    engine.register_fn(
        "declare_metric",
        |ctx: &mut UsfModScriptCtx, metric_name: &str| -> Result<(), Box<EvalAltResult>> {
            let mod_id = normalize_script_identifier("mod_id", ctx.mod_id.as_str())?;
            let metric_name = normalize_script_identifier("metric_name", metric_name)?;
            ensure_mod_and_manifest_registered(mod_id.as_str());
            let mut manifests = USF_MOD_MANIFESTS_BY_ID().lock().unwrap();
            let Some(manifest) = manifests.get_mut(mod_id.as_str()) else {
                return Err(format!("mod '{}' manifest is not registered", mod_id).into());
            };
            manifest.required_metrics.insert(metric_name);
            Ok(())
        },
    );
    engine.register_fn(
        "declare_metric_set",
        |ctx: &mut UsfModScriptCtx, metric_set_id: &str| -> Result<(), Box<EvalAltResult>> {
            let mod_id = normalize_script_identifier("mod_id", ctx.mod_id.as_str())?;
            let metric_set_id = normalize_script_identifier("metric_set_id", metric_set_id)?;
            ensure_mod_and_manifest_registered(mod_id.as_str());
            let mut manifests = USF_MOD_MANIFESTS_BY_ID().lock().unwrap();
            let Some(manifest) = manifests.get_mut(mod_id.as_str()) else {
                return Err(format!("mod '{}' manifest is not registered", mod_id).into());
            };
            manifest.required_metric_sets.insert(metric_set_id);
            Ok(())
        },
    );
    engine.register_fn("declare_zone", |ctx: &mut UsfModScriptCtx, zone_type: &str| -> Result<(), Box<EvalAltResult>> {
        let mod_id = normalize_script_identifier("mod_id", ctx.mod_id.as_str())?;
        let zone_type = normalize_script_identifier("zone_type", zone_type)?;
        ensure_mod_and_manifest_registered(mod_id.as_str());
        let mut manifests = USF_MOD_MANIFESTS_BY_ID().lock().unwrap();
        let Some(manifest) = manifests.get_mut(mod_id.as_str()) else {
            return Err(format!("mod '{}' manifest is not registered", mod_id).into());
        };
        manifest.required_zone_types.insert(zone_type);
        Ok(())
    });
    engine.register_fn(
        "declare_phenomenon",
        |ctx: &mut UsfModScriptCtx, phenomenon_id: &str| -> Result<(), Box<EvalAltResult>> {
            let mod_id = normalize_script_identifier("mod_id", ctx.mod_id.as_str())?;
            let phenomenon_id = normalize_script_identifier("phenomenon_id", phenomenon_id)?;
            ensure_mod_and_manifest_registered(mod_id.as_str());
            let mut manifests = USF_MOD_MANIFESTS_BY_ID().lock().unwrap();
            let Some(manifest) = manifests.get_mut(mod_id.as_str()) else {
                return Err(format!("mod '{}' manifest is not registered", mod_id).into());
            };
            manifest.required_phenomena.insert(phenomenon_id);
            Ok(())
        },
    );
    engine.register_fn(
        "declare_phenomenon_model",
        |ctx: &mut UsfModScriptCtx, model_id: &str| -> Result<(), Box<EvalAltResult>> {
            let mod_id = normalize_script_identifier("mod_id", ctx.mod_id.as_str())?;
            let model_id = normalize_script_identifier("model_id", model_id)?;
            ensure_mod_and_manifest_registered(mod_id.as_str());
            let mut manifests = USF_MOD_MANIFESTS_BY_ID().lock().unwrap();
            let Some(manifest) = manifests.get_mut(mod_id.as_str()) else {
                return Err(format!("mod '{}' manifest is not registered", mod_id).into());
            };
            manifest.required_phenomenon_models.insert(model_id);
            Ok(())
        },
    );
    engine.register_fn("declare_all_scales", |ctx: &mut UsfModScriptCtx| -> Result<(), Box<EvalAltResult>> {
        let mod_id = normalize_script_identifier("mod_id", ctx.mod_id.as_str())?;
        ensure_mod_and_manifest_registered(mod_id.as_str());
        let mut manifests = USF_MOD_MANIFESTS_BY_ID().lock().unwrap();
        let Some(manifest) = manifests.get_mut(mod_id.as_str()) else {
            return Err(format!("mod '{}' manifest is not registered", mod_id).into());
        };
        manifest.required_scales.extend(0..(Scale::SCALE_LEVEL_COUNT as u8));
        Ok(())
    });
    engine.register_fn("declare_all_dpt_schemas", |ctx: &mut UsfModScriptCtx| -> Result<(), Box<EvalAltResult>> {
        let mod_id = normalize_script_identifier("mod_id", ctx.mod_id.as_str())?;
        ensure_mod_and_manifest_registered(mod_id.as_str());
        let mut manifests = USF_MOD_MANIFESTS_BY_ID().lock().unwrap();
        let Some(manifest) = manifests.get_mut(mod_id.as_str()) else {
            return Err(format!("mod '{}' manifest is not registered", mod_id).into());
        };
        manifest.required_dpt_schema_scales.extend(0..(Scale::SCALE_LEVEL_COUNT as u8));
        Ok(())
    });
    engine.register_fn("declare_all_zlms", |ctx: &mut UsfModScriptCtx| -> Result<(), Box<EvalAltResult>> {
        let mod_id = normalize_script_identifier("mod_id", ctx.mod_id.as_str())?;
        ensure_mod_and_manifest_registered(mod_id.as_str());
        let mut manifests = USF_MOD_MANIFESTS_BY_ID().lock().unwrap();
        let Some(manifest) = manifests.get_mut(mod_id.as_str()) else {
            return Err(format!("mod '{}' manifest is not registered", mod_id).into());
        };
        manifest.required_zlm_scales.extend(0..(Scale::SCALE_LEVEL_COUNT as u8));
        Ok(())
    });

    engine.register_type_with_name::<UsfModpackScriptCtx>("UsfModpackScriptCtx");
    engine.register_get("modpack_id", |ctx: &mut UsfModpackScriptCtx| ctx.modpack_id.clone());
    engine.register_get("script_file", |ctx: &mut UsfModpackScriptCtx| ctx.script_file.clone());

    engine.register_fn("register", |ctx: &mut UsfModpackScriptCtx| -> Result<(), Box<EvalAltResult>> {
        let modpack_id = normalize_script_identifier("modpack_id", ctx.modpack_id.as_str())?;
        ensure_modpack_registered(modpack_id.as_str());
        Ok(())
    });
    engine.register_fn("add_mod", |ctx: &mut UsfModpackScriptCtx, mod_id: &str| -> Result<(), Box<EvalAltResult>> {
        let modpack_id = normalize_script_identifier("modpack_id", ctx.modpack_id.as_str())?;
        let mod_id = normalize_script_identifier("mod_id", mod_id)?;
        ensure_modpack_registered(modpack_id.as_str());
        let mut modpacks = USF_MODPACKS_BY_ID().lock().unwrap();
        let Some(modpack) = modpacks.get_mut(modpack_id.as_str()) else {
            return Err(format!("modpack '{}' is not registered", modpack_id).into());
        };
        if !modpack.mod_ids.iter().any(|existing| existing == &mod_id) {
            modpack.mod_ids.push(mod_id);
        }
        Ok(())
    });

    engine.register_type_with_name::<UsfMetricScriptCtx>("UsfMetricScriptCtx");
    engine.register_get("metric_name", |ctx: &mut UsfMetricScriptCtx| ctx.metric_name.clone());
    engine.register_get("owner_mod_id", |ctx: &mut UsfMetricScriptCtx| ctx.owner_mod_id.clone());
    engine.register_get("script_file", |ctx: &mut UsfMetricScriptCtx| ctx.script_file.clone());
    engine.register_fn("scale_level_count", |_ctx: &mut UsfMetricScriptCtx| -> rhai::INT {
        Scale::SCALE_LEVEL_COUNT as rhai::INT
    });
    engine.register_fn("set_id", |ctx: &mut UsfMetricScriptCtx, metric_name: &str| -> Result<(), Box<EvalAltResult>> {
        ctx.metric_name = normalize_script_identifier("metric_name", metric_name)?;
        Ok(())
    });
    engine.register_fn(
        "register",
        |ctx: &mut UsfMetricScriptCtx,
         metric_id: i64,
         value_type: &str,
         semantics_tag: &str,
         storage_class: &str,
         derived: bool,
         min_scale_index: i64,
         max_scale_index: i64|
         -> Result<(), Box<EvalAltResult>> {
            let owner_mod_id = ensure_owner_mod_for_ctx(ctx.owner_mod_id.as_str())?;
            let metric_id = parse_u16_value("metric_id", metric_id)?;
            let metric_name = normalize_script_identifier("metric_name", ctx.metric_name.as_str())?;
            let value_type = parse_metric_value_type(value_type)?;
            let semantics_tag = normalize_script_identifier("semantics_tag", semantics_tag)?;
            let storage_class = parse_metric_storage_class(storage_class)?;
            let min_scale_index = parse_scale_index_with_name("min_scale_index", min_scale_index)?;
            let max_scale_index = parse_scale_index_with_name("max_scale_index", max_scale_index)?;
            if min_scale_index > max_scale_index {
                return Err(format!("invalid metric scale range [{min_scale_index}..{max_scale_index}] for metric '{}'", metric_name).into());
            }

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

            let mut metrics = USF_METRICS_BY_NAME().lock().unwrap();
            if let Some(existing) = metrics.get(&metric_name) {
                if existing != &definition {
                    return Err(format!("metric '{}' already exists with a different definition", metric_name).into());
                }
            } else {
                if let Some(conflict) = metrics.values().find(|value| value.id == metric_id) {
                    return Err(format!("metric_id {} is already assigned to metric '{}'", metric_id, conflict.name).into());
                }
                metrics.insert(metric_name.clone(), definition);
            }
            drop(metrics);

            with_owner_mod_manifest_mut(owner_mod_id.as_str(), |manifest| {
                manifest.required_metrics.insert(metric_name);
            })?;
            Ok(())
        },
    );

    engine.register_type_with_name::<UsfMetricSetScriptCtx>("UsfMetricSetScriptCtx");
    engine.register_get("metric_set_id", |ctx: &mut UsfMetricSetScriptCtx| ctx.metric_set_id.clone());
    engine.register_get("owner_mod_id", |ctx: &mut UsfMetricSetScriptCtx| ctx.owner_mod_id.clone());
    engine.register_get("script_file", |ctx: &mut UsfMetricSetScriptCtx| ctx.script_file.clone());
    engine.register_fn(
        "set_id",
        |ctx: &mut UsfMetricSetScriptCtx, metric_set_id: &str| -> Result<(), Box<EvalAltResult>> {
            ctx.metric_set_id = normalize_script_identifier("metric_set_id", metric_set_id)?;
            Ok(())
        },
    );
    engine.register_fn("register", |ctx: &mut UsfMetricSetScriptCtx| -> Result<(), Box<EvalAltResult>> {
        let owner_mod_id = ensure_owner_mod_for_ctx(ctx.owner_mod_id.as_str())?;
        let metric_set_id = normalize_script_identifier("metric_set_id", ctx.metric_set_id.as_str())?;
        USF_METRIC_SETS_BY_ID().lock().unwrap().entry(metric_set_id.clone()).or_default();
        with_owner_mod_manifest_mut(owner_mod_id.as_str(), |manifest| {
            manifest.required_metric_sets.insert(metric_set_id);
        })?;
        Ok(())
    });
    engine.register_fn(
        "add_metric",
        |ctx: &mut UsfMetricSetScriptCtx, metric_name: &str| -> Result<(), Box<EvalAltResult>> {
            let metric_set_id = normalize_script_identifier("metric_set_id", ctx.metric_set_id.as_str())?;
            let metric_name = normalize_script_identifier("metric_name", metric_name)?;
            let metrics = USF_METRICS_BY_NAME().lock().unwrap();
            if !metrics.contains_key(&metric_name) {
                return Err(format!("metric '{}' is not registered", metric_name).into());
            }
            drop(metrics);

            let mut metric_sets = USF_METRIC_SETS_BY_ID().lock().unwrap();
            let Some(metric_set) = metric_sets.get_mut(&metric_set_id) else {
                return Err(format!("metric_set '{}' is not registered; call ctx.register() first", metric_set_id).into());
            };
            if !metric_set.iter().any(|entry| entry == &metric_name) {
                metric_set.push(metric_name);
            }
            Ok(())
        },
    );

    engine.register_type_with_name::<UsfZoneScriptCtx>("UsfZoneScriptCtx");
    engine.register_get("zone_type", |ctx: &mut UsfZoneScriptCtx| ctx.zone_type.clone());
    engine.register_get("owner_mod_id", |ctx: &mut UsfZoneScriptCtx| ctx.owner_mod_id.clone());
    engine.register_get("script_file", |ctx: &mut UsfZoneScriptCtx| ctx.script_file.clone());
    engine.register_fn("set_id", |ctx: &mut UsfZoneScriptCtx, zone_type: &str| -> Result<(), Box<EvalAltResult>> {
        ctx.zone_type = normalize_zone_type(zone_type)?;
        Ok(())
    });
    engine.register_fn("register", |ctx: &mut UsfZoneScriptCtx| -> Result<(), Box<EvalAltResult>> {
        let owner_mod_id = ensure_owner_mod_for_ctx(ctx.owner_mod_id.as_str())?;
        let zone_type = normalize_zone_type(ctx.zone_type.as_str())?;
        USF_ZONE_TYPES().lock().unwrap().insert(zone_type.clone());
        with_owner_mod_manifest_mut(owner_mod_id.as_str(), |manifest| {
            manifest.required_zone_types.insert(zone_type);
        })?;
        Ok(())
    });
    engine.register_fn(
        "set_density_profile",
        |ctx: &mut UsfZoneScriptCtx,
         density_multiplier: rhai::FLOAT,
         density_offset: rhai::FLOAT,
         density_floor: rhai::FLOAT,
         density_ceil: rhai::FLOAT|
         -> Result<(), Box<EvalAltResult>> {
            let zone_type = normalize_zone_type(ctx.zone_type.as_str())?;
            if !USF_ZONE_TYPES().lock().unwrap().contains(&zone_type) {
                return Err(format!("zone_type '{}' is not registered; call ctx.register() first", zone_type).into());
            }
            let density_multiplier = parse_finite_f32("density_multiplier", density_multiplier)?;
            let density_offset = parse_finite_f32("density_offset", density_offset)?;
            let density_floor = parse_finite_f32("density_floor", density_floor)?;
            let density_ceil = parse_finite_f32("density_ceil", density_ceil)?;
            if density_floor > density_ceil {
                return Err(format!("density_floor ({density_floor}) must be <= density_ceil ({density_ceil})").into());
            }

            USF_ZONE_DENSITY_PROFILE_BY_TYPE().lock().unwrap().insert(
                zone_type,
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
    engine.register_fn(
        "set_selection_policy",
        |ctx: &mut UsfZoneScriptCtx, strategy: &str| -> Result<(), Box<EvalAltResult>> {
            let zone_type = normalize_zone_type(ctx.zone_type.as_str())?;
            if !USF_ZONE_TYPES().lock().unwrap().contains(&zone_type) {
                return Err(format!("zone_type '{}' is not registered; call ctx.register() first", zone_type).into());
            }
            let strategy = normalize_selection_strategy(strategy)?;
            USF_ZONE_SELECTION_POLICY_BY_ZONE_TYPE()
                .lock()
                .unwrap()
                .insert(zone_type, ScriptZoneSelectionPolicyDefinition { strategy });
            Ok(())
        },
    );
    engine.register_fn(
        "add_supported_phenomenon",
        |ctx: &mut UsfZoneScriptCtx,
         phenomenon_id: &str,
         priority: i64,
         weight: rhai::FLOAT,
         spawn_policy: &str,
         max_active: i64|
         -> Result<(), Box<EvalAltResult>> {
            let zone_type = normalize_zone_type(ctx.zone_type.as_str())?;
            if !USF_ZONE_TYPES().lock().unwrap().contains(&zone_type) {
                return Err(format!("zone_type '{}' is not registered; call ctx.register() first", zone_type).into());
            }
            let phenomenon_id = normalize_script_identifier("phenomenon_id", phenomenon_id)?;
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
            let supports = supports_by_zone.entry(zone_type).or_default();
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

    engine.register_type_with_name::<UsfZlmScriptCtx>("UsfZlmScriptCtx");
    engine.register_get("zlm_id", |ctx: &mut UsfZlmScriptCtx| ctx.zlm_id.clone());
    engine.register_get("owner_mod_id", |ctx: &mut UsfZlmScriptCtx| ctx.owner_mod_id.clone());
    engine.register_get("script_file", |ctx: &mut UsfZlmScriptCtx| ctx.script_file.clone());
    engine.register_fn("set_id", |ctx: &mut UsfZlmScriptCtx, zlm_id: &str| -> Result<(), Box<EvalAltResult>> {
        ctx.zlm_id = normalize_script_identifier("zlm_id", zlm_id)?;
        Ok(())
    });
    engine.register_fn("scale_level_count", |_ctx: &mut UsfZlmScriptCtx| -> rhai::INT {
        Scale::SCALE_LEVEL_COUNT as rhai::INT
    });
    engine.register_fn(
        "set_scale",
        |ctx: &mut UsfZlmScriptCtx, scale_index: i64, revision: i64, fallback_zone: &str| -> Result<(), Box<EvalAltResult>> {
            let owner_mod_id = ensure_owner_mod_for_ctx(ctx.owner_mod_id.as_str())?;
            let scale_index = parse_scale_index(scale_index)?;
            let revision = parse_positive_revision(revision)?;
            let fallback_zone = normalize_zone_type(fallback_zone)?;
            USF_ZLM_SCALES_BY_SCALE().lock().unwrap().insert(
                scale_index,
                ScriptZlmScaleDefinition {
                    revision,
                    fallback_zone,
                    rules: Vec::new(),
                },
            );
            with_owner_mod_manifest_mut(owner_mod_id.as_str(), |manifest| {
                manifest.required_zlm_scales.insert(scale_index);
            })?;
            Ok(())
        },
    );
    engine.register_fn(
        "add_rule",
        |_ctx: &mut UsfZlmScriptCtx, scale_index: i64, zone_type: &str| -> Result<rhai::INT, Box<EvalAltResult>> {
            let scale_index = parse_scale_index(scale_index)?;
            let zone_type = normalize_zone_type(zone_type)?;
            let mut maps = USF_ZLM_SCALES_BY_SCALE().lock().unwrap();
            let Some(scale_map) = maps.get_mut(&scale_index) else {
                return Err(format!("no ZLM map defined for scale_index={scale_index}; call ctx.set_scale first").into());
            };
            scale_map.rules.push(ScriptZlmRuleDefinition {
                zone_type,
                metric_bands: Vec::new(),
            });
            Ok((scale_map.rules.len().saturating_sub(1)) as rhai::INT)
        },
    );
    engine.register_fn(
        "add_metric_band",
        |_ctx: &mut UsfZlmScriptCtx, scale_index: i64, rule_index: i64, metric_id: i64, min: rhai::FLOAT, max: rhai::FLOAT| -> Result<(), Box<EvalAltResult>> {
            let scale_index = parse_scale_index(scale_index)?;
            if rule_index < 0 {
                return Err(format!("rule_index must be >= 0, got {rule_index}").into());
            }
            let rule_index = rule_index as usize;
            let metric_id = parse_u16_value("metric_id", metric_id)?;
            let min = parse_finite_f32("min", min)?;
            let max = parse_finite_f32("max", max)?;
            if min > max {
                return Err(format!("zlm metric band min ({min}) must be <= max ({max})").into());
            }

            let mut maps = USF_ZLM_SCALES_BY_SCALE().lock().unwrap();
            let Some(scale_map) = maps.get_mut(&scale_index) else {
                return Err(format!("no ZLM map defined for scale_index={scale_index}; call ctx.set_scale first").into());
            };
            if rule_index >= scale_map.rules.len() {
                return Err(format!(
                    "rule_index {} is out of bounds for scale {} (rule count={})",
                    rule_index,
                    scale_index,
                    scale_map.rules.len()
                )
                .into());
            }
            scale_map.rules[rule_index]
                .metric_bands
                .push(ScriptZlmMetricBandDefinition { metric_id, min, max });
            Ok(())
        },
    );
    engine.register_fn(
        "add_metric_band_by_name",
        |_ctx: &mut UsfZlmScriptCtx,
         scale_index: i64,
         rule_index: i64,
         metric_name: &str,
         min: rhai::FLOAT,
         max: rhai::FLOAT|
         -> Result<(), Box<EvalAltResult>> {
            let metric_name = normalize_script_identifier("metric_name", metric_name)?;
            let Some(metric_id) = USF_METRICS_BY_NAME().lock().unwrap().get(metric_name.as_str()).map(|metric| metric.id as i64) else {
                return Err(format!("metric '{}' is not registered", metric_name).into());
            };
            let scale_index = parse_scale_index(scale_index)?;
            if rule_index < 0 {
                return Err(format!("rule_index must be >= 0, got {rule_index}").into());
            }
            let rule_index = rule_index as usize;
            let metric_id = parse_u16_value("metric_id", metric_id)?;
            let min = parse_finite_f32("min", min)?;
            let max = parse_finite_f32("max", max)?;
            if min > max {
                return Err(format!("zlm metric band min ({min}) must be <= max ({max})").into());
            }
            let mut maps = USF_ZLM_SCALES_BY_SCALE().lock().unwrap();
            let Some(scale_map) = maps.get_mut(&scale_index) else {
                return Err(format!("no ZLM map defined for scale_index={scale_index}; call ctx.set_scale first").into());
            };
            if rule_index >= scale_map.rules.len() {
                return Err(format!(
                    "rule_index {} is out of bounds for scale {} (rule count={})",
                    rule_index,
                    scale_index,
                    scale_map.rules.len()
                )
                .into());
            }
            scale_map.rules[rule_index]
                .metric_bands
                .push(ScriptZlmMetricBandDefinition { metric_id, min, max });
            Ok(())
        },
    );

    engine.register_type_with_name::<UsfScaleScriptCtx>("UsfScaleScriptCtx");
    engine.register_get("scale_script_id", |ctx: &mut UsfScaleScriptCtx| ctx.scale_script_id.clone());
    engine.register_get("owner_mod_id", |ctx: &mut UsfScaleScriptCtx| ctx.owner_mod_id.clone());
    engine.register_get("script_file", |ctx: &mut UsfScaleScriptCtx| ctx.script_file.clone());
    engine.register_fn(
        "set_id",
        |ctx: &mut UsfScaleScriptCtx, scale_script_id: &str| -> Result<(), Box<EvalAltResult>> {
            ctx.scale_script_id = normalize_script_identifier("scale_script_id", scale_script_id)?;
            Ok(())
        },
    );
    engine.register_fn("scale_level_count", |_ctx: &mut UsfScaleScriptCtx| -> rhai::INT {
        Scale::SCALE_LEVEL_COUNT as rhai::INT
    });
    engine.register_fn("default_dpt_sampler_kernel_id", |_ctx: &mut UsfScaleScriptCtx| -> String {
        DPT_SAMPLER_KERNEL_DEFAULT_ID.to_string()
    });
    engine.register_fn("default_dpt_categorizer_kernel_id", |_ctx: &mut UsfScaleScriptCtx| -> String {
        DPT_CATEGORIZER_KERNEL_ZLM_LOOKUP_ID.to_string()
    });
    engine.register_fn(
        "set_dpt_schema_from_metric_set",
        |ctx: &mut UsfScaleScriptCtx, scale_index: i64, revision: i64, fallback_zone: &str, metric_set_id: &str| -> Result<(), Box<EvalAltResult>> {
            let owner_mod_id = ensure_owner_mod_for_ctx(ctx.owner_mod_id.as_str())?;
            let scale_index = parse_scale_index(scale_index)?;
            let revision = parse_positive_revision(revision)?;
            let fallback_zone = normalize_zone_type(fallback_zone)?;
            let metric_set_id = normalize_script_identifier("metric_set_id", metric_set_id)?;

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

            USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap().insert(
                scale_index,
                ScriptDptSchemaDefinition {
                    revision,
                    fallback_zone,
                    metrics: compiled_metrics,
                },
            );
            with_owner_mod_manifest_mut(owner_mod_id.as_str(), |manifest| {
                manifest.required_dpt_schema_scales.insert(scale_index);
            })?;
            Ok(())
        },
    );
    engine.register_fn(
        "set_scale",
        |ctx: &mut UsfScaleScriptCtx,
         scale_index: i64,
         dpt_sampler_id: &str,
         dpt_categorizer_id: &str,
         chunk_store_key: &str|
         -> Result<(), Box<EvalAltResult>> {
            let owner_mod_id = ensure_owner_mod_for_ctx(ctx.owner_mod_id.as_str())?;
            let scale_index = parse_scale_index(scale_index)?;
            let dpt_sampler_id = normalize_script_identifier("dpt_sampler_id", dpt_sampler_id)?;
            let dpt_categorizer_id = normalize_script_identifier("dpt_categorizer_id", dpt_categorizer_id)?;
            let chunk_store_key = normalize_script_identifier("chunk_store_key", chunk_store_key)?;
            USF_SCALES_BY_INDEX().lock().unwrap().insert(
                scale_index,
                ScriptScaleDefinition {
                    dpt_sampler_id,
                    dpt_categorizer_id,
                    chunk_store_key,
                },
            );
            with_owner_mod_manifest_mut(owner_mod_id.as_str(), |manifest| {
                manifest.required_scales.insert(scale_index);
            })?;
            Ok(())
        },
    );

    engine.register_type_with_name::<UsfPhenomenonScriptCtx>("UsfPhenomenonScriptCtx");
    engine.register_get("phenomenon_id", |ctx: &mut UsfPhenomenonScriptCtx| ctx.phenomenon_id.clone());
    engine.register_get("owner_mod_id", |ctx: &mut UsfPhenomenonScriptCtx| ctx.owner_mod_id.clone());
    engine.register_get("script_file", |ctx: &mut UsfPhenomenonScriptCtx| ctx.script_file.clone());
    engine.register_fn(
        "set_id",
        |ctx: &mut UsfPhenomenonScriptCtx, phenomenon_id: &str| -> Result<(), Box<EvalAltResult>> {
            ctx.phenomenon_id = normalize_script_identifier("phenomenon_id", phenomenon_id)?;
            Ok(())
        },
    );
    engine.register_fn(
        "register",
        |ctx: &mut UsfPhenomenonScriptCtx, phenomenon_kind: &str| -> Result<(), Box<EvalAltResult>> {
            let owner_mod_id = ensure_owner_mod_for_ctx(ctx.owner_mod_id.as_str())?;
            let phenomenon_id = normalize_script_identifier("phenomenon_id", ctx.phenomenon_id.as_str())?;
            let phenomenon_kind = normalize_phenomenon_kind(phenomenon_kind)?;
            let kind = PhenomenonKind::try_from_config_value(phenomenon_kind.as_str())
                .map_err(|error| format!("unknown phenomenon_kind '{}': {}", phenomenon_kind, error))?;
            let capabilities = kind
                .declared_capabilities()
                .iter()
                .map(|capability| capability.canonical_id().to_string())
                .collect::<Vec<_>>();

            let mut phenomena = USF_PHENOMENA_BY_ID().lock().unwrap();
            if let Some(existing) = phenomena.get(&phenomenon_id) {
                if existing.kind != phenomenon_kind {
                    return Err(format!(
                        "phenomenon '{}' already exists with kind '{}'; got '{}'",
                        phenomenon_id, existing.kind, phenomenon_kind
                    )
                    .into());
                }
                if existing.capabilities != capabilities {
                    return Err(format!(
                        "phenomenon '{}' already exists with capabilities {:?}; got {:?}",
                        phenomenon_id, existing.capabilities, capabilities
                    )
                    .into());
                }
            } else {
                phenomena.insert(
                    phenomenon_id.clone(),
                    ScriptPhenomenonDefinition {
                        id: phenomenon_id.clone(),
                        kind: phenomenon_kind,
                        capabilities,
                    },
                );
            }
            drop(phenomena);

            with_owner_mod_manifest_mut(owner_mod_id.as_str(), |manifest| {
                manifest.required_phenomena.insert(phenomenon_id);
            })?;
            Ok(())
        },
    );
    engine.register_fn(
        "add_capability",
        |ctx: &mut UsfPhenomenonScriptCtx, capability: &str| -> Result<(), Box<EvalAltResult>> {
            let phenomenon_id = normalize_script_identifier("phenomenon_id", ctx.phenomenon_id.as_str())?;
            let capability = normalize_phenomenon_capability(capability)?;
            let mut phenomena = USF_PHENOMENA_BY_ID().lock().unwrap();
            let Some(phenomenon) = phenomena.get_mut(&phenomenon_id) else {
                return Err(format!(
                    "phenomenon '{}' is not registered; call ctx.register(...) before add_capability(...)",
                    phenomenon_id
                )
                .into());
            };
            if !phenomenon.capabilities.iter().any(|value| value == &capability) {
                phenomenon.capabilities.push(capability);
            }
            Ok(())
        },
    );
    engine.register_fn(
        "set_model_for_scale",
        |ctx: &mut UsfPhenomenonScriptCtx, scale_index: i64, model_id: &str| -> Result<(), Box<EvalAltResult>> {
            let phenomenon_id = normalize_script_identifier("phenomenon_id", ctx.phenomenon_id.as_str())?;
            let scale_index = parse_scale_index(scale_index)?;
            let model_id = normalize_script_identifier("model_id", model_id)?;
            set_phenomenon_model_selection(phenomenon_id.as_str(), scale_index, model_id.as_str())
        },
    );
    engine.register_fn(
        "set_model_for_all_scales",
        |ctx: &mut UsfPhenomenonScriptCtx, model_id: &str| -> Result<(), Box<EvalAltResult>> {
            let phenomenon_id = normalize_script_identifier("phenomenon_id", ctx.phenomenon_id.as_str())?;
            let model_id = normalize_script_identifier("model_id", model_id)?;
            for scale_index in 0..(Scale::SCALE_LEVEL_COUNT as u8) {
                set_phenomenon_model_selection(phenomenon_id.as_str(), scale_index, model_id.as_str())?;
            }
            Ok(())
        },
    );
    engine.register_fn(
        "set_model_for_range",
        |ctx: &mut UsfPhenomenonScriptCtx, min_scale_index: i64, max_scale_index: i64, model_id: &str| -> Result<(), Box<EvalAltResult>> {
            let phenomenon_id = normalize_script_identifier("phenomenon_id", ctx.phenomenon_id.as_str())?;
            let min_scale_index = parse_scale_index_with_name("min_scale_index", min_scale_index)?;
            let max_scale_index = parse_scale_index_with_name("max_scale_index", max_scale_index)?;
            if min_scale_index > max_scale_index {
                return Err(format!("invalid scale range [{min_scale_index}..{max_scale_index}]").into());
            }
            let model_id = normalize_script_identifier("model_id", model_id)?;
            for scale_index in min_scale_index..=max_scale_index {
                set_phenomenon_model_selection(phenomenon_id.as_str(), scale_index, model_id.as_str())?;
            }
            Ok(())
        },
    );
    engine.register_fn(
        "set_model_for_set",
        |ctx: &mut UsfPhenomenonScriptCtx, scale_indices: rhai::Array, model_id: &str| -> Result<(), Box<EvalAltResult>> {
            let phenomenon_id = normalize_script_identifier("phenomenon_id", ctx.phenomenon_id.as_str())?;
            let model_id = normalize_script_identifier("model_id", model_id)?;
            for scale_index in parse_scale_index_set("scale_indices", scale_indices)? {
                set_phenomenon_model_selection(phenomenon_id.as_str(), scale_index, model_id.as_str())?;
            }
            Ok(())
        },
    );

    engine.register_type_with_name::<UsfPhenomenonModelScriptCtx>("UsfPhenomenonModelScriptCtx");
    engine.register_get("model_id", |ctx: &mut UsfPhenomenonModelScriptCtx| ctx.model_id.clone());
    engine.register_get("owner_mod_id", |ctx: &mut UsfPhenomenonModelScriptCtx| ctx.owner_mod_id.clone());
    engine.register_get("script_file", |ctx: &mut UsfPhenomenonModelScriptCtx| ctx.script_file.clone());
    engine.register_fn(
        "set_id",
        |ctx: &mut UsfPhenomenonModelScriptCtx, model_id: &str| -> Result<(), Box<EvalAltResult>> {
            ctx.model_id = normalize_script_identifier("model_id", model_id)?;
            Ok(())
        },
    );
    engine.register_fn(
        "register",
        |ctx: &mut UsfPhenomenonModelScriptCtx, phenomenon_id: &str| -> Result<(), Box<EvalAltResult>> {
            let owner_mod_id = ensure_owner_mod_for_ctx(ctx.owner_mod_id.as_str())?;
            let model_id = normalize_script_identifier("model_id", ctx.model_id.as_str())?;
            let phenomenon_id = normalize_script_identifier("phenomenon_id", phenomenon_id)?;
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
            } else {
                models.insert(
                    model_id.clone(),
                    ScriptPhenomenonModelDefinition {
                        id: model_id.clone(),
                        phenomenon_id,
                        topology: "monolithic_chunk".to_string(),
                        support_chunk_radius: 0,
                        projection_metric_name: "demo_mass_density".to_string(),
                        projection_bias: 0.0,
                        projection_gain: 1.0,
                        manifestation_density: None,
                        manifestation_material: None,
                        manifestation_collider_enabled: false,
                    },
                );
            }
            drop(models);

            with_owner_mod_manifest_mut(owner_mod_id.as_str(), |manifest| {
                manifest.required_phenomenon_models.insert(model_id);
            })?;
            Ok(())
        },
    );
    engine.register_fn(
        "set_manifestation_density_field",
        |ctx: &mut UsfPhenomenonModelScriptCtx,
         coarse_span_units: f64,
         detail_span_units: f64,
         coarse_weight: f64,
         detail_weight: f64,
         bias: f64,
         gain: f64,
         center: f64,
         seed_salt_coarse: i64,
         seed_salt_detail: i64|
         -> Result<(), Box<EvalAltResult>> {
            let model_id = normalize_script_identifier("model_id", ctx.model_id.as_str())?;
            let mut models = USF_PHENOMENON_MODELS_BY_ID().lock().unwrap();
            let Some(model) = models.get_mut(&model_id) else {
                return Err(format!("phenomenon model '{}' is not registered; call ctx.register(...) first", model_id).into());
            };

            let phenomena = USF_PHENOMENA_BY_ID().lock().unwrap();
            let Some(phenomenon) = phenomena.get(model.phenomenon_id.as_str()) else {
                return Err(format!("phenomenon model '{}' references unknown phenomenon '{}'", model_id, model.phenomenon_id).into());
            };
            let required_capability = PhenomenonCapability::ManifestationDensityField.canonical_id();
            if !phenomenon
                .capabilities
                .iter()
                .any(|capability| capability.eq_ignore_ascii_case(required_capability))
            {
                return Err(format!(
                    "phenomenon model '{}' belongs to phenomenon '{}' (kind='{}') without capability '{}'; \
                     set_manifestation_density_field requires that capability.",
                    model_id, model.phenomenon_id, phenomenon.kind, required_capability
                )
                .into());
            }
            drop(phenomena);

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

            model.manifestation_density = Some(ScriptManifestationDensityDefinition {
                coarse_span_units,
                detail_span_units,
                coarse_weight: coarse_weight as f32,
                detail_weight: detail_weight as f32,
                bias: bias as f32,
                gain: gain as f32,
                center: center as f32,
                seed_salt_coarse: seed_salt_coarse as u64,
                seed_salt_detail: seed_salt_detail as u64,
            });
            Ok(())
        },
    );
    engine.register_fn(
        "set_topology",
        |ctx: &mut UsfPhenomenonModelScriptCtx, topology: &str| -> Result<(), Box<EvalAltResult>> {
            let model_id = normalize_script_identifier("model_id", ctx.model_id.as_str())?;
            let topology = normalize_phenomena_model_topology(topology)?;
            let mut models = USF_PHENOMENON_MODELS_BY_ID().lock().unwrap();
            let Some(model) = models.get_mut(&model_id) else {
                return Err(format!("phenomenon model '{}' is not registered; call ctx.register(...) first", model_id).into());
            };
            model.topology = topology.clone();
            if topology == "monolithic_chunk" {
                model.support_chunk_radius = 0;
            }
            Ok(())
        },
    );
    engine.register_fn(
        "set_projection_contract",
        |ctx: &mut UsfPhenomenonModelScriptCtx, projection_metric_name: &str, projection_bias: f64, projection_gain: f64| -> Result<(), Box<EvalAltResult>> {
            let model_id = normalize_script_identifier("model_id", ctx.model_id.as_str())?;
            let projection_metric_name = normalize_script_identifier("projection_metric_name", projection_metric_name)?;
            if !projection_bias.is_finite() {
                return Err("projection_bias must be finite".into());
            }
            if !projection_gain.is_finite() || projection_gain <= 0.0 {
                return Err("projection_gain must be finite and > 0".into());
            }

            let mut models = USF_PHENOMENON_MODELS_BY_ID().lock().unwrap();
            let Some(model) = models.get_mut(&model_id) else {
                return Err(format!("phenomenon model '{}' is not registered; call ctx.register(...) first", model_id).into());
            };
            model.projection_metric_name = projection_metric_name;
            model.projection_bias = projection_bias as f32;
            model.projection_gain = projection_gain as f32;
            Ok(())
        },
    );
    engine.register_fn(
        "set_manifestation_material_profile",
        |ctx: &mut UsfPhenomenonModelScriptCtx,
         albedo_r: f64,
         albedo_g: f64,
         albedo_b: f64,
         alpha: f64,
         perceptual_roughness: f64,
         metallic: f64,
         emissive_strength: f64|
         -> Result<(), Box<EvalAltResult>> {
            let model_id = normalize_script_identifier("model_id", ctx.model_id.as_str())?;
            let mut models = USF_PHENOMENON_MODELS_BY_ID().lock().unwrap();
            let Some(model) = models.get_mut(&model_id) else {
                return Err(format!("phenomenon model '{}' is not registered; call ctx.register(...) first", model_id).into());
            };

            let phenomena = USF_PHENOMENA_BY_ID().lock().unwrap();
            let Some(phenomenon) = phenomena.get(model.phenomenon_id.as_str()) else {
                return Err(format!("phenomenon model '{}' references unknown phenomenon '{}'", model_id, model.phenomenon_id).into());
            };
            let required_capability = PhenomenonCapability::ManifestationMaterialProfile.canonical_id();
            if !phenomenon
                .capabilities
                .iter()
                .any(|capability| capability.eq_ignore_ascii_case(required_capability))
            {
                return Err(format!(
                    "phenomenon model '{}' belongs to phenomenon '{}' (kind='{}') without capability '{}'; \
                     set_manifestation_material_profile requires that capability.",
                    model_id, model.phenomenon_id, phenomenon.kind, required_capability
                )
                .into());
            }
            drop(phenomena);

            if !albedo_r.is_finite() || !albedo_g.is_finite() || !albedo_b.is_finite() {
                return Err("albedo channels must be finite".into());
            }
            if !alpha.is_finite() {
                return Err("alpha must be finite".into());
            }
            if !perceptual_roughness.is_finite() {
                return Err("perceptual_roughness must be finite".into());
            }
            if !metallic.is_finite() {
                return Err("metallic must be finite".into());
            }
            if !emissive_strength.is_finite() {
                return Err("emissive_strength must be finite".into());
            }

            model.manifestation_material = Some(ScriptManifestationMaterialDefinition {
                albedo_r: albedo_r as f32,
                albedo_g: albedo_g as f32,
                albedo_b: albedo_b as f32,
                alpha: alpha as f32,
                perceptual_roughness: perceptual_roughness as f32,
                metallic: metallic as f32,
                emissive_strength: emissive_strength as f32,
            });
            Ok(())
        },
    );
    engine.register_fn(
        "set_manifestation_collider_enabled",
        |ctx: &mut UsfPhenomenonModelScriptCtx, enabled: bool| -> Result<(), Box<EvalAltResult>> {
            let model_id = normalize_script_identifier("model_id", ctx.model_id.as_str())?;
            let mut models = USF_PHENOMENON_MODELS_BY_ID().lock().unwrap();
            let Some(model) = models.get_mut(&model_id) else {
                return Err(format!("phenomenon model '{}' is not registered; call ctx.register(...) first", model_id).into());
            };

            if enabled {
                let phenomena = USF_PHENOMENA_BY_ID().lock().unwrap();
                let Some(phenomenon) = phenomena.get(model.phenomenon_id.as_str()) else {
                    return Err(format!("phenomenon model '{}' references unknown phenomenon '{}'", model_id, model.phenomenon_id).into());
                };
                let required_capability = PhenomenonCapability::ManifestationCollider.canonical_id();
                if !phenomenon
                    .capabilities
                    .iter()
                    .any(|capability| capability.eq_ignore_ascii_case(required_capability))
                {
                    return Err(format!(
                        "phenomenon model '{}' belongs to phenomenon '{}' (kind='{}') without capability '{}'; \
                         set_manifestation_collider_enabled(true) requires that capability.",
                        model_id, model.phenomenon_id, phenomenon.kind, required_capability
                    )
                    .into());
                }
            }

            model.manifestation_collider_enabled = enabled;
            Ok(())
        },
    );
    engine.register_fn(
        "set_support_chunk_radius",
        |ctx: &mut UsfPhenomenonModelScriptCtx, chunk_radius: i64| -> Result<(), Box<EvalAltResult>> {
            let model_id = normalize_script_identifier("model_id", ctx.model_id.as_str())?;
            let chunk_radius = parse_support_chunk_radius(chunk_radius)?;
            let mut models = USF_PHENOMENON_MODELS_BY_ID().lock().unwrap();
            let Some(model) = models.get_mut(&model_id) else {
                return Err(format!("phenomenon model '{}' is not registered; call ctx.register(...) first", model_id).into());
            };
            model.support_chunk_radius = if model.topology == "monolithic_chunk" { 0 } else { chunk_radius.max(1) };
            Ok(())
        },
    );
    engine.register_fn(
        "set_as_default_all_scales",
        |ctx: &mut UsfPhenomenonModelScriptCtx| -> Result<(), Box<EvalAltResult>> {
            let model_id = normalize_script_identifier("model_id", ctx.model_id.as_str())?;
            let Some(model) = USF_PHENOMENON_MODELS_BY_ID().lock().unwrap().get(&model_id).cloned() else {
                return Err(format!("phenomenon model '{}' is not registered; call ctx.register(...) first", model_id).into());
            };
            for scale_index in 0..(Scale::SCALE_LEVEL_COUNT as u8) {
                set_phenomenon_model_selection(model.phenomenon_id.as_str(), scale_index, model_id.as_str())?;
            }
            Ok(())
        },
    );
}

fn register_runtime_bindings(engine: &mut rhai::Engine) {
    register_hook_param_types(engine);
    register_usf_script_ctx_runtime_module(engine);
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
