use super::traits::*;
use std::{any::*, collections::{HashMap, HashSet}, sync::MutexGuard};
use std::sync::{Arc, Mutex};
use bevy::prelude::*;

#[derive(Reflect)]
pub struct StaticKey<T: 'static + Send + Sync> {
    id: &'static str,
    #[reflect(ignore)]
    phantom_data: std::marker::PhantomData<T>,
}
impl<T: 'static + Send + Sync> RegistryKey for StaticKey<T> {
    type ID = &'static str;

    fn new(id: &'static str) -> Self {
        Self { 
            id,
            phantom_data: std::marker::PhantomData,
        }
    }

    fn get(&self) -> &'static str {
        self.id
    }
}
impl<T: 'static + Send + Sync> StaticInstanceRegistryKey for StaticKey<T> {}
impl<T: 'static + Send + Sync> Default for StaticKey<T> {
    fn default() -> Self {
        Self {
            id: "",
            phantom_data: std::marker::PhantomData
        }
    }
}
impl<T: 'static + Send + Sync> std::fmt::Debug for StaticKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = std::any::type_name::<T>();
        let type_name = type_name.split("::").last().unwrap_or(type_name);
        write!(f, "{}ID({})", type_name, self.id)
    }
}
impl<T: 'static + Send + Sync> std::fmt::Display for StaticKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl<T: 'static + Send + Sync> std::clone::Clone for StaticKey<T> {
    fn clone(&self) -> Self { *self }
}
impl<T: 'static + Send + Sync> core::marker::Copy for StaticKey<T> {
}
impl<T: 'static + Send + Sync> std::cmp::PartialEq for StaticKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T: 'static + Send + Sync> std::cmp::Eq for StaticKey<T> {
}
impl<T: 'static + Send + Sync> std::hash::Hash for StaticKey<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

#[derive(Reflect)]
pub struct DynamicKey<T: 'static + Send + Sync> {
    id: u64,
    #[reflect(ignore)]
    phantom_data: std::marker::PhantomData<T>,
}
impl<T: 'static + Send + Sync> RegistryKey for DynamicKey<T> {
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
impl<T: 'static + Send + Sync> DynamicInstanceRegistryKey for DynamicKey<T> {}
impl<T: 'static + Send + Sync> Default for DynamicKey<T> {
    fn default() -> Self {
        Self {
            id: 0,
            phantom_data: std::marker::PhantomData,
        }
    }
}
impl<T: 'static + Send + Sync> std::fmt::Debug for DynamicKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = std::any::type_name::<T>();
        let type_name = type_name.split("::").last().unwrap_or(type_name);
        write!(f, "{}ID({})", type_name, self.id)
    }
}
impl<T: 'static + Send + Sync> std::fmt::Display for DynamicKey<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl<T: 'static + Send + Sync> std::clone::Clone for DynamicKey<T> {
    fn clone(&self) -> Self { *self }
}
impl<T: 'static + Send + Sync> core::marker::Copy for DynamicKey<T> {
}
impl<T: 'static + Send + Sync> std::cmp::PartialEq for DynamicKey<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<T: 'static + Send + Sync> std::cmp::Eq for DynamicKey<T> {
}
impl<T: 'static + Send + Sync> std::hash::Hash for DynamicKey<T> {
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

