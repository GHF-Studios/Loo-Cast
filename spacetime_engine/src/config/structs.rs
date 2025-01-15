use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml::Value;

#[derive(Debug)]
pub struct CachedConfigs {
    configs: HashMap<String, Value>,
}

impl CachedConfigs {
    pub(in super) fn load_from_dir<P: AsRef<Path>>(dir: P) -> Result<Self, Box<dyn std::error::Error>> {
        let mut configs = HashMap::new();
        let dir = dir.as_ref();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("toml") {
                let file_name = path.file_stem().unwrap().to_string_lossy();
                let content = fs::read_to_string(&path)?;
                let parsed: Value = toml::from_str(&content)?;

                // Flatten TOML structure into dot-separated keys
                Self::flatten_toml(&parsed, &file_name, &mut configs);
            }
        }

        Ok(Self { configs })
    }

    pub fn get<T: std::str::FromStr>(&self, key: &str) -> T {
        match self.configs.get(key).and_then(|value| value.as_str()?.parse::<T>().ok()) {
            Some(config) => config,
            None => panic!("Missing config: {:?}", key)
        }
    }

    fn flatten_toml(value: &Value, prefix: &str, map: &mut HashMap<String, Value>) {
        match value {
            Value::Table(table) => {
                for (key, val) in table {
                    let new_prefix = format!("{}.{}", prefix, key);
                    Self::flatten_toml(val, &new_prefix, map);
                }
            }
            _ => {
                map.insert(prefix.to_string(), value.clone());
            }
        }
    }
}
