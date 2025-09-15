use core_lib::*;
use core_runtime_api::CoreRuntimeApi;
use core_mod_macros::*;
use bevy::prelude::*;

#[no_mangle]
pub extern "C" fn init_mod(app: &mut App, runtime_api: *mut CoreRuntimeApi) {
    // unsafe {
    //     MY_RUNTIME_API = Some(&*runtime_api);
    // }
    app.add_systems(Update, || { println!("INITIALIZED MOD :D") });
}