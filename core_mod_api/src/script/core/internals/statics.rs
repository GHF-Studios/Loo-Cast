use core_mod_macros::export_static;
use once_cell::sync::Lazy;
use std::collections::HashSet;
use std::sync::Mutex;

export_static!(self, crate::core_mod_api::script::statics::SCHEDULE_HOOKS: Lazy<Mutex<HashSet<String>>> = Lazy::new(Default::default));