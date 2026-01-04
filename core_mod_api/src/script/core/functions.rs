use bevy::prelude::*;

use super::super::internal::{
    functions::new_hook_runner_system,
    resources::MainScriptEngineHandle,
};
use super::statics::SCHEDULE_HOOK_HANDLERS;

pub fn pre_init(world: &mut World) {
    world.init_resource::<MainScriptEngineHandle>();
}

pub(in super::super) fn init(app: &mut App) {
    let path = "assets/scripts/core/schedule_hook_handlers/";
    for name in SCHEDULE_HOOK_HANDLERS().lock().unwrap().drain() {
        match name.as_str() {
            "pre_startup" => {
                let my_path = format!("{path}{name}.rhai");
                app.add_systems(PreStartup, new_hook_runner_system(my_path));
            }
            "startup" => {
                let my_path = format!("{path}{name}.rhai");
                app.add_systems(Startup, new_hook_runner_system(my_path));
            }
            "post_startup" => {
                let my_path = format!("{path}{name}.rhai");
                app.add_systems(PostStartup, new_hook_runner_system(my_path));
            }
            "first" => {
                let my_path = format!("{path}{name}.rhai");
                app.add_systems(First, new_hook_runner_system(my_path));
            }
            "pre_update" => {
                let my_path = format!("{path}{name}.rhai");
                app.add_systems(PreUpdate, new_hook_runner_system(my_path));
            }
            "update" => {
                let my_path = format!("{path}{name}.rhai");
                app.add_systems(Update, new_hook_runner_system(my_path));
            }
            "post_update" => {
                let my_path = format!("{path}{name}.rhai");
                app.add_systems(PostUpdate, new_hook_runner_system(my_path));
            }
            "last" => {
                let my_path = format!("{path}{name}.rhai");
                app.add_systems(Last, new_hook_runner_system(my_path));
            }
            unknown => {
                panic!("Schedule name '{unknown}' is not known!");
            }
        }
    }
}