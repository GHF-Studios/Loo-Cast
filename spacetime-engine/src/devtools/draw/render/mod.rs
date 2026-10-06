//! Render World Draw primitives and scalar fields from published frames.
//!
//! ## Module map
//!
//! - `primitives`: Gizmo realization of line/arrow/axes/rect/cross/sphere world-draw primitives.
//! - `scalar_field`: Scalar-field manifestation cache, synchronization and mesh construction.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

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
use scalar_field::{WorldDrawCache, setup_world_draw_backend, sync_scalar_field_visuals};

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
