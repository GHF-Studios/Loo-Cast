use super::{enums::*, errors::LockingHierarchyError, traits::*, wrappers::{Type, MainTypeRegistry}};
use std::{any::*, collections::{HashMap, HashSet}, sync::MutexGuard};
use std::sync::{Arc, Mutex};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use bevy::prelude::*;
use super::constants::*;
use super::traits::*;

#[derive(Reflect)]
pub struct StaticID<T: 'static + Send + Sync + LockingNode> {
    id: &'static str,
    #[reflect(ignore)]
    phantom_data: std::marker::PhantomData<T>,
}
impl<T: 'static + Send + Sync + LockingNode> RegistryKey for StaticID<T> {
    type ID = &'static str;

    fn new(id: &'static str) -> Self {
        for reserved_id in RESERVED_STATIC_IDS.iter() {
            if id.eq_ignore_ascii_case(reserved_id) {
                panic!("Cannot use reserved static ID '{}'!", reserved_id);
            }
        }

        Self { 
            id,
            phantom_data: std::marker::PhantomData,
        }
    }

    fn get(&self) -> &'static str {
        self.id
    }
}
impl<T: 'static + Send + Sync + LockingNode> StaticInstanceRegistryKey for StaticID<T> {}
impl<T: 'static + Send + Sync + LockingNode> Default for StaticID<T> {
    fn default() -> Self {
        Self {
            id: "",
            phantom_data: std::marker::PhantomData
        }
    }
}
impl<T: 'static + Send + Sync + LockingNode> std::fmt::Debug for StaticID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = std::any::type_name::<T>();
        let type_name = type_name.split("::").last().unwrap_or(type_name);
        write!(f, "{}ID({})", type_name, self.id)
    }
}
impl<T: 'static + Send + Sync + LockingNode> std::fmt::Display for StaticID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl<T: 'static + Send + Sync + LockingNode> std::clone::Clone for StaticID<T> {
    fn clone(&self) -> Self { *self }
}
impl<T: 'static + Send + Sync + LockingNode> core::marker::Copy for StaticID<T> {
}
impl<T: 'static + Send + Sync + LockingNode> std::cmp::PartialEq for StaticID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T: 'static + Send + Sync + LockingNode> std::cmp::Eq for StaticID<T> {
}
impl<T: 'static + Send + Sync + LockingNode> std::hash::Hash for StaticID<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Reflect)]
pub struct DynamicID<T: 'static + Send + Sync + LockingNode> {
    id: u64,
    #[reflect(ignore)]
    phantom_data: std::marker::PhantomData<T>,
}
impl<T: 'static + Send + Sync + LockingNode> RegistryKey for DynamicID<T> {
    type ID = u64;

    fn new(id: u64) -> Self {
        Self {
            id,
            phantom_data: std::marker::PhantomData,
        }
    }

    fn get(&self) -> u64 {
        self.id
    }
}
impl<T: 'static + Send + Sync + LockingNode> DynamicInstanceRegistryKey for DynamicID<T> {}
impl<T: 'static + Send + Sync + LockingNode> Default for DynamicID<T> {
    fn default() -> Self {
        Self {
            id: 0,
            phantom_data: std::marker::PhantomData,
        }
    }
}
impl<T: 'static + Send + Sync + LockingNode> std::fmt::Debug for DynamicID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = std::any::type_name::<T>();
        let type_name = type_name.split("::").last().unwrap_or(type_name);
        write!(f, "{}ID({})", type_name, self.id)
    }
}
impl<T: 'static + Send + Sync + LockingNode> std::fmt::Display for DynamicID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl<T: 'static + Send + Sync + LockingNode> std::clone::Clone for DynamicID<T> {
    fn clone(&self) -> Self { *self }
}
impl<T: 'static + Send + Sync + LockingNode> core::marker::Copy for DynamicID<T> {
}
impl<T: 'static + Send + Sync + LockingNode> std::cmp::PartialEq for DynamicID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T: 'static + Send + Sync + LockingNode> std::cmp::Eq for DynamicID<T> {
}
impl<T: 'static + Send + Sync + LockingNode> std::hash::Hash for DynamicID<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Reflect)]
pub struct StaticUniversalID {
    id: &'static str,
}
impl StaticUniversalID {
    pub fn new<T: 'static + Send + Sync + LockingNode>(id: StaticID<T>) -> Self {
        Self {
            id: id.id,
        }
    }
    pub fn get<T: 'static + Send + Sync + LockingNode>(&self) -> StaticID<T> {
        StaticID::new(self.id)
    }
}
impl Debug for StaticUniversalID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.id)
    }
}
impl Display for StaticUniversalID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.id)
    }
}
impl Clone for StaticUniversalID {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
        }
    }
}
impl Copy for StaticUniversalID {}
impl PartialEq for StaticUniversalID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for StaticUniversalID {}
impl Hash for StaticUniversalID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Reflect)]
pub struct DynamicUniversalID {
    id: u64,
}
impl DynamicUniversalID {
    pub fn new<T: 'static + Send + Sync + LockingNode>(id: DynamicID<T>) -> Self {
        Self {
            id: id.id,
        }
    }
    pub fn get<T: 'static + Send + Sync + LockingNode>(&self) -> DynamicID<T> {
        DynamicID::new(self.id)
    }
}
impl Debug for DynamicUniversalID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.id)
    }
}
impl Display for DynamicUniversalID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({})", self.id)
    }
}
impl Clone for DynamicUniversalID {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
        }
    }
}
impl Copy for DynamicUniversalID {}
impl PartialEq for DynamicUniversalID {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for DynamicUniversalID {}
impl Hash for DynamicUniversalID {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive()]
pub enum LockingPathSegment {
    Root,
    Static(StaticUniversalID),
    Dynamic(DynamicUniversalID),
}
impl LockingPathSegment {
    pub fn new_root() -> Self {
        LockingPathSegment::Root
    }

