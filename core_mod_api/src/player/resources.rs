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
pub struct PlayerControlSettings {
    pub first_person_fov_degrees: f32,
    pub mouse_look_sensitivity: f32,
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub sprint: KeyCode,
    pub look_left: KeyCode,
    pub look_right: KeyCode,
    pub look_up: KeyCode,
    pub look_down: KeyCode,
}
impl Default for PlayerControlSettings {
    fn default() -> Self {
        Self {
            first_person_fov_degrees: CONFIG().get::<f32>("player/first_person_fov_degrees"),
            mouse_look_sensitivity: CONFIG().get::<f32>("player/mouse_look_sensitivity"),
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            sprint: KeyCode::ShiftLeft,
            look_left: KeyCode::ArrowLeft,
            look_right: KeyCode::ArrowRight,
            look_up: KeyCode::ArrowUp,
            look_down: KeyCode::ArrowDown,
        }
    }
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
