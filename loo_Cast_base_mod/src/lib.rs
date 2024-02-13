pub mod test;

use bevy::app::*;
use spacetime_engine::*;
use std::any::*;
use test::*;

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

#[no_mangle]
pub extern "C" fn get_mod() -> *mut dyn Mod {
    println!("1");
    let loo_cast_base_mod_plugins = LooCastBaseModPlugins {};

    println!("2");
    let loo_cast_base_mod_plugins = Box::new(loo_cast_base_mod_plugins);

    println!("3");
    let loo_cast_base_mod_plugins = Box::into_raw(loo_cast_base_mod_plugins) as *mut dyn Mod;

    println!("4");
    loo_cast_base_mod_plugins
}
