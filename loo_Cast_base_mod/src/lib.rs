pub mod test;

use test::*;
use bevy::app::*;
use std::any::*;
use spacetime_engine::*;

pub struct LooCastBaseModPlugins;

impl Mod for LooCastBaseModPlugins {
    fn id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn dependencies(&self) -> Vec<TypeId> {
        vec![]
    }

    fn register_mod(&self, app: &mut App) {
        app.add_plugins(TestPlugin);
    }
}

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn get_mod() -> Box<dyn Mod> {
    Box::new(LooCastBaseModPlugins {}) as Box<dyn Mod>
}