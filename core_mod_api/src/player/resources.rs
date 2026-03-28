use crate::bevy::prelude::*;

use crate::config::statics::CONFIG;

#[derive(Resource, Reflect, Debug, Clone, Copy, PartialEq, Eq, Default)]
#[reflect(Resource)]
pub enum PlayerCameraMode {
    #[default]
    FirstPerson,
    ThirdPerson,
}

#[derive(Resource, Reflect, Debug, Clone, Copy)]
#[reflect(Resource)]
pub struct PlayerLookState {
    pub pitch_radians: f32,
}
impl Default for PlayerLookState {
    fn default() -> Self {
        Self { pitch_radians: 0.0 }
    }
}

#[derive(Resource, Reflect, Debug, Clone, Copy)]
#[reflect(Resource)]
pub struct PlayerCameraRigSettings {
    pub first_person_camera_height: f32,
    pub third_person_follow_distance: f32,
    pub third_person_camera_height: f32,
    pub first_person_camera_smoothness: f32,
    pub third_person_camera_smoothness: f32,
}
impl Default for PlayerCameraRigSettings {
    fn default() -> Self {
        Self {
            first_person_camera_height: CONFIG().get::<f32>("player/first_person_camera_height"),
            third_person_follow_distance: CONFIG().get::<f32>("player/third_person_follow_distance"),
            third_person_camera_height: CONFIG().get::<f32>("player/third_person_camera_height"),
            first_person_camera_smoothness: CONFIG().get::<f32>("player/first_person_camera_smoothness"),
            third_person_camera_smoothness: CONFIG().get::<f32>("player/third_person_camera_smoothness"),
        }
    }
}
