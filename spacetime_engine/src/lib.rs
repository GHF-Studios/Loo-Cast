pub mod camera;
pub mod camera_2d_bundle;
pub mod chunk;
pub mod chunk_actor;
pub mod chunk_loader;
pub mod command;
pub mod core;
pub mod entity;
pub mod math;
pub mod operation;
pub mod player;
pub mod sprite_bundle;

use bevy::{app::PluginGroupBuilder, prelude::*};
use camera_2d_bundle::structs::Camera2DBundle;
use chunk::components::Chunk;
use chunk_actor::components::ChunkActor;
use chunk_loader::components::ChunkLoader;
use command::structs::Command;
use operation::structs::Operation;
use player::components::Player;
use core::constants::*;
use std::any::Any;
use core::structs::*;
use core::traits::*;
use std::any::TypeId;
use std::collections::*;
use std::fmt::{Debug, Display};
use std::hash::*;

use camera::CameraPlugin;
use camera_2d_bundle::Camera2dBundlePlugin;
use chunk::ChunkPlugin;
use chunk_actor::ChunkActorPlugin;
use chunk_loader::ChunkLoaderPlugin;
use command::CommandPlugin;
use core::CorePlugin;
use entity::EntityPlugin;
use math::MathPlugin;
use operation::OperationPlugin;
use player::PlayerPlugin;
use sprite_bundle::SpriteBundlePlugin;


pub struct SpacetimeEnginePlugins;
impl PluginGroup for SpacetimeEnginePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(CameraPlugin)
            .add(Camera2dBundlePlugin)
            .add(ChunkPlugin)
            .add(ChunkActorPlugin)
            .add(ChunkLoaderPlugin)
            .add(CommandPlugin)	
            .add(CorePlugin)
            .add(EntityPlugin)
            .add(MathPlugin)
            .add(OperationPlugin)
            .add(PlayerPlugin)
            .add(SpriteBundlePlugin)
    }
}

#[derive(Deref, DerefMut)]
pub struct MainTypeRegistry(TypeRegistry);
impl MainTypeRegistry {
    pub fn new() -> Self {
        Self(TypeRegistry::new())
    }
}
impl LockingNodeData for MainTypeRegistry {
    fn pre_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        let root_path = AbsoluteLockingPath::new();
        let root_mutex = hierarchy.get_node_raw(root_path).unwrap();

        let core_path_segment = LockingPathSegment::new_string("core");
        let core_path = AbsoluteLockingPath::new().push(core_path_segment);
        let core = Type::new::<Core>("core");
        hierarchy.insert_branch::<MainTypeRegistry, Type, TypeData>(root_path, root_mutex, core_path_segment, core).unwrap();
        hierarchy.pre_startup::<Type>(core_path).unwrap();

        let operation_path_segment = LockingPathSegment::new_string("operation");
        let operation_path = AbsoluteLockingPath::new().push(operation_path_segment);
        let operation = Type::new::<Operation>("operation");
        hierarchy.insert_branch::<MainTypeRegistry, Type, TypeData>(root_path, root_mutex, operation_path_segment, operation).unwrap();
        hierarchy.pre_startup::<Type>(operation_path).unwrap();
        
        let command_path_segment = LockingPathSegment::new_string("command");
        let command_path = AbsoluteLockingPath::new().push(command_path_segment);
        let command = Type::new::<Command>("command");
        hierarchy.insert_branch::<MainTypeRegistry, Type, TypeData>(root_path, root_mutex, command_path_segment, command).unwrap();
        hierarchy.pre_startup::<Type>(command_path).unwrap();

        let entity_path_segment = LockingPathSegment::new_string("entity");
        let entity_path = AbsoluteLockingPath::new().push(entity_path_segment);
        let entity = Type::new::<Entity>("entity");
        hierarchy.insert_branch::<MainTypeRegistry, Type, TypeData>(root_path, root_mutex, entity_path_segment, entity).unwrap();
        hierarchy.pre_startup::<Type>(entity_path).unwrap();

        let chunk_path_segment = LockingPathSegment::new_string("chunk");
        let chunk_path = AbsoluteLockingPath::new().push(chunk_path_segment);
        let chunk = Type::new::<Chunk>("chunk");
        hierarchy.insert_branch::<MainTypeRegistry, Type, TypeData>(root_path, root_mutex, chunk_path_segment, chunk).unwrap();
        hierarchy.pre_startup::<Type>(chunk_path).unwrap();

