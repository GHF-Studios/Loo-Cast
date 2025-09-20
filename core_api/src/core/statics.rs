use core_mod_macros::export_static;
use std::time::Instant;
use tokio::runtime::Runtime;

export_static!(self, crate::core::statics::TOKIO_RUNTIME: Runtime = Runtime::new().unwrap());
export_static!(self, crate::core::statics::START_TIME: Instant = Instant::now());
