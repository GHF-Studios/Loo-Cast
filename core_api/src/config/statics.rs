use once_cell::sync::OnceCell;

use crate::statics::get_ref;

use super::structs::Config;

pub fn init_config() -> Config {
    Config::from_file("core_api/configs/config.toml").unwrap()
}

pub fn config() -> &'static Config {
    get_ref("config")
}
