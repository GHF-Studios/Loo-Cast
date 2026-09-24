//! Multiscale voxel realization policy.
//!
//! Generic spatial interest says where gameplay currently cares about reality.
//! [`SpatialRefinementDemand`] says how fine nearby capability realization is
//! requested to become. This module translates those independent inputs into
//! voxel-specific realization scopes.
//!
//! The resulting scopes also contribute runtime-context residency requirements.
//! Residency does not choose the voxel plan; it records the canonical contexts
//! required by the plan.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    ecs::UsfManifestationOf,
    spatial::{
        SpatialDemandScope, SpatialDemandSnapshot, SpatialRefinementDemand, SpatialScale,
        UsfChartMask, UsfResidencyRequestBuffer, UsfScaleLayer,
    },
};

use super::{
    CelestialVoxelField, MATERIALIZATION_CHUNK_SIZE, VoxelMaterializationDemand,
    VoxelPinnedDemand, VoxelWorld,
};

const DEFAULT_REFINEMENT_ACTIVATION_NATIVE: f32 = 8_192.0;
const DEFAULT_LOCAL_PATCH_HALF_EXTENT_NATIVE: f32 = 32.0;

/// Scale-Slice participation and current voxel realization policy.
///
/// The masks describe mechanism capability. Activation radius and patch extent
/// are representation policy, not semantic body identity.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct VoxelScaleDomain {
    realization_slices: UsfChartMask,
    collision_slices: UsfChartMask,
    editing_slices: UsfChartMask,
    refinement_activation_native: f32,
    local_patch_half_extent_native: f32,
}

impl VoxelScaleDomain {
    pub fn contiguous(lower: SpatialScale, upper: SpatialScale) -> Self {
        Self {
            realization_slices: UsfChartMask::inclusive_range(lower, upper),
            collision_slices: UsfChartMask::NONE,
            editing_slices: UsfChartMask::NONE,
            refinement_activation_native: DEFAULT_REFINEMENT_ACTIVATION_NATIVE,
            local_patch_half_extent_native: DEFAULT_LOCAL_PATCH_HALF_EXTENT_NATIVE,
        }
    }

    pub const fn realization_slices(self) -> UsfChartMask {
        self.realization_slices
    }

    pub const fn collision_slices(self) -> UsfChartMask {
        self.collision_slices
    }

    pub const fn editing_slices(self) -> UsfChartMask {
        self.editing_slices
    }

    pub const fn realizes(self, scale: SpatialScale) -> bool {
        self.realization_slices.contains(scale)
    }

    pub const fn collides(self, scale: SpatialScale) -> bool {
        self.collision_slices.contains(scale)
    }

    pub const fn editable(self, scale: SpatialScale) -> bool {
        self.editing_slices.contains(scale)
    }

    pub fn with_collision_slices(mut self, slices: UsfChartMask) -> Self {
        self.collision_slices = slices;
        self
    }

    pub fn with_editing_slices(mut self, slices: UsfChartMask) -> Self {
        self.editing_slices = slices;
        self
    }

    pub fn with_refinement_activation_native(mut self, radius: f32) -> Self {
        if radius.is_finite() && radius > 0.0 {
            self.refinement_activation_native = radius;
        }
        self
    }

