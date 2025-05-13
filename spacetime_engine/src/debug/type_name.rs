use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::RwLock;
use once_cell::sync::Lazy;

type TypeNameFn = fn() -> &'static str;

static REGISTRY: Lazy<RwLock<HashMap<TypeId, TypeNameFn>>> = Lazy::new(|| {
    RwLock::new(HashMap::new())
});

fn register_type<T: 'static + Any + Send + Sync>() {
    let mut map = REGISTRY.write().unwrap();
    map.entry(TypeId::of::<T>()).or_insert(|| std::any::type_name::<T>());
}

/// To debug: Search `Box::new` and replace with `Box::new` and vice versa.
pub fn box_any<T: 'static + Any + Send + Sync>(val: T) -> Box<dyn Any + Send + Sync> {
    register_type::<T>();
    Box::new(val)
}

pub fn get_type_name(value: &dyn Any) -> &'static str {
    REGISTRY
        .read()
        .unwrap()
        .get(&value.type_id())
        .copied()
        .expect("Type not registered")
        ()
}
