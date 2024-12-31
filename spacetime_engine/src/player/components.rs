use bevy::prelude::*;

#[derive(Component)]
pub struct PlayerComponent;
impl Default for PlayerComponent {
    fn default() -> Self {
        PlayerComponent
    }
}