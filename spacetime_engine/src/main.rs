extern crate spacetime_engine;

use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy_rapier2d::prelude::*;
use spacetime_engine::*;
use spacetime_engine::core::singletons::*;

// Primary tasks
// TODO: Implement chunk loaders

// Secondary tasks
// TODO: Implement default for all registries and registry wrappers instead of the new function 

// Fun tasks
// TODO: Implement inventory + hotbar, so that you can select different types of chunk actors to place. 

// Less fun tasks
// TODO: Implement sub-chunking/fields
// TODO: Implement gravity via sub-chunking/fields
// TODO: Implement electromagnetism via sub-chunking/fields
// TODO: Implement planets via gravity
// TODO: Implement magnets via electromagnetism
// TODO: Implement stars via gravity and electromagnetism

fn main() {
    //env::set_var("RUST_BACKTRACE", "1");
    
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,spacetime_engine=debug".into(),
            level: bevy::log::Level::INFO,
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(SpacetimeEnginePlugins)
        .add_systems(PreStartup, pre_startup)
        .add_systems(Startup, startup)
        .add_systems(PostStartup, post_startup)
        .run();
}

fn pre_startup(world: &mut World) {
    let mut rapier_configuration = world.get_resource_mut::<RapierConfiguration>().unwrap();
    rapier_configuration.gravity = Vec2::new(0.0, 0.0);
    drop(rapier_configuration);

    let mut locking_hierarchy = LOCKING_HIERARCHY.lock().unwrap();
    locking_hierarchy.pre_startup::<MainTypeRegistry>(AbsoluteLockingPath::new()).unwrap();
}

fn startup() {
    let mut locking_hierarchy = LOCKING_HIERARCHY.lock().unwrap();
    locking_hierarchy.startup::<MainTypeRegistry>(AbsoluteLockingPath::new()).unwrap();
}

fn post_startup() {
    let mut locking_hierarchy = LOCKING_HIERARCHY.lock().unwrap();
    locking_hierarchy.post_startup::<MainTypeRegistry>(AbsoluteLockingPath::new()).unwrap();
}