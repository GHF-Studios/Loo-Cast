use bevy::prelude::*;

#[derive(Component)]
pub struct MainCameraFollowComponent {
    pub target: Option<Entity>,
    pub speed: f32,
}
