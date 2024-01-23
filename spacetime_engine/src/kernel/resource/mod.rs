// Modules

// Local imports

// Internal imports
use super::manager::*;

// External imports
use lazy_static::*;
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use bevy::log::*;

// Static variables
lazy_static! {
    pub static ref RESOURCE_MANAGER: Arc<Mutex<ResourceManager>> =
        Arc::new(Mutex::new(ResourceManager::new()));
}

// Constant variables

// Types

// Traits
pub trait Resource: Any + Send + Sync {
    fn new(file_path: &Path) -> Self;
    fn get_file_path(&self) -> &Path;
    fn get_file_content(&self) -> Result<Box<[u8]>, String>;
    fn set_file_content(&mut self, file_content: &[u8]) -> Result<(), String>;
}

// Enums

// Structs
pub struct ResourceManager {
    manager_state: ManagerState,
    resource_hashmap: HashMap<TypeId, HashMap<PathBuf, Box<dyn Any + Send + Sync>>>,
}

// Implementations
impl Manager for ResourceManager {
    fn initialize(&mut self) -> Result<(), ManagerInitializeError> {
        match self.manager_state {
            ManagerState::Created => {}
            ManagerState::Initialized => {
                return Err(ManagerInitializeError::ManagerAlreadyInitialized);
            }
            ManagerState::Finalized => {
                return Err(ManagerInitializeError::ManagerAlreadyFinalized);
            }
        }

        self.manager_state = ManagerState::Initialized;

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ManagerFinalizeError> {
        match self.manager_state {
            ManagerState::Created => {
                return Err(ManagerFinalizeError::ManagerNotInitialized);
            }
            ManagerState::Initialized => {}
            ManagerState::Finalized => {
                return Err(ManagerFinalizeError::ManagerAlreadyFinalized);
            }
        }

        self.manager_state = ManagerState::Finalized;

        Ok(())
    }

    fn get_manager_state(&self) -> &ManagerState {
        &self.manager_state
    }
}

impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager {
            manager_state: ManagerState::Created,
            resource_hashmap: HashMap::new(),
        }
    }

    pub fn register_resource_type<T: Resource>(&mut self) -> Result<(), String> {
        if self.resource_hashmap.contains_key(&TypeId::of::<T>()) {
            return Err(format!(
                "Resource type already registered: {}",
                std::any::type_name::<T>()
            ));
        }

        self.resource_hashmap
            .insert(TypeId::of::<T>(), HashMap::new());

        Ok(())
    }

    pub fn unregister_resource_type<T: Resource>(&mut self) -> Result<(), String> {
        if !self.resource_hashmap.contains_key(&TypeId::of::<T>()) {
            return Err(format!(
                "Resource type not registered: {}",
                std::any::type_name::<T>()
            ));
        }

        self.resource_hashmap.remove(&TypeId::of::<T>());

        Ok(())
    }

    pub fn is_resource_type_registered<T: Resource>(&self) -> Result<bool, String> {
        Ok(self.resource_hashmap.contains_key(&TypeId::of::<T>()))
    }

    pub fn register_resource<T: Resource>(&mut self, resource: T) -> Result<(), String> {
        let id = resource.get_file_path();

        let resource_hashmap = match self.resource_hashmap.get_mut(&TypeId::of::<T>()) {
            Some(resource_hashmap) => resource_hashmap,
            None => {
                return Err(format!(
                    "Resource type not registered: {}",
                    std::any::type_name::<T>()
                ))
            }
        };

        if resource_hashmap.contains_key(id) {
            return Err(format!("Resource already registered: {}", id.display()));
        }

        resource_hashmap.insert(id.to_path_buf(), Box::new(resource));

        Ok(())
    }

    pub fn unregister_resource<T: Resource>(&mut self, resource: T) -> Result<T, String> {
        let resource_hashmap = match self.resource_hashmap.get_mut(&TypeId::of::<T>()) {
            Some(resource_hashmap) => resource_hashmap,
            None => {
                return Err(format!(
                    "Resource type not registered: {}",
                    std::any::type_name::<T>()
                ))
            }
        };

        if !resource_hashmap.contains_key(resource.get_file_path()) {
            return Err(format!(
                "Resource not registered: {}",
                resource.get_file_path().display()
            ));
        }

        let resource = resource_hashmap.remove(resource.get_file_path()).unwrap();

        Ok(*resource.downcast::<T>().unwrap())
    }

    pub fn is_resource_registered<T: Resource>(&self, resource: T) -> Result<bool, String> {
        let resource_hashmap = match self.resource_hashmap.get(&TypeId::of::<T>()) {
            Some(resource_hashmap) => resource_hashmap,
            None => {
                return Err(format!(
                    "Resource type not registered: {}",
                    std::any::type_name::<T>()
                ))
            }
        };

        Ok(resource_hashmap.contains_key(resource.get_file_path()))
    }

    pub fn get_resource<T: Resource>(&self, resource: T) -> Result<Option<&T>, String> {
        let resource_hashmap = match self.resource_hashmap.get(&TypeId::of::<T>()) {
            Some(resource_hashmap) => resource_hashmap,
            None => {
                return Err(format!(
                    "Resource type not registered: {}",
                    std::any::type_name::<T>()
                ))
            }
        };

        if !resource_hashmap.contains_key(resource.get_file_path()) {
            return Ok(None);
        }

        let resource = resource_hashmap.get(resource.get_file_path()).unwrap();

        Ok(resource.downcast_ref::<T>())
    }

    pub fn get_resource_mut<T: Resource>(&mut self, resource: T) -> Result<Option<&mut T>, String> {
        let resource_hashmap = match self.resource_hashmap.get_mut(&TypeId::of::<T>()) {
            Some(resource_hashmap) => resource_hashmap,
            None => {
                return Err(format!(
                    "Resource type not registered: {}",
                    std::any::type_name::<T>()
                ))
            }
        };

        if !resource_hashmap.contains_key(resource.get_file_path()) {
            return Ok(None);
        }

        let resource = resource_hashmap.get_mut(resource.get_file_path()).unwrap();

        Ok(resource.downcast_mut::<T>())
    }
}

