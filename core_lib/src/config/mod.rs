pub mod enums;
pub mod statics;
pub mod structs;

use bevy::prelude::*;
use enums::ConfigValue;
use structs::Config;

pub(crate) struct ConfigPlugin;
impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ConfigValue>().register_type::<Config>();
    }
}