        let chunk_actor_path_segment = LockingPathSegment::new_string("chunk_actor");
        let chunk_actor_path = AbsoluteLockingPath::new().push(chunk_actor_path_segment);
        let chunk_actor = Type::new::<ChunkActor>("chunk_actor");
        hierarchy.insert_branch::<MainTypeRegistry, Type, TypeData>(root_path, root_mutex, chunk_actor_path_segment, chunk_actor).unwrap();
        hierarchy.pre_startup::<Type>(chunk_actor_path).unwrap();

        let chunk_loader_path_segment = LockingPathSegment::new_string("chunk_loader");
        let chunk_loader_path = AbsoluteLockingPath::new().push(chunk_loader_path_segment);
        let chunk_loader = Type::new::<ChunkLoader>("chunk_loader");
        hierarchy.insert_branch::<MainTypeRegistry, Type, TypeData>(root_path, root_mutex, chunk_loader_path_segment, chunk_loader).unwrap();
        hierarchy.pre_startup::<Type>(chunk_loader_path).unwrap();

        let camera_path_segment = LockingPathSegment::new_string("camera");
        let camera_path = AbsoluteLockingPath::new().push(camera_path_segment);
        let camera = Type::new::<Camera>("camera");
        hierarchy.insert_branch::<MainTypeRegistry, Type, TypeData>(root_path, root_mutex, camera_path_segment, camera).unwrap();
        hierarchy.pre_startup::<Type>(camera_path).unwrap();

        let camera_2d_bundle_path_segment = LockingPathSegment::new_string("camera_2d_bundle");
        let camera_2d_bundle_path = AbsoluteLockingPath::new().push(camera_2d_bundle_path_segment);
        let camera_2d_bundle = Type::new::<Camera2DBundle>("camera_2d_bundle");
        hierarchy.insert_branch::<MainTypeRegistry, Type, TypeData>(root_path, root_mutex, camera_2d_bundle_path_segment, camera_2d_bundle).unwrap();
        hierarchy.pre_startup::<Type>(camera_2d_bundle_path).unwrap();

        let player_path_segment = LockingPathSegment::new_string("player");
        let player_path = AbsoluteLockingPath::new().push(player_path_segment);
        let player = Type::new::<Player>("player");
        hierarchy.insert_branch::<MainTypeRegistry, Type, TypeData>(root_path, root_mutex, player_path_segment, player).unwrap();
        hierarchy.pre_startup::<Type>(player_path).unwrap();

