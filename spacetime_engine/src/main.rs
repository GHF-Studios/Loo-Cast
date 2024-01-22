// Crates
extern crate spacetime_engine;

// Modules

// Local imports


// Internal imports
use spacetime_engine::kernel::*;
use spacetime_engine::kernel::manager::*;

// External imports
use std::any::TypeId;


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



/*

*/
fn main() {
    // initialize the basics(aka the engine kernel) such as data management, resource mangement, and event management, maybe also stuff like log management, debug management, etc.

    let main_manager_mutex = manager::MAIN_MANAGER.clone();
    let mut main_manager = main_manager_mutex.lock().unwrap();
    let config_manager_mutex = config::CONFIG_MANAGER.clone();
    let mut config_manager = config_manager_mutex.lock().unwrap();
    let data_manager_mutex = data::DATA_MANAGER.clone();
    let mut data_manager = data_manager_mutex.lock().unwrap();
    let debug_manager_mutex = debug::DEBUG_MANAGER.clone();
    let mut debug_manager = debug_manager_mutex.lock().unwrap();
    let event_manager_mutex = event::EVENT_MANAGER.clone();
    let mut event_manager = event_manager_mutex.lock().unwrap();
    let plugin_manager_mutex = plugin::PLUGIN_MANAGER.clone();
    let mut plugin_manager = plugin_manager_mutex.lock().unwrap();
    let resource_manager_mutex = resource::RESOURCE_MANAGER.clone();
    let mut resource_manager = resource_manager_mutex.lock().unwrap();

    let main_manager_id = TypeId::of::<manager::MainManager>();

    match config_manager.register_dependency(main_manager_id, Box::new(manager::MAIN_MANAGER.clone())) {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to register config manager dependency! Error: {:?}", err);
        },
    }
    match data_manager.register_dependency(main_manager_id, Box::new(manager::MAIN_MANAGER.clone())) {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to register data manager dependency! Error: {:?}", err);
        },
    }
    match debug_manager.register_dependency(main_manager_id, Box::new(manager::MAIN_MANAGER.clone())) {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to register debug manager dependency! Error: {:?}", err);
        },
    }
    match event_manager.register_dependency(main_manager_id, Box::new(manager::MAIN_MANAGER.clone())) {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to register event manager dependency! Error: {:?}", err);
        },
    }
    match plugin_manager.register_dependency(main_manager_id, Box::new(manager::MAIN_MANAGER.clone())) {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to register plugin manager dependency! Error: {:?}", err);
        },
    }
    match resource_manager.register_dependency(main_manager_id, Box::new(manager::MAIN_MANAGER.clone())) {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to register resource manager dependency! Error: {:?}", err);
        },
    }
    println!("Registered core manager dependencies!");

    match main_manager.initialize() {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to initialize main manager! Error: {:?}", err);
        },
    }
    drop(main_manager);
    match config_manager.initialize() {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to initialize config manager! Error: {:?}", err);
        },
    }
    drop(config_manager);
    match data_manager.initialize() {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to initialize data manager! Error: {:?}", err);
        },
    }
    drop(data_manager);
    match debug_manager.initialize() {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to initialize debug manager! Error: {:?}", err);
        },
    }
    drop(debug_manager);
    match event_manager.initialize() {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to initialize event manager! Error: {:?}", err);
        },
    }
    drop(event_manager);
    match plugin_manager.initialize() {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to initialize plugin manager! Error: {:?}", err);
        },
    }
    drop(plugin_manager);
    match resource_manager.initialize() {
        Ok(_) => {},
        Err(err) => {
            panic!("Failed to initialize resource manager! Error: {:?}", err);
        },
    }
    drop(resource_manager);
    println!("Initialized core managers!");



    // initialize the emergent systems(aka the engine system) such as mod management, USF(massive oversimplification, but baaaaaasically USF = ECS) management, player management, savegame management, camera management, UI Management, etc.
    


    


    // initialize the bevy engine, acting as the user interface for the engine, essentially being the first visual indication that the game has started
}