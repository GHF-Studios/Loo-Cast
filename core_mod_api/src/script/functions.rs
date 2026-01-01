use bevy::prelude::{Mut, World as BevyWorld};
use rhai::Engine;

use super::resources::MainScriptEngineHandle;
use super::statics::SCHEDULE_HOOK_HANDLERS;
use super::types::World;

pub(super) fn new_hook_runner_system(path: String) -> impl FnMut(&mut BevyWorld) {
    move |world: &mut BevyWorld| {
        world.resource_scope(|world, mut engine: Mut<MainScriptEngineHandle>| {
            let engine = &mut engine.0;
            let hook_code = std::fs::read_to_string(&path).unwrap();
            let ast = engine.compile(&hook_code).unwrap();
            let mut scope = rhai::Scope::new();
            let world_api = World::start_access(std::mem::take(world));
            engine.call_fn::<()>(&mut scope, &ast, "main", (world_api.clone(), )).unwrap();
            *world = world_api.end_access();
        });
    }
}

pub fn enable_main_script_engine(world: &mut BevyWorld) {
    world.init_resource::<MainScriptEngineHandle>();
}

pub(super) fn boot_main_script_engine() -> Engine {
    let mut engine = Engine::new();

    internal_init(&mut engine);

    let boot_script = std::fs::read_to_string("assets/scripts/core/boot.rhai").unwrap();
    let boot_script = engine.compile(boot_script).unwrap();
    engine.eval_ast::<()>(&boot_script).unwrap();

    engine
}

fn internal_init(engine: &mut rhai::Engine) {
    engine.register_fn("add_hook_handler", |hook: &str| {
        SCHEDULE_HOOK_HANDLERS().lock().unwrap().insert(hook.into());
    });

    engine.register_type::<World>()
        .register_fn("spawn_named_entity", World::spawn_named_entity);
}
