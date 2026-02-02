pub mod core;
pub mod ecs;
pub mod usf;

use crate::bevy::prelude::*;

pub(crate) struct ScriptPlugin;
impl Plugin for ScriptPlugin {
    fn build(&self, app: &mut App) {
        core::internals::functions::init(app);
    }
}