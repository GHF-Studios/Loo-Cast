pub use core_mod_api;
pub use core_mod_macros;

use core_mod_macros::api_initializer;

api_initializer!(
    "core_mod_api",
    crate::core_mod_api::config::statics::CONFIG,
    crate::core_mod_api::core::statics::TOKIO_RUNTIME,
    crate::core_mod_api::core::statics::START_TIME,
    crate::core_mod_api::entity::statics::ENTITY_RESERVATION_BUFFER,
    crate::core_mod_api::logging::statics::LOG_ID_COUNTER,
    crate::core_mod_api::logging::statics::SPAN_EVENT_BUFFER,
    crate::core_mod_api::logging::statics::LOG_EVENT_BUFFER,
    crate::core_mod_api::time::statics::ELAPSED_VIRTUAL_NANOS,
    crate::core_mod_api::time::statics::PENDING_VIRTUAL_SLEEPS,
    crate::core_mod_api::workflow::statics::WORKFLOW_TOKIO_RUNTIME,
    crate::core_mod_api::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME,
);
