use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fs;
use std::sync::RwLock;

use super::enums::ConfigValue;

#[derive(Debug)]
pub struct Config {
    pub data: HashMap<String, ConfigValue>,
    cache: RwLock<HashMap<String, HashMap<TypeId, Box<dyn Any + Send + Sync>>>>,
}

impl Config {
    /// Load configuration from a file
    pub fn from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let toml_str = fs::read_to_string(path)?;
        let raw_data: toml::Value = toml::from_str(&toml_str)?;

        let mut data = HashMap::new();
        Self::flatten("", &raw_data, &mut data)?;

        Ok(Self { 
            data,
            cache: RwLock::new(HashMap::new()),
        })
    }

    /// Flatten nested TOML tables
    fn flatten(
        prefix: &str,
        value: &toml::Value,
        data: &mut HashMap<String, ConfigValue>,
    ) -> Result<(), String> {
        match value {
            toml::Value::Table(table) => {
                for (key, val) in table {
                    let new_prefix = if prefix.is_empty() {
                        key.clone()
                    } else {
                        format!("{}/{}", prefix, key)
                    };
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
                    return cached_value.downcast_ref::<T>()
                        .expect("Cached value type mismatch")
                        .clone();
                }
            }
        }

        // Compute the value if not cached
        let value = self
            .data
            .get(path)
            .unwrap_or_else(|| panic!("Config key not found: {}", path))
            .clone();

        let typed_value: T = T::try_from(value)
            .unwrap_or_else(|err| panic!("Type conversion error: {}", err));

        // Cache the computed value
        {
            let mut cache = self.cache.write().unwrap();
            let type_map = cache.entry(path.to_string()).or_default();
            type_map.insert(TypeId::of::<T>(), Box::new(typed_value.clone()));
        }

        typed_value
    }
}