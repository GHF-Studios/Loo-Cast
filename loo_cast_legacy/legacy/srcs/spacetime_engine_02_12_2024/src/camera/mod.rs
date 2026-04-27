// Data types

// Functions

// Integrations
pub mod commands;

// Miscelaneous

use bevy::prelude::*;
use crate::traits::*;
use crate::structs::*;

pub(in crate) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, _app: &mut App) {
    }
}
impl LockingNodeData for CameraPlugin {
    fn on_insert(&mut self, _hierarchy: &mut LockingHierarchy) {
    }

    fn on_remove(&mut self, _hierarchy: &mut LockingHierarchy) {
    }
}