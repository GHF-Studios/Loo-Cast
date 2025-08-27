use std::time::Instant;

use bevy::prelude::*;

#[derive(Resource, Debug, Clone, Reflect)]
#[reflect(Resource)]
pub struct GameTime {
    pub paused: bool,
    pub step_once: bool,
    pub time_scale: f32,
    pub(super) elapsed_seconds: f32,
    pub(super) delta_seconds: f32,
    pub(super) startup_timestamp: Instant,
    pub(super) update_timestamp: Instant,
}

impl Default for GameTime {
    fn default() -> Self {
        Self {
            paused: false,
            step_once: false,
            time_scale: 1.0,
            elapsed_seconds: 0.0,
            delta_seconds: 0.0,
            startup_timestamp: Instant::now(),
            update_timestamp: Instant::now(),
        }
    }
}

impl GameTime {
    pub fn elapsed_secs(&self) -> f32 {
        self.elapsed_seconds
    }

    pub fn delta_secs(&self) -> f32 {
        self.delta_seconds
    }
}

