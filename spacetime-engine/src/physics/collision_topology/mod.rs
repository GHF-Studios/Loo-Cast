//! Runtime collision-space modification.
//!
//! Higher-level mechanics publish [`CollisionStencil`] components. Hosts keep
//! immutable [`CollisionClipSource`] geometry. This module rebuilds the actual
//! Avian collider only when the effective stencil set changes.

mod csg;
mod source;

use std::collections::HashMap;

use avian3d::prelude::Collider;
use bevy::prelude::*;

use csg::{RectangularCut, subtract_rectangular_cuts_from_cuboid};

pub use source::{CollisionClipSource, supports_rectangular_stencil};

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
pub struct AppliedCollisionTopology {
    fingerprint: u64,
}

/// Rebuilds only hosts whose effective stencil set changed.
///
/// The collider presented to Avian is the real collision topology: character
/// casts, ground probing, rigid-body contacts and ordinary spatial queries all
/// observe the same hole without portal-specific filters.
pub(crate) fn rebuild_clipped_colliders(
    mut commands: Commands,
    stencils: Query<(Entity, &CollisionStencil)>,
    mut hosts: Query<(
        Entity,
        &CollisionClipSource,
        &Transform,
        &mut Collider,
        Option<&mut AppliedCollisionTopology>,
    )>,
) {
    let mut by_target: HashMap<Entity, Vec<(Entity, CollisionStencil)>> = HashMap::new();
    for (entity, stencil) in &stencils {
        if !stencil.enabled {
            continue;
        }
        let Some(target) = stencil.target else {
            continue;
        };
        by_target.entry(target).or_default().push((entity, *stencil));
    }

    for (entity, source, transform, mut collider, applied) in &mut hosts {
        let mut effective = by_target.remove(&entity).unwrap_or_default();
        effective.sort_by_key(|(stencil, _)| stencil.to_bits());
        let fingerprint = topology_fingerprint(&effective);

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

fn topology_fingerprint(stencils: &[(Entity, CollisionStencil)]) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
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
