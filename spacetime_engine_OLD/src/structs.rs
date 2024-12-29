use bevy::prelude::*;
use std::any::{Any, TypeId};
use std::collections::{HashSet, HashMap};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard};

use crate::{dispatch_cmd, dispatch_cmds};
use crate::components::*;
use crate::enums::*;
use crate::errors::*;
use crate::commands::*;
use crate::hooks::*;
use crate::systems::*;
use crate::constants::*;
use crate::singletons::*;
use crate::traits::*;

use crate::camera::CameraPlugin;
//use crate::camera_2d_bundle::Camera2dBundlePlugin;
//use crate::chunk::ChunkPlugin;
//use crate::chunk_actor::ChunkActorPlugin;
//use crate::chunk_loader::ChunkLoaderPlugin;
//use crate::core::CorePlugin;
//use crate::entity::EntityPlugin;
//use crate::math::MathPlugin;
//use crate::player::PlayerPlugin;
//use crate::sprite_bundle::SpriteBundlePlugin;

pub(crate) struct Root;
impl LockingNodeData for Root {
    fn on_insert(&mut self, hierarchy: &mut LockingHierarchy) {
        let root_path = AbsoluteLockingPath::new();
        let root_mutex = hierarchy.try_get_node_raw(root_path.clone()).unwrap();

        let camera_path_segment = LockingPathSegment::new_string("camera");
        hierarchy.insert(root_path, root_mutex, camera_path_segment, CameraPlugin).unwrap();

//        let camera_2d_bundle_path_segment = LockingPathSegment::new_string("camera_2d_bundle");
//        hierarchy.insert(root_path.clone(), root_mutex.clone(), camera_2d_bundle_path_segment, Camera2dBundlePlugin).unwrap();
//
//        let chunk_path_segment = LockingPathSegment::new_string("chunk");
//        hierarchy.insert(root_path.clone(), root_mutex.clone(), chunk_path_segment, ChunkPlugin).unwrap();
//
//        let chunk_actor_path_segment = LockingPathSegment::new_string("chunk_actor");
//        hierarchy.insert(root_path.clone(), root_mutex.clone(), chunk_actor_path_segment, ChunkActorPlugin).unwrap();
//
//        let chunk_loader_path_segment = LockingPathSegment::new_string("chunk_loader");
//        hierarchy.insert(root_path.clone(), root_mutex.clone(), chunk_loader_path_segment, ChunkLoaderPlugin).unwrap();
//
//        let entity_path_segment = LockingPathSegment::new_string("entity");
//        hierarchy.insert(root_path.clone(), root_mutex.clone(), entity_path_segment, EntityPlugin).unwrap();
//
//        let math_path_segment = LockingPathSegment::new_string("math");
//        hierarchy.insert(root_path.clone(), root_mutex.clone(), math_path_segment, MathPlugin).unwrap();
//
//        let player_path_segment = LockingPathSegment::new_string("player");
//        hierarchy.insert(root_path.clone(), root_mutex.clone(), player_path_segment, PlayerPlugin).unwrap();
//
//        let sprite_bundle_path_segment = LockingPathSegment::new_string("sprite_bundle");
//        hierarchy.insert(root_path.clone(), root_mutex.clone(), sprite_bundle_path_segment, SpriteBundlePlugin).unwrap();

        dispatch_cmds!(async, sequence, [
            ("core.command_types.spawn_main_camera"),
            ("core.command_types.spawn_start_chunks", 2),
            ("core.command_types.spawn_start_chunk_actors", 2),
        ]);
    }

    fn on_remove(&mut self, hierarchy: &mut LockingHierarchy) {
        dispatch_cmds!(async, sequence, [
            ("core.command_types.despawn_start_chunk_actors", 2),
            ("core.command_types.despawn_start_chunks", 2),
            ("core.command_types.despawn_main_camera"),
        ]);

        let root_path = AbsoluteLockingPath::new();
        let root_mutex = hierarchy.try_get_node_raw(root_path.clone()).unwrap();

        let camera_path_segment = LockingPathSegment::new_string("camera");
        let camera_path = root_path.clone().push(camera_path_segment).unwrap();
        hierarchy.remove(camera_path).unwrap();

//        let camera_2d_bundle_path_segment = LockingPathSegment::new_string("camera_2d_bundle");
//        let camera_2d_bundle_path = root_path.clone().push(camera_2d_bundle_path_segment).unwrap();
//        hierarchy.remove(camera_2d_bundle_path).unwrap();
//
//        let chunk_path_segment = LockingPathSegment::new_string("chunk");
//        let chunk_path = root_path.clone().push(chunk_path_segment).unwrap();
//        hierarchy.remove(chunk_path).unwrap();
//
//        let chunk_actor_path_segment = LockingPathSegment::new_string("chunk_actor");
//        let chunk_actor_path = root_path.clone().push(chunk_actor_path_segment).unwrap();
//        hierarchy.remove(chunk_actor_path).unwrap();
//
//        let chunk_loader_path_segment = LockingPathSegment::new_string("chunk_loader");
//        let chunk_loader_path = root_path.clone().push(chunk_loader_path_segment).unwrap();
//        hierarchy.remove(chunk_loader_path).unwrap();
//
//        let entity_path_segment = LockingPathSegment::new_string("entity");
//        let entity_path = root_path.clone().push(entity_path_segment).unwrap();
//        hierarchy.remove(entity_path).unwrap();
//
//        let math_path_segment = LockingPathSegment::new_string("math");
//        let math_path = root_path.clone().push(math_path_segment).unwrap();
//        hierarchy.remove(math_path).unwrap();
//
//        let player_path_segment = LockingPathSegment::new_string("player");
//        let player_path = root_path.clone().push(player_path_segment).unwrap();
//        hierarchy.remove(player_path).unwrap();
//
//        let sprite_bundle_path_segment = LockingPathSegment::new_string("sprite_bundle");
//        let sprite_bundle_path = root_path.clone().push(sprite_bundle_path_segment).unwrap();
//        hierarchy.remove(sprite_bundle_path).unwrap();
    }
}

