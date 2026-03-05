pub mod aspects;
pub mod phenomenon;
pub mod pos;
pub mod scale;
pub mod transform;

use crate::bevy::prelude::*;

pub(crate) struct UsfPlugin;
impl Plugin for UsfPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(pos::PosPlugin).add_plugins(transform::TransformPlugin);
    }
}
