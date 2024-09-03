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
#[derive(Reflect)]
pub struct InstanceID<T: 'static + Send + Sync>(u64, #[reflect(ignore)]std::marker::PhantomData<T>);

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
    managed: HashMap<TypeId, HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
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

    pub fn manage<T: 'static>(&mut self) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is already managed!", type_id);
        }

        self.managed.insert(type_id, HashMap::new());
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



// Operation

// Wrappers
#[derive(Deref, DerefMut)]
pub struct OperationTypeRegistry(TypeRegistry);

impl OperationTypeRegistry {
    pub fn new() -> Self {
        Self(TypeRegistry::new())
    }
}

// Trait
pub trait Operation: 'static + Send + Sync {
    fn execute(&self, world: &mut World, main_type_registry: &mut MainTypeRegistry, operation_queue: &mut OperationQueue);
}

// Operation structs
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

    pub fn remove_operations(&mut self) -> Vec<Box<dyn Operation>> {
        self.queue.drain(..).collect()
    }
}



// MainManager

// Wrappers
#[derive(Deref, DerefMut)]
pub struct MainTypeRegistry(TypeRegistry);

impl MainTypeRegistry {
    pub fn new() -> Self {
        Self(TypeRegistry::new())
    }
}

// MainManager struct
pub struct MainManager {
    main_type_registry: MainTypeRegistry,
    operation_queue: OperationQueue,
}

impl MainManager {
    pub fn new() -> Self {
        Self {
            main_type_registry: MainTypeRegistry(TypeRegistry::new()),
            operation_queue: OperationQueue::new(),
        }
    }

    pub fn get_main_type_registry(&self) -> &MainTypeRegistry {
        &self.main_type_registry
    }

    pub fn get_main_type_registry_mut(&mut self) -> &mut MainTypeRegistry {
        &mut self.main_type_registry
    }

    pub fn get_operation_queue(&self) -> &OperationQueue {
        &self.operation_queue
    }

    pub fn get_operation_queue_mut(&mut self) -> &mut OperationQueue {
        &mut self.operation_queue
    }
}



// Statics
lazy_static! {
    static ref MAIN_TYPE_REGISTRY: Arc<Mutex<MainTypeRegistry>> = Arc::new(Mutex::new(MainTypeRegistry(TypeRegistry::new())));
    static ref OPERATION_QUEUE: Arc<Mutex<OperationQueue>> = Arc::new(Mutex::new(OperationQueue::new()));
}

// Systems
fn start_main_manager() {
    let mut main_type_registry = MAIN_TYPE_REGISTRY.lock().unwrap();

    init_entity_type(&mut *main_type_registry);
    init_chunk_type(&mut *main_type_registry);
    init_chunk_actor_type(&mut *main_type_registry);
    init_chunk_loader_type(&mut *main_type_registry);
}

fn process_operations(world: &mut World) {
    let mut main_type_registry = MAIN_TYPE_REGISTRY.lock().unwrap();
    let mut operation_queue = OPERATION_QUEUE.lock().unwrap();

    let mut operations = operation_queue.remove_operations();
    while let Some(operation_box) = operations.pop() {
        operation_box.execute(world, &mut *main_type_registry, &mut *operation_queue);
    }
}









// TODO: Create wrappers/containers for XYZOperationArgs and XYZOperationResult
// TODO: Implement operations for all types









// Entity

// Wrappers
#[derive(Deref, DerefMut)]
pub struct EntityInstanceRegistry(InstanceRegistry<InstanceID<Entity>, Entity>);

impl EntityInstanceRegistry {
    pub fn new() -> Self {
        Self(InstanceRegistry::new())
    }
}

#[derive(Deref, DerefMut)]
pub struct EntityOperationTypeRegistry(OperationTypeRegistry);

impl EntityOperationTypeRegistry {
    pub fn new() -> Self {
        Self(OperationTypeRegistry(TypeRegistry::new()))
    }
}

// Operations
pub struct CreateEntity {
    args: EntityPosition,
    callback: fn(&mut OperationQueue, InstanceID<Entity>),
}

