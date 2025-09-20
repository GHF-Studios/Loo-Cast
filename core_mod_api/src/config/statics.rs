use core_mod_macros::export_static;

use super::structs::Config;

export_static!(self, crate::core_mod_api::config::statics::CONFIG: Config = Config::from_file("core_mod/configs/config.toml").unwrap());
