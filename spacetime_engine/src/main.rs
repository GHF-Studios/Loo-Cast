extern crate spacetime_engine;

use bevy::{log::LogPlugin, prelude::*};
use bevy_rapier2d::prelude::*;
use spacetime_engine::SpacetimeEnginePlugins;


// NEW TODOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOOO

// Repeating tasks:
// TODO: Take a look at all TODOs outside this file

// Bug fix tasks:
// TODO: Sometimes when moving too erratically, the player fails to load some chunks, inevitably causing the player to despawn and constituting a bug.

// "Optional" tasks
// TODO: Bring the quality of the root 'chunk' module up to the standard of it's sub-modules 'loader' and 'actor'  
// TODO: Add good logging everywhere for all events and handlers and shit

// Mandatory tasks
// TODO: Implement downgrading of chunks, chunk actors, chunk loaders, players, etc.
// TODO: Implement entity creation/destruction/upgrading/downgrading for all entity "types" like Player, Chunk, etc.
// TODO: Ensure that destroyed entities always have all component properly unloaded and unregistered;
//       like have some way to notify the associated systems to downgrade the entity until we are left with a barebones entity that we can safely despawn
//       instead of just rawdogging it and immediately deleting the entity from existence without notifying any registries and whatnot (which is bad, duh)
//       This is very apparent when the player despawns (which currently happens when you move about too erratically),
//       as the chunk loader component of the player never properly disposes of the remaining loaded chunks and makes them be loaded forever
//       AKA: Make the player destruction (and general entity destruction) more graceful.
//       Maybe a sort of "destructible" component which works like a tiny registry so you can register
//       different component types of an entity that need to be taken care of before destruction.
// TODO: Make internal event not fully public but restricted to super

// Fun tasks
// TODO: Implement inventory + hotbar, so that you can select different types of chunk actors to place. 



// DUSTY ASS OLD ASS TODOOOOOOOOO

// TODO: Change player creation so it uses a chunk actor factory thingy which provides you with a blank chunk actor entity but is inherently asynchronous and powered by events and a proxy chunk actor component which is replaced by a real chunk actor once the proxy chunk actor's current chunk is registered and loaded.
// TODO: Implement/Refactor & integrate custom position types for flat world position, deep world position(flat world position, but including the z-axis as a depth index) and chunk position, essentially generalizing chunk positionionate and chunk actor position and other current position/position types
// TODO: Implement checks in the chunk and chunk actor and entity registries to prevent invalid chunk/entity states as represented by the data in the registries
// TODO: Automate registration of specific types of components or entire entities for registrys. 
//       Like, you register the thing to get a registered unused id, and then it is *automatically* loaded when a system detects the added component. And when you remove the component, it is *automatically* unloaded.
// TODO: Implement world border
// TODO: Fix chunk loading by implementing serializable proxies for all necessary rapier components (necessary as of now)
//       Kinda worked, but Velocity somehow does not survive serialization.
//       Also when we start the game and immediately start moving erratically, at some point the game crashes.
// TODO: Implement sub-chunking/fields
// TODO: Implement gravity via sub-chunking/fields
// TODO: Implement electromagnetism via sub-chunking/fields
// TODO: Implement planets via gravity
// TODO: Implement magnets via electromagnetism
// TODO: Implement stars via gravity and electromagnetism

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            filter: "info,spacetime_engine=debug".into(),
            level: bevy::log::Level::INFO,
            ..Default::default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(SpacetimeEnginePlugins)
        .add_systems(PreStartup, pre_start)
        .run();
}

fn pre_start(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.gravity = Vec2::new(0.0, 0.0);
}