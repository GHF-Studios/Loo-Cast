use crate::bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;
impl Default for Player {
    fn default() -> Self {
        Self
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerVisual3dLink {
    pub entity: Entity,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerSpawnRecovery {
    pub active: bool,
    pub frames_overlapping: u32,
    pub preferred_push_direction: Vec3,
}
impl Default for PlayerSpawnRecovery {
    fn default() -> Self {
        Self {
            active: false,
            frames_overlapping: 0,
            preferred_push_direction: Vec3::Z,
        }
    }
}
