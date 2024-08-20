pub mod requesters;
pub mod resources;

use std::any::{Any, TypeId};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

use bevy::prelude::*;
pub(in crate) struct OperationsPlugin;

impl Plugin for OperationsPlugin {
    fn build(&self, app: &mut App) {

    }
}

pub struct ID<T>(u64, std::marker::PhantomData<T>);

impl<T> ID<T> {
    pub(in crate) fn new(id: u64) -> Self {
        Self(id, std::marker::PhantomData)
    }

    pub(in crate) fn get(&self) -> u64 {
        self.0
    }
}

impl<T> std::fmt::Debug for ID<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ID({})", self.0)
    }
}

impl<T> std::clone::Clone for ID<T> {
    fn clone(&self) -> Self {
        Self(self.0, std::marker::PhantomData)
    }
}

impl<T> core::marker::Copy for ID<T> {
}

impl<T> std::cmp::PartialEq for ID<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> std::cmp::Eq for ID<T> {
}

impl<T> std::hash::Hash for ID<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

#[derive(Clone)]
pub struct Registry<T> {
    registered: HashSet<ID<T>>,
    managed: HashMap<ID<T>, T>,
    next_id: ID<T>,
    recycled_ids: Vec<ID<T>>,
}

impl<T> Registry<T> {
    pub fn new() -> Self {
        Self {
            registered: HashSet::new(),
            managed: HashMap::new(),
            next_id: ID::new(1),
            recycled_ids: Vec::new(),
        }
    }

    fn get_unused_id(&mut self) -> ID<T> {
        if let Some(recycled_id) = self.recycled_ids.pop() {
            trace!("Used recycled id: '{:?}'", recycled_id);

            recycled_id
        } else {
            let id = self.next_id;
            self.next_id = ID::new(self.next_id.get() + 1);
            id
        }
    }

    fn recycle_id(&mut self, id: ID<T>) {
        if !self.registered.contains(&id) {
            panic!("ID '{:?}' is not registered in the first place!", id);
        }

        if self.recycled_ids.contains(&id) {
            panic!("ID '{:?}' is already recycled!", id);
        }

        self.recycled_ids.push(id);
    }

    pub fn register(&mut self) -> ID<T> {
        let id = self.get_unused_id();

        self.registered.insert(id);

        id
    }

    pub fn unregister(&mut self, id: ID<T>) {
        if !self.registered.contains(&id) {
            panic!("ID '{:?}' is invalid!", id);
        }

        if self.managed.contains_key(&id) {
            panic!("ID '{:?}' is still managed!", id);
        }

        self.registered.retain(|other_id| id != *other_id);

        self.recycle_id(id);
    }

    pub fn manage(&mut self, id: ID<T>, value: T) {
        if !self.registered.contains(&id) {
            panic!("ID '{:?}' is invalid!", id);
        }

        if self.managed.contains_key(&id) {
            panic!("ID '{:?}' is already managed!", id);
        }

        self.managed.insert(id, value);
    }

    pub fn unmanage(&mut self, id: ID<T>) -> T {
        if !self.registered.contains(&id) {
            panic!("ID '{:?}' is invalid!", id);
        }

        if !self.managed.contains_key(&id) {
            panic!("ID '{:?}' is already unmanaged!", id);
        }

        self.managed.remove(&id).unwrap()
    }
}

#[derive(Clone)]
pub struct TypeRegistry {
    registered: HashSet<TypeId>,
    managed: HashMap<TypeId, HashMap<TypeId, Arc<Mutex<dyn Any + Send + Sync>>>>,
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

    pub fn set_data<T: 'static, D: 'static + Clone + Send + Sync>(&mut self, data: D) {
        let type_id = TypeId::of::<T>();
        let data_type_id = TypeId::of::<D>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is not managed!", type_id);
        }

        let type_data_map = self.managed.entry(type_id).or_insert_with(HashMap::new);
        type_data_map.insert(data_type_id, Arc::new(Mutex::new(data)));
    }

    // TODO: Make data be a reference instead of a clone, and also present a mutable reference version 
    pub fn get_data<T: 'static, D: 'static + Clone + Send + Sync>(&self) -> Option<D> {
        let type_id = TypeId::of::<T>();
        let data_type_id = TypeId::of::<D>();

        if !self.registered.contains(&type_id) {
            panic!("Type '{:?}' is not registered!", type_id);
        }

        if !self.managed.contains_key(&type_id) {
            panic!("Type '{:?}' is not managed!", type_id);
        }

        let type_data_map = self.managed.get(&type_id).unwrap();

        let data_mutex = match type_data_map.get(&data_type_id) {
            Some(data_box) => data_box,
            None => return None,
        };

        let data_guard = match data_mutex.lock() {
            Ok(data_guard) => data_guard,
            Err(_) => panic!("Data mutex poisoned!"),
        };

        let data = match data_guard.downcast_ref::<D>() {
            Some(data) => data.clone(),
            None => unreachable!(),
        };

        return Some(data);
    }
}

lazy_static! {
    pub static ref TYPE_REGISTRY: Arc<Mutex<TypeRegistry>> = Arc::new(Mutex::new(TypeRegistry::new()));
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