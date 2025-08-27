use bevy::prelude::*;

#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub struct GameTimeControl {
    pub paused: bool,
    pub step_once: bool,
    pub speed: f32,
}
impl Default for GameTimeControl {
    fn default() -> Self {
        Self {
            paused: false,
            step_once: false,
            speed: 1.0,
        }
    }
}