impl CreateEntity {
    pub fn new(args: EntityPosition, callback: Option<fn(&mut OperationQueue, InstanceID<Entity>)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_, _| {}),
        }
    }
}

impl Operation for CreateEntity {
    fn execute(&self, world: &mut World, main_type_registry: &mut MainTypeRegistry, operation_queue: &mut OperationQueue) {
        let entity_instance_registry = main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>().unwrap();

        let entity_id = entity_instance_registry.register();

        let entity = world.spawn((
            Transform::from_translation(self.args.extend(0.0)),
            SpacetimeEntity {
                id: entity_id,
            },
        )).id();

        entity_instance_registry.manage(entity_id, entity);

        (self.callback)(operation_queue, entity_id);
    }
}

pub struct DestroyEntity {
    args: InstanceID<Entity>,
    callback: fn(&mut OperationQueue),
}

impl DestroyEntity {
    pub fn new(args: InstanceID<Entity>, callback: Option<fn(&mut OperationQueue)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}

impl Operation for DestroyEntity {
    fn execute(&self, world: &mut World, main_type_registry: &mut MainTypeRegistry, operation_queue: &mut OperationQueue) {
        let entity_instance_registry = main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>().unwrap();

        let entity = entity_instance_registry.unmanage(self.args);

        world.despawn(entity);

        entity_instance_registry.unregister(self.args);

        (self.callback)(operation_queue);
    }
}

// Initialization
pub fn init_entity_type(main_type_registry: &mut MainTypeRegistry) {
    main_type_registry.register::<Entity>();
    main_type_registry.manage::<Entity>();

    main_type_registry.set_data::<Entity, _>(EntityInstanceRegistry::new());
    main_type_registry.set_data::<Entity, _>(EntityOperationTypeRegistry::new());

    let entity_operation_type_registry: &mut EntityOperationTypeRegistry = main_type_registry.get_data_mut::<Entity, _>().unwrap();

    entity_operation_type_registry.register::<CreateEntity>();
    entity_operation_type_registry.manage::<CreateEntity>();

    entity_operation_type_registry.register::<DestroyEntity>();
    entity_operation_type_registry.manage::<DestroyEntity>();
}



// Chunk

// Imports
use crate::chunk::components::Chunk;

// Wrappers
#[derive(Deref, DerefMut)]
pub struct ChunkInstanceRegistry(InstanceRegistry<InstanceID<Chunk>, Entity>);

impl ChunkInstanceRegistry {
    pub fn new() -> Self {
        Self(InstanceRegistry::new())
    }
}

#[derive(Deref, DerefMut)]
pub struct ChunkOperationTypeRegistry(OperationTypeRegistry);

impl ChunkOperationTypeRegistry {
    pub fn new() -> Self {
        Self(OperationTypeRegistry(TypeRegistry::new()))
    }
}

// Operations
pub struct UpgradeToChunk {
    args: (InstanceID<Entity>, ChunkPosition, Option<InstanceID<ChunkLoader>>),
    callback: fn(&mut OperationQueue, InstanceID<Chunk>),
}

impl UpgradeToChunk {
    pub fn new(args: (InstanceID<Entity>, ChunkPosition, Option<InstanceID<ChunkLoader>>), callback: Option<fn(&mut OperationQueue, InstanceID<Chunk>)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_, _| {}),
        }
    }
}

impl Operation for UpgradeToChunk {
    fn execute(&self, world: &mut World, main_type_registry: &mut MainTypeRegistry, operation_queue: &mut OperationQueue) {
        todo!(); // TODO
    }
}

pub struct DowngradeFromChunk {
    args: (InstanceID<Entity>, InstanceID<Chunk>),
    callback: fn(&mut OperationQueue),
}

