use std::sync::OnceLock;

use crate::statics::get_ref;

use super::structs::Config;

pub fn init_config() -> Config {
    Config::from_file("core_api/configs/config.toml").unwrap()
}

static CONFIG_CACHE: OnceLock<&'static Config> = OnceLock::new();

pub fn config() -> &'static Config {
    CONFIG_CACHE.get_or_init(|| get_ref::<Config>("config"))
}
