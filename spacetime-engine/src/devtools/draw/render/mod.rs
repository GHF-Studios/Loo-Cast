use std::collections::{HashMap, HashSet};

use bevy::{asset::RenderAssetUsages, mesh::PrimitiveTopology, prelude::*};

use super::{DrawDepth, DrawId, ScalarFieldMode, WorldDrawFrame, WorldPrimitive, WorldScalarField};
use crate::devtools::{DeveloperArtifact, DeveloperSet};

#[derive(Default, Reflect, GizmoConfigGroup)]
struct DeveloperWorldGizmos;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct DeveloperOverlayGizmos;

mod primitives;
mod scalar_field;

use primitives::render_primitives;
use scalar_field::{setup_world_draw_backend, sync_scalar_field_visuals, WorldDrawCache};

pub(super) fn configure(app: &mut App) {
    app.init_gizmo_group::<DeveloperWorldGizmos>()
        .init_gizmo_group::<DeveloperOverlayGizmos>()
        .init_resource::<WorldDrawCache>()
        .add_systems(Startup, setup_world_draw_backend)
        .add_systems(
            PostUpdate,
            (render_primitives, sync_scalar_field_visuals).in_set(DeveloperSet::RenderWorldDraw),
        );
}