        let sprite_bundle_path_segment = LockingPathSegment::new_string("sprite_bundle");
        let sprite_bundle_path = AbsoluteLockingPath::new().push(sprite_bundle_path_segment);
        let sprite_bundle = Type::new::<SpriteBundle>("sprite_bundle");
        hierarchy.insert_branch::<MainTypeRegistry, Type, TypeData>(root_path, root_mutex, sprite_bundle_path_segment, sprite_bundle).unwrap();
        hierarchy.pre_startup::<Type>(sprite_bundle_path).unwrap();
    }

    fn startup(&mut self, hierarchy: &mut LockingHierarchy) {
        let core_path = AbsoluteLockingPath::new_from_literal("core");
        hierarchy.startup::<Type>(core_path).unwrap();

        let operation_path = AbsoluteLockingPath::new_from_literal("operation");
        hierarchy.startup::<Type>(operation_path).unwrap();

        let command_path = AbsoluteLockingPath::new_from_literal("command");
        hierarchy.startup::<Type>(command_path).unwrap();

        let entity_path = AbsoluteLockingPath::new_from_literal("entity");
        hierarchy.startup::<Type>(entity_path).unwrap();

        let chunk_path = AbsoluteLockingPath::new_from_literal("chunk");
        hierarchy.startup::<Type>(chunk_path).unwrap();

        let chunk_actor_path = AbsoluteLockingPath::new_from_literal("chunk_actor");
        hierarchy.startup::<Type>(chunk_actor_path).unwrap();

        let chunk_loader_path = AbsoluteLockingPath::new_from_literal("chunk_loader");
        hierarchy.startup::<Type>(chunk_loader_path).unwrap();

        let camera_path = AbsoluteLockingPath::new_from_literal("camera");
        hierarchy.startup::<Type>(camera_path).unwrap();

        let camera_2d_bundle_path = AbsoluteLockingPath::new_from_literal("camera_2d_bundle");
        hierarchy.startup::<Type>(camera_2d_bundle_path).unwrap();

        let player_path = AbsoluteLockingPath::new_from_literal("player");
        hierarchy.startup::<Type>(player_path).unwrap();

        let sprite_bundle_path = AbsoluteLockingPath::new_from_literal("sprite_bundle");
        hierarchy.startup::<Type>(sprite_bundle_path).unwrap();
    }

    fn post_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        let core_path = AbsoluteLockingPath::new_from_literal("core");
        hierarchy.post_startup::<Type>(core_path).unwrap();

        let operation_path = AbsoluteLockingPath::new_from_literal("operation");
        hierarchy.post_startup::<Type>(operation_path).unwrap();

        let command_path = AbsoluteLockingPath::new_from_literal("command");
        hierarchy.post_startup::<Type>(command_path).unwrap();

        let entity_path = AbsoluteLockingPath::new_from_literal("entity");
        hierarchy.post_startup::<Type>(entity_path).unwrap();

        let chunk_path = AbsoluteLockingPath::new_from_literal("chunk");
        hierarchy.post_startup::<Type>(chunk_path).unwrap();

        let chunk_actor_path = AbsoluteLockingPath::new_from_literal("chunk_actor");
        hierarchy.post_startup::<Type>(chunk_actor_path).unwrap();

        let chunk_loader_path = AbsoluteLockingPath::new_from_literal("chunk_loader");
        hierarchy.post_startup::<Type>(chunk_loader_path).unwrap();

        let camera_path = AbsoluteLockingPath::new_from_literal("camera");
        hierarchy.post_startup::<Type>(camera_path).unwrap();

        let camera_2d_bundle_path = AbsoluteLockingPath::new_from_literal("camera_2d_bundle");
        hierarchy.post_startup::<Type>(camera_2d_bundle_path).unwrap();

        let player_path = AbsoluteLockingPath::new_from_literal("player");
        hierarchy.post_startup::<Type>(player_path).unwrap();

        let sprite_bundle_path = AbsoluteLockingPath::new_from_literal("sprite_bundle");
        hierarchy.post_startup::<Type>(sprite_bundle_path).unwrap();
    }
}

#[derive(Deref, DerefMut)]
pub struct TypeDataRegistry(SingletonRegistry<TypeData>);
impl TypeDataRegistry {
    pub fn new() -> Self {
        Self(SingletonRegistry::new())
    }
}

#[derive(Deref, DerefMut)]
pub struct Type(TypeId, &'static str, #[deref]TypeDataRegistry);
impl Type {
    pub fn new<T: 'static + Send + Sync>(type_name: &'static str) -> Self {
        Self(TypeId::of::<T>(), type_name, TypeDataRegistry::new())
    }

    pub fn new_unchecked(type_id: TypeId, type_name: &'static str) -> Self {
        Self(type_id, type_name, TypeDataRegistry::new())
    }

    pub fn type_data_registry(&self) -> &TypeDataRegistry {
        &self.2
    }

    pub fn type_data_registry_mut(&mut self) -> &mut TypeDataRegistry {
        &mut self.2
    }
}
impl LockingNodeData for Type {
    fn pre_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        let core_path = AbsoluteLockingPath::new();
        let core_mutex = hierarchy.get_node_raw(core_path).unwrap();
        let type_path_segment = LockingPathSegment::new_string(self.1);
        let type_path = AbsoluteLockingPath::new().push(type_path_segment);
        let type_ = Type::new_unchecked(self.0, self.1);
        hierarchy.insert_branch::<Type, TypeData, TypeData>(core_path, core_mutex, type_path_segment, type_).unwrap();
        let type_ = hierarchy.get_mut::<Type>(type_path).unwrap();
        let type_binding = TYPE_BINDINGS.iter().find(|type_binding| type_binding.type_id == self.0).unwrap();
        (type_binding.type_pre_setup)(type_, hierarchy);
    }

    fn startup(&mut self, hierarchy: &mut LockingHierarchy) {
        let core_path = AbsoluteLockingPath::new();
        let type_path_segment = LockingPathSegment::new_string(self.1);
        let type_path = AbsoluteLockingPath::new().push(type_path_segment);
        let type_ = hierarchy.get_mut::<Type>(type_path).unwrap();
        let type_binding = TYPE_BINDINGS.iter().find(|type_binding| type_binding.type_id == self.0).unwrap();
        (type_binding.type_setup)(type_, hierarchy);
    }

    fn post_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        let core_path = AbsoluteLockingPath::new();
        let type_path_segment = LockingPathSegment::new_string(self.1);
        let type_path = AbsoluteLockingPath::new().push(type_path_segment);
        let type_ = hierarchy.get_mut::<Type>(type_path).unwrap();
        let type_binding = TYPE_BINDINGS.iter().find(|type_binding| type_binding.type_id == self.0).unwrap();
        (type_binding.type_post_setup)(type_, hierarchy);
    }
}

