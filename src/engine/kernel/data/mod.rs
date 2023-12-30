// Modules

// Local imports

// Internal imports

// External imports
use serde::{Serialize, Deserialize};
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;

// Static variables

// Constant variables

// Types

// Traits
pub trait Data {
    fn get_metadata(&self) -> Metadata;
    fn load_data<'a, TData: 'static + Any + super::data::Data + Deserialize<'a>, TResource: 'static + Any + super::resource::Resource>(resource: &TResource) -> Result<TData, String>;
    fn save_data<TData: 'static + Any + super::data::Data + Serialize, TResource: 'static + Any + super::resource::Resource>(self, resource: &mut TResource) -> Result<(), String>;
}

// Enums

// Structs
#[derive(Default)]
pub struct Metadata {
    id: Option<u64>,
}

pub struct DataManager {
    data_hashmap: HashMap<TypeId, HashMap<u64, Box<dyn Any>>>,
    next_data_id: u64,
}

// Implementations
impl Metadata {
    fn get_data_id(&self) -> Option<u64> {
        self.id
    }
}

impl DataManager {
    pub fn new() -> Self {
        DataManager { data_hashmap: HashMap::new(), next_data_id: 0 }
    }

    pub fn register_data_type<T: 'static + Any + Data>(&mut self) -> Result<(), String> {
        if self.data_hashmap.contains_key(&TypeId::of::<T>()) {
            return Err(format!("Data type already registered: {}", std::any::type_name::<T>()));
        }

        self.data_hashmap.insert(TypeId::of::<T>(), HashMap::new());

        Ok(())
    }

    pub fn unregister_data_type<T: 'static + Any + Data>(&mut self) -> Result<(), String> {
        if !self.data_hashmap.contains_key(&TypeId::of::<T>()) {
            return Err(format!("Data type not registered: {}", std::any::type_name::<T>()));
        }

        self.data_hashmap.remove(&TypeId::of::<T>());

        Ok(())
    }

    pub fn is_data_type_registered<T: 'static + Any + Data>(&self) -> Result<bool, String> {
        Ok(self.data_hashmap.contains_key(&TypeId::of::<T>()))
    }

    pub fn register_data<T: 'static + Any + Data>(&mut self, data: T) -> Result<(), String> {
        if let Some(id) = data.get_metadata().get_data_id() {
            return Err(format!("Data already registered: {}", id));
        }
        
        let data_hashmap = match self.data_hashmap.get_mut(&TypeId::of::<T>()) {
            Some(data_hashmap) => data_hashmap,
            None => return Err(format!("Data type not registered: {}", std::any::type_name::<T>())),
        };

        let id = self.next_data_id;
        self.next_data_id += 1;

        if data_hashmap.contains_key(&id) {
            return Err(format!("Supposedly unused data ID '{}' already in use!", id));
        }

        data_hashmap.insert(id, Box::new(data));

        Ok(())
    }

    pub fn unregister_data<T: 'static + Any + Data>(&mut self, id: u64) -> Result<T, String> {
        let data_hashmap = match self.data_hashmap.get_mut(&TypeId::of::<T>()) {
            Some(data_hashmap) => data_hashmap,
            None => return Err(format!("Data type not registered: {}", std::any::type_name::<T>())),
        };

        if !data_hashmap.contains_key(&id) {
            return Err(format!("Data not registered: {}", id));
        }

        let data = data_hashmap.remove(&id).unwrap();

        Ok(*data.downcast::<T>().unwrap())
    }

    pub fn is_data_registered<T: 'static + Any + Data>(&self, id: u64) -> Result<bool, String> {
        let data_hashmap = match self.data_hashmap.get(&TypeId::of::<T>()) {
            Some(data_hashmap) => data_hashmap,
            None => return Err(format!("Data type not registered: {}", std::any::type_name::<T>())),
        };

        Ok(data_hashmap.contains_key(&id))
    }

    pub fn get_data<T: 'static + Any + Data>(&self, id: u64) -> Result<Option<&T>, String> {
        let data_hashmap = match self.data_hashmap.get(&TypeId::of::<T>()) {
            Some(data_hashmap) => data_hashmap,
            None => return Err(format!("Data type not registered: {}", std::any::type_name::<T>())),
        };

        if !data_hashmap.contains_key(&id) {
            return Ok(None);
        }

        let data = data_hashmap.get(&id).unwrap();

        Ok(data.downcast_ref::<T>())
    }

    pub fn get_data_mut<T: 'static + Any + Data>(&mut self, id: u64) -> Result<Option<&mut T>, String> {
        let data_hashmap = match self.data_hashmap.get_mut(&TypeId::of::<T>()) {
            Some(data_hashmap) => data_hashmap,
            None => return Err(format!("Data type not registered: {}", std::any::type_name::<T>())),
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
        Ok(_) => println!("Registered data type: {}", std::any::type_name::<TestData>()),
        Err(error) => println!("Failed to register data type: {}", error),
    };

    match manager.register_data(TestData { metadata: Metadata { id: None } }) {
        Ok(_) => println!("Registered data: {}", std::any::type_name::<TestData>()),
        Err(error) => println!("Failed to register data: {}", error),
    };
}

// TEMPORARY
#[derive(Serialize, Deserialize)]
pub struct TestData {
    #[serde(skip)]
    metadata: Metadata
}

impl Data for TestData {
    fn get_metadata(&self) -> Metadata {
        self.metadata
    }

    fn load_data<'a, TData: 'static + Any + super::data::Data + Deserialize<'a>, TResource: 'static + Any + super::resource::Resource>(resource: &TResource) -> Result<TData, String> {
        let mut serialized_data = match resource.get_file_content() {
            Ok(data) => data,
            Err(error) => return Err(format!("Failed to read resource file: {}", error)),
        };
        let serialized_data: String = match String::from_utf8(serialized_data.to_vec()) {
            Ok(data) => data,
            Err(error) => return Err(format!("Failed to convert resource file to string: {}", error)),
        };

        match serde_json::from_str(&serialized_data) {
            Ok(data) => Ok(data),
            Err(error) => Err(format!("Failed to deserialize data: {}", error)),
        }
    }

    fn save_data<TData: 'static + Any + super::data::Data + Serialize, TResource: 'static + Any + super::resource::Resource>(self, resource: &mut TResource) -> Result<(), String> {
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