use crate::bevy::prelude::*;

use super::types::Axis3D;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct GizmoRoot;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct GizmoArrow {
    pub axis: Axis3D,
}
