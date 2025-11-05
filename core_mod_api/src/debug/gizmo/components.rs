use bevy::prelude::*;

use super::types::Axis2D;

#[derive(Component)]
pub(super) struct GizmoArrow {
    pub axis: Axis2D,
}