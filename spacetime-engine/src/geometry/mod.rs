//! Hot-reloadable authored geometry asset/compiler/runtime facility.
//!
//! The engine intentionally knows about geometric primitives, parametric repetition,
//! markers, simple lights and simple kinematic motion. It does *not* know what a
//! "movement lab", "surf foundry" or "portal wing" is; those are authored-map concepts.
//!
//! ## Integration
//!
//! Authored assets compile into reusable scene descriptions. Runtime reconciliation creates
//! disposable manifestations and updates their motion; game scenarios supply the authored content.
//!
//! ## Module map
//!
//! - `asset`: Authored geometry asset schema, validation and loading.
//! - `compile`: Compile authored map definitions into runtime geometry and lighting inputs.
//! - `mesh`: Construct reusable convex-prism geometry for authored scenes.
//! - `runtime`: Reconcile authored scenes with disposable runtime entities and motion.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

mod asset;
mod compile;
mod mesh;
mod runtime;

pub use asset::{AuthoredMap, ZoneDef};
pub use runtime::{AuthoredMapMarker, AuthoredMapObject, AuthoredMapScene};

use bevy::prelude::*;

use crate::spatial::UsfSpatialSet;
use asset::AuthoredMapLoader;
use runtime::{
    rebase_authored_motion_origins, reconcile_authored_map_scenes, simulate_authored_motion,
};

pub struct AuthoredGeometryPlugin;

impl Plugin for AuthoredGeometryPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AuthoredMap>()
            .init_asset_loader::<AuthoredMapLoader>()
            .add_systems(Update, reconcile_authored_map_scenes)
            .add_systems(
                PostUpdate,
                rebase_authored_motion_origins.in_set(UsfSpatialSet::RuntimeProjection),
            )
            .add_systems(FixedUpdate, simulate_authored_motion);
    }
}
