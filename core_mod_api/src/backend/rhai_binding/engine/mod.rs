mod preprocess;
pub mod resources;

use crate::bevy::prelude::*;

pub struct RhaiEnginePlugin;
impl Plugin for RhaiEnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::MainScriptEngineHandle>();
    }
}

pub(super) fn new_main_script_engine() -> rhai::Engine {
    rhai::Engine::new()
}