impl DowngradeFromChunk {
    pub fn new(args: (InstanceID<Entity>, InstanceID<Chunk>), callback: Option<fn(&mut OperationQueue)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}

impl Operation for DowngradeFromChunk {
    fn execute(&self, world: &mut World, main_type_registry: &mut MainTypeRegistry, operation_queue: &mut OperationQueue) {
        todo!(); // TODO
    }
}

pub struct LoadChunk {
    args: InstanceID<Chunk>,
    callback: fn(&mut OperationQueue),
}

impl LoadChunk {
    pub fn new(args: InstanceID<Chunk>, callback: Option<fn(&mut OperationQueue)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}

impl Operation for LoadChunk {
    fn execute(&self, world: &mut World, main_type_registry: &mut MainTypeRegistry, operation_queue: &mut OperationQueue) {
        todo!(); // TODO
    }
}

pub struct SaveChunk {
    args: InstanceID<Chunk>,
    callback: fn(&mut OperationQueue),
}

impl SaveChunk {
    pub fn new(args: InstanceID<Chunk>, callback: Option<fn(&mut OperationQueue)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}

impl Operation for SaveChunk {
    fn execute(&self, world: &mut World, main_type_registry: &mut MainTypeRegistry, operation_queue: &mut OperationQueue) {
        todo!(); // TODO
    }
}

// Initialization
pub fn init_chunk_type(main_type_registry: &mut MainTypeRegistry) {
    main_type_registry.register::<Chunk>();
    main_type_registry.manage::<Chunk>();

    main_type_registry.set_data::<Chunk, _>(ChunkInstanceRegistry::new());
    main_type_registry.set_data::<Chunk, _>(ChunkOperationTypeRegistry::new());

    let chunk_operation_type_registry: &mut ChunkOperationTypeRegistry = main_type_registry.get_data_mut::<Chunk, _>().unwrap();

    chunk_operation_type_registry.register::<UpgradeToChunk>();
    chunk_operation_type_registry.manage::<UpgradeToChunk>();

    chunk_operation_type_registry.register::<DowngradeFromChunk>();
    chunk_operation_type_registry.manage::<DowngradeFromChunk>();

    chunk_operation_type_registry.register::<LoadChunk>();
    chunk_operation_type_registry.manage::<LoadChunk>();

    chunk_operation_type_registry.register::<SaveChunk>();
    chunk_operation_type_registry.manage::<SaveChunk>();
}



// Chunk Actor

// Imports
use crate::chunk::actor::components::ChunkActor;

// Wrappers
#[derive(Deref, DerefMut)]
pub struct ChunkActorInstanceRegistry(InstanceRegistry<InstanceID<ChunkActor>, Entity>);

impl ChunkActorInstanceRegistry {
    pub fn new() -> Self {
        Self(InstanceRegistry::new())
    }
}

#[derive(Deref, DerefMut)]
pub struct ChunkActorOperationTypeRegistry(OperationTypeRegistry);

impl ChunkActorOperationTypeRegistry {
    pub fn new() -> Self {
        Self(OperationTypeRegistry(TypeRegistry::new()))
    }
}

// Operations
pub struct UpgradeToChunkActor {
    args: InstanceID<Entity>,
    callback: fn(&mut OperationQueue, InstanceID<ChunkActor>),
}

impl UpgradeToChunkActor {
    pub fn new(args: InstanceID<Entity>, callback: Option<fn(&mut OperationQueue, InstanceID<ChunkActor>)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_, _| {}),
        }
    }
}

impl Operation for UpgradeToChunkActor {
    fn execute(&self, world: &mut World, main_type_registry: &mut MainTypeRegistry, operation_queue: &mut OperationQueue) {
        todo!(); // TODO
    }
}

pub struct DowngradeFromChunkActor {
    args: (InstanceID<Entity>, InstanceID<ChunkActor>),
    callback: fn(&mut OperationQueue),
}

impl DowngradeFromChunkActor {
    pub fn new(args: (InstanceID<Entity>, InstanceID<ChunkActor>), callback: Option<fn(&mut OperationQueue)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}

impl Operation for DowngradeFromChunkActor {
    fn execute(&self, world: &mut World, main_type_registry: &mut MainTypeRegistry, operation_queue: &mut OperationQueue) {
        todo!(); // TODO
    }
}

// Initialization
pub fn init_chunk_actor_type(main_type_registry: &mut MainTypeRegistry) {
    main_type_registry.register::<ChunkActor>();
    main_type_registry.manage::<ChunkActor>();

    main_type_registry.set_data::<ChunkActor, _>(ChunkActorInstanceRegistry::new());
    main_type_registry.set_data::<ChunkActor, _>(ChunkActorOperationTypeRegistry::new());

    let chunk_actor_operation_type_registry: &mut ChunkActorOperationTypeRegistry = main_type_registry.get_data_mut::<ChunkActor, _>().unwrap();

    chunk_actor_operation_type_registry.register::<UpgradeToChunkActor>();
    chunk_actor_operation_type_registry.manage::<UpgradeToChunkActor>();

    chunk_actor_operation_type_registry.register::<DowngradeFromChunkActor>();
    chunk_actor_operation_type_registry.manage::<DowngradeFromChunkActor>();
}



// Chunk Loader

// Imports
use crate::chunk::loader::components::ChunkLoader;
use crate::chunk::position::structs::ChunkPosition;
use crate::entity::components::SpacetimeEntity;
use crate::entity::position::structs::EntityPosition;

// Wrappers
#[derive(Deref, DerefMut)]
pub struct ChunkLoaderInstanceRegistry(InstanceRegistry<InstanceID<ChunkLoader>, Entity>);

impl ChunkLoaderInstanceRegistry {
    pub fn new() -> Self {
        Self(InstanceRegistry::new())
    }
}

#[derive(Deref, DerefMut)]
pub struct ChunkLoaderOperationTypeRegistry(OperationTypeRegistry);

impl ChunkLoaderOperationTypeRegistry {
    pub fn new() -> Self {
        Self(OperationTypeRegistry(TypeRegistry::new()))
    }
}

// Operations
pub struct UpgradeToChunkLoader {
    args: InstanceID<Entity>,
    callback: fn(&mut OperationQueue, InstanceID<ChunkLoader>),
}

impl UpgradeToChunkLoader {
    pub fn new(args: InstanceID<Entity>, callback: Option<fn(&mut OperationQueue, InstanceID<ChunkLoader>)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_, _| {}),
        }
    }
}

