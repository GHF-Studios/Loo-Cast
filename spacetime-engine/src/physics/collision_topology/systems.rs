//! ECS reconciliation of dirty clip hosts with the physics collider.

use super::{
    AppliedCollisionTopology, CollisionClipSource, CollisionStencil, CollisionTopologyState,
    csg::{RectangularCut, subtract_rectangular_cuts_from_cuboid},
    topology_fingerprint,
};
use avian3d::prelude::Collider;
use bevy::{ecs::lifecycle::RemovedComponents, prelude::*};

/// Rebuilds only hosts whose effective stencil set changed.
///
/// The collider presented to Avian is the real collision topology: character
/// casts, ground probing, rigid-body contacts and ordinary spatial queries all
/// observe the same hole without portal-specific filters.
pub(in crate::physics) fn rebuild_clipped_colliders(
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
