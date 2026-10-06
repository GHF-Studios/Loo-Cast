//! Typed runtime-variable adapters for the shared developer console.
//!
//! The registry is metadata and typed ingress, not configuration authority.
//! Semantic operations remain owned by causal ingress (#17).

use super::{
    AppConsoleExt, ConsoleArgumentCompletion, ConsoleCommandInvocation, ConsoleCommandResult,
    ConsoleCommandSpec,
};
use bevy::prelude::*;
use std::collections::BTreeMap;

mod commands;
mod contract;
mod registry;

pub(super) use commands::configure;
pub use contract::{
    RuntimeVariableAuthority, RuntimeVariableBinding, RuntimeVariableDomain, RuntimeVariableSpec,
    RuntimeVariableValueType,
};
use registry::normalize_path;
pub use registry::{AppRuntimeVariableExt, RuntimeVariableRegistry};