    pub fn with_local_patch_half_extent_native(mut self, extent: f32) -> Self {
        if extent.is_finite() && extent > 0.0 {
            self.local_patch_half_extent_native = extent;
        }
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct VoxelRealizationDemand {
    target_world: Entity,
    scope: SpatialDemandScope,
}

#[derive(Debug, Clone, Copy)]
struct VoxelDemandSource {
    scope: SpatialDemandScope,
    minimum_realization_scale: Option<SpatialScale>,
}

#[derive(Resource, Debug, Default)]
pub(in crate::voxel) struct VoxelRealizationDemandSnapshot {
    demands: Vec<VoxelRealizationDemand>,
}

impl VoxelRealizationDemandSnapshot {
    pub(in crate::voxel) fn scopes_for(
        &self,
        world: Entity,
    ) -> impl Iterator<Item = SpatialDemandScope> + '_ {
        self.demands
            .iter()
            .filter(move |demand| demand.target_world == world)
            .map(|demand| demand.scope)
    }

    fn push(&mut self, target_world: Entity, scope: SpatialDemandScope) {
        self.demands.push(VoxelRealizationDemand {
            target_world,
            scope,
        });
    }
}

pub(super) fn collect_voxel_realization_demand(
    spatial: Res<SpatialDemandSnapshot>,
    voxel_sources: Query<
        Option<&SpatialRefinementDemand>,
        With<VoxelMaterializationDemand>,
    >,
    worlds: Query<
        (
            Entity,
            &UsfScaleLayer,
            Option<&UsfManifestationOf>,
            Option<&VoxelPinnedDemand>,
        ),
        With<VoxelWorld>,
    >,
    celestial_authorities: Query<(&CelestialVoxelField, &VoxelScaleDomain)>,
    mut residency_requests: ResMut<UsfResidencyRequestBuffer>,
    mut output: ResMut<VoxelRealizationDemandSnapshot>,
) {
    let mut next = VoxelRealizationDemandSnapshot::default();

    // Generic interest remains one canonical scope per source. Voxel-specific
    // refinement policy is read separately rather than smuggled into extra
    // generic demand scopes.
    let mut sources = HashMap::<Entity, VoxelDemandSource>::new();
    for scope in spatial.iter() {
        let Ok(refinement) = voxel_sources.get(scope.source()) else {
            continue;
        };

        sources.insert(
            scope.source(),
            VoxelDemandSource {
                scope,
                minimum_realization_scale: refinement.and_then(|value| value.minimum_scale()),
            },
        );
    }

    for (world_entity, layer, manifestation, pinned) in &worlds {
        let scale = layer.scale();

        // Persistent capability-local residency is explicit voxel policy.
        if let Some(pinned) = pinned {
            next.push(
                world_entity,
                SpatialDemandScope::at_scale(
                    world_entity,
                    scale,
                    pinned.center(),
                    pinned.half_extent_native(),
                    pinned.priority(),
                ),
            );
        }

        if let Some(manifestation) = manifestation
            && let Ok((field, domain)) = celestial_authorities.get(manifestation.0)
        {
            if !domain.realizes(scale) {
                continue;
            }

            for source in sources.values().copied() {
                if !refinement_requests_scale(source.minimum_realization_scale, scale) {
                    continue;
                }

                if let Some(scope) =
                    celestial_surface_demand(*field, *domain, source.scope, scale)
                {
                    next.push(world_entity, scope);
                }
            }
            continue;
        }

        // Standalone/non-celestial worlds consume interest only in their own
        // numerical chart. They do not inherit a made-up multi-scale spine.
        for source in sources.values().copied() {
            if source.scope.scale() == scale {
                next.push(world_entity, source.scope);
            }
        }
    }

    next.demands.sort_by_key(|demand| {
        (
            demand.target_world.to_bits(),
            demand.scope.source().to_bits(),
            demand.scope.scale().exponent(),
        )
    });

    // Materialization chunks are capability-local 10-native-unit addresses and
    // may straddle a 1000-native-unit USF context boundary. Pad the residency
    // request by half a materialization chunk so every chosen chunk center lies
    // under a resident canonical context.
    for demand in &next.demands {
        let scope = demand.scope;
        residency_requests.request(SpatialDemandScope::at_scale(
            scope.source(),
            scope.scale(),
            scope.center(),
            scope.half_extent_native()
                + Vec3::splat(MATERIALIZATION_CHUNK_SIZE as f32 * 0.5),
            scope.priority(),
        ));
    }

    if output.demands != next.demands {
        output.demands = next.demands;
    }
}

fn refinement_requests_scale(
    minimum_scale: Option<SpatialScale>,
    target_scale: SpatialScale,
) -> bool {
    // "Request through Sx" means Sx and every coarser supported realization
    // may prepare context; finer slices stay absent until explicitly requested.
    minimum_scale.is_some_and(|minimum| target_scale >= minimum)
}

fn celestial_surface_demand(
    field: CelestialVoxelField,
    domain: VoxelScaleDomain,
    source: SpatialDemandScope,
    target_scale: SpatialScale,
) -> Option<SpatialDemandScope> {
    let body = field.realization(target_scale);
    let observer = source.center().reexpressed_at(target_scale).ok()?;
    let activation = domain.refinement_activation_native;
    let bound = body.radius_native() + activation + domain.local_patch_half_extent_native;

    let relative = observer
        .relative_at_scale_bounded(&field.center(), target_scale, bound)
        .ok()?;
    let radial = relative.length();
    if radial <= f32::EPSILON {
        return None;
    }

    let direction = relative / radial;
    let surface_radius = body.surface_radius_native(direction);
    let signed_clearance = radial - surface_radius;

    if signed_clearance.abs() > activation {
        return None;
    }

    let center = field
        .center()
        .reexpressed_at(target_scale)
        .ok()?
        .translated_native(direction * surface_radius)
        .ok()?;

    Some(SpatialDemandScope::at_scale(
        source.source(),
        target_scale,
        center,
        Vec3::splat(domain.local_patch_half_extent_native),
        source.priority() + 1_000,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refinement_target_is_an_actual_realization_boundary() {
        let s3 = SpatialScale::new(3).unwrap();
        let s4 = SpatialScale::new(4).unwrap();
        let s5 = SpatialScale::new(5).unwrap();

        assert!(!refinement_requests_scale(None, s4));
        assert!(!refinement_requests_scale(Some(s4), s3));
        assert!(refinement_requests_scale(Some(s4), s4));
        assert!(refinement_requests_scale(Some(s4), s5));
    }
}
