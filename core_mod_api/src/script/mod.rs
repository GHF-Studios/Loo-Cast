pub mod functions;
pub mod resources;
pub mod statics;
pub mod types;

use bevy::prelude::*;
use functions::new_hook_runner_system;

use crate::script::statics::SCHEDULE_HOOK_HANDLERS;

pub(crate) struct ScriptPlugin;
impl Plugin for ScriptPlugin {
    fn build(&self, app: &mut App) {
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
}
