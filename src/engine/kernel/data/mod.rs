// Modules

// Local imports

// Internal imports

// External imports
use std::any::Any;
use std::collections::HashMap;
use std::any::TypeId;

// Static variables

// Constant variables

// Types

// Traits
trait Data {
    fn get_id(&self) -> Option<u64>;
}

// Enums

// Structs
struct DataManager {
    data_hashmap: HashMap<TypeId, HashMap<u64, Box<dyn Any>>>,
    unused_data_id: u64,
}

// Implementations
impl DataManager {
    pub fn new() -> Self {
        DataManager { data_hashmap: HashMap::new(), unused_data_id: 0 }
    }

    pub fn register_data_type<T: 'static + Any + Data + Default>(&mut self) -> Result<(), String> {
        if self.data_hashmap.contains_key(&TypeId::of::<T>()) {
            return Err(format!("Data type already registered: {}", std::any::type_name::<T>()));
        }

        self.data_hashmap.insert(TypeId::of::<T>(), HashMap::new());

        Ok(())
    }

    pub fn unregister_data_type<T: 'static + Any + Data + Default>(&mut self) -> Result<(), String> {
        if !self.data_hashmap.contains_key(&TypeId::of::<T>()) {
            return Err(format!("Data type not registered: {}", std::any::type_name::<T>()));
        }

        self.data_hashmap.remove(&TypeId::of::<T>());

        Ok(())
    }

    pub fn is_data_type_registered<T: 'static + Any + Data + Default>(&self) -> Result<bool, String> {
        Ok(self.data_hashmap.contains_key(&TypeId::of::<T>()))
    }

    pub fn register_data<T: 'static + Any + Data + Default>(&mut self, data: T) -> Result<(), String> {
        if let Some(id) = data.get_id() {
            return Err(format!("Data already registered: {}", id));
        }
        
        let data_hashmap = match self.data_hashmap.get_mut(&TypeId::of::<T>()) {
            Some(data_hashmap) => data_hashmap,
            None => return Err(format!("Data type not registered: {}", std::any::type_name::<T>())),
        };

        let id = self.unused_data_id;
        self.unused_data_id += 1;

        if data_hashmap.contains_key(&id) {
            return Err(format!("Data id '{}' has already been registered. The registration will be aborted and the data manager should be restarted immediately!", id));
        }

        data_hashmap.insert(id, Box::new(data));

        Ok(())
    }

    pub fn unregister_data<T: 'static + Any + Data + Default>(&mut self, id: u64) -> Result<T, String> {
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

    pub fn is_data_registered<T: 'static + Any + Data + Default>(&self, id: u64) -> Result<bool, String> {
        let data_hashmap = match self.data_hashmap.get(&TypeId::of::<T>()) {
            Some(data_hashmap) => data_hashmap,
            None => return Err(format!("Data type not registered: {}", std::any::type_name::<T>())),
        };

        Ok(data_hashmap.contains_key(&id))
    }

    pub fn get_data<T: 'static + Any + Data + Default>(&self, id: u64) -> Result<Option<&T>, String> {
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

    pub fn get_data_mut<T: 'static + Any + Data + Default>(&mut self, id: u64) -> Result<Option<&mut T>, String> {
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
    manager.register_data_type::<TestData>();
    manager.register_data(TestData::default());
}

// TEMPORARY
pub struct TestData {
    id: Option<u64>,
}

impl Data for TestData {
    fn get_id(&self) -> Option<u64> {
        self.id
    }
}

impl Default for TestData {
    fn default() -> Self {
        TestData { id: None }
    }
}