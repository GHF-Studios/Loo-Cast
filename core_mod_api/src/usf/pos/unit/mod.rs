pub mod tests;
pub mod types;

use bevy::prelude::*;

use types::UnitVec;

pub(crate) struct UnitPlugin;
impl Plugin for UnitPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UnitVec>();
    }
}
