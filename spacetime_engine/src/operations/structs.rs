use super::traits::*;
use std::{any::*, collections::{HashMap, HashSet}};
use bevy::prelude::*;

#[derive(Reflect)]
pub struct InstanceID<T: 'static + Send + Sync>(u64, #[reflect(ignore)]std::marker::PhantomData<T>);
impl<T: 'static + Send + Sync> InstanceRegistryKey for InstanceID<T> {
    fn new(id: u64) -> Self {
        Self(id, std::marker::PhantomData)
    }

    fn get(&self) -> u64 {
        self.0
    }
}
impl<T: 'static + Send + Sync> Default for InstanceID<T> {
    fn default() -> Self {
        Self(0, std::marker::PhantomData)
    }
}
impl<T: 'static + Send + Sync> std::fmt::Debug for InstanceID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_name = std::any::type_name::<T>();
        let type_name = type_name.split("::").last().unwrap_or(type_name);
        write!(f, "{}ID({})", type_name, self.0)
    }
}
impl<T: 'static + Send + Sync> std::fmt::Display for InstanceID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl<T: 'static + Send + Sync> std::clone::Clone for InstanceID<T> {
    fn clone(&self) -> Self { *self }
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
