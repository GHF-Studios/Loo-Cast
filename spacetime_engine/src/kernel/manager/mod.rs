// Modules

// Local imports

// Internal imports
use super::*;

// External imports
use lazy_static::*;
use std::{any::TypeId, collections::HashMap};
use std::sync::{Arc, Mutex};

// Static variables
lazy_static! {
    pub static ref MAIN_MANAGER: Arc<Mutex<MainManager>> = Arc::new(Mutex::new(MainManager::new()));
}

// Constant variables

// Types

// Traits
pub trait Manager {
    fn initialize(&mut self) -> Result<(), ManagerInitializeError>;
    fn finalize(&mut self) -> Result<(), ManagerFinalizeError>;
    fn get_state(&self) -> &ManagerState;
    fn register_dependency(&mut self, dependency_id: TypeId, dependency: Box<Arc<Mutex<dyn Manager + Sync + Send>>>) -> Result<(), ManagerRegisterDependencyError>;
    fn get_dependencies(&self) -> Result<&HashMap<TypeId, Box<Arc<Mutex<dyn Manager + Sync + Send>>>>, ManagerGetDependenciesError>;
    fn get_dependencies_mut(&mut self) -> Result<&mut HashMap<TypeId, Box<Arc<Mutex<dyn Manager + Sync + Send>>>>, ManagerGetDependenciesMutError>;
}

// Enums
pub enum ManagerState {
    Created,
    Initialized,
    Finalized,
}

#[derive(Debug)]
pub enum ManagerInitializeError {
    DependencyNotInitialized,
    DependencyAlreadyFinalized,
    ManagerAlreadyInitialized,
    ManagerAlreadyFinalized,
}

#[derive(Debug)]
pub enum ManagerFinalizeError {
    DependencyNotFinalized,
    ManagerNotInitialized,
    ManagerAlreadyFinalized,
}

#[derive(Debug)]
pub enum ManagerRegisterDependencyError {
    DependencyAlreadyRegistered,
    ManagerAlreadyInitialized,
    ManagerAlreadyFinalized,
    InvokerIsRootManager,
}

#[derive(Debug)]
pub enum ManagerGetDependenciesError {
    ManagerNotInitialized,
    ManagerAlreadyFinalized,
    InvokerIsRootManager,
}

#[derive(Debug)]
pub enum ManagerGetDependenciesMutError {
    ManagerNotInitialized,
    ManagerAlreadyFinalized,
    InvokerIsRootManager,
}

// Structs
pub struct MainManager {
    dependencies: HashMap<TypeId, Box<Arc<Mutex<dyn Manager + Sync + Send>>>>,
    state: ManagerState,
    spacetime_engine_main_thread_handle: Option<std::thread::JoinHandle<()>>,
}

