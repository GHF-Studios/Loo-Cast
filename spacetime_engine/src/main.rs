// Crates
extern crate spacetime_engine;

// Modules

// Local imports


// Internal imports
use spacetime_engine::kernel::*;
use spacetime_engine::kernel::manager::*;
use spacetime_engine::system::*;

// External imports
use std::any::TypeId;
use ::bevy::prelude::*;


// Static variables


// Constant variables


// Types


// Enums


// Structs


// Implementations


// Module Functions
fn main() {
    println!("Initializing spacetime engine main manager.");

    let main_manager = manager::MAIN_MANAGER.clone();

    let mut main_manager = match main_manager.lock() {
        Ok(main_manager) => {
            println!("Successfully locked main manager mutex.");
            main_manager
        },
        Err(err) => {
            panic!("Failed to lock main manager mutex! Error: {:?}", err);
        },
    };

    match main_manager.initialize() {
        Ok(_) => {
            println!("Successfully initialized main manager.");
        },
        Err(err) => {
            panic!("Failed to initialize main manager! Error: {:?}", err);
        },
    };

    drop(main_manager);

    println!("Starting bevy engine.");

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
        .add_plugins(SpacetimeEngineSystemPlugins)
        .add_plugins(RapierPlugins)
        // States
        .add_state::<AppState>()
        // Run
        .run();
}