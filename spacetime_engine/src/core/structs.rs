use super::traits::*;
use std::{any::*, collections::{HashMap, HashSet}};
use std::sync::{Arc, Mutex, RwLock};
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

pub struct HierarchicalLockingMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone + std::fmt::Debug,
{
    map: Arc<Mutex<HashMap<K, Arc<RwLock<V>>>>>,
    lock_state: Arc<Mutex<HierarchyLockState<K>>>,
}

#[derive(Debug)]
enum HierarchyLockState<K>
where
    K: std::cmp::Eq + std::hash::Hash + Clone + std::fmt::Debug,
{
    Unlocked,
    MapLocked,
    EntryLocked(HashMap<K, bool>), // Track locked entries
}

// Instead of giving you the guard directly, we now return an Arc<RwLock>
pub struct HierarchicalMapHandle<K, V> 
where
    K: std::cmp::Eq + std::hash::Hash + Clone + std::fmt::Debug,
{
    map: Arc<Mutex<HashMap<K, Arc<RwLock<V>>>>>,
    lock_state: Arc<Mutex<HierarchyLockState<K>>>,
}

pub struct HierarchicalEntryHandle<K, V> 
where
    K: std::cmp::Eq + std::hash::Hash + Clone + std::fmt::Debug,
{
    entry: Arc<RwLock<V>>,
    lock_state: Arc<Mutex<HierarchyLockState<K>>>,
    key: K,
}

impl<K, V> HierarchicalLockingMap<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone + std::fmt::Debug,
{
    pub fn new() -> Self {
        HierarchicalLockingMap {
            map: Arc::new(Mutex::new(HashMap::new())),
            lock_state: Arc::new(Mutex::new(HierarchyLockState::Unlocked)),
        }
    }

    // Full map lock, returns an Arc<Mutex<HashMap>> instead of a guard
    pub fn lock_map(&self) -> Option<HierarchicalMapHandle<K, V>> {
        let mut lock_state = self.lock_state.lock().unwrap();
        println!("    [DEBUG] Current lock state before map lock attempt: {:?}", *lock_state);  // DEBUG

        match *lock_state {
            HierarchyLockState::Unlocked => {
                println!("    [DEBUG] Locking the entire map");
                *lock_state = HierarchyLockState::MapLocked;
                Some(HierarchicalMapHandle {
                    map: Arc::clone(&self.map),
                    lock_state: Arc::clone(&self.lock_state),
                })
            }
            _ => {
                println!("    [DEBUG] Failed to lock the map because it's already locked");
                None // Map or entries are already locked
            }
        }
    }

    // Lock individual entry, returns an Arc<RwLock<V>> instead of the guard
    pub fn lock_entry(&self, key: K) -> Option<HierarchicalEntryHandle<K, V>> {
        let mut lock_state = self.lock_state.lock().unwrap();
        println!("    [DEBUG] Current lock state before entry lock attempt: {:?}", *lock_state);  // DEBUG

        match *lock_state {
            HierarchyLockState::Unlocked | HierarchyLockState::EntryLocked(_) => {
                let mut locked_entries = match *lock_state {
                    HierarchyLockState::Unlocked => {
                        println!("    [DEBUG] No entries locked, proceeding to lock entry");
                        HashMap::new()
                    }
                    HierarchyLockState::EntryLocked(ref entries) => entries.clone(),
                    _ => unreachable!(),
                };

                if locked_entries.contains_key(&key) {
                    println!("    [DEBUG] Entry {:?} is already locked, cannot lock again", key);  // DEBUG
                    return None; // Entry is already locked
                }

                let map = self.map.lock().unwrap();
                if let Some(entry_lock) = map.get(&key) {
                    println!("    [DEBUG] Locking entry {:?}", key);  // DEBUG
                    let entry_guard = entry_lock.write().unwrap();
                    locked_entries.insert(key.clone(), true);
                    *lock_state = HierarchyLockState::EntryLocked(locked_entries);
                    Some(HierarchicalEntryHandle {
                        entry: Arc::clone(entry_lock),
                        lock_state: Arc::clone(&self.lock_state),
                        key,
                    })
                } else {
                    println!("    [DEBUG] Entry {:?} not found", key);  // DEBUG
                    None // Entry not found
                }
            }
            _ => {
                println!("    [DEBUG] Failed to lock entry {:?} because the map is locked", key);  // DEBUG
                None // Map is locked
            }
        }
    }

    // Add an entry to the map
    pub fn insert(&self, key: K, value: V)
    where
        K: std::cmp::Eq + std::hash::Hash + Clone + std::fmt::Debug,
    {
        let mut map = self.map.lock().unwrap();
        map.insert(key.clone(), Arc::new(RwLock::new(value)));
        println!("    [DEBUG] Inserted entry with key: {:?}", key);  // DEBUG
    }
}

impl<'a, K, V> HierarchicalMapHandle<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone + std::fmt::Debug,
{
    // User will lock the map manually using .lock() on the Mutex
    pub fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<K, Arc<RwLock<V>>>> {
        self.map.lock().unwrap()
    }
}

impl<'a, K, V> HierarchicalEntryHandle<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone + std::fmt::Debug,
{
    // User will lock the entry manually using .write() on the RwLock
    pub fn lock(&self) -> std::sync::RwLockWriteGuard<'_, V> {
        self.entry.write().unwrap()
    }
}

// Drop logic to unlock map/entry when the handle goes out of scope
impl<'a, K, V> Drop for HierarchicalEntryHandle<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone + std::fmt::Debug,
{
    fn drop(&mut self) {
        let mut lock_state = self.lock_state.lock().unwrap();
        if let HierarchyLockState::EntryLocked(ref mut entries) = *lock_state {
            entries.remove(&self.key);

            // If no entries are locked, transition back to `Unlocked`
            if entries.is_empty() {
                *lock_state = HierarchyLockState::Unlocked;
                println!("    [DEBUG] All entries unlocked, transitioning to Unlocked");
            }
        }
    }
}

impl<'a, K, V> Drop for HierarchicalMapHandle<K, V>
where
    K: std::cmp::Eq + std::hash::Hash + Clone + std::fmt::Debug,
{
    fn drop(&mut self) {
        let mut lock_state = self.lock_state.lock().unwrap();
        *lock_state = HierarchyLockState::Unlocked;
    }
}