    pub fn new_static<T: 'static + Send + Sync + LockingNode>(id: &'static str) -> Self {
        LockingPathSegment::Static(StaticUniversalID::new(StaticID::<T>::new(id)))
    }

    pub fn new_dynamic<T: 'static + Send + Sync + LockingNode>(id: u64) -> Self {
        LockingPathSegment::Dynamic(DynamicUniversalID::new(DynamicID::<T>::new(id)))
    }
}
impl Debug for LockingPathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingPathSegment::Root => write!(f, "Root"),
            LockingPathSegment::Static(id) => write!(f, "{}", id),
            LockingPathSegment::Dynamic(id) => write!(f, "{}", id),
        }
    }
}
impl Display for LockingPathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingPathSegment::Root => write!(f, "Root"),
            LockingPathSegment::Static(id) => write!(f, "{}", id),
            LockingPathSegment::Dynamic(id) => write!(f, "{}", id),
        }
    }
}
impl Clone for LockingPathSegment {
    fn clone(&self) -> Self {
        match self {
            LockingPathSegment::Root => LockingPathSegment::Root,
            LockingPathSegment::Static(id) => LockingPathSegment::Static(id.clone()),
            LockingPathSegment::Dynamic(id) => LockingPathSegment::Dynamic(id.clone()),
        }
    }
}
impl Copy for LockingPathSegment {}
impl PartialEq for LockingPathSegment {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LockingPathSegment::Static(id), LockingPathSegment::Static(other_id)) => id == other_id,
            (LockingPathSegment::Dynamic(id), LockingPathSegment::Dynamic(other_id)) => id == other_id,
            _ => false,
        }
    }
}
impl Eq for LockingPathSegment {}
impl Hash for LockingPathSegment {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            LockingPathSegment::Root => "Root".hash(state),
            LockingPathSegment::Static(id) => id.hash(state),
            LockingPathSegment::Dynamic(id) => id.hash(state),
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

