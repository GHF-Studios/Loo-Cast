pub mod requesters;
pub mod resources;

use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::hash::Hash;
use std::fmt::Debug;
use lazy_static::lazy_static;
use log::trace;

pub(in crate) struct OperationsPlugin;
impl Plugin for OperationsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(PostUpdate, post_update);
    }
}



// Operation

// Wrappers
#[derive(Deref, DerefMut)]
pub struct MainTypeRegistry(TypeRegistry);
impl MainTypeRegistry {
    pub fn new() -> Self {
        Self(TypeRegistry::new())
    }
}

#[derive(Deref, DerefMut)]
pub struct OperationTypeRegistry(TypeRegistry);
impl OperationTypeRegistry {
    pub fn new() -> Self {
        Self(TypeRegistry::new())
    }
}

// InstanceID struct
#[derive(Reflect)]
pub struct InstanceID<T: 'static + Send + Sync>(u64, #[reflect(ignore)]std::marker::PhantomData<T>);
impl<T: 'static + Send + Sync> Default for InstanceID<T> {
    fn default() -> Self {
        Self(0, std::marker::PhantomData)
    }
}
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

    pub fn get_key(&self, value: &V) -> Option<&K> {
        self.managed.iter().find_map(|(key, other_value)| {
            if value == other_value {
                Some(key)
            } else {
                None
            }
        })
    }
}
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

// Traits
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

pub trait InstanceRegistryValue: 'static + PartialEq + Send + Sync {
}
impl InstanceRegistryValue for Entity {
}

pub trait Operation: 'static + Send + Sync {
    fn execute(&self, world: &mut World);
}

// Systems
fn startup() {
    let mut main_type_registry = MAIN_TYPE_REGISTRY.lock().unwrap();

    init_entity_type(&mut *main_type_registry);
    init_chunk_type(&mut *main_type_registry);
    init_chunk_actor_type(&mut *main_type_registry);
    init_chunk_loader_type(&mut *main_type_registry);
}

fn post_update(world: &mut World) {
    let mut operations = OPERATION_QUEUE.lock().unwrap().remove_operations();

    while let Some(operation_box) = operations.pop() {
        operation_box.execute(world);
    }
}

// Private singletons
lazy_static! {
    static ref MAIN_TYPE_REGISTRY: Arc<Mutex<MainTypeRegistry>> = Arc::new(Mutex::new(MainTypeRegistry(TypeRegistry::new())));
    static ref OPERATION_QUEUE: Arc<Mutex<OperationQueue>> = Arc::new(Mutex::new(OperationQueue::new()));
}









// TODO: Implement operations and hooks for all types (Optional: Extend to 'Camera', 'Player', 'Follower', and 'Physics', essentially reworking the entire code base; I guess; framework richie go brr)
// TODO: Integrate and Implement operations module into existing modules, and bundle that operation-related code in an 'operations' sub-module for each existing module, essentially finalizing the code base rework









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

// Hooks
fn on_add_entity(
    _world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
        Ok(main_type_registry) => main_type_registry,
        Err(_) => {
            return;
        },
    };

    let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
        Some(entity_instance_registry) => entity_instance_registry,
        None => {
            return;
        },
    };

    let entity_id = entity_instance_registry.register();
    entity_instance_registry.manage(entity_id, entity);
}

fn on_remove_chunk(
    _world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
        Ok(main_type_registry) => main_type_registry,
        Err(_) => {
            return;
        },
    };

    let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
        Some(entity_instance_registry) => entity_instance_registry,
        None => {
            return;
        },
    };

    let entity_id = match entity_instance_registry.get_key(&entity) {
        Some(entity_id) => *entity_id,
        None => {
            return;
        },
    };

    entity_instance_registry.unmanage(entity_id);
    entity_instance_registry.unregister(entity_id);
}

