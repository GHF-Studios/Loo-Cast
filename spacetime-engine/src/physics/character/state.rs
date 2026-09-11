use bevy::prelude::*;

/// Runtime contact state produced by [`super::CharacterMotor`].
#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct CharacterGroundState {
    pub grounded: bool,
    pub ground_entity: Option<Entity>,
    pub ground_normal: Vec3,
    pub just_landed: bool,
    pub just_left_ground: bool,
    pub just_jumped: bool,
}

impl Default for CharacterGroundState {
    fn default() -> Self {
        Self {
            grounded: false,
            ground_entity: None,
            ground_normal: Vec3::Y,
            just_landed: false,
            just_left_ground: false,
            just_jumped: false,
        }
    }
}
