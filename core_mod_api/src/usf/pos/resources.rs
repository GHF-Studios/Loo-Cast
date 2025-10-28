use bevy::prelude::*;

use super::grid::types::GridVec;

#[derive(Resource, Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
#[reflect(Resource)]
#[repr(transparent)]
pub struct OriginOffset(pub GridVec);