//! Hot-reloadable, human-authored runtime geometry.
//!
//! The engine intentionally knows about geometric primitives, parametric repetition,
//! markers, simple lights and simple kinematic motion. It does *not* know what a
//! "movement lab", "surf foundry" or "portal wing" is; those are authored-map concepts.

mod asset;
mod compile;
mod mesh;
mod runtime;

pub use asset::{AuthoredMap, ZoneDef};
pub use runtime::{AuthoredMapMarker, AuthoredMapObject, AuthoredMapScene};

use bevy::prelude::*;

use asset::AuthoredMapLoader;
use runtime::{animate_authored_movers, rebuild_authored_maps};

pub struct AuthoredGeometryPlugin;

impl Plugin for AuthoredGeometryPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<AuthoredMap>()
            .init_asset_loader::<AuthoredMapLoader>()
            .add_systems(Update, rebuild_authored_maps)
            .add_systems(FixedUpdate, animate_authored_movers);
    }
}
