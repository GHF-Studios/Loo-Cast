use bevy::prelude::Reflect;
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::sync::RwLock;

use crate::core::functions::asset_root;

use super::enums::ConfigValue;

#[derive(Debug, Reflect)]
pub struct Config {
    pub data: HashMap<String, ConfigValue>,
    #[reflect(ignore)]
    cache: RwLock<HashMap<String, HashMap<TypeId, Box<dyn Any + Send + Sync>>>>,
}

impl Config {
    /// Load configuration from a file
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut abs_path = PathBuf::from(path);
        if abs_path.is_relative() {
            abs_path = asset_root().join(path);
        }

        let toml_str = fs::read_to_string(&abs_path).unwrap_or_else(|_| panic!("Failed to load config at path: {:?}", abs_path));
        let raw_data: toml::Value = toml::from_str(&toml_str)?;

        let mut data = HashMap::new();
        Self::flatten("", &raw_data, &mut data)?;

        println!("Loaded config from {:?}: {:?}", abs_path, data);

        Ok(Self {
            data,
            cache: RwLock::new(HashMap::new()),
        })
    }

    /// Flatten nested TOML tables
    fn flatten(prefix: &str, value: &toml::Value, data: &mut HashMap<String, ConfigValue>) -> Result<(), String> {
        match value {
            toml::Value::Table(table) => {
                for (key, val) in table {
                    let new_prefix = if prefix.is_empty() { key.clone() } else { format!("{}/{}", prefix, key) };
                    Self::flatten(&new_prefix, val, data)?;
                }
            }
            toml::Value::Integer(i) => {
                data.insert(prefix.to_string(), ConfigValue::Integer(*i));
            }
            toml::Value::Float(f) => {
                data.insert(prefix.to_string(), ConfigValue::Float(*f));
            }
            toml::Value::Boolean(b) => {
                data.insert(prefix.to_string(), ConfigValue::Boolean(*b));
            }
            toml::Value::String(s) => {
                data.insert(prefix.to_string(), ConfigValue::String(s.clone()));
            }
            _ => {
                return Err(format!("Unsupported value at key: {}", prefix));
            }
        }
        Ok(())
    }

    // TODO: MAJOR: This has a critical design-flaw in regard to performance!!!
    // Change this design somehow, or like, add to it, to allow more "inlined" or actually "static" config values
    // The current design does a fair amount of stuff just to get a value that should ideally be  `const`.
    // Maybe we can perform some build.rs/macro trickery.

    /// Generic getter with caching
    pub fn get<T>(&self, path: &str) -> T
    where
        T: TryFrom<ConfigValue, Error = String> + Clone + Send + Sync + 'static,
    {
        // Attempt to retrieve from cache
        {
            let cache = self.cache.read().unwrap();
            if let Some(type_map) = cache.get(path) {
                if let Some(cached_value) = type_map.get(&TypeId::of::<T>()) {
                    return cached_value.downcast_ref::<T>().expect("Cached value type mismatch").clone();
                }
            }
        }

        // Compute the value if not cached
        let value = self.data.get(path).unwrap_or_else(|| unreachable!("Config key not found: {}", path)).clone();

        let typed_value: T = T::try_from(value).unwrap_or_else(|err| unreachable!("Type conversion error: {}", err));

        // Cache the computed value
        {
            let mut cache = self.cache.write().unwrap();
            let type_map = cache.entry(path.to_string()).or_default();
            type_map.insert(TypeId::of::<T>(), Box::new(typed_value.clone()));
        }

        typed_value
    }
}
