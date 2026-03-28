use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::bevy::ecs::schedule::IntoScheduleConfigs;
use crate::bevy::prelude::{App, First, Last, PostStartup, PostUpdate, PreStartup, PreUpdate, Startup, Update};
use crate::config::statics::CONFIG;
use crate::core::functions::asset_root;
use crate::rhai_binding::bind::engine_ext::EngineExt;
use crate::rhai_binding::engine::hook::{new_hook_runner_system, register_hook_param_types};
use crate::rhai_binding::engine::preprocess::preprocess_script_source;
use crate::rhai_binding::engine::resources::MainScriptEngineHandle;
use crate::rhai_binding::engine::statics::{
    SCHEDULE_HOOKS, USF_DPT_CATEGORIZER_IDS, USF_DPT_SAMPLER_IDS, USF_DPT_SCHEMAS_BY_SCALE, USF_METRIC_SETS_BY_ID, USF_METRICS_BY_NAME, USF_PHENOMENA_BY_ID,
    USF_PHENOMENON_MODELS_BY_ID, USF_PRIMARY_PHENOMENON_MODEL_BY_PHENOMENON_ID, USF_SCALE_BINDINGS_BY_SCALE, USF_ZLM_SCALES_BY_SCALE,
    USF_ZONE_DENSITY_PROFILE_BY_TYPE, USF_ZONE_PHENOMENON_SUPPORT_BY_ZONE_TYPE, USF_ZONE_SELECTION_POLICY_BY_ZONE_TYPE, USF_ZONE_TYPES,
};
use crate::rhai_binding::runtime::ecs::message::bindings::types::ScriptProbeMessage;
use crate::usf::schedule::{UsfPhenomenonSet, UsfSubstrateSet, UsfZoneSet};
use rhai::Engine;

#[derive(Clone, Copy)]
struct UsfScriptTypeSpec {
    relative_dir: &'static str,
    suffix: &'static str,
    entrypoint: &'static str,
}

const USF_SCRIPT_TYPE_SPECS: [UsfScriptTypeSpec; 9] = [
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
        relative_dir: "dpt_samplers",
        suffix: ".dpt_sampler.rhai",
        entrypoint: "register_dpt_sampler",
    },
    UsfScriptTypeSpec {
        relative_dir: "dpt_categorizers",
        suffix: ".dpt_categorizer.rhai",
        entrypoint: "register_dpt_categorizer",
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
    USF_ZONE_TYPES().lock().unwrap().clear();
    USF_ZONE_DENSITY_PROFILE_BY_TYPE().lock().unwrap().clear();
    USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap().clear();
    USF_ZLM_SCALES_BY_SCALE().lock().unwrap().clear();
    USF_DPT_SAMPLER_IDS().lock().unwrap().clear();
    USF_DPT_CATEGORIZER_IDS().lock().unwrap().clear();
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

fn run_usf_script_type_bootstrap(engine: &Engine, usf_root: &Path, spec: UsfScriptTypeSpec) {
    let script_dir = usf_root.join(spec.relative_dir);
    if !script_dir.is_dir() {
        return;
    }

    let mut files = Vec::new();
    collect_usf_registration_scripts(&script_dir, spec.suffix, &mut files);
    files.sort_by(|a, b| a.to_string_lossy().cmp(&b.to_string_lossy()));

    for file in files {
        let file_path = file.display().to_string();
        let source = std::fs::read_to_string(&file).unwrap_or_else(|error| panic!("Failed to read USF script '{}': {error}", file.display()));
        let source = preprocess_script_source(&source, &file_path);
        let ast = engine
            .compile(source)
            .unwrap_or_else(|error| panic!("Failed to compile USF script '{}': {error}", file.display()));
        let mut scope = rhai::Scope::new();
        if let Err(error) = engine.call_fn::<()>(&mut scope, &ast, spec.entrypoint, ()) {
            panic!("USF script '{}' failed calling entrypoint '{}': {}", file.display(), spec.entrypoint, error);
        }
    }
}

fn run_usf_content_bootstrap(engine: &Engine) {
    let usf_root = asset_root().join("core_mod/scripts/usf");
    if !usf_root.is_dir() {
        return;
    }

    clear_usf_bootstrap_statics();
    for spec in USF_SCRIPT_TYPE_SPECS {
        run_usf_script_type_bootstrap(engine, &usf_root, spec);
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
