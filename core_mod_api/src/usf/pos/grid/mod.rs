pub mod tests;
pub mod types;

use bevy::prelude::*;
use bevy_inspector_egui::inspector_egui_impls::InspectorEguiImpl;

use types::GridVec;

pub(crate) struct GridPlugin;
impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<GridVec>()
            .register_type_data::<GridVec, InspectorEguiImpl>();
    }
}