    fn push(&mut self, segment: LockingPathSegment) -> Result<(), String> {
        let last_segment = self.segments.last();
        
        match last_segment {
            Some(LockingPathSegment::Root) => {
                unreachable!()
            },
            Some(LockingPathSegment::Static(_)) => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment after static segment in relative path!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(())
                    },
                }
            },
            Some(LockingPathSegment::Dynamic(_)) => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment after dynamic segment in relative path!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(())
                    },
                }
            },
            None => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment after no segments in relative path!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(())
                    },
                }
            },
        }
    }

    fn pop(&mut self) -> Result<LockingPathSegment, String> {
        match self.segments.pop() {
            Some(segment) => Ok(segment),
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
            segments: vec![LockingPathSegment::Root],
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

    fn push(&mut self, segment: LockingPathSegment) -> Result<(), String> {
        let last_segment = self.segments.last();

        match last_segment {
            Some(LockingPathSegment::Root) => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment after root segment in absolute path!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(())
                    },
                }
            },
            Some(LockingPathSegment::Static(_)) => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment after static segment in absolute path!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(())
                    },
                }
            },
            Some(LockingPathSegment::Dynamic(_)) => {
                match segment {
                    LockingPathSegment::Root => {
                        return Err("Cannot push root segment after dynamic segment in absolute path!".to_string())
                    },
                    _ => {
                        self.segments.push(segment);
                        Ok(())
                    },
                }
            },
            None => {
                unreachable!()
            }
        }
    }

    fn pop(&mut self) -> Result<LockingPathSegment, String> {
        if self.segments.len() == 1 {
            return Err("Cannot pop root segment from absolute path!".to_string());
        }

        match self.segments.pop() {
            Some(segment) => Ok(segment),
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
impl<K: StaticInstanceRegistryKey, V: InstanceRegistryValue> LockingNodePartialData for StaticInstanceRegistry<K, V> {}
impl<K: StaticInstanceRegistryKey, V: InstanceRegistryValue> LockingNodeData for StaticInstanceRegistry<K, V> {}

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
impl<K: DynamicInstanceRegistryKey, V: InstanceRegistryValue> LockingNodePartialData for DynamicInstanceRegistry<K, V> {}
impl<K: DynamicInstanceRegistryKey, V: InstanceRegistryValue> LockingNodeData for DynamicInstanceRegistry<K, V> {}

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

        self.managed.insert(type_id, Type::new::<T>());
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
impl LockingNodePartialData for TypeRegistry {}
impl LockingNodeData for TypeRegistry {}

pub struct SingletonRegistry {
    registered: HashSet<TypeId>,
    managed: HashMap<TypeId, Box<dyn Any>>,
}
impl SingletonRegistry {
    pub fn new() -> Self {
        Self {
            registered: HashSet::new(),
            managed: HashMap::new(),
        }
    }

    pub fn register<T: 'static + Singleton>(&mut self) {
        let type_id = TypeId::of::<T>();

        if self.registered.contains(&type_id) {
            panic!("Type '{:?}' is already registered!", type_id);
        }

        self.registered.insert(type_id);
    }

    pub fn unregister<T: 'static + Singleton>(&mut self) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is still managed!", type_id);
        }

        self.registered.retain(|other_type_id| type_id != *other_type_id);
    }

    pub fn manage<T: 'static + Singleton>(&mut self, singleton: T) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is already managed!", type_id);
        }

        self.managed.insert(type_id, Box::new(singleton));
    }

    pub fn unmanage<T: 'static + Singleton>(&mut self) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is already unmanaged!", type_id);
        }

        self.managed.remove(&type_id);
    }

    pub fn get<T: 'static + Singleton>(&self) -> Option<&T> {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is not managed!", type_id);
        }

        self.managed.get(&type_id).and_then(|singleton| singleton.downcast_ref::<T>())
    }

    pub fn get_mut<T: 'static + Singleton>(&mut self) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is not managed!", type_id);
        }

        self.managed.get_mut(&type_id).and_then(|singleton| singleton.downcast_mut::<T>())
    }

    pub fn registered(&self) -> &HashSet<TypeId> {
        &self.registered
    }

    pub fn managed(&self) -> &HashMap<TypeId, Box<dyn Any>> {
        &self.managed
    }

    pub fn is_registered<T: 'static + Singleton>(&self) -> bool {
        self.registered.contains(&TypeId::of::<T>())
    }

    pub fn is_managed<T: 'static + Singleton>(&self) -> bool {
        self.managed.contains_key(&TypeId::of::<T>())
    }
}


pub struct EntityReference {
    bevy_entity_reference: Entity,
}

pub struct ComponentReference<T: Component> {
    phantom_data: std::marker::PhantomData<T>,
    bevy_entity_reference: Entity,
}

pub struct BundleReference<T: Bundle> {
    phantom_data: std::marker::PhantomData<T>,
    bevy_entity_reference: Entity,
}



#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct LockingNodeInfo {
    path_segment: LockingPathSegment,
    locking_state: LockingState,
}
impl LockingNodeInfo {
    pub fn new(path_segment: LockingPathSegment, locking_state: LockingState) -> Self {
        Self {
            path_segment,
            locking_state,
        }
    }

    pub fn path_segment(&self) -> LockingPathSegment {
        self.path_segment
    }

    pub fn locking_state(&self) -> LockingState {
        self.locking_state
    }
}
impl Debug for LockingNodeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.path_segment, self.locking_state)
    }
}
impl Display for LockingNodeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.path_segment, self.locking_state)
    }
}



