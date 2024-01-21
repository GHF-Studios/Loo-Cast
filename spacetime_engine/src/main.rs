// Crates
extern crate spacetime_engine;

// Modules

// Local imports


// Internal imports
use spacetime_engine::system::*;


// External imports
use bevy::prelude::*;

// Static variables


// Constant variables


// Types


// Enums


// Structs


// Implementations


// Module Functions
/*
fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                asset_folder: "mods".to_string(),
                ..default()
            })
        )
        // Plugins
        .add_plugins(SpacetimeEnginePlugins)
        .add_plugins(RapierPlugins)
        // States
        .add_state::<AppState>()
        // Run
        .run();
}
*/
fn main() {
    // initialize the basics(aka the engine kernel) such as data management, resource mangement, and event management, maybe also stuff like log management, debug management, etc.
    // initialize the emergent systems(aka the engine system) such as mod management, USF(massive oversimplification, but baaaaaasically USF = ECS) management, player management, savegame management, camera management, UI Management, etc.
    // initialize the bevy engine, acting as the user interface for the engine, essentially being the first visual indication that the game has started
}