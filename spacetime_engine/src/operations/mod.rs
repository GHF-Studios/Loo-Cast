pub mod requesters;
pub mod resources;

use bevy::prelude::*;
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::hash::Hash;
use std::fmt::Debug;
use lazy_static::lazy_static;
use log::trace;

// Plugin struct
pub(in crate) struct OperationsPlugin;

impl Plugin for OperationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, start_main_manager)
            .add_systems(PostUpdate, process_operations);
    }
}



// InstanceID struct
pub struct InstanceID<T: 'static + Send + Sync>(u64, std::marker::PhantomData<T>);

impl<T: 'static + Send + Sync> std::fmt::Debug for InstanceID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID({})", self.0)
    }
}

impl<T: 'static + Send + Sync> std::clone::Clone for InstanceID<T> {
    fn clone(&self) -> Self {
        Self(self.0, std::marker::PhantomData)
    }
}

impl<T: 'static + Send + Sync> core::marker::Copy for InstanceID<T> {
}

impl<T: 'static + Send + Sync> std::cmp::PartialEq for InstanceID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: 'static + Send + Sync> std::cmp::Eq for InstanceID<T> {
}

impl<T: 'static + Send + Sync> std::hash::Hash for InstanceID<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}



// InstanceRegistry traits
pub trait InstanceRegistryKey: 'static + Clone + Copy + Debug + PartialEq + Eq + Hash + Send + Sync {
    fn new(id: u64) -> Self;
    fn get(&self) -> u64;
}

impl<T: 'static + Send + Sync> InstanceRegistryKey for InstanceID<T> {
    fn new(id: u64) -> Self {
        Self(id, std::marker::PhantomData)
    }

    fn get(&self) -> u64 {
        self.0
    }
}

pub trait InstanceRegistryValue: 'static + Send + Sync {
}

impl InstanceRegistryValue for Entity {
}



// InstanceRegistry struct
pub struct InstanceRegistry<K: InstanceRegistryKey, V: InstanceRegistryValue> {
    registered: HashSet<K>,
    managed: HashMap<K, V>,
    next_key: K,
    recycled_keys: Vec<K>,
}

impl<K: InstanceRegistryKey, V: InstanceRegistryValue> InstanceRegistry<K, V> {
    pub fn new() -> Self {
        Self {
            registered: HashSet::new(),
            managed: HashMap::new(),
            next_key: K::new(1),
            recycled_keys: Vec::new(),
        }
    }

    fn get_unused_key(&mut self) -> K {
        if let Some(recycled_key) = self.recycled_keys.pop() {
            trace!("Used recycled key: '{:?}'", recycled_key);

            recycled_key
        } else {
            let key = self.next_key;
            self.next_key = K::new(self.next_key.get() + 1);
            key
        }
    }

    fn recycle_key(&mut self, key: K) {
        if !self.registered.contains(&key) {
            panic!("Key '{:?}' is not registered!", key);
        }

        if self.recycled_keys.contains(&key) {
            panic!("Key '{:?}' is already recycled!", key);
        }

        self.recycled_keys.push(key);
    }

    pub fn register(&mut self) -> K {
        let key = self.get_unused_key();

        self.registered.insert(key);

        key
    }

    pub fn unregister(&mut self, key: K) {
        if !self.registered.contains(&key) {
            panic!("Key '{:?}' is invalid!", key);
        }

        if self.managed.contains_key(&key) {
            panic!("Entry '{:?}' is still managed!", key);
        }

        self.registered.retain(|other_key| key != *other_key);

        self.recycle_key(key);
    }

    pub fn manage(&mut self, key: K, value: V) {
        if !self.registered.contains(&key) {
            panic!("Key '{:?}' is invalid!", key);
        }

        if self.managed.contains_key(&key) {
            panic!("Entry '{:?}' is already managed!", key);
        }

        self.managed.insert(key, value);
    }

    pub fn unmanage(&mut self, key: K) -> V {
        if !self.registered.contains(&key) {
            panic!("Key '{:?}' is invalid!", key);
        }

        if !self.managed.contains_key(&key) {
            panic!("Entry '{:?}' is already unmanaged!", key);
        }

        self.managed.remove(&key).unwrap()
    }

