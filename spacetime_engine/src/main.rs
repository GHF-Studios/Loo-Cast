// Crates
extern crate spacetime_engine;

// Modules

// Local imports

// Internal imports
use spacetime_engine::kernel::manager::*;
use spacetime_engine::kernel::*;
use spacetime_engine::system::*;

// External imports
use bevy::log::*;
use bevy::prelude::*;
use bevy::app::AppExit;

// Static variables

// Constant variables

// Types

// Enums

// Structs

// Implementations

// Module Functions
fn main() {
    App::new()
        // Startup Systems
        .add_systems(PreStartup, spacetime_engine_startup)
        // Update Systems
        .add_systems(Update, spacetime_engine_shutdown)
        // Bevy Plugins
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    asset_folder: "mods".to_string(),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    ..default()
                }),
        )
        // Plugins
        .add_plugins(SpacetimeEngineSystemPlugins)
        .add_plugins(RapierPlugins)
        // States
        .add_state::<AppState>()
        // Run
        .run();

}

fn spacetime_engine_startup() {
    info!("Initializing spacetime engine...");

    let main_manager = MAIN_MANAGER.clone();

    let mut main_manager = match main_manager.lock() {
        Ok(main_manager) => {
            trace!("Successfully locked main manager mutex.");
            main_manager
        }
        Err(err) => {
            panic!("Failed to lock main manager mutex! Error: {:?}", err);
        }
    };

    match main_manager.initialize() {
        Ok(_) => {
            info!("Successfully initialized spacetime engine.");
            drop(main_manager);
        }
        Err(err) => {
            panic!("Failed to initialize spacetime engine! Error: {:?}", err);
        }
    };
}

fn spacetime_engine_shutdown(mut exit_events: EventReader<AppExit>) {
    for _ in exit_events.iter() {
        info!("Finalizing spacetime engine...");
        
        let main_manager = MAIN_MANAGER.clone();

        let mut main_manager = match main_manager.lock() {
            Ok(main_manager) => {
                trace!("Successfully locked main manager mutex.");
                main_manager
            }
            Err(err) => {
                panic!("Failed to lock main manager mutex! Error: {:?}", err);
            }
        };

        match main_manager.finalize() {
            Ok(_) => {
                info!("Successfully finalized spacetime engine.");
                drop(main_manager);
            }
            Err(err) => {
                panic!("Failed to finalize spacetime engine! Error: {:?}", err);
            }
        };
    }
}
