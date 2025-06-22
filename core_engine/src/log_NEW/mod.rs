pub mod resources;
pub mod statics;
pub mod traits;
pub mod types;

use bevy::prelude::*;

use crate::log::{resources::*, statics::LOG_STORAGE_HANDLE, systems::*};

pub(crate) struct LogPlugin;
impl Plugin for LogPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LOG_STORAGE_HANDLE.clone());
    }
}