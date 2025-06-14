use bevy::prelude::*;
use std::sync::{Arc, Mutex};

#[repr(transparent)]
#[derive(Resource, Clone)]
pub struct LogTreeHandle(pub Arc<Mutex<super::types::LogTree>>);