use super::components::*;

use bevy::prelude::*;

#[derive(Resource)]
pub struct UniverseManager {
    pub scales: Vec<Scale>,
}