pub struct OperationQueue {
    queue: Vec<Box<dyn DynOperation>>,
}
impl OperationQueue {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            queue: Vec::new(),
        }
    }

    pub fn add_operation(&mut self, operation: Box<dyn DynOperation>) {
        self.queue.push(operation);
    }

    pub fn remove_operations(&mut self) -> Vec<Box<dyn DynOperation>> {
        self.queue.drain(..).collect()
    }
}

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

        if id.chars().next().unwrap().is_numeric() {
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

pub enum LockingPathSegment {
    Root,
    String(StringID),
    Numeric(NumericID),
}
impl LockingPathSegment {
    pub fn new_root() -> Self {
        LockingPathSegment::Root
    }

    pub fn new_string(id: &'static str) -> Self {
        LockingPathSegment::String(StringID::new(id))
    }

    pub fn new_number(id: u64) -> Self {
        LockingPathSegment::Numeric(NumericID::new(id))
    }
}
impl Debug for LockingPathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingPathSegment::Root => write!(f, "root"),
            LockingPathSegment::String(id) => write!(f, "{}", id),
            LockingPathSegment::Numeric(id) => write!(f, "{}", id),
        }
    }
}
impl Display for LockingPathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingPathSegment::Root => write!(f, "root"),
            LockingPathSegment::String(id) => write!(f, "{}", id),
            LockingPathSegment::Numeric(id) => write!(f, "{}", id),
        }
    }
}
impl Clone for LockingPathSegment {
    fn clone(&self) -> Self {
        match self {
            LockingPathSegment::Root => LockingPathSegment::Root,
            LockingPathSegment::String(id) => LockingPathSegment::String(id.clone()),
            LockingPathSegment::Numeric(id) => LockingPathSegment::Numeric(id.clone()),
        }
    }
}
impl Copy for LockingPathSegment {}
impl PartialEq for LockingPathSegment {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LockingPathSegment::String(id), LockingPathSegment::String(other_id)) => id == other_id,
            (LockingPathSegment::Numeric(id), LockingPathSegment::Numeric(other_id)) => id == other_id,
            _ => false,
        }
    }
}
impl Eq for LockingPathSegment {}
impl Hash for LockingPathSegment {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            LockingPathSegment::Root => "Root".hash(state),
            LockingPathSegment::String(id) => id.hash(state),
            LockingPathSegment::Numeric(id) => id.hash(state),
        }
    }
}

pub struct RelativeLockingPath {
    segments: Vec<LockingPathSegment>,
}
impl RelativeLockingPath {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }
}
impl LockingPath for RelativeLockingPath {
    fn segments(&self) -> &Vec<LockingPathSegment> {
        &self.segments
    }
    
    fn segments_mut(&mut self) -> &mut Vec<LockingPathSegment> {
        &mut self.segments
    }

    fn push(mut self, segment: LockingPathSegment) -> Result<RelativeLockingPath, String> {
        let last_segment = self.segments.last();
        
        match last_segment {
            Some(LockingPathSegment::Root) => {
                unreachable!()
            },
            Some(LockingPathSegment::String(_)) => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(self)
                    },
                }
            },
            Some(LockingPathSegment::Numeric(_)) => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(self)
                    },
                }
            },
            None => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(self)
                    },
                }
            },
        }
    }

    fn pop(mut self) -> Result<(RelativeLockingPath, LockingPathSegment), String> {
        match self.segments.pop() {
            Some(segment) => Ok((self, segment)),
            None => Err("Cannot pop segment from empty relative path!".to_string()),
        }
    }
}
impl Debug for RelativeLockingPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();

        for segment in &self.segments {
            path.push_str(&format!("{}/", segment));
        }

        write!(f, "{}", path)
    }
}
impl Display for RelativeLockingPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();

        for segment in &self.segments {
            path.push_str(&format!("{}/", segment));
        }

        write!(f, "{}", path)
    }
}
impl Clone for RelativeLockingPath {
    fn clone(&self) -> Self {
        Self {
            segments: self.segments.clone(),
        }
    }
}
impl PartialEq for RelativeLockingPath {
    fn eq(&self, other: &Self) -> bool {
        self.segments == other.segments
    }
}
impl Eq for RelativeLockingPath {}
impl Hash for RelativeLockingPath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.segments.hash(state);
    }
}

