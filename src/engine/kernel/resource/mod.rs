// Modules

// Local imports

// Internal imports

// External imports
use std::any::Any;
use std::any::TypeId;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};

// Static variables

// Constant variables

// Types

// Traits
pub trait Resource {
    fn new(file_path: &Path) -> Self;
    fn get_file_path(&self) -> &Path;
    fn get_file_content(&self) -> Result<&[u8], String>;
    fn set_file_content(&mut self, file_content: &[u8]) -> Result<(), String>;
}

// Enums

// Structs
pub struct ResourceManager {
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

    let file_path = Path::new("test.txt").to_path_buf();

    let file_handle = match File::open(file_path) {
        Ok(file_handle) => file_handle,
        Err(error) => panic!("Failed to open file: {}", error),
    };

    let resource = TestResource { file_handle, file_path };

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
    fn new(file_path: &Path) -> Self {
        let file_handle = match File::open(file_path) {
            Ok(file_handle) => file_handle,
            Err(error) => panic!("Failed to open file: {}", error),
        };

        TestResource { file_handle, file_path: file_path.to_path_buf() }
    }

    fn get_file_path(&self) -> &Path {
        &self.file_path
    }

    fn get_file_content(&self) -> Result<&[u8], String> {
        let mut file_handle = match File::open(self.get_file_path()) {
            Ok(file_handle) => file_handle,
            Err(error) => return Err(format!("Failed to open file: {}", error)),
        };

        let mut file_content = Vec::new();

        match file_handle.read(&mut file_content) {
            Ok(_) => (),
            Err(error) => return Err(format!("Failed to read file: {}", error)),
        };

        Ok(&file_content)
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
