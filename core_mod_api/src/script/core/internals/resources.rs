use crate::bevy::prelude::*;

use super::functions::new_main_script_engine;

#[repr(transparent)]
#[derive(Resource)]
pub struct MainScriptEngineHandle(pub rhai::Engine);
impl Default for MainScriptEngineHandle {
    fn default() -> Self {
        MainScriptEngineHandle(new_main_script_engine())
    }
}