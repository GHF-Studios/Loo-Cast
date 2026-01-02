pub mod binding;
pub mod core;
pub mod internal;

use bevy::prelude::*;

pub(crate) struct ScriptPlugin;
impl Plugin for ScriptPlugin {
    fn build(&self, app: &mut App) {
        core::functions::init(app);
    }
}