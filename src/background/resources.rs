use bevy::prelude::*;

#[derive(Resource)]
pub struct BackgroundManager {
    pub background_origin_x: i32,
    pub background_origin_y: i32,
}

impl Default for BackgroundManager {
    fn default() -> BackgroundManager {
        BackgroundManager {
            background_origin_x: 0,
            background_origin_y: 0,
        }
    }
}
