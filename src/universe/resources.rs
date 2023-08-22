use super::components::*;

use bevy::prelude::*;
use serde::*;

#[derive(Resource, Serialize, Deserialize)]
pub struct Universe {
    pub name: String
}