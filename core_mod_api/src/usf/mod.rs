pub mod aspects;
pub mod components;
pub mod pos;
pub mod scale;
pub mod systems;

use bevy::prelude::*;

pub(crate) struct UsfPlugin;
impl Plugin for UsfPlugin {
    fn build(&self, _app: &mut App) {
    }
}
