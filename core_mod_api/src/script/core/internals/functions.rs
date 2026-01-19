use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use bevy::prelude::{Mut, World as BevyWorld, App, PreStartup, Startup, PostStartup, First, PreUpdate, Update, PostUpdate, Last};
use rhai::{Engine, FnPtr, NativeCallContext, Shared};

use crate::core::functions::asset_root;
use crate::script::core::internals::types::{ScopedAccess, ScopedAccessHandle};
use crate::script::ecs::world::internals::traits::WorldApi;

use super::resources::MainScriptEngineHandle;
use super::super::super::ecs::world::bindings::types::World;
use super::super::super::core::internals::statics::SCHEDULE_HOOK_HANDLERS;

pub fn pre_init(world: &mut BevyWorld) {
    world.init_resource::<MainScriptEngineHandle>();
}

pub fn init(app: &mut App) {
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

    engine.register_type_with_name::<Shared<World>>("World");

    engine.register_fn("flush", Shared::<World>::flush);
    // engine.register_fn("commands", |world: &Shared<World>, ctx: NativeCallContext, cb: FnPtr| {
    //     world.commands(ctx, cb)
    // });
}

pub(in super::super) fn new_hook_runner_system(path: String) -> impl FnMut(&mut BevyWorld) {
    move |world: &mut BevyWorld| {
        world.resource_scope(|source_world, mut engine: Mut<MainScriptEngineHandle>| {
            // Setup
            // bevy::prelude::warn!("new_hook_runner_system path: {path}");
            let engine = &mut engine.0;
            let hook_code = std::fs::read_to_string(&path).unwrap();
            let ast = engine.compile(&hook_code).unwrap();
            let mut scope = rhai::Scope::new();

            // Start world access
            let world = std::mem::take(source_world);
            let world_raw_handle: ScopedAccessHandle<BevyWorld> = Arc::new(RwLock::new(ScopedAccess::new(world)));
            let world_binding = World { world: world_raw_handle.clone() };
            let shared_world = Shared::new(world_binding);

            // Execute hook runner system script
            engine.call_fn::<()>(&mut scope, &ast, "main", (shared_world,)).unwrap();

            // End world access
            let mut world_raw_scoped = Arc::into_inner(world_raw_handle)
                .expect("World handle leaked or cloned")
                .into_inner()
                .expect("RwLock poisoned");

            let returned_world = world_raw_scoped
                .invalidate()
                .expect("World handle was already invalidated");

            *source_world = returned_world;
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
