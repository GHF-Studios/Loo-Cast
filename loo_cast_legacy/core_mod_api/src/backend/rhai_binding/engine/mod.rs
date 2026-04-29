mod preprocess;
pub mod resources;

use crate::bevy::prelude::*;
use crate::rhai_binding::bind::engine_ext::EngineExt;

pub struct RhaiEnginePlugin;
impl Plugin for RhaiEnginePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<resources::MainScriptEngineHandle>();
    }
}

pub(super) fn new_main_script_engine() -> rhai::Engine {
    let mut engine = rhai::Engine::new();
    engine.register_binding_graph();
    engine
}

pub(crate) fn preprocess_script_source(source: &str, source_name: &str) -> String {
    preprocess::preprocess_script_source(source, source_name)
}
