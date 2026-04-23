use crate::bevy::prelude::*;

use super::types::DebugObjectMovement;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct DebugObjectComponent {
    pub movement: DebugObjectMovement,
}
