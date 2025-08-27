use bevy::prelude::*;

use super::types::PauseState;

#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub struct GameTimeInfo {
    pub pause_state: PauseState,
}
impl Default for GameTimeInfo {
    fn default() -> Self {
        Self {
            pause_state: PauseState::Running,
        }
    }
}
