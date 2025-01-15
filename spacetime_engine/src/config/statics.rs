use lazy_static::lazy_static;

use super::structs::CachedConfigs;

lazy_static!{
    pub static ref CACHED_CONFIGS: CachedConfigs = CachedConfigs::load_from_dir("data/config").expect("Failed to load configs");
}