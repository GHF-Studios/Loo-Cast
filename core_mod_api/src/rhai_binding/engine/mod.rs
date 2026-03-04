pub mod bootstrap;
pub mod hook;
mod preprocess;
pub mod resources;
pub mod statics;

use crate::bevy::prelude::*;

pub struct RhaiEnginePlugin;
impl Plugin for RhaiEnginePlugin {
    fn build(&self, app: &mut App) {
        bootstrap::build(app);
    }
}