// Operations
pub struct CreateEntityArgs {
    pub entity_position: EntityPosition,
}
pub enum CreateEntityResult {
    Ok{
        entity_id: InstanceID<Entity>
    },
    Err(()),
}
pub struct CreateEntity {
    args: CreateEntityArgs,
    callback: fn(CreateEntityResult),
}
impl CreateEntity {
    pub fn new(args: CreateEntityArgs, callback: Option<fn(CreateEntityResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for CreateEntity {
    fn execute(&self, world: &mut World) {
        let entity = world.spawn((
            Transform::from_translation(self.args.entity_position.extend(0.0)),
            SpacetimeEntity {
                id: InstanceID::default(),
            },
        )).id();

        let spacetime_entity_component = match world.get::<SpacetimeEntity>(entity) {
            Some(spacetime_entity_component) => spacetime_entity_component,
            None => {
                (self.callback)(CreateEntityResult::Err(()));
                return;
            },
        };

        (self.callback)(CreateEntityResult::Ok {
            entity_id: spacetime_entity_component.id,
        });
    }
}

pub struct DestroyEntityArgs {
    pub entity_id: InstanceID<Entity>,
}
pub enum DestroyEntityResult {
    Ok(()),
    Err(()),
}
pub struct DestroyEntity {
    args: DestroyEntityArgs,
    callback: fn(DestroyEntityResult),
}
impl DestroyEntity {
    pub fn new(args: DestroyEntityArgs, callback: Option<fn(DestroyEntityResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for DestroyEntity {
    fn execute(&self, world: &mut World) {
        let entity = {
            let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
                Ok(main_type_registry) => main_type_registry,
                Err(_) => {
                    (self.callback)(DestroyEntityResult::Err(()));
                    return;
                },
            };

            let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
                Some(entity_instance_registry) => entity_instance_registry,
                None => {
                    (self.callback)(DestroyEntityResult::Err(()));
                    return;
                },
            };

            match entity_instance_registry.get(self.args.entity_id) {
                Some(entity) => *entity,
                None => {
                    (self.callback)(DestroyEntityResult::Err(()));
                    return;
                },
            }
        };

        if !world.despawn(entity) {
            (self.callback)(DestroyEntityResult::Err(()));
            return;
        }

        (self.callback)(DestroyEntityResult::Ok(()));
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
pub struct UpgradeToChunkArgs {
    pub target_entity_id: InstanceID<Entity>,
    pub chunk_position: ChunkPosition,
    pub chunk_owner: Option<InstanceID<ChunkLoader>>,
}
pub enum UpgradeToChunkResult {
    Ok{
        chunk_id: InstanceID<Chunk>,
    },
    Err(()),
}
pub struct UpgradeToChunk {
    args: UpgradeToChunkArgs,
    callback: fn(UpgradeToChunkResult),
}
impl UpgradeToChunk {
    pub fn new(args: UpgradeToChunkArgs, callback: Option<fn(UpgradeToChunkResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for UpgradeToChunk {
    fn execute(&self, world: &mut World) {
        todo!(); // TODO
    }
}

pub struct DowngradeFromChunkArgs {
    pub chunk_entity_id: InstanceID<Entity>,
    pub chunk_id: InstanceID<Chunk>,
}
pub enum DowngradeFromChunkResult {
    Ok(()),
    Err(()),
}
pub struct DowngradeFromChunk {
    args: DowngradeFromChunkArgs,
    callback: fn(DowngradeFromChunkResult),
}
impl DowngradeFromChunk {
    pub fn new(args: DowngradeFromChunkArgs, callback: Option<fn(DowngradeFromChunkResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for DowngradeFromChunk {
    fn execute(&self, world: &mut World) {
        todo!(); // TODO
    }
}

pub struct LoadChunkArgs {
    pub chunk_position: ChunkPosition,
}
pub enum LoadChunkResult {
    Ok(()),
    Err(()),
}
pub struct LoadChunk {
    args: LoadChunkArgs,
    callback: fn(LoadChunkResult),
}
impl LoadChunk {
    pub fn new(args: LoadChunkArgs, callback: Option<fn(LoadChunkResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for LoadChunk {
    fn execute(&self, world: &mut World) {
        todo!(); // TODO
    }
}

pub struct SaveChunkArgs {
    pub chunk_position: ChunkPosition,
}
pub enum SaveChunkResult {
    Ok(()),
    Err(()),
}
pub struct SaveChunk {
    args: SaveChunkArgs,
    callback: fn(SaveChunkResult),
}
impl SaveChunk {
    pub fn new(args: SaveChunkArgs, callback: Option<fn(SaveChunkResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for SaveChunk {
    fn execute(&self, world: &mut World) {
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
pub struct UpgradeToChunkActorArgs {
    pub target_entity_id: InstanceID<Entity>,
    pub chunk_actor_start_chunk_id: InstanceID<Chunk>,
}
pub enum UpgradeToChunkActorResult {
    Ok{
        chunk_actor_id: InstanceID<ChunkActor>,
    },
    Err(()),
}
pub struct UpgradeToChunkActor {
    args: UpgradeToChunkActorArgs,
    callback: fn(UpgradeToChunkActorResult),
}
impl UpgradeToChunkActor {
    pub fn new(args: UpgradeToChunkActorArgs, callback: Option<fn(UpgradeToChunkActorResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for UpgradeToChunkActor {
    fn execute(&self, world: &mut World) {
        todo!(); // TODO
    }
}

pub struct DowngradeFromChunkActorArgs {
    pub chunk_actor_entity_id: InstanceID<Entity>,
    pub chunk_actor_id: InstanceID<ChunkActor>,
}
pub enum DowngradeFromChunkActorResult {
    Ok(()),
    Err(()),
}
pub struct DowngradeFromChunkActor {
    args: DowngradeFromChunkActorArgs,
    callback: fn(DowngradeFromChunkActorResult),
}
impl DowngradeFromChunkActor {
    pub fn new(args: DowngradeFromChunkActorArgs, callback: Option<fn(DowngradeFromChunkActorResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for DowngradeFromChunkActor {
    fn execute(&self, world: &mut World) {
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
pub struct UpgradeToChunkLoaderArgs {
    pub target_entity_id: InstanceID<Entity>,
    pub chunk_loader_load_radius: u16
}
pub enum UpgradeToChunkLoaderResult {
    Ok{
        chunk_loader_id: InstanceID<ChunkLoader>,
    },
    Err(()),
}
pub struct UpgradeToChunkLoader {
    args: UpgradeToChunkLoaderArgs,
    callback: fn(UpgradeToChunkLoaderResult),
}
impl UpgradeToChunkLoader {
    pub fn new(args: UpgradeToChunkLoaderArgs, callback: Option<fn(UpgradeToChunkLoaderResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for UpgradeToChunkLoader {
    fn execute(&self, world: &mut World) {
        todo!(); // TODO
    }
}

pub struct DowngradeFromChunkLoaderArgs {
    pub chunk_loader_entity_id: InstanceID<Entity>,
    pub chunk_loader_id: InstanceID<ChunkLoader>,
}
pub enum DowngradeFromChunkLoaderResult {
    Ok(()),
    Err(()),
}
pub struct DowngradeFromChunkLoader {
    args: DowngradeFromChunkLoaderArgs,
    callback: fn(DowngradeFromChunkLoaderResult),
}
impl DowngradeFromChunkLoader {
    pub fn new(args: DowngradeFromChunkLoaderArgs, callback: Option<fn(DowngradeFromChunkLoaderResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for DowngradeFromChunkLoader {
    fn execute(&self, world: &mut World) {
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