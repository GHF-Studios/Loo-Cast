use bevy::prelude::*;
use std::sync::Arc;

use crate::log_NEW::types::LogStorage;

#[repr(transparent)]
#[derive(Resource, Clone)]
pub struct LogStorageHandle(pub Arc<LogStorage>);