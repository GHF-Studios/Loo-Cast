pub mod bindings;
mod internal;

use bevy::prelude::*;

pub(crate) struct ScriptPlugin;
impl Plugin for ScriptPlugin {
    fn build(&self, app: &mut App) {
        internal::functions::init(app);
    }
}