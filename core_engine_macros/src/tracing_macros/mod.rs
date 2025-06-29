macro_rules! log_span {
    ($name:expr) => {
        tracing::info_span!(crate_name = env!("CARGO_PKG_NAME"), name = $name)
    };
}

macro_rules! log_trace {
    ($($tt:tt)*) => {
        bevy::prelude::trace!(crate_name = env!("CARGO_PKG_NAME"), $($tt)*)
    };
}

macro_rules! log_debug {
    ($($tt:tt)*) => {
        bevy::prelude::debug!(crate_name = env!("CARGO_PKG_NAME"), $($tt)*)
    };
}

macro_rules! log_info {
    ($($tt:tt)*) => {
        bevy::prelude::info!(crate_name = env!("CARGO_PKG_NAME"), $($tt)*)
    };
}

macro_rules! log_warn {
    ($($tt:tt)*) => {
        bevy::prelude::warn!(crate_name = env!("CARGO_PKG_NAME"), $($tt)*)
    };
}

macro_rules! log_error {
    ($($tt:tt)*) => {
        bevy::prelude::error!(crate_name = env!("CARGO_PKG_NAME"), $($tt)*)
    };
}