pub struct AbsoluteLockingPath {
    segments: Vec<LockingPathSegment>,
}
impl AbsoluteLockingPath {
    pub fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }
    pub fn new_from_literal(path: &'static str) -> Self {
        let mut segments = Vec::new();

        for segment in path.split('.') {
            if let Ok(id) = segment.parse::<u64>() {
                segments.push(LockingPathSegment::Numeric(NumericID::new(id)));
            } else {
                segments.push(LockingPathSegment::String(StringID::new(segment)));
            }
        }

        Self {
            segments,
        }
    }
}
impl LockingPath for AbsoluteLockingPath {
    fn segments(&self) -> &Vec<LockingPathSegment> {
        &self.segments
    }

    fn segments_mut(&mut self) -> &mut Vec<LockingPathSegment> {
        &mut self.segments
    }

    fn push(mut self, segment: LockingPathSegment) -> Result<AbsoluteLockingPath, String> {
        let last_segment = self.segments.last();

        match last_segment {
            Some(LockingPathSegment::Root) => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(self)
                    },
                }
            },
            Some(LockingPathSegment::String(_)) => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(self)
                    },
                }
            },
            Some(LockingPathSegment::Numeric(_)) => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(self)
                    },
                }
            },
            None => {
                unreachable!()
            }
        }
    }

    fn pop(mut self) -> Result<(AbsoluteLockingPath, LockingPathSegment), String> {
        if self.segments.len() == 1 {
            return Err("Cannot pop root segment from absolute path!".to_string());
        }

        match self.segments.pop() {
            Some(segment) => Ok((self, segment)),
            None => unreachable!(),
        }
    }
}
impl Debug for AbsoluteLockingPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();

        for segment in &self.segments {
            path.push_str(&format!("{}/", segment));
        }

        write!(f, "{}", path)
    }
}
impl Display for AbsoluteLockingPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut path = String::new();

        for segment in &self.segments {
            path.push_str(&format!("{}/", segment));
        }

        write!(f, "{}", path)
    }
}
impl Clone for AbsoluteLockingPath {
    fn clone(&self) -> Self {
        Self {
            segments: self.segments.clone(),
        }
    }
}
impl PartialEq for AbsoluteLockingPath {
    fn eq(&self, other: &Self) -> bool {
        self.segments == other.segments
    }
}
impl Eq for AbsoluteLockingPath {}
impl Hash for AbsoluteLockingPath {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.segments.hash(state);
    }
}

pub(crate) struct LockingType(TypeBinding, LockingTypeDataRegistry);
impl LockingType {
    pub fn new<T: 'static + Send + Sync>(type_binding: TypeBinding) -> Self {
        if type_binding.type_id != TypeId::of::<T>() {
            panic!("Type ID '{:?}' does not match type '{:?}'!", type_binding.type_id, TypeId::of::<T>());
        }

        Self(type_binding, LockingTypeDataRegistry::new())
    }

    pub fn new_unchecked(type_binding: TypeBinding) -> Self {
        Self(type_binding, LockingTypeDataRegistry::new())
    }

    pub fn type_binding(&self) -> &TypeBinding {
        &self.0
    }

    pub fn type_data_registry(&self) -> &LockingTypeDataRegistry {
        &self.1
    }

    pub fn type_data_registry_mut(&mut self) -> &mut LockingTypeDataRegistry {
        &mut self.1
    }
}

pub(crate) struct LockingTypeData(Box<dyn Any + Send + Sync>);
impl LockingTypeData {
    pub fn new<T: 'static + LockingNodeData>(data_type_binding: TypeBinding, data: T) -> Self {
        Self(Box::new(data))
    }

    pub fn data<T: 'static + LockingNodeData>(&self) -> Option<&T> {
        self.0.downcast_ref::<T>()
    }

    pub fn data_mut<T: 'static + LockingNodeData>(&mut self) -> Option<&mut T> {
        self.0.downcast_mut::<T>()
    }
}
impl LockingTypeDataTrait for LockingTypeData {}

pub(crate) struct LockingTypeRegistry {
    registered: HashSet<TypeId>,
    managed: HashMap<TypeId, LockingType>,
}
impl LockingTypeRegistry {
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