pub struct LockingRootNode<D: LockingNodeData, CD: LockingNodeData> {
    data: D,
    child_phantom_data: std::marker::PhantomData<CD>,
    node_info: LockingNodeInfo,
}
impl<D: LockingNodeData, CD: LockingNodeData> LockingRootNode<D, CD> {
    pub fn children(&self) -> MutexGuard<HashMap<LockingPathSegment, Arc<Mutex<CD>>>> {

    }
}
impl<D: LockingNodeData, CD: LockingNodeData> LockingNode for LockingRootNode<D, CD> {
    fn node_info(&self) -> LockingNodeInfo {
        self.node_info
    }
}
impl<D: LockingNodeData, CD: LockingNodeData> LockingNodeParent for LockingRootNode<D, CD> {}



pub struct LockingBranchNode<D: LockingNodeData, PD: LockingNodeData, CD: LockingNodeData> {
    data: D,
    parent_phantom_data: std::marker::PhantomData<PD>,
    child_phantom_data: std::marker::PhantomData<CD>,
    node_info: LockingNodeInfo,
}
impl<D: LockingNodeData, PD: LockingNodeData, CD: LockingNodeData> LockingBranchNode<D, PD, CD> {
    pub fn parent(&self) -> MutexGuard<(LockingPathSegment, Arc<Mutex<PD>>)> {

    }

    pub fn children(&self) -> MutexGuard<HashMap<LockingPathSegment, Arc<Mutex<CD>>>> {

    }
}
impl<D: LockingNodeData, PD: LockingNodeData, CD: LockingNodeData> LockingNode for LockingBranchNode<D, PD, CD> {
    fn node_info(&self) -> LockingNodeInfo {
        self.node_info
    }
}
impl<D: LockingNodeData, PD: LockingNodeData, CD: LockingNodeData> LockingNodeParent for LockingBranchNode<D, PD, CD> {}
impl<D: LockingNodeData, PD: LockingNodeData, CD: LockingNodeData> LockingNodeChild for LockingBranchNode<D, PD, CD> {}
impl<D: LockingNodeData, PD: LockingNodeData, CD: LockingNodeData> LockingNodeParentChild for LockingBranchNode<D, PD, CD> {}



pub struct LockingLeafNode<D: LockingNodePartialData, PD: LockingNodeData> {
    data: D,
    parent_phantom_data: std::marker::PhantomData<PD>,
    node_info: LockingNodeInfo,
}
impl<D: LockingNodeData, PD: LockingNodeData> LockingLeafNode<D, PD> {
    pub fn parent(&self) -> MutexGuard<(LockingPathSegment, Arc<Mutex<PD>>)> {

    }
}
impl<D: LockingNodeData, PD: LockingNodeData> LockingNode for LockingLeafNode<D, PD> {
    fn node_info(&self) -> LockingNodeInfo {
        self.node_info
    }
}
impl<D: LockingNodeData, PD: LockingNodeData> LockingNodeChild for LockingLeafNode<D, PD> {}






pub struct LockingHierarchy {
    root: Arc<Mutex<LockingRootNode<MainTypeRegistry, Type>>>,
}
impl LockingHierarchy {
    pub fn new(root: LockingRootNode<MainTypeRegistry, Type>) -> Self {
        Self {
            root: Arc::new(Mutex::new(root)),
        }
    }
    pub fn insert(&mut self, path: AbsoluteLockingPath, entry: Box<dyn Any>) -> Result<(), LockingHierarchyError> {
        
    }
    
    pub fn remove(&mut self, path: AbsoluteLockingPath) -> Result<Box<dyn Any>, LockingHierarchyError> {
        
    }
    
    pub fn contains(&self, path: AbsoluteLockingPath) -> Result<bool, LockingHierarchyError> {
        
    }
    
    pub fn lock(&self, path: AbsoluteLockingPath) -> Result<MutexGuard<dyn Any>, LockingHierarchyError> {
        
    }
    
    pub fn unlock(&self, path: AbsoluteLockingPath, entry_guard: MutexGuard<dyn Any>) -> Result<(), LockingHierarchyError> {
        
    }
    
    pub fn is_locked(&self, path: AbsoluteLockingPath) -> Result<bool, LockingHierarchyError> {
        
    }
    
    fn get(&self, path: AbsoluteLockingPath) -> Result<MutexGuard<dyn Any>, LockingHierarchyError> {

    }
    
    fn get_mut(
        &mut self, 
        path: AbsoluteLockingPath
    ) -> Result<MutexGuard<dyn Any>, LockingHierarchyError> {
        let current_node = self.root;
        let current_segment = path.segments().first().unwrap();

        for i in 0..path.segments().len()-1 {

        }

        let last_node = current_node;
        let last_path_segment = path.segments().last().unwrap();
        let last_node_guard = 
        
        todo!()
    }
}