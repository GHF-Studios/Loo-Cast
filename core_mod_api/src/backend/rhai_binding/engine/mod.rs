pub mod bootstrap;
mod preprocess;
pub mod resources;
pub mod schedule_entrypoint;
pub mod statics;

use crate::bevy::prelude::*;

pub struct RhaiEnginePlugin;
impl Plugin for RhaiEnginePlugin {
    fn build(&self, app: &mut App) {
        bootstrap::build(app);
    }
}