    pub fn get(&self, key: K) -> Option<&V> {
        if !self.registered.contains(&key) {
            panic!("Key '{:?}' is invalid!", key);
        }

        self.managed.get(&key)
    }

    pub fn get_mut(&mut self, key: K) -> Option<&mut V> {
        if !self.registered.contains(&key) {
            panic!("Key '{:?}' is invalid!", key);
        }

        self.managed.get_mut(&key)
    }
}



// TypeRegistry struct
pub struct TypeRegistry {
    registered: HashSet<TypeId>,
    managed: HashMap<TypeId, (String, HashMap<TypeId, Box<dyn Any + Send + Sync>>)>,
    operation_queue: OperationQueue,
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self {
            registered: HashSet::new(),
            managed: HashMap::new(),
            operation_queue: OperationQueue::new(),
        }
    }

    pub fn register<T: 'static>(&mut self) {
        let type_id = TypeId::of::<T>();

        if self.registered.contains(&type_id) {
            panic!("Type '{:?}' is already registered!", type_id);
        }

        self.registered.insert(type_id);
    }

    pub fn unregister<T: 'static>(&mut self) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is still managed!", type_id);
        }

        self.registered.retain(|other_type_id| type_id != *other_type_id);
    }

    pub fn manage<T: 'static>(&mut self, name: &str) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is already managed!", type_id);
        }

        self.managed.insert(type_id, (name.to_string(), HashMap::new()));
    }

    pub fn unmanage<T: 'static>(&mut self) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is already unmanaged!", type_id);
        }

        self.managed.remove(&type_id);
    }

    pub fn get_name<T: 'static>(&self) -> Option<&String> {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            return None;
        }

        if !self.managed.contains_key(&type_id) {
            return None;
        }

        let (name, _) = self.managed.get(&type_id).unwrap();

        Some(name)
    }

    pub fn set_data<T: 'static, D: 'static + Send + Sync>(&mut self, data: D) {
        let type_id = TypeId::of::<T>();
        let data_type_id = TypeId::of::<D>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is not managed!", type_id);
        }

        let data_map = self.managed.entry(type_id).or_insert_with(HashMap::new);
        data_map.insert(data_type_id, Box::new(data));
    }

    pub fn get_data<T: 'static, D: 'static + Send + Sync>(&self) -> Option<&D> {
        let type_id = TypeId::of::<T>();
        let data_type_id = TypeId::of::<D>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is not managed!", type_id);
        }

        let data_map = self.managed.get(&type_id).unwrap();

        let data_box = data_map.get(&data_type_id)?;

        let data_ref = match data_box.downcast_ref::<D>() {
            Some(data_ref) => data_ref,
            None => panic!("Data type mismatch!"),
        };

        return Some(data_ref);
    }

    pub fn get_data_mut<T: 'static, D: 'static + Send + Sync>(&mut self) -> Option<&mut D> {
        let type_id = TypeId::of::<T>();
        let data_type_id = TypeId::of::<D>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is not managed!", type_id);
        }

        let data_map = self.managed.get_mut(&type_id).unwrap();

        let data_box = data_map.get_mut(&data_type_id)?;

        let data_ref = match data_box.downcast_mut::<D>() {
            Some(data_ref) => data_ref,
            None => panic!("Data type mismatch!"),
        };

        return Some(data_ref);
    }
}



// Operation trait
pub trait Operation: 'static + Send + Sync {
    fn execute(&self, main_type_registry: &mut TypeRegistry, world: &mut World);
}

// OperationQueue struct
pub struct OperationQueue {
    queue: Vec<Box<dyn Operation>>,
}

impl OperationQueue {
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }

    pub fn add_operation(&mut self, operation: Box<dyn Operation>) {
        self.queue.push(operation);
    }

    pub fn execute_operations(&mut self, world: &mut World) {
        let mut main_manager = MAIN_MANAGER.lock().unwrap();
        let main_type_registry = main_manager.get_type_registry_mut();

        while let Some(operation_box) = self.queue.pop() {
            operation_box.execute(main_type_registry, world);
        }
    }
}



// MainManager struct
pub struct MainManager {
    type_registry: TypeRegistry,
    operation_queue: OperationQueue,
}

