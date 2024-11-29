use std::any::TypeId;
use std::marker::PhantomData;
use bevy::prelude::*;
use tokio::task::JoinHandle;
use std::fmt::{Debug, Display};
use crate::*;
use crate::wrappers::*;

pub(crate) struct Root;
impl LockingNodeData for Root {
    fn pre_startup(&mut self, hierarchy: &mut LockingHierarchy) {
        let root_path = AbsoluteLockingPath::new();
        let root_mutex = hierarchy.get_node_raw(root_path.clone()).unwrap();
    
        let core_path_segment = LockingPathSegment::new_string("core");
        let core_path = root_path.clone().push(core_path_segment).unwrap();
        hierarchy.insert_branch::<RootTypeRegistry, Core, RootTypeData<Core>>(
            root_path.clone(), 
            root_mutex.clone(), 
            core_path_segment, 
            Core
        ).unwrap();
        hierarchy.pre_startup::<Core>(core_path).unwrap();
    
        let command_path_segment = LockingPathSegment::new_string("command");
        let command_path = root_path.clone().push(command_path_segment).unwrap();
        hierarchy.insert_branch::<RootTypeRegistry, Command, RootTypeData<Command>>(
            root_path.clone(), 
            root_mutex.clone(), 
            command_path_segment, 
            Command
        ).unwrap();
        hierarchy.pre_startup::<Command>(command_path).unwrap();
    
        let operation_path_segment = LockingPathSegment::new_string("operation");
        let operation_path = root_path.clone().push(operation_path_segment).unwrap();
        hierarchy.insert_branch::<RootTypeRegistry, Operation, RootTypeData<Operation>>(
            root_path.clone(), 
            root_mutex.clone(), 
            operation_path_segment, 
            Operation
        ).unwrap();
        hierarchy.pre_startup::<Operation>(operation_path).unwrap();
    }

    fn startup(&mut self, _hierarchy: &mut LockingHierarchy) {
        
    }

    fn post_startup(&mut self, _hierarchy: &mut LockingHierarchy) {
        
    }

    fn update(&mut self, _hierarchy: &mut LockingHierarchy) {
        
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
        &self.2
    }

    pub fn type_data_registry_mut(&mut self) -> &mut LockingTypeDataRegistry {
        &mut self.2
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

        self.managed.insert(type_id, LockingType::new::<T>(type_binding).unwrap().0);
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
            type_id.to_string()
        }).collect::<Vec<String>>().join(", ");

        let managed_types = self.managed.iter().map(|(type_id, _)| {
            type_id.to_string()
        }).collect::<Vec<String>>().join(", ");

        write!(f, "TypeRegistry{{ registered[ {:?} ], managed[ {:?} ] }}", registered_types, managed_types)
    }
}
impl Display for LockingTypeRegistry {
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

    pub fn manage<T: 'static + LockingTypeDataTrait>(&mut self, singleton: T) {
        let type_id = TypeId::of::<T>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is already managed!", type_id);
        }

        self.managed.insert(type_id, singleton);
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

        self.managed.get(&type_id).unwrap_or_else(|| None).downcast_ref::<T>()
    }

    pub fn get_mut<T: 'static + LockingTypeDataTrait>(&mut self) -> Option<&mut T> {
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

    pub fn managed<T: 'static + LockingTypeDataTrait>(&self) -> &HashMap<TypeId, T> {
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
            type_id.to_string()
        }).collect::<Vec<String>>().join(", ");

        let managed_types = self.managed.iter().map(|(type_id, _)| {
            type_id.to_string()
        }).collect::<Vec<String>>().join(", ");

        write!(f, "SingletonRegistry {{ registered[ {:?} ], managed[ {:?} ] }}", registered_types, managed_types)
    }
}
impl Display for LockingTypeDataRegistry {
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
