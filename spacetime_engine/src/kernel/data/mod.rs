// Modules

// Local imports

// Internal imports
use super::manager::*;

// External imports
use lazy_static::*;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bevy::log::*;

// Static variables
lazy_static! {
    pub static ref DATA_MANAGER: Arc<Mutex<DataManager>> = Arc::new(Mutex::new(DataManager::new()));
}

// Constant variables

// Types

// Traits
pub trait Data: Any + Send + Sync {
    fn get_runtime_id(&self) -> Option<u64>;
    fn load_data<
        TData: super::data::Data + for<'de> Deserialize<'de>,
        TResource: super::resource::Resource,
    >(
        &self,
        resource: &TResource,
    ) -> Result<TData, String>;
    fn save_data<
        'a,
        TData: 'a + super::data::Data + Serialize,
        TResource: 'a + super::resource::Resource,
    >(
        &'a self,
        resource: &'a mut TResource,
    ) -> Result<(), String>;
}

// Enums

// Structs
pub struct DataManager {
    state: ManagerState,
    data_hashmap: HashMap<TypeId, HashMap<u64, Box<dyn Any + Send + Sync>>>,
    next_data_id: u64,
}

// Implementations
impl Manager for DataManager {
    fn initialize(&mut self) -> Result<(), ManagerInitializeError> {
        match self.state {
            ManagerState::Created => {}
            ManagerState::Initialized => {
                return Err(ManagerInitializeError::ManagerAlreadyInitialized);
            }
            ManagerState::Finalized => {
                return Err(ManagerInitializeError::ManagerAlreadyFinalized);
            }
        }

        self.state = ManagerState::Initialized;

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ManagerFinalizeError> {
        match self.state {
            ManagerState::Created => {
                return Err(ManagerFinalizeError::ManagerNotInitialized);
            }
            ManagerState::Initialized => {}
            ManagerState::Finalized => {
                return Err(ManagerFinalizeError::ManagerAlreadyFinalized);
            }
        }

        self.state = ManagerState::Finalized;

        Ok(())
    }

    fn get_state(&self) -> &ManagerState {
        &self.state
    }
}

impl DataManager {
    fn new() -> Self {
        DataManager {
            state: ManagerState::Created,
            data_hashmap: HashMap::new(),
            next_data_id: 0,
        }
    }

    pub fn register_data_type<T: Data>(&mut self) -> Result<(), String> {
        if self.data_hashmap.contains_key(&TypeId::of::<T>()) {
            return Err(format!(
                "Data type already registered: {}",
                std::any::type_name::<T>()
            ));
        }

        self.data_hashmap.insert(TypeId::of::<T>(), HashMap::new());

        Ok(())
    }

    pub fn unregister_data_type<T: Data>(&mut self) -> Result<(), String> {
        if !self.data_hashmap.contains_key(&TypeId::of::<T>()) {
            return Err(format!(
                "Data type not registered: {}",
                std::any::type_name::<T>()
            ));
        }

        self.data_hashmap.remove(&TypeId::of::<T>());

        Ok(())
    }

    pub fn is_data_type_registered<T: Data>(&self) -> Result<bool, String> {
        Ok(self.data_hashmap.contains_key(&TypeId::of::<T>()))
    }

    pub fn register_data<T: Data>(&mut self, data: T) -> Result<(), String> {
        if let Some(id) = data.get_runtime_id() {
            return Err(format!("Data already registered: {}", id));
        }

        let data_hashmap = match self.data_hashmap.get_mut(&TypeId::of::<T>()) {
            Some(data_hashmap) => data_hashmap,
            None => {
                return Err(format!(
                    "Data type not registered: {}",
                    std::any::type_name::<T>()
                ))
            }
        };

        let id = self.next_data_id;
        self.next_data_id += 1;

        if data_hashmap.contains_key(&id) {
            return Err(format!(
                "Supposedly unused data ID '{}' already in use!",
                id
            ));
        }

        data_hashmap.insert(id, Box::new(data));

        Ok(())
    }

