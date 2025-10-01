pub mod components;
pub mod functions;
pub mod premium_box;
pub mod progress;
pub mod types;

use bevy::prelude::*;

pub(crate) struct UtilsPlugin;
impl Plugin for UtilsPlugin {
    fn build(&self, _app: &mut App) {}
}
