extern crate spacetime_engine;

use bevy::prelude::*;
use bevy::log::LogPlugin;
use bevy_rapier2d::prelude::*;
use spacetime_engine::*;
use tokio::task::JoinHandle;

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
    std::env::set_var("RUST_BACKTRACE", "1");

    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,spacetime_engine=debug".into(),
            level: bevy::log::Level::INFO,
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(SpacetimeEnginePlugins)
        .run();
}



/*
Implement Entity Classes, which are kind of just wrappers around bevy archetypes.
An Entity Class is a collection of components that are used to create an entity.
Components can both be mandatory and optional.
Mandatory components are added to an entity class at compile time, into the literal type, hence compile-time.
Optional components are added to an entity class *instance* at *runtime*, into the literal *value*, hence runtime-time.
Entity Classes can be used to create entities with a specific set of starting components.
Entity Classes can be used to interpret entities as a specific set of components, and interact with them as such.
Entity Classes are like post-procesing passes inside a gpu pipeline, but for Entities, on the CPU, specifically in a multi-threaded fashion.
The limitation with that is that you can only perform "Class-level" operations, 
    meaning that the most fundamental pieces of data for that "Class-level" of data would be either a pre-defined primitive struct, or a user-defined class;
    and that the most fundamental pieces of logic for that "Class-level" of data would be either a pre-defined primitive *function*, or a user-defined *method* on a user-defined class.
*/





