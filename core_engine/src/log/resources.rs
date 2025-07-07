use bevy::prelude::*;
use std::sync::{Arc, Mutex};

use crate::log::types::LogRegistry;

#[repr(transparent)]
#[derive(Resource, Clone)]
pub struct LogRegistryHandle(pub Arc<Mutex<LogRegistry>>);