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