use super::{enums::*, errors::LockingHierarchyError, traits::*, wrappers::*};
use std::{any::*, collections::{HashMap, HashSet}};
use std::sync::{Arc, Mutex};
use std::fmt::{Debug, Display};
use std::hash::Hash;
use bevy::prelude::*;
use super::constants::*;
use super::traits::*;

pub struct Core;

#[derive(Reflect)]
pub struct StaticID<T: 'static + Send + Sync> {
    id: &'static str,
    #[reflect(ignore)]
    phantom_data: std::marker::PhantomData<T>,
}
impl<T: 'static + Send + Sync> RegistryKey for StaticID<T> {
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
impl<T: 'static + Send + Sync> StaticInstanceRegistryKey for StaticID<T> {}
impl<T: 'static + Send + Sync> Default for StaticID<T> {
    fn default() -> Self {
        Self {
            id: "",
            phantom_data: std::marker::PhantomData
        }
    }
}
impl<T: 'static + Send + Sync> std::fmt::Debug for StaticID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = std::any::type_name::<T>();
        let type_name = type_name.split("::").last().unwrap_or(type_name);
        write!(f, "{}ID({})", type_name, self.id)
    }
}
impl<T: 'static + Send + Sync> std::fmt::Display for StaticID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl<T: 'static + Send + Sync> std::clone::Clone for StaticID<T> {
    fn clone(&self) -> Self { *self }
}
impl<T: 'static + Send + Sync> core::marker::Copy for StaticID<T> {
}
impl<T: 'static + Send + Sync> std::cmp::PartialEq for StaticID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T: 'static + Send + Sync> std::cmp::Eq for StaticID<T> {
}
impl<T: 'static + Send + Sync> std::hash::Hash for StaticID<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Reflect)]
pub struct DynamicID<T: 'static + Send + Sync> {
    id: u64,
    #[reflect(ignore)]
    phantom_data: std::marker::PhantomData<T>,
}
impl<T: 'static + Send + Sync> RegistryKey for DynamicID<T> {
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
impl<T: 'static + Send + Sync> DynamicInstanceRegistryKey for DynamicID<T> {}
impl<T: 'static + Send + Sync> Default for DynamicID<T> {
    fn default() -> Self {
        Self {
            id: 0,
            phantom_data: std::marker::PhantomData,
        }
    }
}
impl<T: 'static + Send + Sync> std::fmt::Debug for DynamicID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = std::any::type_name::<T>();
        let type_name = type_name.split("::").last().unwrap_or(type_name);
        write!(f, "{}ID({})", type_name, self.id)
    }
}
impl<T: 'static + Send + Sync> std::fmt::Display for DynamicID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl<T: 'static + Send + Sync> std::clone::Clone for DynamicID<T> {
    fn clone(&self) -> Self { *self }
}
impl<T: 'static + Send + Sync> core::marker::Copy for DynamicID<T> {
}
impl<T: 'static + Send + Sync> std::cmp::PartialEq for DynamicID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T: 'static + Send + Sync> std::cmp::Eq for DynamicID<T> {
}
impl<T: 'static + Send + Sync> std::hash::Hash for DynamicID<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Reflect)]
pub struct StaticUniversalID {
    id: &'static str,
}
impl StaticUniversalID {
    pub fn new<T: 'static + Send + Sync>(id: StaticID<T>) -> Self {
        Self {
            id: id.id,
        }
    }
    pub fn get<T: 'static + Send + Sync>(&self) -> StaticID<T> {
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
    pub fn new<T: 'static + Send + Sync>(id: DynamicID<T>) -> Self {
        Self {
            id: id.id,
        }
    }
    pub fn get<T: 'static + Send + Sync>(&self) -> DynamicID<T> {
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

pub enum LockingPathSegment {
    Root,
    Text(StaticUniversalID),
    Number(DynamicUniversalID),
}
impl LockingPathSegment {
    pub fn new_root() -> Self {
        LockingPathSegment::Root
    }

