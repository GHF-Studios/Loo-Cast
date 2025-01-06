use bevy::prelude::*;

#[derive(Component)]
pub struct MainCameraFollow {
    pub target: Option<Entity>,
    pub speed: f32,
}