impl MainManager {
    pub fn new() -> Self {
        Self {
            type_registry: TypeRegistry::new(),
            operation_queue: OperationQueue::new(),
        }
    }

    pub fn get_type_registry(&self) -> &TypeRegistry {
        &self.type_registry
    }

    pub fn get_type_registry_mut(&mut self) -> &mut TypeRegistry {
        &mut self.type_registry
    }

    pub fn get_operation_queue(&self) -> &OperationQueue {
        &self.operation_queue
    }

    pub fn get_operation_queue_mut(&mut self) -> &mut OperationQueue {
        &mut self.operation_queue
    }
}



// Static
lazy_static! {
    pub static ref MAIN_MANAGER: Arc<Mutex<MainManager>> = Arc::new(Mutex::new(MainManager::new()));
}

// Systems
fn start_main_manager() {
    init_entity_type();
    init_chunk_type();
    init_chunk_actor_type();
    init_chunk_loader_type();
}

fn process_operations(world: &mut World) {
    let mut main_manager = MAIN_MANAGER.lock().unwrap();
    let operation_queue = main_manager.get_operation_queue_mut();

    operation_queue.execute_operations(world);
}









// Entity

// Aliases
pub type EntityRegistry = InstanceRegistry<InstanceID<Entity>, Entity>;
pub type EntityOperationTypeRegistry = TypeRegistry;

// Operations
pub struct CreateEntity {
    args: (),
    callback: fn(InstanceID<Entity>),
}

impl CreateEntity {
    pub fn new(callback: Option<fn(InstanceID<Entity>)>) -> Self {
        Self {
            args: (),
            callback: callback.unwrap_or(|_| {}),
        }
    }
}

impl Operation for CreateEntity {
    fn execute(&self, main_type_registry: &mut TypeRegistry, world: &mut World) {
        todo!() //TODO
    }
}

pub struct DestroyEntity {
    args: InstanceID<Entity>,
    callback: fn(),
}

impl DestroyEntity {
    pub fn new(args: InstanceID<Entity>, callback: Option<fn()>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|| {}),
        }
    }
}

impl Operation for DestroyEntity {
    fn execute(&self, main_type_registry: &mut TypeRegistry, world: &mut World) {
        todo!() //TODO
    }
}

// Initialization
pub fn init_entity_type() {
    let mut main_manager = MAIN_MANAGER.lock().unwrap();

    let main_type_registry = main_manager.get_type_registry_mut();
    {
        let entity_instance_registry = EntityRegistry::new();
        {

        }
        main_type_registry.set_data::<Entity, EntityRegistry>("instance_registry", entity_instance_registry);

        let mut entity_operation_type_registry = EntityOperationTypeRegistry::new();
        {
            entity_operation_type_registry.register::<CreateEntity>();
            entity_operation_type_registry.manage::<CreateEntity>("create");
        
            entity_operation_type_registry.register::<DestroyEntity>();
            entity_operation_type_registry.manage::<DestroyEntity>("destroy");

        }
        main_type_registry.set_data::<Entity, EntityOperationTypeRegistry>("operation_type_registry", entity_operation_type_registry);
    }
    main_type_registry.register::<Entity>();
    main_type_registry.manage::<Entity>("entity");



    // TODO
    // Implement like data and type names so we can request an operation mostly by just specifying the path to that operation,
    // instead of having to manually write the required boilerplate code for each operation request
}



// Chunk
use crate::chunk::components::Chunk;
pub type ChunkRegistry = InstanceRegistry<InstanceID<Chunk>, Entity>;
pub type ChunkOperationTypeRegistry = TypeRegistry;

pub fn init_chunk_type() {
    let mut main_manager = MAIN_MANAGER.lock().unwrap();
    let main_type_registry = main_manager.get_type_registry_mut();

    main_type_registry.register::<Chunk>();
    main_type_registry.manage::<Chunk>("chunk");

    let chunk_registry = ChunkRegistry::new();
    let mut chunk_operation_type_registry = ChunkOperationTypeRegistry::new();

    chunk_operation_type_registry.register::<UpgradeToChunk>();
    chunk_operation_type_registry.manage::<UpgradeToChunk>("upgradeTo");

    main_type_registry.set_data::<Chunk, ChunkRegistry>("registry", chunk_registry);
    main_type_registry.set_data::<Chunk, ChunkOperationTypeRegistry>(chunk_operation_type_registry);
}



