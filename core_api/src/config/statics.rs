use crate::statics::get_ref;
use super::structs::Config;

/// Wrapper around registry key "config"
pub fn config() -> &'static Config {
    get_ref("config")
}
