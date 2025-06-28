use bevy::prelude::*;
use std::sync::Arc;

use crate::log_NEW::types::LogRegistry;

#[repr(transparent)]
#[derive(Resource, Clone)]
pub struct LogRegistryHandle(pub Arc<LogRegistry>);