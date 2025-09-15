use lazy_static::lazy_static;

use super::structs::Config;

lazy_static! {
    pub static ref CONFIG: Config = Config::from_file("core_lib/configs/config.toml").expect("Failed to load config");
}
