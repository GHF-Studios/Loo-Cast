use std::collections::HashMap;

use avian3d::prelude::{Collider, LinearVelocity, RigidBody};
use bevy::prelude::*;

use crate::physics::collision_topology::CollisionClipSource;

use super::{
    asset::AuthoredMap,
    compile::{CompiledGeometry, CompiledMotion, CompiledNode, CompiledShape, compile_map},
    mesh::{convex_prism_mesh, convex_prism_points},
};

#[derive(Component, Debug, Clone)]
pub struct AuthoredMapScene {
    handle: Handle<AuthoredMap>,
    dirty: bool,
}

impl AuthoredMapScene {
    pub fn new(handle: Handle<AuthoredMap>) -> Self {
        Self {
            handle,
            dirty: true,
        }
    }

    pub fn handle(&self) -> &Handle<AuthoredMap> {
        &self.handle
    }
}

/// Metadata preserved on each generated geometry entity for inspection/debug tooling.
#[derive(Component, Debug, Clone)]
pub struct AuthoredMapObject {
    pub id: String,
    pub zone: String,
    pub tags: Vec<String>,
}

/// A data-authored non-rendering anchor. Gameplay systems may opt into specific marker kinds
/// without the authored-map runtime itself knowing what they mean.
#[derive(Component, Debug, Clone)]
pub struct AuthoredMapMarker {
    pub id: String,
    pub zone: String,
    pub kind: String,
    pub tags: Vec<String>,
}

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct GeneratedFromMap {
    source: Entity,
}

#[derive(Component, Debug, Clone, Copy)]
pub(super) struct AuthoredMotion {
    base: Transform,
    travel: Vec3,
    period_seconds: f32,
    phase: f32,
    elapsed_seconds: f32,
}

mod motion;
mod rebuild;
mod spawning;

pub(super) use motion::animate_authored_movers;
pub(super) use rebuild::rebuild_authored_maps;

use motion::{authored_motion, motion_state};
use spawning::spawn_geometry;