    pub fn manage<T: 'static + Send + Sync>(&mut self, type_binding: TypeBinding) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is already managed!", type_id);
        }

        self.managed.insert(type_id, LockingType::new::<T>(type_binding));
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

    pub fn get<T: 'static + Send + Sync>(&self) -> Option<&LockingType> {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is not managed!", type_id);
        }

        self.managed.get(&type_id)
    }

    pub fn get_mut<T: 'static + Send + Sync>(&mut self) -> Option<&mut LockingType> {
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

    pub fn managed(&self) -> &HashMap<TypeId, LockingType> {
        &self.managed
    }

    pub fn is_registered<T: 'static + Send + Sync>(&self) -> bool {
        self.registered.contains(&TypeId::of::<T>())
    }

    pub fn is_managed<T: 'static + Send + Sync>(&self) -> bool {
        self.managed.contains_key(&TypeId::of::<T>())
    }
}
impl Debug for LockingTypeRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let registered_types = self.registered.iter().map(|type_id| {
            format!("{:?}", type_id)
        }).collect::<Vec<String>>().join(", ");

        let managed_types = self.managed.iter().map(|(type_id, _)| {
            format!("{:?}", type_id)
        }).collect::<Vec<String>>().join(", ");

        write!(f, "TypeRegistry{{ registered[ {:?} ], managed[ {:?} ] }}", registered_types, managed_types)
    }
}
impl Display for LockingTypeRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let registered_types = self.registered.iter().map(|type_id| {
            format!("{:?}", type_id)
        }).collect::<Vec<String>>().join(", ");

        let managed_types = self.managed.iter().map(|(type_id, _)| {
            format!("{:?}", type_id)
        }).collect::<Vec<String>>().join(", ");

        write!(f, "TypeRegistry {{ registered[ {:?} ], managed[ {:?} ] }}", registered_types, managed_types)
    }
}

pub(crate) struct LockingTypeDataRegistry {
    registered: HashSet<TypeId>,
    managed: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}
impl LockingTypeDataRegistry {
    pub fn new() -> Self {
        Self {
            registered: HashSet::new(),
            managed: HashMap::new(),
        }
    }

    pub fn register<T: 'static + LockingTypeDataTrait>(&mut self) {
        let type_id = TypeId::of::<T>();

        if self.registered.contains(&type_id) {
            panic!("Type '{:?}' is already registered!", type_id);
        }

        self.registered.insert(type_id);
    }

    pub fn unregister<T: 'static + LockingTypeDataTrait>(&mut self) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is still managed!", type_id);
        }

        self.registered.retain(|other_type_id| type_id != *other_type_id);
    }

    pub fn manage<T: 'static + LockingTypeDataTrait>(&mut self, type_data: T) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is already managed!", type_id);
        }

        let type_data: Box<dyn Any + Send + Sync> = Box::new(type_data) as Box<dyn Any + Send + Sync>;

        self.managed.insert(type_id, type_data);
    }

    pub fn unmanage<T: 'static + LockingTypeDataTrait>(&mut self) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is already unmanaged!", type_id);
        }

        self.managed.remove(&type_id);
    }

    pub fn get<T: 'static + LockingTypeDataTrait>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is not managed!", type_id);
        }

        match self.managed.get(&type_id) {
            Some(type_data) => type_data.downcast_ref::<T>(),
            None => None,
        }
    }

    pub fn get_mut<T: 'static + LockingTypeDataTrait>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is not managed!", type_id);
        }

        match self.managed.get_mut(&type_id) {
            Some(type_data) => type_data.downcast_mut::<T>(),
            None => None,
        }
    }

    pub fn registered(&self) -> &HashSet<TypeId> {
        &self.registered
    }

    pub fn managed(&self) -> &HashMap<TypeId, Box<dyn Any + Send + Sync>> {
        &self.managed
    }

    pub fn is_registered<T: 'static + LockingTypeDataTrait>(&self) -> bool {
        self.registered.contains(&TypeId::of::<T>())
    }

    pub fn is_managed<T: 'static + LockingTypeDataTrait>(&self) -> bool {
        self.managed.contains_key(&TypeId::of::<T>())
    }
}
impl Debug for LockingTypeDataRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let registered_types = self.registered.iter().map(|type_id| {
            format!("{:?}", type_id)
        }).collect::<Vec<String>>().join(", ");

        let managed_types = self.managed.iter().map(|(type_id, _)| {
            format!("{:?}", type_id)
        }).collect::<Vec<String>>().join(", ");

        write!(f, "SingletonRegistry {{ registered[ {:?} ], managed[ {:?} ] }}", registered_types, managed_types)
    }
}
impl Display for LockingTypeDataRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let registered_types = self.registered.iter().map(|type_id| {
            format!("{:?}", type_id)
        }).collect::<Vec<String>>().join(", ");

        let managed_types = self.managed.iter().map(|(type_id, _)| {
            format!("{:?}", type_id)
        }).collect::<Vec<String>>().join(", ");

        write!(f, "SingletonRegistry {{ registered[ {:?} ], managed[ {:?} ] }}", registered_types, managed_types)
    }
}

pub struct TypeBinding {
    pub type_name: &'static str,
    pub type_id: TypeId,
    pub type_pre_setup: fn(hierarchy: &mut LockingHierarchy),
    pub type_setup: fn(hierarchy: &mut LockingHierarchy),
    pub type_post_setup: fn(hierarchy: &mut LockingHierarchy),
}

