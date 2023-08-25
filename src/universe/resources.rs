use super::components::*;

use bevy::prelude::*;
use serde::*;

#[derive(Resource)]
pub struct Universe {
    pub name: String,
    pub scales: Vec<Scale>,
}
