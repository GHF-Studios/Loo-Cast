use core_mod_macros::*;
use core_lib::*;

use bevy::prelude::*;

#[unsafe(no_mangle)]
pub extern "C" fn init_mod(app: &mut App) {
    app.add_systems(Update, || { panic!("INITIALIZED MOD :D") });
}

