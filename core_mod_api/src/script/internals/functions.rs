use std::path::PathBuf;

use bevy::prelude::{Mut, World as BevyWorld, App, PreStartup, Startup, PostStartup, First, PreUpdate, Update, PostUpdate, Last};
use rhai::Engine;

use crate::core::functions::asset_root;

use super::resources::MainScriptEngineHandle;
use super::super::bindings::ecs::world::types::World;
use super::super::internals::statics::SCHEDULE_HOOK_HANDLERS;

pub fn pre_init(world: &mut BevyWorld) {
    world.init_resource::<MainScriptEngineHandle>();
}

pub(in super::super) fn init(app: &mut App) {
    let path = "core_mod/scripts/core/schedule_hook_handlers/";
    let mut abs_path = PathBuf::from(path);
    if abs_path.is_relative() {
        abs_path = asset_root().join(path);
    }
    let mut path = abs_path;

    for name in SCHEDULE_HOOK_HANDLERS().lock().unwrap().drain() {
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

pub(in super::super) fn register_internal_bindings(engine: &mut rhai::Engine) {
    engine.register_fn("add_hook_handler", |hook: &str| {
        SCHEDULE_HOOK_HANDLERS().lock().unwrap().insert(hook.into());
    });

    engine.register_type::<World>()
        .register_fn("spawn_named_entity", World::spawn_named_entity);
}

pub(in super::super) fn new_hook_runner_system(path: String) -> impl FnMut(&mut BevyWorld) {
    move |world: &mut BevyWorld| {
        world.resource_scope(|world, mut engine: Mut<MainScriptEngineHandle>| {
            let engine = &mut engine.0;
            bevy::prelude::warn!("new_hook_runner_system path: {path}");
            let hook_code = std::fs::read_to_string(&path).unwrap();
            let ast = engine.compile(&hook_code).unwrap();
            let mut scope = rhai::Scope::new();
            let world_api = World::start_access(std::mem::take(world));
            engine.call_fn::<()>(&mut scope, &ast, "main", (world_api.clone(), )).unwrap();
            *world = world_api.end_access();
        });
    }
}

pub(super) fn new_main_script_engine() -> Engine {
    let mut engine = Engine::new();

    register_internal_bindings(&mut engine);

    let boot_script_path = "core_mod/scripts/core/boot.rhai";

    let mut abs_boot_script_path = PathBuf::from(boot_script_path);
    if abs_boot_script_path.is_relative() {
        abs_boot_script_path = asset_root().join(boot_script_path);
    }
    let boot_script_path = abs_boot_script_path.to_string_lossy().to_string();

    bevy::prelude::warn!("boot_script_path: {}", boot_script_path);

    let boot_script = std::fs::read_to_string(boot_script_path).unwrap();
    let boot_script = engine.compile(boot_script).unwrap();
    engine.eval_ast::<()>(&boot_script).unwrap();

    engine
}
