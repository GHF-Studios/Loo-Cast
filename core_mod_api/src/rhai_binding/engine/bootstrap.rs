use std::{path::PathBuf, sync::Arc};

use crate::bevy::prelude::{App, First, Last, PostStartup, PostUpdate, PreStartup, PreUpdate, Startup, Update};
use crate::config::statics::CONFIG;
use crate::core::functions::asset_root;
use crate::rhai_binding::bind::engine_ext::EngineExt;
use crate::rhai_binding::engine::hook::new_hook_runner_system;
use crate::rhai_binding::engine::preprocess::preprocess_script_source;
use crate::rhai_binding::engine::resources::MainScriptEngineHandle;
use crate::rhai_binding::engine::statics::SCHEDULE_HOOKS;
use crate::rhai_binding::runtime::ecs::message::bindings::types::ScriptProbeMessage;
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

    for name in SCHEDULE_HOOKS().lock().unwrap().drain() {
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
            "update" => {
                let file = format!("{name}.rhai");
                let file_path = path.join(file);
                app.add_systems(Update, new_hook_runner_system(file_path.display().to_string()));
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
    register_schedule_hooks_runtime_module(engine);
    register_testing_runtime_module(engine);
}

fn register_schedule_hooks_runtime_module(engine: &mut rhai::Engine) {
    let mut schedule_hooks_module = rhai::Module::new();
    schedule_hooks_module.set_native_fn("add", |hook: &str| -> Result<(), Box<rhai::EvalAltResult>> {
        SCHEDULE_HOOKS().lock().unwrap().insert(hook.into());
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