impl Operation for UpgradeToChunkLoader {
    fn execute(&self, world: &mut World, main_type_registry: &mut MainTypeRegistry, operation_queue: &mut OperationQueue) {
        todo!(); // TODO
    }
}

pub struct DowngradeFromChunkLoader {
    args: (InstanceID<Entity>, InstanceID<ChunkLoader>),
    callback: fn(&mut OperationQueue),
}

impl DowngradeFromChunkLoader {
    pub fn new(args: (InstanceID<Entity>, InstanceID<ChunkLoader>), callback: Option<fn(&mut OperationQueue)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}

impl Operation for DowngradeFromChunkLoader {
    fn execute(&self, world: &mut World, main_type_registry: &mut MainTypeRegistry, operation_queue: &mut OperationQueue) {
        todo!(); // TODO
    }
}

// Initialization
pub fn init_chunk_loader_type(main_type_registry: &mut MainTypeRegistry) {
    main_type_registry.register::<ChunkLoader>();
    main_type_registry.manage::<ChunkLoader>();

    main_type_registry.set_data::<ChunkLoader, _>(ChunkLoaderInstanceRegistry::new());
    main_type_registry.set_data::<ChunkLoader, _>(ChunkLoaderOperationTypeRegistry::new());

    let chunk_loader_operation_type_registry: &mut ChunkLoaderOperationTypeRegistry = main_type_registry.get_data_mut::<ChunkLoader, _>().unwrap();

    chunk_loader_operation_type_registry.register::<UpgradeToChunkLoader>();
    chunk_loader_operation_type_registry.manage::<UpgradeToChunkLoader>();

    chunk_loader_operation_type_registry.register::<DowngradeFromChunkLoader>();
    chunk_loader_operation_type_registry.manage::<DowngradeFromChunkLoader>();
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