// Implementations
impl Manager for MainManager {
    fn initialize(&mut self) -> Result<(), ManagerInitializeError> {
        match self.state {
            ManagerState::Created => {},
            ManagerState::Initialized => {
                return Err(ManagerInitializeError::ManagerAlreadyInitialized);
            },
            ManagerState::Finalized => {
                return Err(ManagerInitializeError::ManagerAlreadyFinalized);
            },
        }

        for (_, dependency) in self.dependencies.iter_mut() {
            let dependency = dependency.lock().unwrap();

            match dependency.get_state() {
                ManagerState::Created => {
                    return Err(ManagerInitializeError::DependencyNotInitialized);
                },
                ManagerState::Initialized => {
                },
                ManagerState::Finalized => {
                    return Err(ManagerInitializeError::DependencyAlreadyFinalized);
                },
            }
        }

        self.spacetime_engine_main_thread_handle = Some(std::thread::spawn(|| {
            let main_manager = MAIN_MANAGER.clone();

            let mut main_manager = match main_manager.lock() {
                Ok(main_manager) => {
                    println!("Successfully locked main manager mutex.");
                    main_manager
                },
                Err(err) => {
                    panic!("Failed to lock main manager mutex! Error: {:?}", err);
                },
            };

            println!("Locking all kernel manager mutexes.");

            let bevy_manager = bevy::BEVY_MANAGER.clone();
            let mut bevy_manager = match bevy_manager.lock() {
                Ok(bevy_manager) => {
                    println!("Successfully locked bevy manager mutex.");
                    bevy_manager
                },
                Err(err) => {
                    panic!("Failed to lock bevy manager mutex! Error: {:?}", err);
                },
            };
            let config_manager = config::CONFIG_MANAGER.clone();
            let mut config_manager = match config_manager.lock() {
                Ok(config_manager) => {
                    println!("Successfully locked config manager mutex.");
                    config_manager
                },
                Err(err) => {
                    panic!("Failed to lock config manager mutex! Error: {:?}", err);
                },
            };
            let data_manager = data::DATA_MANAGER.clone();
            let mut data_manager = match data_manager.lock() {
                Ok(data_manager) => {
                    println!("Successfully locked data manager mutex.");
                    data_manager
                },
                Err(err) => {
                    panic!("Failed to lock data manager mutex! Error: {:?}", err);
                },
            };
            let debug_manager = debug::DEBUG_MANAGER.clone();
            let mut debug_manager = match debug_manager.lock() {
                Ok(debug_manager) => {
                    println!("Successfully locked debug manager mutex.");
                    debug_manager
                },
                Err(err) => {
                    panic!("Failed to lock debug manager mutex! Error: {:?}", err);
                },
            };
            let event_manager = event::EVENT_MANAGER.clone();
            let mut event_manager = match event_manager.lock() {
                Ok(event_manager) => {
                    println!("Successfully locked event manager mutex.");
                    event_manager
                },
                Err(err) => {
                    panic!("Failed to lock event manager mutex! Error: {:?}", err);
                },
            };
            let plugin_manager = plugin::PLUGIN_MANAGER.clone();
            let mut plugin_manager = match plugin_manager.lock() {
                Ok(plugin_manager) => {
                    println!("Successfully locked plugin manager mutex.");
                    plugin_manager
                },
                Err(err) => {
                    panic!("Failed to lock plugin manager mutex! Error: {:?}", err);
                },
            };
            let resource_manager = resource::RESOURCE_MANAGER.clone();
            let mut resource_manager = match resource_manager.lock() {
                Ok(resource_manager) => {
                    println!("Successfully locked resource manager mutex.");
                    resource_manager
                },
                Err(err) => {
                    panic!("Failed to lock resource manager mutex! Error: {:?}", err);
                },
            };

            let main_manager_id = TypeId::of::<manager::MainManager>();

            println!("Registering all kernel manager dependencies.");

            println!("Registering bevy manager dependency.");
            match bevy_manager.register_dependency(main_manager_id, Box::new(manager::MAIN_MANAGER.clone())) {
                Ok(_) => {
                    println!("Successfully registered bevy manager dependency.");
                },
                Err(err) => {
                    panic!("Failed to register bevy manager dependency! Error: {:?}", err);
                },
            }

            println!("Registering config manager dependency.");
            match config_manager.register_dependency(main_manager_id, Box::new(manager::MAIN_MANAGER.clone())) {
                Ok(_) => {
                    println!("Successfully registered config manager dependency.");
                },
                Err(err) => {
                    panic!("Failed to register config manager dependency! Error: {:?}", err);
                },
            }

            println!("Registering data manager dependency.");
            match data_manager.register_dependency(main_manager_id, Box::new(manager::MAIN_MANAGER.clone())) {
                Ok(_) => {
                    println!("Successfully registered data manager dependency.");
                },
                Err(err) => {
                    panic!("Failed to register data manager dependency! Error: {:?}", err);
                },
            }

            println!("Registering debug manager dependency.");
            match debug_manager.register_dependency(main_manager_id, Box::new(manager::MAIN_MANAGER.clone())) {
                Ok(_) => {
                    println!("Successfully registered debug manager dependency.");
                },
                Err(err) => {
                    panic!("Failed to register debug manager dependency! Error: {:?}", err);
                },
            }

            println!("Registering event manager dependency.");
            match event_manager.register_dependency(main_manager_id, Box::new(manager::MAIN_MANAGER.clone())) {
                Ok(_) => {
                    println!("Successfully registered event manager dependency.");
                },
                Err(err) => {
                    panic!("Failed to register event manager dependency! Error: {:?}", err);
                },
            }

            println!("Registering plugin manager dependency.");
            match plugin_manager.register_dependency(main_manager_id, Box::new(manager::MAIN_MANAGER.clone())) {
                Ok(_) => {
                    println!("Successfully registered plugin manager dependency.");
                },
                Err(err) => {
                    panic!("Failed to register plugin manager dependency! Error: {:?}", err);
                },
            }

            println!("Registering resource manager dependency.");
            match resource_manager.register_dependency(main_manager_id, Box::new(manager::MAIN_MANAGER.clone())) {
                Ok(_) => {
                    println!("Successfully registered resource manager dependency.");
                },
                Err(err) => {
                    panic!("Failed to register resource manager dependency! Error: {:?}", err);
                },
            }

            println!("Successfully registered all kernel manager dependencies.");

            println!("Initializing all kernel managers.");

            println!("Initializing bevy manager.");
            match bevy_manager.initialize() {
                Ok(_) => {
                    println!("Successfully initialized bevy manager.");
                },
                Err(err) => {
                    panic!("Failed to initialize bevy manager! Error: {:?}", err);
                },
            }

            println!("Initializing config manager.");
            match config_manager.initialize() {
                Ok(_) => {
                    println!("Successfully initialized config manager.");
                },
                Err(err) => {
                    panic!("Failed to initialize config manager! Error: {:?}", err);
                },
            }

            println!("Initializing data manager.");
            match data_manager.initialize() {
                Ok(_) => {
                    println!("Successfully initialized data manager.");
                },
                Err(err) => {
                    panic!("Failed to initialize data manager! Error: {:?}", err);
                },
            }

            println!("Initializing debug manager.");
            match debug_manager.initialize() {
                Ok(_) => {
                    println!("Successfully initialized debug manager.");
                },
                Err(err) => {
                    panic!("Failed to initialize debug manager! Error: {:?}", err);
                },
            }

            println!("Initializing event manager.");
            match event_manager.initialize() {
                Ok(_) => {
                    println!("Successfully initialized event manager.");
                },
                Err(err) => {
                    panic!("Failed to initialize event manager! Error: {:?}", err);
                },
            }

            println!("Initializing plugin manager.");
            match plugin_manager.initialize() {
                Ok(_) => {
                    println!("Successfully initialized plugin manager.");
                },
                Err(err) => {
                    panic!("Failed to initialize plugin manager! Error: {:?}", err);
                },
            }

            println!("Initializing resource manager.");
            match resource_manager.initialize() {
                Ok(_) => {
                    println!("Successfully initialized resource manager.");
                },
                Err(err) => {
                    panic!("Failed to initialize resource manager! Error: {:?}", err);
                },
            }

            println!("Successfully initialized all kernel managers.");




            // initialize the emergent systems(aka the engine system) such as mod management, USF(massive oversimplification, but baaaaaasically USF = ECS) management, player management, savegame management, camera management, UI Management, etc.






            // initialize the bevy engine, acting as the user interface for the engine, essentially being the first visual indication that the game has started

        }));

        self.state = ManagerState::Initialized;

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ManagerFinalizeError> {
        match self.state {
            ManagerState::Created => {
                return Err(ManagerFinalizeError::ManagerNotInitialized);
            },
            ManagerState::Initialized => {},
            ManagerState::Finalized => {
                return Err(ManagerFinalizeError::ManagerAlreadyFinalized);
            },
        }

        for (_, dependency) in self.dependencies.iter_mut() {
            let dependency = dependency.lock().unwrap();

            match dependency.get_state() {
                ManagerState::Created => {
                    return Err(ManagerFinalizeError::DependencyNotFinalized);
                },
                ManagerState::Initialized => {
                    return Err(ManagerFinalizeError::DependencyNotFinalized);
                },
                ManagerState::Finalized => {},
            }
        }

        self.dependencies.clear();

        self.state = ManagerState::Finalized;

        Ok(())
    }

    fn get_state(&self) -> &ManagerState {
        &self.state
    }

    fn register_dependency(&mut self, _: TypeId, _: Box<Arc<Mutex<dyn Manager + Sync + Send>>>) -> Result<(), ManagerRegisterDependencyError> {
        Err(ManagerRegisterDependencyError::InvokerIsRootManager)
    }

    fn get_dependencies(&self) -> Result<&HashMap<TypeId, Box<Arc<Mutex<dyn Manager + Sync + Send>>>>, ManagerGetDependenciesError> {
        Err(ManagerGetDependenciesError::InvokerIsRootManager)
    }

    fn get_dependencies_mut(&mut self) -> Result<&mut HashMap<TypeId, Box<Arc<Mutex<dyn Manager + Sync + Send>>>>, ManagerGetDependenciesMutError> {
        Err(ManagerGetDependenciesMutError::InvokerIsRootManager)
    }
}

impl MainManager {
    fn new() -> MainManager {
        MainManager {
            dependencies: HashMap::new(),
            state: ManagerState::Created,
            spacetime_engine_main_thread_handle: None,
        }
    }
}

// Module Functions
