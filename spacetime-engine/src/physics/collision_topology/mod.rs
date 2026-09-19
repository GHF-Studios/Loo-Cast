//! Runtime collision-space modification.
//!
//! Higher-level mechanics publish [`CollisionStencil`] components. Hosts keep
//! immutable [`CollisionClipSource`] geometry. This module rebuilds the actual
//! Avian collider only when the effective stencil set changes.

mod csg;
mod source;
mod stencil_fit;

use std::collections::{HashMap, HashSet};

use avian3d::prelude::Collider;
use bevy::{ecs::lifecycle::RemovedComponents, prelude::*};

use csg::{RectangularCut, subtract_rectangular_cuts_from_cuboid};

pub use source::CollisionClipSource;
pub use stencil_fit::{
    RectangularStencilFit, fit_rectangular_stencil, supports_rectangular_stencil,
};

/// A bounded subtractive region in collision space.
///
/// The stencil transform and dimensions are world-space gameplay state. The
/// target host keeps its own transform and immutable [`CollisionClipSource`].
#[derive(Component, Debug, Clone, Copy)]
pub struct CollisionStencil {
    pub enabled: bool,
    pub target: Option<Entity>,
    pub transform: Transform,
    pub half_size: Vec2,
    /// Small expansion around the visible aperture used to prevent numerical
    /// lips from snagging movers exactly on the boundary.
    pub clearance: f32,
}

impl CollisionStencil {
    pub fn rectangular(transform: Transform, half_size: Vec2, clearance: f32) -> Self {
        Self {
            enabled: false,
            target: None,
            transform,
            half_size,
            clearance: clearance.max(0.0),
        }
    }
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub(super) struct AppliedCollisionTopology {
    fingerprint: u64,
}

/// Change-driven index of the currently effective stencil topology.
///
/// Stencils are authored gameplay state. Rebuilding this index from every
/// stencil and scanning every clip host each frame made unchanged topology pay
/// a permanent polling cost. The index instead records component changes and
/// marks only affected hosts dirty.
#[derive(Default)]
pub(super) struct CollisionTopologyState {
    stencil_targets: HashMap<Entity, Option<Entity>>,
    stencils_by_target: HashMap<Entity, HashMap<Entity, CollisionStencil>>,
    dirty_hosts: HashSet<Entity>,
}

impl CollisionTopologyState {
    fn update_stencil(&mut self, entity: Entity, stencil: CollisionStencil) {
        let target = if stencil.enabled { stencil.target } else { None };

        if let Some(previous) = self.stencil_targets.insert(entity, target).flatten() {
            self.remove_from_target(previous, entity);
        }

        if let Some(target) = target {
            self.stencils_by_target
                .entry(target)
                .or_default()
                .insert(entity, stencil);
            self.dirty_hosts.insert(target);
        }
    }

    fn remove_stencil(&mut self, entity: Entity) {
        if let Some(target) = self.stencil_targets.remove(&entity).flatten() {
            self.remove_from_target(target, entity);
        }
    }

    fn remove_from_target(&mut self, target: Entity, stencil: Entity) {
        let remove_bucket = if let Some(bucket) = self.stencils_by_target.get_mut(&target) {
            bucket.remove(&stencil);
            bucket.is_empty()
        } else {
            false
        };
        if remove_bucket {
            self.stencils_by_target.remove(&target);
        }
        self.dirty_hosts.insert(target);
    }

    fn effective_stencils(&self, target: Entity) -> Vec<(Entity, CollisionStencil)> {
        let Some(stencils) = self.stencils_by_target.get(&target) else {
            return Vec::new();
        };

        let mut effective = stencils
            .iter()
            .map(|(&entity, &stencil)| (entity, stencil))
            .collect::<Vec<_>>();
        effective.sort_unstable_by_key(|(entity, _)| entity.to_bits());
        effective
    }
}

/// Rebuilds only hosts whose effective stencil set changed.
///
/// The collider presented to Avian is the real collision topology: character
/// casts, ground probing, rigid-body contacts and ordinary spatial queries all
/// observe the same hole without portal-specific filters.
pub(super) fn rebuild_clipped_colliders(
    mut commands: Commands,
    mut removed_stencils: RemovedComponents<CollisionStencil>,
    changed_stencils: Query<(Entity, &CollisionStencil), Changed<CollisionStencil>>,
    changed_hosts: Query<
        Entity,
        (
            With<CollisionClipSource>,
            Or<(Changed<CollisionClipSource>, Changed<Transform>)>,
        ),
    >,
    mut hosts: Query<(
        Entity,
        &CollisionClipSource,
        &Transform,
        &mut Collider,
        Option<&mut AppliedCollisionTopology>,
    )>,
    mut topology: Local<CollisionTopologyState>,
) {
    for entity in removed_stencils.read() {
        topology.remove_stencil(entity);
    }
    for (entity, stencil) in &changed_stencils {
        topology.update_stencil(entity, *stencil);
    }
    topology.dirty_hosts.extend(changed_hosts.iter());

    let dirty_hosts = std::mem::take(&mut topology.dirty_hosts);
    for entity in dirty_hosts {
        let Ok((_, source, transform, mut collider, applied)) = hosts.get_mut(entity) else {
            continue;
        };

        let effective = topology.effective_stencils(entity);
        let fingerprint = topology_fingerprint(transform, &effective);

        if applied
            .as_ref()
            .is_some_and(|applied| applied.fingerprint == fingerprint)
        {
            continue;
        }

        *collider = clipped_collider(*source, transform, &effective);

        if let Some(mut applied) = applied {
            applied.fingerprint = fingerprint;
        } else {
            commands
                .entity(entity)
                .insert(AppliedCollisionTopology { fingerprint });
        }
    }
}

fn clipped_collider(
    source: CollisionClipSource,
    host: &Transform,
    stencils: &[(Entity, CollisionStencil)],
) -> Collider {
    if stencils.is_empty() {
        return source.original_collider();
    }

    match source {
        CollisionClipSource::Cuboid { half_extents } => {
            let cuts = stencils.iter().map(|(_, stencil)| RectangularCut {
                transform: stencil.transform,
                half_size: stencil.half_size,
                clearance: stencil.clearance,
            });

            subtract_rectangular_cuts_from_cuboid(half_extents, host, cuts)
                .unwrap_or_else(|| source.original_collider())
        }
    }
}

fn topology_fingerprint(
    host: &Transform,
    stencils: &[(Entity, CollisionStencil)],
) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;

    // World-space stencil geometry is converted relative to the host transform,
    // so a moving host is itself a topology change while a stencil is active.
    if !stencils.is_empty() {
        for value in [
            host.translation.x,
            host.translation.y,
            host.translation.z,
            host.rotation.x,
            host.rotation.y,
            host.rotation.z,
            host.rotation.w,
            host.scale.x,
            host.scale.y,
            host.scale.z,
        ] {
            hash_value(&mut hash, value.to_bits() as u64);
        }
    }

    for (entity, stencil) in stencils {
        hash_value(&mut hash, entity.to_bits());
        for value in [
            stencil.transform.translation.x,
            stencil.transform.translation.y,
            stencil.transform.translation.z,
            stencil.transform.rotation.x,
            stencil.transform.rotation.y,
            stencil.transform.rotation.z,
            stencil.transform.rotation.w,
            stencil.half_size.x,
            stencil.half_size.y,
            stencil.clearance,
        ] {
            hash_value(&mut hash, value.to_bits() as u64);
        }
    }
    hash
}

fn hash_value(hash: &mut u64, value: u64) {
    *hash ^= value;
    *hash = hash.wrapping_mul(0x100000001b3);
}
