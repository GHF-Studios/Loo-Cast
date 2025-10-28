pub mod tests;
pub mod types;

use bevy::prelude::*;

use types::GridVec;

pub(crate) struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GridVec>();
    }
}