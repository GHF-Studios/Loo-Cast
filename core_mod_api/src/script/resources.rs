use bevy::prelude::*;

use super::functions::boot_main_script_engine;

#[repr(transparent)]
#[derive(Resource)]
pub struct MainScriptEngineHandle(pub rhai::Engine);
impl Default for MainScriptEngineHandle {
    fn default() -> Self {
        MainScriptEngineHandle(boot_main_script_engine())
    }
}