    pub fn new_text<T: 'static + Send + Sync>(id: &'static str) -> Self {
        LockingPathSegment::Text(StaticUniversalID::new(StaticID::<T>::new(id)))
    }

    pub fn new_number<T: 'static + Send + Sync>(id: u64) -> Self {
        LockingPathSegment::Number(DynamicUniversalID::new(DynamicID::<T>::new(id)))
    }
}
impl Debug for LockingPathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingPathSegment::Root => write!(f, "root"),
            LockingPathSegment::Text(id) => write!(f, "{}", id),
            LockingPathSegment::Number(id) => write!(f, "{}", id),
        }
    }
}
impl Display for LockingPathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingPathSegment::Root => write!(f, "root"),
            LockingPathSegment::Text(id) => write!(f, "{}", id),
            LockingPathSegment::Number(id) => write!(f, "{}", id),
        }
    }
}
impl Clone for LockingPathSegment {
    fn clone(&self) -> Self {
        match self {
            LockingPathSegment::Root => LockingPathSegment::Root,
            LockingPathSegment::Text(id) => LockingPathSegment::Text(id.clone()),
            LockingPathSegment::Number(id) => LockingPathSegment::Number(id.clone()),
        }
    }
}
impl Copy for LockingPathSegment {}
impl PartialEq for LockingPathSegment {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (LockingPathSegment::Text(id), LockingPathSegment::Text(other_id)) => id == other_id,
            (LockingPathSegment::Number(id), LockingPathSegment::Number(other_id)) => id == other_id,
            _ => false,
        }
    }
}
impl Eq for LockingPathSegment {}
impl Hash for LockingPathSegment {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            LockingPathSegment::Root => "Root".hash(state),
            LockingPathSegment::Text(id) => id.hash(state),
            LockingPathSegment::Number(id) => id.hash(state),
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
            Some(LockingPathSegment::Text(_)) => {
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
            Some(LockingPathSegment::Number(_)) => {
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
            Some(LockingPathSegment::Text(_)) => {
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
            Some(LockingPathSegment::Number(_)) => {
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
impl LockingNodeData for TypeRegistry {}

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





pub(in super) enum LockingNodeMetadata {
    Root {
        state: LockingState,
        children: HashMap<LockingPathSegment, Arc<Mutex<dyn Any + Send + Sync>>>,
    },
    Branch {
        path_segment: LockingPathSegment,
        state: LockingState,
        children: HashMap<LockingPathSegment, Arc<Mutex<dyn Any + Send + Sync>>>,
    },
    Leaf {
        path_segment: LockingPathSegment,
        state: LockingState,
    },
}

pub(in super) struct LockingNode {
    pub metadata: LockingNodeMetadata,
    pub data: Arc<Mutex<dyn LockingNodeData>>,
}

pub struct LockingHierarchy {
    root: Arc<Mutex<LockingNode>>,
}
impl LockingHierarchy {
    pub fn new() -> Self {
        let root = Arc::new(Mutex::new(LockingNode {
            metadata: LockingNodeMetadata::Root {
                state: LockingState::Unlocked,
                children: HashMap::new(),
            },
            data: Arc::new(Mutex::new(MainTypeRegistry::new())),
        }));

        Self {
            root
        }
    }
    
    pub fn insert(&mut self, node: LockingNode) -> Result<(), LockingHierarchyError> {
        
    }
    
    pub fn remove(&mut self, path: AbsoluteLockingPath) -> Result<LockingNode, LockingHierarchyError> {
        
    }

    pub fn contains(&self, path: AbsoluteLockingPath) -> bool {
        
    }

    pub fn get(&self, path: AbsoluteLockingPath) -> Result<LockingNode, LockingHierarchyError> {
        
    }

    pub fn get_mut(&mut self, path: AbsoluteLockingPath) -> Result<LockingNode, LockingHierarchyError> {
        
    }

    pub fn lock(&mut self, path: AbsoluteLockingPath) -> Result<(), LockingHierarchyError> {
        
    }

    pub fn unlock(&mut self, path: AbsoluteLockingPath) -> Result<(), LockingHierarchyError> {
        
    }
}