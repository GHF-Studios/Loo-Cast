// Modules

// Local imports

// Internal imports
use super::data::*;

// External imports
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

// Static variables

// Constant variables

// Types

// Traits
trait Resource {
    fn get_file_handle(&self) -> &File;
    fn get_file_path(&self) -> &Path;
    fn load_data<T: 'static + Any + super::data::Data>(&self) -> Result<T, String>;
}

// Enums

// Structs
struct ResourceManager {
    resource_hashmap: HashMap<TypeId, HashMap<PathBuf, Box<dyn Any>>>
}

// Implementations
impl ResourceManager {
    pub fn new() -> Self {
        ResourceManager { resource_hashmap: HashMap::new() }
    }

    pub fn register_resource_type<T: 'static + Any + Resource>(&mut self) -> Result<(), String> {
        if self.resource_hashmap.contains_key(&TypeId::of::<T>()) {
            return Err(format!("Resource type already registered: {}", std::any::type_name::<T>()));
        }

        self.resource_hashmap.insert(TypeId::of::<T>(), HashMap::new());

        Ok(())
    }

    pub fn unregister_resource_type<T: 'static + Any + Resource>(&mut self) -> Result<(), String> {
        if !self.resource_hashmap.contains_key(&TypeId::of::<T>()) {
            return Err(format!("Resource type not registered: {}", std::any::type_name::<T>()));
        }

        self.resource_hashmap.remove(&TypeId::of::<T>());

        Ok(())
    }

    pub fn is_resource_type_registered<T: 'static + Any + Resource>(&self) -> Result<bool, String> {
        Ok(self.resource_hashmap.contains_key(&TypeId::of::<T>()))
    }

    pub fn register_resource<T: 'static + Any + Resource>(&mut self, resource: T) -> Result<(), String> {
        let id = resource.get_file_path();

        let resource_hashmap = match self.resource_hashmap.get_mut(&TypeId::of::<T>()) {
            Some(resource_hashmap) => resource_hashmap,
            None => return Err(format!("Resource type not registered: {}", std::any::type_name::<T>())),
        };

        if resource_hashmap.contains_key(id) {
            return Err(format!("Resource already registered: {}", id.display()));
        }

        resource_hashmap.insert(id.to_path_buf(), Box::new(resource));

        Ok(())
    }

    pub fn unregister_resource<T: 'static + Any + Resource>(&mut self, resource: T) -> Result<T, String> {
        let resource_hashmap = match self.resource_hashmap.get_mut(&TypeId::of::<T>()) {
            Some(resource_hashmap) => resource_hashmap,
            None => return Err(format!("Resource type not registered: {}", std::any::type_name::<T>())),
        };

        if !resource_hashmap.contains_key(resource.get_file_path()) {
            return Err(format!("Resource not registered: {}", resource.get_file_path().display()));
        }

        let resource = resource_hashmap.remove(resource.get_file_path()).unwrap();

        Ok(*resource.downcast::<T>().unwrap())
    }

    pub fn is_resource_registered<T: 'static + Any + Resource>(&self, resource: T) -> Result<bool, String> {
        let resource_hashmap = match self.resource_hashmap.get(&TypeId::of::<T>()) {
            Some(resource_hashmap) => resource_hashmap,
            None => return Err(format!("Resource type not registered: {}", std::any::type_name::<T>())),
        };

        Ok(resource_hashmap.contains_key(resource.get_file_path()))
    }

    pub fn get_resource<T: 'static + Any + Resource>(&self, resource: T) -> Result<Option<&T>, String> {
        let resource_hashmap = match self.resource_hashmap.get(&TypeId::of::<T>()) {
            Some(resource_hashmap) => resource_hashmap,
            None => return Err(format!("Resource type not registered: {}", std::any::type_name::<T>())),
        };

        if !resource_hashmap.contains_key(resource.get_file_path()) {
            return Ok(None);
        }

        let resource = resource_hashmap.get(resource.get_file_path()).unwrap();

        Ok(resource.downcast_ref::<T>())
    }

    pub fn get_resource_mut<T: 'static + Any + Resource>(&mut self, resource: T) -> Result<Option<&mut T>, String> {
        let resource_hashmap = match self.resource_hashmap.get_mut(&TypeId::of::<T>()) {
            Some(resource_hashmap) => resource_hashmap,
            None => return Err(format!("Resource type not registered: {}", std::any::type_name::<T>())),
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
        Ok(_) => println!("Registered resource type: {}", std::any::type_name::<TestResource>()),
        Err(error) => println!("Failed to register resource type: {}", error),
    };

    let file_path = Path::new("test.txt");

    let file_handle = match File::open(file_path) {
        Ok(file_handle) => file_handle,
        Err(error) => panic!("Failed to open file: {}", error),
    };

    let resource = TestResource { file_handle, file_path: file_path.to_path_buf() };

    match resource_manager.register_resource(resource) {
        Ok(_) => println!("Registered resource: {}", file_path.display()),
        Err(error) => println!("Failed to register resource: {}", error),
    };
}

// TEMPORARY
pub struct TestResource {
    file_handle: File,
    file_path: PathBuf,
}

impl Resource for TestResource {
    fn get_file_handle(&self) -> &File {
        &self.file_handle
    }

    fn get_file_path(&self) -> &Path {
        &self.file_path
    }
}
