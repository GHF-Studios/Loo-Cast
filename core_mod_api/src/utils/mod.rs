pub mod i128vec2;
pub mod lifecycle_hook;
pub mod logic_safety;
pub mod premium_box;
pub mod progress;

use bevy::prelude::*;

pub(crate) struct UtilsPlugin;
impl Plugin for UtilsPlugin {
    fn build(&self, _app: &mut App) {}
}
