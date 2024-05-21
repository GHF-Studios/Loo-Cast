extern crate spacetime_engine;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use spacetime_engine::SpacetimeEnginePlugins;




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
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(SpacetimeEnginePlugins)
        .add_systems(Startup, startup)
        .run();
}

fn startup(mut rapier_configuration: ResMut<RapierConfiguration>) {
    rapier_configuration.gravity = Vec2::new(0.0, 0.0);
}