use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    devtools::{
        AppDeveloperToolsExt, DeveloperArtifact, DeveloperSet, DeveloperTools, DrawDepth,
        VisualizationId, VisualizationSpec, WorldDrawBatch, WorldDrawFrame,
    },
    ecs::UsfManifestationOf,
    ui::{UiTextRole, UiTheme},
};

use super::{SpatialDemandSnapshot, UsfPosition, UsfSpatialAnchor, UsfSpatialFrame, UsfSpatialSet};

const USF_SPATIAL_VISUALIZATION: VisualizationId = VisualizationId("usf_spatial");
pub(crate) const SPATIAL_DEMAND_VISUALIZATION: VisualizationId =
    VisualizationId("usf_spatial_demand");
const DEMAND_DEBUG_PROJECTION_BOUND: f32 = 1_000_000.0;

mod panel;
mod world_draw;

pub(super) fn configure(app: &mut App) {
    app.register_developer_visualization(VisualizationSpec::new(
        USF_SPATIAL_VISUALIZATION,
        "USF Spatial",
        25,
        false,
    ))
    .register_developer_visualization(VisualizationSpec::new(
        SPATIAL_DEMAND_VISUALIZATION,
        "Spatial demand / materialization",
        26,
        false,
    ))
    .add_systems(Startup, panel::spawn_debug_panel)
    .add_systems(
        PostUpdate,
        panel::update_debug_panel
            .after(UsfSpatialSet::Rebase)
            .in_set(DeveloperSet::RenderUi),
    )
    .add_systems(
        PostUpdate,
        world_draw::collect_spatial_demand_world_draw.in_set(DeveloperSet::CollectWorldDraw),
    );
}