// Chunk Actor
use crate::chunk::actor::components::ChunkActor;
pub type ChunkActorRegistry = InstanceRegistry<InstanceID<ChunkActor>, Entity>;
pub type ChunkActorOperationTypeRegistry = TypeRegistry;

pub fn init_chunk_actor_type() {
    let mut main_manager = MAIN_MANAGER.lock().unwrap();
    let main_type_registry = main_manager.get_type_registry_mut();

    main_type_registry.register::<ChunkActor>();
    main_type_registry.manage::<ChunkActor>();

    let chunk_actor_registry = ChunkActorRegistry::new();
    let chunk_actor_operation_type_registry = ChunkActorOperationTypeRegistry::new();

    main_type_registry.set_data::<ChunkActor, ChunkActorRegistry>(chunk_actor_registry);
    main_type_registry.set_data::<ChunkActor, ChunkActorOperationTypeRegistry>(chunk_actor_operation_type_registry);
}



// Chunk Loader
use crate::chunk::loader::components::ChunkLoader;
pub type ChunkLoaderRegistry = InstanceRegistry<InstanceID<ChunkLoader>, Entity>;
pub type ChunkLoaderOperationTypeRegistry = TypeRegistry;

pub fn init_chunk_loader_type() {
    let mut main_manager = MAIN_MANAGER.lock().unwrap();
    let main_type_registry = main_manager.get_type_registry_mut();

    main_type_registry.register::<ChunkLoader>();
    main_type_registry.manage::<ChunkLoader>();

    let chunk_loader_registry = ChunkLoaderRegistry::new();
    let chunk_loader_operation_type_registry = ChunkLoaderOperationTypeRegistry::new();
    
    main_type_registry.set_data::<ChunkLoader, ChunkLoaderRegistry>(chunk_loader_registry);
    main_type_registry.set_data::<ChunkLoader, ChunkLoaderOperationTypeRegistry>(chunk_loader_operation_type_registry);
}




// EXPERIMENTAL CODE
/*
use mlua::{FromLuaMulti, Lua, Result, Table, TableExt, ToLuaMulti};

fn define_primitive<'lua, 'callback, A, R, F>(lua: &'lua Lua, primitive_operation_id: &str, primitive_operation_func: F) -> Result<()>
where
    'lua: 'callback,
    A: FromLuaMulti<'callback>,
    R: ToLuaMulti<'callback>,
    F: 'static + Send + Fn(&'callback Lua, A) -> Result<R>
{
    let globals = lua.globals();
    let ops: Table = globals.get("ops")?;
    let compiled_primitives: Table = ops.get("compiledPrimitives")?;
    
    let lua_func = lua.create_function(move |lua, args: A| primitive_operation_func(lua, args))?;

    compiled_primitives.set(primitive_operation_id, lua_func)?;

    Ok(())
}



fn setup_lua_env() -> Result<Lua> {
    let lua = Lua::new();

    lua.load(include_str!("../../scripts/main.lua")).exec()?;

    fn add_integers(a: i32, b: i32) -> i32 {
        a + b
    }
    fn multiply_integers(a: i32, b: i32) -> i32 {
        a * b
    }

    fn request_create_entity() -> u64 {
        0
    }

    define_primitive(&lua, "math.add_integers", |_, (a, b): (i32, i32)|
        Ok(add_integers(a, b))
    )?;
    define_primitive(&lua, "math.multiply_integers", |_, (a, b): (i32, i32)|
        Ok(multiply_integers(a, b))
    )?;
    define_primitive(&lua, "entity.request_create", |_, ()|
        Ok(request_create_entity())
    )?;

    Ok(lua)
}

fn main() -> Result<()> {
    let lua = setup_lua_env()?;
    let globals = lua.globals();
    let test_ops: Table = globals.get("testOps")?;
    let test_func = test_ops.get::<_, mlua::Function>("test")?;

    test_func.call(())?;  // Pass any arguments inside the tuple if needed

    Ok(())
}
*/