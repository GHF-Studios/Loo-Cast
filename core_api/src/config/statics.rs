use core_api_macros::export_static;

use super::structs::Config;

export_static!(self, crate::config::statics::CONFIG: Config = Config::from_file("core_api/configs/config.toml").unwrap());
