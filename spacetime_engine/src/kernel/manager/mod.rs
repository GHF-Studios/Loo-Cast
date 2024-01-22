// Modules

// Local imports

// Internal imports

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
        }
    }
}

// Module Functions