#[derive(Deref, DerefMut)]
pub struct Type<T: 'static + Send + Sync>(#[deref]LockingType, PhantomData<T>);
impl<T: 'static + Send + Sync> Type<T> {
    pub fn new(type_binding: TypeBinding) -> Self {
        Self(LockingType::new::<T>(type_binding), PhantomData)
    }

    pub fn new_unchecked(type_binding: TypeBinding) -> Self {
        Self(LockingType::new_unchecked(type_binding), PhantomData)
    }
}

#[derive(Deref, DerefMut)]
pub struct TypeData<T: 'static + Send + Sync + LockingNodeData>(#[deref]LockingTypeData, PhantomData<T>);
impl<T: 'static + Send + Sync + LockingNodeData> TypeData<T> {
    pub fn new(data_type_binding: TypeBinding, data: T) -> Self {
        Self(LockingTypeData::new::<T>(data_type_binding, data), PhantomData)
    }
}

#[derive(Deref, DerefMut)]
pub struct TypeRegistry(LockingTypeRegistry);
impl TypeRegistry {
    pub fn new() -> Self {
        Self(LockingTypeRegistry::new())
    }
}

#[derive(Deref, DerefMut)]
pub struct TypeDataRegistry(LockingTypeDataRegistry);
impl TypeDataRegistry {
    pub fn new() -> Self {
        Self(LockingTypeDataRegistry::new())
    }
}

pub(in super) enum LockingNodeMetadata {
    Root {
        state: LockingState,
        children: HashMap<LockingPathSegment, Arc<Mutex<LockingNode>>>,
    },
    Branch {
        absolute_path: AbsoluteLockingPath,
        path_segment: LockingPathSegment,
        state: LockingState,
        parent_type_id: TypeId,
        parent: (LockingPathSegment, Arc<Mutex<LockingNode>>),
        children: HashMap<LockingPathSegment, Arc<Mutex<LockingNode>>>,
    },
    Leaf {
        absolute_path: AbsoluteLockingPath,
        path_segment: LockingPathSegment,
        state: LockingState,
        parent_type_id: TypeId,
        parent: (LockingPathSegment, Arc<Mutex<LockingNode>>),
    },
}
impl LockingNodeMetadata {
    pub fn get_absolute_path(&self) -> AbsoluteLockingPath {
        match self {
            LockingNodeMetadata::Root { .. } => AbsoluteLockingPath::new(),
            LockingNodeMetadata::Branch { absolute_path, .. } => absolute_path.clone(),
            LockingNodeMetadata::Leaf { absolute_path, .. } => absolute_path.clone(),
        }
    }

    pub fn get_path_segment(&self) -> LockingPathSegment {
        match self {
            LockingNodeMetadata::Root { .. } => LockingPathSegment::Root,
            LockingNodeMetadata::Branch { path_segment, .. } => path_segment.clone(),
            LockingNodeMetadata::Leaf { path_segment, .. } => path_segment.clone(),
        }
    }

    pub fn get_state(&self) -> &LockingState {
        match self {
            LockingNodeMetadata::Root { state, .. } => state,
            LockingNodeMetadata::Branch { state, .. } => state,
            LockingNodeMetadata::Leaf { state, .. } => state,
        }
    }

    pub fn get_state_mut(&mut self) -> &mut LockingState {
        match self {
            LockingNodeMetadata::Root { state, .. } => state,
            LockingNodeMetadata::Branch { state, .. } => state,
            LockingNodeMetadata::Leaf { state, .. } => state,
        }
    }

    pub fn get_parent_type_id(&self) -> Option<TypeId> {
        match self {
            LockingNodeMetadata::Root { .. } => None,
            LockingNodeMetadata::Branch { parent_type_id, .. } => Some(*parent_type_id),
            LockingNodeMetadata::Leaf { parent_type_id, .. } => Some(*parent_type_id),
        }
    }

    pub fn get_parent(&self) -> Option<(LockingPathSegment, Arc<Mutex<LockingNode>>)> {
        match self {
            LockingNodeMetadata::Root { .. } => None,
            LockingNodeMetadata::Branch { parent, .. } => Some(parent.clone()),
            LockingNodeMetadata::Leaf { parent, .. } => Some(parent.clone()),
        }
    }

    pub fn get_children(&self) -> Option<&HashMap<LockingPathSegment, Arc<Mutex<LockingNode>>>> {
        match self {
            LockingNodeMetadata::Root { children, .. } => Some(children),
            LockingNodeMetadata::Branch { children, .. } => Some(children),
            LockingNodeMetadata::Leaf { .. } => None,
        }
    }
}
impl Debug for LockingNodeMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingNodeMetadata::Root { state, children } => {
                let children_string = children.keys().map(|child| {
                    child.to_string()
                }).collect::<Vec<String>>().join(", ");

