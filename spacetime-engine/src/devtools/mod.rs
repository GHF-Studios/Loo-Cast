//! Developer-facing inspection and visualization foundations.
//!
//! This is the replacement path for the current broad `observability` umbrella.
//! Migration is staged; legacy observability remains active until individual
//! responsibilities have working replacements.

mod focus;
mod inspect;
mod tools;

pub use focus::{DeveloperFocus, FocusHit, FocusTarget};
pub use inspect::{
    InspectField, InspectNumberFormat, InspectSection, InspectSectionId, InspectUnit,
    InspectValue, InspectionFrame,
};
pub use tools::{DeveloperTools, VisualizationId};

use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeveloperSet {
    ResolveFocus,
    CollectInspection,
    CollectWorldDraw,
    RenderUi,
    RenderWorldDraw,
}

pub struct DeveloperToolsPlugin;

impl Plugin for DeveloperToolsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DeveloperFocus>()
            .init_resource::<InspectionFrame>()
            .init_resource::<DeveloperTools>()
            .configure_sets(
                PostUpdate,
                (
                    DeveloperSet::ResolveFocus,
                    DeveloperSet::CollectInspection,
                    DeveloperSet::CollectWorldDraw,
                    DeveloperSet::RenderUi,
                    DeveloperSet::RenderWorldDraw,
                )
                    .chain(),
            )
            .add_systems(
                PostUpdate,
                inspect::clear_inspection_frame.in_set(DeveloperSet::ResolveFocus),
            );
    }
}
