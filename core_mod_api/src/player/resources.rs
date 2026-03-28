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
    pub third_person_mouse_look_sensitivity: f32,
    pub move_forward: KeyCode,
    pub move_backward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_up: KeyCode,
    pub move_down: KeyCode,
    pub sprint: KeyCode,
    pub roll_left: KeyCode,
    pub roll_right: KeyCode,
    pub invert_move_x_axis: bool,
    pub invert_move_y_axis: bool,
    pub invert_move_z_axis: bool,
    pub invert_look_x_axis: bool,
    pub invert_look_y_axis: bool,
    pub invert_third_person_look_x_axis: bool,
    pub invert_third_person_look_y_axis: bool,
    pub invert_roll_axis: bool,
}
impl Default for PlayerControlSettings {
    fn default() -> Self {
        Self {
            first_person_fov_degrees: CONFIG().get::<f32>("player/first_person_fov_degrees"),
            mouse_look_sensitivity: CONFIG().get::<f32>("player/mouse_look_sensitivity"),
            third_person_mouse_look_sensitivity: CONFIG().get::<f32>("player/third_person_mouse_look_sensitivity"),
            move_forward: KeyCode::KeyW,
            move_backward: KeyCode::KeyS,
            move_left: KeyCode::KeyA,
            move_right: KeyCode::KeyD,
            move_up: KeyCode::Space,
            move_down: KeyCode::ControlLeft,
            sprint: KeyCode::ShiftLeft,
            roll_left: KeyCode::KeyQ,
            roll_right: KeyCode::KeyE,
            invert_move_x_axis: false,
            invert_move_y_axis: false,
            invert_move_z_axis: false,
            invert_look_x_axis: false,
            invert_look_y_axis: false,
            invert_third_person_look_x_axis: false,
            invert_third_person_look_y_axis: false,
            invert_roll_axis: false,
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone, Copy)]
#[reflect(Resource)]
pub struct PlayerLookState {
    pub yaw_radians: f32,
    pub pitch_radians: f32,
    pub roll_radians: f32,
}
impl Default for PlayerLookState {
    fn default() -> Self {
        Self {
            yaw_radians: 0.0,
            pitch_radians: 0.0,
            roll_radians: 0.0,
        }
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
