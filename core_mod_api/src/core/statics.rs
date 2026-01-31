use core_mod_core_macros::export_static;
use std::time::Instant;
use tokio::runtime::Runtime;

export_static!(self, crate::core_mod_api::core::statics::TOKIO_RUNTIME: Runtime = Runtime::new().unwrap());
export_static!(self, crate::core_mod_api::core::statics::START_TIME: Instant = Instant::now());
