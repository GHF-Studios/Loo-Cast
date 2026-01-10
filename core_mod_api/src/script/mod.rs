pub mod bindings;
pub mod internals;

use bevy::prelude::*;

pub(crate) struct ScriptPlugin;
impl Plugin for ScriptPlugin {
    fn build(&self, app: &mut App) {
        internals::functions::init(app);
    }
}