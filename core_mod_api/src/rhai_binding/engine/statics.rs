use std::collections::HashSet;
use std::sync::Mutex;

use core_mod_macros::export_static;
use once_cell::sync::Lazy;

export_static!(
    self,
    crate::rhai_binding::engine::statics::SCHEDULE_HOOKS: Lazy<Mutex<HashSet<String>>> = Lazy::new(Default::default)
);