pub struct TypeRegistry {
    registered: HashSet<TypeId>,
    managed: HashMap<TypeId, HashMap<TypeId, Box<dyn Any + Send + Sync>>>,
}
impl TypeRegistry {
    pub fn new() -> Self {
        Self {
            registered: HashSet::new(),
            managed: HashMap::new(),
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

pub struct ExampleHierarchy {
    root: ExampleRoot,
}
impl LockingHierarchy<ExampleRoot, StaticKey<ExampleRegistry>, ExampleRegistry> for ExampleHierarchy {
    fn root(&self) -> &ExampleRoot {
        &self.root
    }

    fn root_mut(&mut self) -> &mut ExampleRoot {
        &mut self.root
    }
}

pub struct ExampleRoot {
    children: Arc<Mutex<HashMap<StaticKey<ExampleRegistry>, ExampleRegistry>>>,
}
impl LockingNodeParent for ExampleRoot {}
impl LockingRootNode<StaticKey<ExampleRegistry>, ExampleRegistry> for ExampleRoot {
    fn children(&self) -> MutexGuard<HashMap<StaticKey<ExampleRegistry>, ExampleRegistry>> {
        self.children.lock().unwrap()
    }
}

pub struct ExampleRegistry {
    parent: Arc<Mutex<(StaticKey<ExampleRoot>, ExampleRoot)>>,
    children: Arc<Mutex<HashMap<StaticKey<ExampleObject>, ExampleObject>>>,
}
impl LockingNodeParent for ExampleRegistry {}
impl LockingNodeChild for ExampleRegistry {}
impl LockingNodeParentChild for ExampleRegistry {}
impl LockingBranchNode<StaticKey<ExampleRoot>, ExampleRoot, StaticKey<ExampleObject>, ExampleObject> for ExampleRegistry {
    fn parent(&self) -> MutexGuard<(StaticKey<ExampleRoot>, ExampleRoot)> {
        self.parent.lock().unwrap()
    }

    fn children(&self) -> MutexGuard<HashMap<StaticKey<ExampleObject>, ExampleObject>> {
        self.children.lock().unwrap()
    }
}

pub struct ExampleObject {
    parent: Arc<Mutex<(StaticKey<ExampleRegistry>, ExampleRegistry)>>,
}
impl LockingNodeChild for ExampleObject {}
impl LockingLeafNode<StaticKey<ExampleRegistry>, ExampleRegistry> for ExampleObject {
    fn parent(&self) -> MutexGuard<(StaticKey<ExampleRegistry>, ExampleRegistry)> {
        self.parent.lock().unwrap()
    }
}

/*
pub struct LockingRootNodeHandle(Box<dyn Any>);
impl LockingRootNodeHandle {
    pub fn new<T: 'static + LockingNodeParent>(node: T) -> Self {
        Self(Box::new(node))
    }

    pub fn get<T: 'static + LockingNodeParent>(&self) -> &T {
        self.0.downcast_ref::<T>().expect("Failed to downcast internal value")
    }

    pub fn get_mut<T: 'static + LockingNodeParent>(&mut self) -> &mut T {
        self.0.downcast_mut::<T>().expect("Failed to downcast internal value")
    }
}

pub struct LockingBranchNodeHandle(Box<dyn Any>);
impl LockingBranchNodeHandle {
    pub fn new<T: 'static + LockingNodeParentChild>(node: T) -> Self {
        Self(Box::new(node))
    }

    pub fn get<T: 'static + LockingNodeParentChild>(&self) -> &T {
        self.0.downcast_ref::<T>().expect("Failed to downcast internal value")
    }

    pub fn get_mut<T: 'static + LockingNodeParentChild>(&mut self) -> &mut T {
        self.0.downcast_mut::<T>().expect("Failed to downcast internal value")
    }
}

pub struct LockingLeafNodeHandle(Box<dyn Any>);
impl LockingLeafNodeHandle {
    pub fn new<T: 'static + LockingNodeChild>(node: T) -> Self {
        Self(Box::new(node))
    }

    pub fn get<T: 'static + LockingNodeChild>(&self) -> &T {
        self.0.downcast_ref::<T>().expect("Failed to downcast internal value")
    }

    pub fn get_mut<T: 'static + LockingNodeChild>(&mut self) -> &mut T {
        self.0.downcast_mut::<T>().expect("Failed to downcast internal value")
    }
}

pub enum LockingNode {
    Root(LockingRootNodeHandle),
    Branch(LockingBranchNodeHandle),
    Leaf(LockingLeafNodeHandle),
}
*/