#[derive(Deref, DerefMut)]
pub struct TypeData(Box<dyn Any + Send + Sync>);
impl TypeData {
    pub fn new<T: 'static + Send + Sync + LockingNodeData>(data: T) -> Self {
        Self(Box::new(data))
    }

    pub fn data<T: 'static + Send + Sync + LockingNodeData>(&self) -> Option<&T> {
        self.0.downcast_ref::<T>()
    }

    pub fn data_mut<T: 'static + Send + Sync + LockingNodeData>(&mut self) -> Option<&mut T> {
        self.0.downcast_mut::<T>()
    }
}
impl LockingNodeData for TypeData {}
impl Singleton for TypeData {}


#[derive(Reflect)]
pub struct StringID {
    id: &'static str,
}
impl RegistryKey for StringID {
    type ID = &'static str;

    fn new(id: &'static str) -> Self {
        if id.is_empty() {
            panic!("String ID cannot be empty!");
        }

        if id.chars().first().unwrap().is_numeric() {
            panic!("String ID '{}' cannot start with a number!", id);
        }

        for reserved_id in RESERVED_STRING_IDS.iter() {
            if id.eq_ignore_ascii_case(reserved_id) {
                panic!("Cannot use reserved string ID '{}'!", reserved_id);
            }
        }

        Self { 
            id,
        }
    }

    fn get(&self) -> &'static str {
        self.id
    }
}
impl StaticInstanceRegistryKey for StringID {}
impl Default for StringID {
    fn default() -> Self {
        Self {
            id: "",
        }
    }
}
impl Debug for StringID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.id)
    }
}
impl Display for StringID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.id)
    }
}
impl Clone for StringID {
    fn clone(&self) -> Self { *self }
}
impl Copy for StringID {
}
impl PartialEq for StringID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for StringID {
}
impl Hash for StringID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Reflect)]
pub struct NumericID {
    id: u64,
}
impl RegistryKey for NumericID {
    type ID = u64;

    fn new(id: u64) -> Self {
        for reserved_id in RESERVED_NUMERIC_IDS.iter() {
            if id == *reserved_id {
                panic!("Cannot use reserved numeric ID '{}'!", reserved_id);
            }
        }

        Self {
            id,
        }
    }

    fn get(&self) -> u64 {
        self.id
    }
}
impl DynamicInstanceRegistryKey for NumericID {}
impl Default for NumericID {
    fn default() -> Self {
        Self {
            id: 0,
        }
    }
}
impl Debug for NumericID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.id)
    }
}
impl Display for NumericID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.id)
    }
}
impl Clone for NumericID {
    fn clone(&self) -> Self { *self }
}
impl Copy for NumericID {
}
impl PartialEq for NumericID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for NumericID {
}
impl Hash for NumericID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub struct StaticInstanceRegistry<K: StaticInstanceRegistryKey, V: InstanceRegistryValue> {
    registered: HashSet<K>,
    managed: HashMap<K, V>,
}
impl<K: StaticInstanceRegistryKey, V: InstanceRegistryValue> StaticInstanceRegistry<K, V> {
    pub fn new() -> Self {
        Self {
            registered: HashSet::new(),
            managed: HashMap::new(),
        }
    }

    pub fn register(&mut self, key: K) {
        if self.registered.contains(&key) {
            panic!("Key '{:?}' is already registered!", key);
        }

        self.registered.insert(key);
    }

    pub fn unregister(&mut self, key: K) {
        if !self.registered.contains(&key) {
            panic!("Key '{:?}' is invalid!", key);
        }

        if self.managed.contains_key(&key) {
            panic!("Entry '{:?}' is still managed!", key);
        }

        self.registered.retain(|other_key| key != *other_key);
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

    pub fn registered(&self) -> &HashSet<K> {
        &self.registered
    }

    pub fn managed(&self) -> &HashMap<K, V> {
        &self.managed
    }

    pub fn is_registered(&self, key: K) -> bool {
        self.registered.contains(&key)
    }

    pub fn is_managed(&self, key: K) -> bool {
        self.managed.contains_key(&key)
    }
}
impl<K: StaticInstanceRegistryKey, V: InstanceRegistryValue> Debug for StaticInstanceRegistry<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let registered_keys = self.registered.iter().map(|key| {
            key.to_string()
        }).collect::<Vec<String>>().join(", ");

