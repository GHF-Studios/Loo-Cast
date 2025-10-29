use bevy::prelude::*;

use super::grid::types::GridVec;

#[derive(Component, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Component)]
#[repr(transparent)]
pub struct OriginOffset(pub GridVec);