                write!(f, "Root {{ state[ {:?} ], children[ {:?} ] }}", state, children_string)
            },
            LockingNodeMetadata::Branch { absolute_path, state, parent_type_id, parent, children, .. } => {
                let children_string = children.keys().map(|child| {
                    child.to_string()
                }).collect::<Vec<String>>().join(", ");

                write!(f, "Branch {{ absolute_path[ {:?} ], state[ {:?} ], parent_type_id[ {:?} ], parent[ {:?} ], children[ {:?} ] }}", absolute_path, state, parent_type_id, parent, children_string)
            },
            LockingNodeMetadata::Leaf { absolute_path, state, parent_type_id, parent, .. } => {
                write!(f, "Leaf {{ absolute_path[ {:?} ], state[ {:?} ], parent_type_id[ {:?} ], parent[ {:?} ] }}", absolute_path, state, parent_type_id, parent)
            },
        }
    }
}
impl Display for LockingNodeMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingNodeMetadata::Root { state, children } => {
                let children_string = children.keys().map(|child| {
                    child.to_string()
                }).collect::<Vec<String>>().join(", ");

                write!(f, "Root {{ state[ {:?} ], children[ {:?} ] }}", state, children_string)
            },
            LockingNodeMetadata::Branch { path_segment, state, parent_type_id, parent, children, .. } => {
                let children_string = children.keys().map(|child| {
                    child.to_string()
                }).collect::<Vec<String>>().join(", ");

                write!(f, "Branch {{ path_segment[ {:?} ], state[ {:?} ], parent_type_id[ {:?} ], parent[ {:?} ], children[ {:?} ] }}", path_segment, state, parent_type_id, parent, children_string)
            },
            LockingNodeMetadata::Leaf { path_segment, state, parent_type_id, parent, .. } => {
                write!(f, "Leaf {{ path_segment[ {:?} ], state[ {:?} ], parent_type_id[ {:?} ], parent[ {:?} ] }}", path_segment, state, parent_type_id, parent)
            },
        }
    }
}









// CLOSELY REINSPECT THIS
pub struct TrackedMutex<T> {
    path: AbsoluteLockingPath,
    data: Mutex<T>,
    finalized: Mutex<bool>,
}
impl<T> TrackedMutex<T> {
    pub(in crate) fn new(path: AbsoluteLockingPath, data: T) -> Self {
        Self {
            path,
            data: Mutex::new(data),
            finalized: Mutex::new(false),
        }
    }

    pub fn path(&self) -> AbsoluteLockingPath {
        self.path.clone()
    }

    pub fn lock(&self) -> MutexGuard<'_, T> {
        self.data.lock().unwrap()
    }

    pub(in crate) fn finalize(&self) {
        let mut finalized = self.finalized.lock().unwrap();
        *finalized = true;
    }
}
impl<T> Drop for TrackedMutex<T> {
    fn drop(&mut self) {
        // TODO: Maybe instead of a finalization flag, we add an UnlockRequest to the UnlockQueue,
        //       and in the PostUpdate schedule we process these requests



        let finalized = match self.finalized.try_lock() {
            Ok(finalized) => finalized,
            Err(error) => match error {
                std::sync::TryLockError::Poisoned(_) => {
                    panic!("Mutex has been poisoned!")
                },
                std::sync::TryLockError::WouldBlock => {
                    panic!("Mutex is already locked!")
                },
            },
        };
        
        if !*finalized {
            panic!("TrackedMutex {:?} was dropped without being finalized!", self.path);
        }
    }
}

pub struct TrackedMutexGuard<'a, T>(pub MutexGuard<'a, T>);
impl<'a, T> TrackedMutex<'a, T> {
    pub(in crate) fn new(path: AbsoluteLockingPath, data: T) -> Self {
        Self {
            path,
            data: Mutex::new(data),
            finalized: Mutex::new(false),
        }
    }

    pub fn path 
}

pub(in super) struct LockingNode {
    metadata: LockingNodeMetadata,
    data: Arc<TrackedMutex<Box<dyn Any + Send + Sync>>>,
}
impl LockingNode {
    pub fn new(metadata: LockingNodeMetadata, data: Box<dyn Any + Send + Sync>) -> Self {
        let data = TrackedMutex::new(metadata.get_absolute_path(), data);

        Self {
            metadata,
            data: Arc::new(data),
        }
    }