    pub fn unregister_data<T: Data>(&mut self, id: u64) -> Result<T, String> {
        let data_hashmap = match self.data_hashmap.get_mut(&TypeId::of::<T>()) {
            Some(data_hashmap) => data_hashmap,
            None => {
                return Err(format!(
                    "Data type not registered: {}",
                    std::any::type_name::<T>()
                ))
            }
        };

        if !data_hashmap.contains_key(&id) {
            return Err(format!("Data not registered: {}", id));
        }

        let data = data_hashmap.remove(&id).unwrap();

        Ok(*data.downcast::<T>().unwrap())
    }

    pub fn is_data_registered<T: Data>(&self, id: u64) -> Result<bool, String> {
        let data_hashmap = match self.data_hashmap.get(&TypeId::of::<T>()) {
            Some(data_hashmap) => data_hashmap,
            None => {
                return Err(format!(
                    "Data type not registered: {}",
                    std::any::type_name::<T>()
                ))
            }
        };

        Ok(data_hashmap.contains_key(&id))
    }

    pub fn get_data<T: Data>(&self, id: u64) -> Result<Option<&T>, String> {
        let data_hashmap = match self.data_hashmap.get(&TypeId::of::<T>()) {
            Some(data_hashmap) => data_hashmap,
            None => {
                return Err(format!(
                    "Data type not registered: {}",
                    std::any::type_name::<T>()
                ))
            }
        };

        if !data_hashmap.contains_key(&id) {
            return Ok(None);
        }

        let data = data_hashmap.get(&id).unwrap();

        Ok(data.downcast_ref::<T>())
    }

    pub fn get_data_mut<T: Data>(&mut self, id: u64) -> Result<Option<&mut T>, String> {
        let data_hashmap = match self.data_hashmap.get_mut(&TypeId::of::<T>()) {
            Some(data_hashmap) => data_hashmap,
            None => {
                return Err(format!(
                    "Data type not registered: {}",
                    std::any::type_name::<T>()
                ))
            }
        };

        if !data_hashmap.contains_key(&id) {
            return Ok(None);
        }

        let data = data_hashmap.get_mut(&id).unwrap();

        Ok(data.downcast_mut::<T>())
    }
}

// Module Functions
pub fn test() {
    let mut manager = DataManager::new();

    match manager.register_data_type::<TestData>() {
        Ok(_) => debug!(
            "Registered data type: {}",
            std::any::type_name::<TestData>()
        ),
        Err(error) => panic!("Failed to register data type: {}", error),
    };

    let data = TestData { runtime_id: None };

    match manager.register_data(data) {
        Ok(_) => debug!("Registered data: {}", std::any::type_name::<TestData>()),
        Err(error) => panic!("Failed to register data: {}", error),
    };
}

// TEMPORARY
#[derive(Serialize, Deserialize)]
pub struct TestData {
    #[serde(skip)]
    runtime_id: Option<u64>,
}

impl Data for TestData {
    fn get_runtime_id(&self) -> Option<u64> {
        self.runtime_id
    }

    fn load_data<
        TData: super::data::Data + for<'de> Deserialize<'de>,
        TResource: super::resource::Resource,
    >(
        &self,
        resource: &TResource,
    ) -> Result<TData, String> {
        let serialized_data = match resource.get_file_content() {
            Ok(data) => data,
            Err(error) => return Err(format!("Failed to read resource file: {}", error)),
        };
        let serialized_data: String = match String::from_utf8(serialized_data.to_vec()) {
            Ok(data) => data,
            Err(error) => {
                return Err(format!(
                    "Failed to convert resource file to string: {}",
                    error
                ))
            }
        };

        match serde_json::from_str(&serialized_data) {
            Ok(data) => Ok(data),
            Err(error) => Err(format!("Failed to deserialize data: {}", error)),
        }
    }

    fn save_data<
        'a,
        TData: 'a + super::data::Data + Serialize,
        TResource: 'a + super::resource::Resource,
    >(
        &'a self,
        resource: &'a mut TResource,
    ) -> Result<(), String> {
        let serialized_data = match serde_json::to_string_pretty(&self) {
            Ok(data) => data,
            Err(error) => return Err(format!("Failed to serialize data: {}", error)),
        };
        let serialized_data: &[u8] = (&serialized_data).as_bytes();

        match resource.set_file_content(serialized_data) {
            Ok(_) => Ok(()),
            Err(error) => Err(format!("Failed to write resource file: {}", error)),
        }
    }
}