        let managed_keys = self.managed.iter().map(|(key, _)| {
            key.to_string()
        }).collect::<Vec<String>>().join(", ");

        write!(f, "StaticInstanceRegistry {{ registered[ {:?} ], managed[ {:?} ] }}", registered_keys, managed_keys)
    }
}
impl<K: StaticInstanceRegistryKey, V: InstanceRegistryValue> Display for StaticInstanceRegistry<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let registered_keys = self.registered.iter().map(|key| {
            key.to_string()
        }).collect::<Vec<String>>().join(", ");

        let managed_keys = self.managed.iter().map(|(key, _)| {
            key.to_string()
        }).collect::<Vec<String>>().join(", ");

        write!(f, "StaticInstanceRegistry {{ registered[ {:?} ], managed[ {:?} ] }}", registered_keys, managed_keys)
    }
}

pub struct DynamicInstanceRegistry<K: DynamicInstanceRegistryKey, V: InstanceRegistryValue> {
    registered: HashSet<K>,
    managed: HashMap<K, V>,
    next_key: K,
    recycled_keys: Vec<K>,
}
impl<K: DynamicInstanceRegistryKey, V: InstanceRegistryValue> DynamicInstanceRegistry<K, V> {
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

    pub fn registered(&self) -> &HashSet<K> {
        &self.registered
    }

    pub fn managed(&self) -> &HashMap<K, V> {
        &self.managed
    }

    pub fn is_registered(&self, key: K) -> bool {
        self.registered.contains(&key)
    }

    pub fn is_managed(&self, key: K) -> bool {
        self.managed.contains_key(&key)
    }
}
impl<K: DynamicInstanceRegistryKey, V: InstanceRegistryValue> Debug for DynamicInstanceRegistry<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let registered_keys = self.registered.iter().map(|key| {
            key.to_string()
        }).collect::<Vec<String>>().join(", ");

        let managed_keys = self.managed.iter().map(|(key, _)| {
            key.to_string()
        }).collect::<Vec<String>>().join(", ");

        write!(f, "DynamicInstanceRegistry {{ registered[ {:?} ], managed[ {:?} ] }}", registered_keys, managed_keys)
    }
}
impl<K: DynamicInstanceRegistryKey, V: InstanceRegistryValue> Display for DynamicInstanceRegistry<K, V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let registered_keys = self.registered.iter().map(|key| {
            key.to_string()
        }).collect::<Vec<String>>().join(", ");

        let managed_keys = self.managed.iter().map(|(key, _)| {
            key.to_string()
        }).collect::<Vec<String>>().join(", ");

        write!(f, "DynamicInstanceRegistry {{ registered[ {:?} ], managed[ {:?} ] }}", registered_keys, managed_keys)
    }
}

pub struct TypeBinding {
    pub type_name: &'static str,
    pub type_id: TypeId,
    pub type_pre_setup: fn(&mut Type, hierarchy: &mut LockingHierarchy),
    pub type_setup: fn(&mut Type, hierarchy: &mut LockingHierarchy),
    pub type_post_setup: fn(&mut Type, hierarchy: &mut LockingHierarchy),
}

pub struct TypeRegistry {
    registered: HashSet<TypeId>,
    managed: HashMap<TypeId, Type>,
}
impl TypeRegistry {
    pub fn new() -> Self {
        Self {
            registered: HashSet::new(),
            managed: HashMap::new(),
        }
    }

    pub fn register<T: 'static + Send + Sync>(&mut self) {
        let type_id = TypeId::of::<T>();

        if self.registered.contains(&type_id) {
            panic!("Type '{:?}' is already registered!", type_id);
        }