    pub fn lock(&mut self) -> Result<(), LockingNodeError> {
        match self.metadata.get_state() {
            LockingState::Unlocked => {},
            LockingState::PartiallyLocked { .. } => {
                return Err(LockingNodeError::AlreadyPartiallyLocked);
            },
            LockingState::FullyLocked => {
                return Err(LockingNodeError::AlreadyFullyLocked);
            },
        }

        let (parent_path, parent_mutex) = match &self.metadata {
            LockingNodeMetadata::Root { .. } => {
                *self.metadata.get_state_mut() = LockingState::FullyLocked;

                let locked_children = match self.metadata.get_children() {
                    Some(children) => children,
                    None => {
                        return Ok(());
                    },
                };
                for (child_path, child_mutex) in locked_children {
                    let mut child = match child_mutex.try_lock() {
                        Ok(child) => child,
                        Err(error) => match error {
                            std::sync::TryLockError::Poisoned(_) => {
                                return Err(LockingNodeError::ChildPoisoned);
                            },
                            std::sync::TryLockError::WouldBlock => {
                                return Err(LockingNodeError::ChildFullyLocked);
                            },
                        },
                    };
                
                    match child.lock() {
                        Ok(()) => {},
                        Err(error) => {
                            return Err(LockingNodeError::LockChildError(Box::new(error)));
                        },
                    };
                }

                return Ok(());
            },
            LockingNodeMetadata::Branch { parent, .. } => parent.clone(),
            LockingNodeMetadata::Leaf { parent, .. } => parent.clone(),
        };

        let mut parent = match parent_mutex.try_lock() {
            Ok(parent) => parent,
            Err(error) => match error {
                std::sync::TryLockError::Poisoned(_) => {
                    return Err(LockingNodeError::ParentPoisoned);
                },
                std::sync::TryLockError::WouldBlock => {
                    return Err(LockingNodeError::ParentFullyLocked);
                },
            },
        };

        match parent.metadata.get_state_mut() {
            LockingState::FullyLocked => {
                unreachable!();
            },
            LockingState::PartiallyLocked { locked_children } => {
                if !locked_children.contains(&self.metadata.get_path_segment()) {
                    locked_children.push(self.metadata.get_path_segment());
                } else {
                    unreachable!();
                }
            },
            LockingState::Unlocked => {
                *parent.metadata.get_state_mut() = LockingState::PartiallyLocked {
                    locked_children: vec![self.metadata.get_path_segment()],
                };
            },
        }

        *self.metadata.get_state_mut() = LockingState::FullyLocked;

        let children = match self.metadata.get_children() {
            Some(children) => children,
            None => {
                return Ok(());
            },
        };
        for (child_path, child_mutex) in children {
            let mut child = match child_mutex.try_lock() {
                Ok(child) => child,
                Err(error) => match error {
                    std::sync::TryLockError::Poisoned(_) => {
                        return Err(LockingNodeError::ChildPoisoned);
                    },
                    std::sync::TryLockError::WouldBlock => {
                        return Err(LockingNodeError::ChildFullyLocked);
                    },
                },
            };

            match child.lock() {
                Ok(()) => {},
                Err(error) => {
                    return Err(LockingNodeError::LockChildError(Box::new(error)));
                },
            };
        }

        return Ok(());
    }

    pub fn unlock(&mut self) -> Result<(), LockingNodeError> {
        match self.metadata.get_state() {
            LockingState::Unlocked => {
                return Err(LockingNodeError::AlreadyUnlocked);
            },
            LockingState::PartiallyLocked { .. } => {
                return Err(LockingNodeError::CannotUnlockPartiallyLocked);
            },
            LockingState::FullyLocked => {},
        }

        let (parent_path, parent_mutex) = match &self.metadata {
            LockingNodeMetadata::Root { .. } => {
                *self.metadata.get_state_mut() = LockingState::Unlocked;

                let children = match self.metadata.get_children() {
                    Some(children) => children,
                    None => {
                        return Ok(());
                    },
                };
                for (child_path, child_mutex) in children {
                    let mut child = match child_mutex.try_lock() {
                        Ok(child) => child,
                        Err(error) => match error {
                            std::sync::TryLockError::Poisoned(_) => {
                                return Err(LockingNodeError::ChildPoisoned);
                            },
                            std::sync::TryLockError::WouldBlock => {
                                return Err(LockingNodeError::ChildFullyLocked);
                            },
                        },
                    };
                
                    match child.unlock() {
                        Ok(()) => {},
                        Err(error) => {
                            return Err(LockingNodeError::UnlockChildError(Box::new(error)));
                        },
                    };
                }

                return Ok(());
            },
            LockingNodeMetadata::Branch { parent, .. } => parent.clone(),
            LockingNodeMetadata::Leaf { parent, .. } => parent.clone(),
        };

        let mut parent = match parent_mutex.try_lock() {
            Ok(parent) => parent,
            Err(error) => match error {
                std::sync::TryLockError::Poisoned(_) => {
                    return Err(LockingNodeError::ParentPoisoned);
                },
                std::sync::TryLockError::WouldBlock => {
                    return Err(LockingNodeError::ParentFullyLocked);
                },
            },
        };

        match parent.metadata.get_state_mut() {
            LockingState::FullyLocked => {
                unreachable!();
            },
            LockingState::PartiallyLocked { locked_children: locked_siblings } => {
                *self.metadata.get_state_mut() = LockingState::Unlocked;

                let self_path_segment = self.metadata.get_path_segment();
                locked_siblings.retain(|segment| segment != &self_path_segment);
                if locked_siblings.is_empty() {
                    match parent.unlock() {
                        Ok(()) => {},
                        Err(error) => {
                            return Err(LockingNodeError::UnlockParentError(Box::new(error)));
                        },
                    }
                }

                let children = match self.metadata.get_children() {
                    Some(children) => children,
                    None => {
                        return Ok(());
                    },
                };
                for (child_path, child_mutex) in children {
                    let mut child = match child_mutex.try_lock() {
                        Ok(child) => child,
                        Err(error) => match error {
                            std::sync::TryLockError::Poisoned(_) => {
                                return Err(LockingNodeError::ChildPoisoned);
                            },
                            std::sync::TryLockError::WouldBlock => {
                                return Err(LockingNodeError::ChildFullyLocked);
                            },
                        },
                    };
                
                    match child.unlock() {
                        Ok(()) => {},
                        Err(error) => {
                            return Err(LockingNodeError::UnlockChildError(Box::new(error)));
                        },
                    };
                }

                return Ok(());
            },
            LockingState::Unlocked => {
                unreachable!();
            },
        }
    }
}
impl Debug for LockingNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LockingNode {{ metadata[ {:?} ] }}", self.metadata)
    }
}
impl Display for LockingNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LockingNode {{ metadata[ {:?} ] }}", self.metadata)
    }
}