// Module Functions
pub fn test() {
    let mut resource_manager = ResourceManager::new();

    match resource_manager.register_resource_type::<TestResource>() {
        Ok(_) => debug!(
            "Registered resource type: {}",
            std::any::type_name::<TestResource>()
        ),
        Err(error) => panic!("Failed to register resource type: {}", error),
    };

    let file_path = Path::new("test.txt").to_path_buf();

    let file_handle = match File::open(file_path.clone()) {
        Ok(file_handle) => file_handle,
        Err(error) => panic!("Failed to open file: {}", error),
    };

    let resource = TestResource {
        file_handle,
        file_path: file_path.clone(),
    };

    match resource_manager.register_resource(resource) {
        Ok(_) => debug!("Registered resource: {}", file_path.display()),
        Err(error) => panic!("Failed to register resource: {}", error),
    };
}

// TEMPORARY
pub struct TestResource {
    file_handle: File,
    file_path: PathBuf,
}

impl Resource for TestResource {
    fn new(file_path: &Path) -> Self {
        let file_handle = match File::open(file_path) {
            Ok(file_handle) => file_handle,
            Err(error) => panic!("Failed to open file: {}", error),
        };

        TestResource {
            file_handle,
            file_path: file_path.to_path_buf(),
        }
    }

    fn get_file_path(&self) -> &Path {
        &self.file_path
    }

    fn get_file_content(&self) -> Result<Box<[u8]>, String> {
        let mut file_handle = match File::open(self.get_file_path()) {
            Ok(file_handle) => file_handle,
            Err(error) => return Err(format!("Failed to open file: {}", error)),
        };

        let mut file_content = Vec::new();

        match file_handle.read(&mut file_content) {
            Ok(_) => (),
            Err(error) => return Err(format!("Failed to read file: {}", error)),
        };

        Ok(file_content.into_boxed_slice())
    }

    fn set_file_content(&mut self, file_content: &[u8]) -> Result<(), String> {
        let mut file_handle = match File::create(self.get_file_path()) {
            Ok(file_handle) => file_handle,
            Err(error) => return Err(format!("Failed to create file: {}", error)),
        };

        match file_handle.write_all(file_content) {
            Ok(_) => (),
            Err(error) => return Err(format!("Failed to write file: {}", error)),
        };

        Ok(())
    }
}
