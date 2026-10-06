//! Authored stencil state, change index and effective-topology fingerprint.

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

/// A bounded subtractive region in collision space.
///
/// The stencil transform and dimensions are world-space gameplay state. The
/// target host keeps its own transform and immutable [`crate::physics::collision_topology::CollisionClipSource`].
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
pub(in crate::physics) struct AppliedCollisionTopology {
    pub(super) fingerprint: u64,
}

/// Change-driven index of the currently effective stencil topology.
///
/// Stencils are authored gameplay state. Rebuilding this index from every
/// stencil and scanning every clip host each frame made unchanged topology pay
/// a permanent polling cost. The index instead records component changes and
/// marks only affected hosts dirty.
#[derive(Default)]
pub(in crate::physics) struct CollisionTopologyState {
    stencil_targets: HashMap<Entity, Option<Entity>>,
    stencils_by_target: HashMap<Entity, HashMap<Entity, CollisionStencil>>,
    pub(super) dirty_hosts: HashSet<Entity>,
}

impl CollisionTopologyState {
    pub(super) fn update_stencil(&mut self, entity: Entity, stencil: CollisionStencil) {
        let target = if stencil.enabled {
            stencil.target
        } else {
            None
        };

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

    pub(super) fn remove_stencil(&mut self, entity: Entity) {
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

    pub(super) fn effective_stencils(&self, target: Entity) -> Vec<(Entity, CollisionStencil)> {
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

pub(super) fn topology_fingerprint(
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
