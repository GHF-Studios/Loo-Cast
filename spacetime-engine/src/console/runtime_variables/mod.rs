//! Typed runtime-variable adapters for the shared developer console.
//!
//! The registry is metadata and typed ingress, not configuration authority.
//! Semantic operations remain owned by causal ingress (#17).
//!
//! ## Module map
//!
//! - `commands`: Console ingress for typed runtime variables.
//! - `contract`: Typed variable contract and authority restrictions.
//! - `registry`: Path registration and lookup for typed runtime variables.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

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