pub struct UnlockRequest {
    pub node_path: AbsoluteLockingPath,
}

pub struct LockingHierarchy {
    root_node: Arc<Mutex<LockingNode>>
}
impl LockingHierarchy {
    pub fn new() -> Self {
        let root_metadata = LockingNodeMetadata::Root {
            state: LockingState::Unlocked,
            children: HashMap::new(),
        };
        let root_data = Box::new(TypeRegistry::new());

        Self {
            root_node: Arc::new(Mutex::new(LockingNode::new(root_metadata, root_data)))
        }
    }
    
    pub fn insert<T: LockingNodeData>(&mut self, parent_path: AbsoluteLockingPath, parent_mutex: Arc<Mutex<LockingNode>>, path_segment: LockingPathSegment, data: T) -> Result<(), LockingHierarchyError> {
        todo!();
    }
    
    pub fn remove<T: LockingNodeData>(&mut self, path: AbsoluteLockingPath) -> Result<T, LockingHierarchyError> {
        todo!();
    }

    pub fn open<'a, T: 'a, LockingNodeData>(&'a self, path: AbsoluteLockingPath) -> Result<TrackedMutexGuard<'a, T>, LockingHierarchyError> {
        todo!();
    }

    pub fn close<'a, T: 'a, LockingNodeData>(&'a self, handle: TrackedMutexGuard<'a, T>) -> Result<(), LockingHierarchyError> {
        todo!();
    }

    pub(in crate) fn try_get_node(&self, path: AbsoluteLockingPath) -> Result<&LockingNode, LockingHierarchyError> {
        todo!();
    }

    pub(in crate) fn try_get_node_mut(&mut self, path: AbsoluteLockingPath) -> Result<&mut LockingNode, LockingHierarchyError> {
        todo!();
    }
    
    fn try_get_node_raw(&self, path: AbsoluteLockingPath) -> Result<Arc<Mutex<LockingNode>>, LockingHierarchyError> {
        todo!();
    }

    pub fn contains(&self, path: AbsoluteLockingPath) -> bool {
        todo!();
    }

    pub fn is<T: LockingNodeData>(&self, path: AbsoluteLockingPath) -> bool {
        todo!();
    }

    pub fn is_open<T: LockingNodeData>(&self, path: AbsoluteLockingPath) -> bool {
        todo!();
    }
}
impl Debug for LockingHierarchy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root_node = match self.root_node.try_lock() {
            Ok(root_node) => root_node,
            Err(error) => match error {
                std::sync::TryLockError::Poisoned(_) => {
                    panic!("Root node mutex is poisoned!");
                },
                std::sync::TryLockError::WouldBlock => {
                    panic!("Root node mutex is locked!");
                },
            },
        };

        write!(f, "LockingHierarchy {{ root_node[ {:?} ] }}", root_node)
    }
}
impl Display for LockingHierarchy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let root_node = match self.root_node.try_lock() {
            Ok(root_node) => root_node,
            Err(error) => match error {
                std::sync::TryLockError::Poisoned(_) => {
                    panic!("Root node mutex is poisoned!");
                },
                std::sync::TryLockError::WouldBlock => {
                    panic!("Root node mutex is locked!");
                },
            },
        };

        write!(f, "LockingHierarchy {{ root_node[ {:?} ] }}", root_node)
    }
}























// The real good shit, the lit af new shit bro, this shit is the shit, this shit is fire af, lit af on fireeee bro
pub struct EntityInstance {
    bevy_entity_reference: Entity,
}

pub struct ComponentInstance<T: Component> {
    phantom_data: std::marker::PhantomData<T>,
    bevy_entity_reference: Entity,
}

pub struct BundleInstance<T: Bundle> {
    phantom_data: std::marker::PhantomData<T>,
    bevy_entity_reference: Entity,
}