        self.registered.insert(type_id);
    }

    pub fn unregister<T: 'static + Send + Sync>(&mut self) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is still managed!", type_id);
        }

        self.registered.retain(|other_type_id| type_id != *other_type_id);
    }

    pub fn manage<T: 'static + Send + Sync>(&mut self) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is already managed!", type_id);
        }

        self.managed.insert(type_id, Type::new::<T>(TYPE_BINDINGS.iter().find(|(type_name, other_type_id)| {
            type_id == *other_type_id
        }).unwrap().0));
    }

    pub fn unmanage<T: 'static + Send + Sync>(&mut self) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is already unmanaged!", type_id);
        }

        self.managed.remove(&type_id);
    }

    pub fn get<T: 'static + Send + Sync>(&self) -> Option<&Type> {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is not managed!", type_id);
        }

        self.managed.get(&type_id)
    }

    pub fn get_mut<T: 'static + Send + Sync>(&mut self) -> Option<&mut Type> {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is not managed!", type_id);
        }

        self.managed.get_mut(&type_id)
    }

    pub fn registered(&self) -> &HashSet<TypeId> {
        &self.registered
    }

    pub fn managed(&self) -> &HashMap<TypeId, Type> {
        &self.managed
    }

    pub fn is_registered<T: 'static + Send + Sync>(&self) -> bool {
        self.registered.contains(&TypeId::of::<T>())
    }

    pub fn is_managed<T: 'static + Send + Sync>(&self) -> bool {
        self.managed.contains_key(&TypeId::of::<T>())
    }
}
impl Debug for TypeRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let registered_types = self.registered.iter().map(|type_id| {
            type_id.to_string()
        }).collect::<Vec<String>>().join(", ");

        let managed_types = self.managed.iter().map(|(type_id, _)| {
            type_id.to_string()
        }).collect::<Vec<String>>().join(", ");

        write!(f, "TypeRegistry{{ registered[ {:?} ], managed[ {:?} ] }}", registered_types, managed_types)
    }
}
impl Display for TypeRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let registered_types = self.registered.iter().map(|type_id| {
            type_id.to_string()
        }).collect::<Vec<String>>().join(", ");

        let managed_types = self.managed.iter().map(|(type_id, _)| {
            type_id.to_string()
        }).collect::<Vec<String>>().join(", ");

        write!(f, "TypeRegistry {{ registered[ {:?} ], managed[ {:?} ] }}", registered_types, managed_types)
    }
}

pub struct SingletonRegistry<T: 'static + Singleton> {
    registered: HashSet<TypeId>,
    managed: HashMap<TypeId, T>,
}
impl<T: 'static + Singleton> SingletonRegistry<T> {
    pub fn new() -> Self {
        Self {
            registered: HashSet::new(),
            managed: HashMap::new(),
        }
    }

    pub fn register(&mut self) {
        let type_id = TypeId::of::<T>();

        if self.registered.contains(&type_id) {
            panic!("Type '{:?}' is already registered!", type_id);
        }

        self.registered.insert(type_id);
    }

    pub fn unregister(&mut self) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is still managed!", type_id);
        }

        self.registered.retain(|other_type_id| type_id != *other_type_id);
    }

    pub fn manage(&mut self, singleton: T) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is already managed!", type_id);
        }

        self.managed.insert(type_id, singleton);
    }

    pub fn unmanage(&mut self) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is already unmanaged!", type_id);
        }

        self.managed.remove(&type_id);
    }

    pub fn get(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is not managed!", type_id);
        }

        self.managed.get(&type_id)
    }

    pub fn get_mut(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is not managed!", type_id);
        }

        self.managed.get_mut(&type_id)
    }

    pub fn registered(&self) -> &HashSet<TypeId> {
        &self.registered
    }

    pub fn managed(&self) -> &HashMap<TypeId, T> {
        &self.managed
    }

    pub fn is_registered(&self) -> bool {
        self.registered.contains(&TypeId::of::<T>())
    }

    pub fn is_managed(&self) -> bool {
        self.managed.contains_key(&TypeId::of::<T>())
    }
}
impl<T: 'static + Singleton> Debug for SingletonRegistry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let registered_types = self.registered.iter().map(|type_id| {
            type_id.to_string()
        }).collect::<Vec<String>>().join(", ");

        let managed_types = self.managed.iter().map(|(type_id, _)| {
            type_id.to_string()
        }).collect::<Vec<String>>().join(", ");

        write!(f, "SingletonRegistry {{ registered[ {:?} ], managed[ {:?} ] }}", registered_types, managed_types)
    }
}
impl<T: 'static + Singleton> Display for SingletonRegistry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let registered_types = self.registered.iter().map(|type_id| {
            type_id.to_string()
        }).collect::<Vec<String>>().join(", ");

        let managed_types = self.managed.iter().map(|(type_id, _)| {
            type_id.to_string()
        }).collect::<Vec<String>>().join(", ");

        write!(f, "SingletonRegistry {{ registered[ {:?} ], managed[ {:?} ] }}", registered_types, managed_types)
    }
}
