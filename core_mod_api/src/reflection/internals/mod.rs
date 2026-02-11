pub mod functions;
pub mod resources;
pub mod statics;
pub mod managed_traits;
pub mod traits;

use crate::bevy::prelude::*;

pub(crate) struct ReflectionPlugin;
impl Plugin for ReflectionPlugin {
    fn build(&self, app: &mut App) {
        functions::init(app);
    }
}