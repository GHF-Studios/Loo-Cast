//! Lazy celestial voxel-realization lifecycle.
//!
//! Semantic celestial authority exists independently of scale-local voxel worlds.
//! This module owns only the disposable authority+Scale -> `VoxelWorld` mapping.
//! Demand policy stays in `realization`; dense chunk residency stays in `streaming`.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    ecs::{UsfAuthorityPartitions, UsfLogicalRealizationOf},
    spatial::{SpatialScale, UsfPosition, UsfScaleLayer, UsfSemanticFrame},
};

use super::{
    CelestialVoxelField, VoxelBase, VoxelCollisionDisabled, VoxelEditingDisabled,
    VoxelPinnedDemand, VoxelPresentationMaterial, VoxelScaleDomain, VoxelStreaming,
    VoxelWorld,
    realization::{VoxelRealizationIntentSnapshot, VoxelRealizationTarget},
};

#[derive(Component, Debug, Clone)]
pub struct CelestialVoxelRealizationPolicy {
    presentation_material: Handle<StandardMaterial>,
    bootstrap_shell_margin_native: f32,
}

impl CelestialVoxelRealizationPolicy {
    pub fn new(
        presentation_material: Handle<StandardMaterial>,
        bootstrap_shell_margin_native: f32,
    ) -> Self {
        assert!(
            bootstrap_shell_margin_native.is_finite()
                && bootstrap_shell_margin_native >= 0.0,
            "celestial bootstrap shell margin must be finite and non-negative"
        );
        Self {
            presentation_material,
            bootstrap_shell_margin_native,
        }
    }

    pub(crate) fn presentation_material(&self) -> &Handle<StandardMaterial> {
        &self.presentation_material
    }

    pub(crate) const fn bootstrap_shell_margin_native(&self) -> f32 {
        self.bootstrap_shell_margin_native
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::voxel) struct CelestialVoxelRealization {
    target: VoxelRealizationTarget,
}

impl CelestialVoxelRealization {
    const fn new(target: VoxelRealizationTarget) -> Self {
        Self { target }
    }

    pub(in crate::voxel) const fn target(self) -> VoxelRealizationTarget {
        self.target
    }

    pub(in crate::voxel) const fn authority(self) -> Entity {
        self.target.authority()
    }
}

#[derive(Resource, Debug, Default)]
pub(in crate::voxel) struct CelestialVoxelRealizationRegistry {
    worlds: HashMap<VoxelRealizationTarget, Entity>,
}

impl CelestialVoxelRealizationRegistry {
    pub(in crate::voxel) fn world_for(
        &self,
        target: VoxelRealizationTarget,
    ) -> Option<Entity> {
        self.worlds.get(&target).copied()
    }
}

/// Reconcile disposable scale-local worlds against already-approved
/// authority+Scale intent. Make-before-break remains owned by demand/coverage.
pub(super) fn sync_celestial_voxel_realizations(
    config: Res<EngineConfig>,
    intents: Res<VoxelRealizationIntentSnapshot>,
    mut registry: ResMut<CelestialVoxelRealizationRegistry>,
    mut commands: Commands,
    authorities: Query<(
        Option<&Name>,
        &UsfPosition,
        &UsfSemanticFrame,
        &CelestialVoxelField,
        &VoxelScaleDomain,
        &CelestialVoxelRealizationPolicy,
        &UsfAuthorityPartitions,
    )>,
    existing: Query<(Entity, &CelestialVoxelRealization)>,
) {
    let desired = intents.celestial_targets().collect::<HashSet<_>>();
    registry.worlds.clear();

    for (entity, realization) in &existing {
        let target = realization.target();
        if !desired.contains(&target) {
            commands.entity(entity).despawn();
            continue;
        }

        if let Some(duplicate_of) = registry.worlds.get(&target).copied() {
            warn!(
                ?entity,
                ?duplicate_of,
                authority = ?target.authority(),
                scale = %target.scale(),
                "duplicate celestial voxel realization retired"
            );
            commands.entity(entity).despawn();
            continue;
        }
        registry.worlds.insert(target, entity);
    }

    for target in desired {
        if registry.world_for(target).is_some() {
            continue;
        }

        let Ok((name, body_origin, body_frame, field, domain, policy, partitions)) =
            authorities.get(target.authority())
        else {
            continue;
        };

        if !domain.realizes(target.scale()) {
            warn!(
                authority = ?target.authority(),
                scale = %target.scale(),
                "celestial voxel realization demand targets unsupported scale"
            );
            continue;
        }

        let mut partitions = partitions.iter();
        let Some(partition) = partitions.next() else {
            warn!(
                authority = ?target.authority(),
                scale = %target.scale(),
                "celestial voxel realization has no authority partition"
            );
            continue;
        };
        if partitions.next().is_some() {
            error!(
                authority = ?target.authority(),
                scale = %target.scale(),
                "celestial voxel realization requires one unambiguous ordinary authority partition"
            );
            continue;
        }

        let Ok(grid_origin) = body_origin.reexpressed_at(target.scale()) else {
            error!(
                authority = ?target.authority(),
                scale = %target.scale(),
                "celestial voxel realization origin cannot re-express at requested scale"
            );
            continue;
        };

        let base = field.realization(*body_origin, *body_frame, target.scale());
        let body_name = name.map(|value| value.as_str()).unwrap_or("Celestial Body");
        let mut world = commands.spawn((
            Name::new(format!("{body_name} S{} Terrain", target.scale())),
            UsfScaleLayer::new(target.scale()),
            UsfLogicalRealizationOf(partition),
            CelestialVoxelRealization::new(target),
            VoxelWorld::new_at(VoxelBase::celestial_body(base), grid_origin),
            VoxelStreaming::new(config.voxel.streaming.default_load_budget_per_frame),
            VoxelPresentationMaterial::new(policy.presentation_material().clone()),
            Transform::IDENTITY,
            Visibility::Inherited,
        ));

        if target.scale() == field.coarsest_detail_scale() {
            let radius_native = target.scale().metres_to_native_f64(field.radius_metres());
            if radius_native.is_finite()
                && radius_native >= 0.0
                && radius_native <= f64::from(f32::MAX)
            {
                world.insert(VoxelPinnedDemand::shell(
                    grid_origin,
                    radius_native as f32,
                    policy.bootstrap_shell_margin_native(),
                ));
            } else {
                error!(
                    authority = ?target.authority(),
                    scale = %target.scale(),
                    radius_native,
                    "celestial bootstrap radius does not fit bounded realization chart"
                );
            }
        }

        if !domain.collides(target.scale()) {
            world.insert(VoxelCollisionDisabled);
        }
        if !domain.editable(target.scale()) {
            world.insert(VoxelEditingDisabled);
        }

        registry.worlds.insert(target, world.id());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn realization_identity_is_authority_plus_scale() {
        let mut ecs = World::new();
        let authority = ecs.spawn_empty().id();
        let other = ecs.spawn_empty().id();
        let s0 = SpatialScale::ZERO;
        let s1 = SpatialScale::new(1).unwrap();

        let unique = HashSet::from([
            VoxelRealizationTarget::new(authority, s0),
            VoxelRealizationTarget::new(authority, s1),
            VoxelRealizationTarget::new(other, s0),
        ]);
        assert_eq!(unique.len(), 3);
    }
}
