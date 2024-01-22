// Modules

// Local imports

// Internal imports
use super::manager::*;

// External imports
use std::any::TypeId;
use std::sync::{Arc, Mutex};
use lazy_static::*;
use std::collections::HashMap;

// Static variables
lazy_static! {
    pub static ref DEBUG_MANAGER: Arc<Mutex<DebugManager>> = Arc::new(Mutex::new(DebugManager::new()));
}

// Constant variables

// Types

// Traits

// Enums

// Structs
pub struct DebugManager {
    state: ManagerState,
    dependencies: HashMap<TypeId, Box<Arc<Mutex<dyn Manager + Sync + Send>>>>,
}

// Implementations
impl Manager for DebugManager {
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

    fn register_dependency(&mut self, dependency_id: TypeId, dependency: Box<Arc<Mutex<dyn Manager + Sync + Send>>>) -> Result<(), ManagerRegisterDependencyError> {
        match self.state {
            ManagerState::Created => {
                if self.dependencies.contains_key(&dependency_id) {
                    return Err(ManagerRegisterDependencyError::DependencyAlreadyRegistered);
                }

                self.dependencies.insert(dependency_id, dependency);

                Ok(())
            },
            ManagerState::Initialized => {
                Err(ManagerRegisterDependencyError::ManagerAlreadyInitialized)

            },
            ManagerState::Finalized => {
                Err(ManagerRegisterDependencyError::ManagerAlreadyFinalized)
            },
        }
    }

    fn get_dependencies(&self) -> Result<&HashMap<TypeId, Box<Arc<Mutex<dyn Manager + Sync + Send>>>>, ManagerGetDependenciesError> {
        Ok(&self.dependencies)
    }

    fn get_dependencies_mut(&mut self) -> Result<&mut HashMap<TypeId, Box<Arc<Mutex<dyn Manager + Sync + Send>>>>, ManagerGetDependenciesMutError> {
        Ok(&mut self.dependencies)
    }
}

impl DebugManager {
    fn new() -> DebugManager {
        DebugManager {
            state: ManagerState::Created,
            dependencies: HashMap::new(),
        }
    }
}

// Module Functions
