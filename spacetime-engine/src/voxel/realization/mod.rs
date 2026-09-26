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

use std::cmp::Reverse;

use bevy::prelude::*;

use crate::{
    ecs::UsfManifestationOf,
    spatial::{
        SpatialDemandScope, SpatialDemandSnapshot, SpatialRefinementDemand, SpatialScale,
        UsfChartMask, UsfPosition, UsfRefinementPlan, UsfResidencyRequestBuffer,
        UsfScaleCoverageSnapshot, UsfScaleLayer, UsfScaleRoleMask,
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

    pub const fn refinement_activation_native(self) -> f32 {
        self.refinement_activation_native
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
    residency_half_extent_native: Vec3,
}

#[derive(Debug, Clone, Copy)]
struct VoxelDemandSource {
    scope: SpatialDemandScope,
    minimum_realization_scale: Option<SpatialScale>,
    refinement_half_extent_native: Option<Vec3>,
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

    fn push(
        &mut self,
        target_world: Entity,
        scope: SpatialDemandScope,
        residency_half_extent_native: Vec3,
    ) {
        self.demands.push(VoxelRealizationDemand {
            target_world,
            scope,
            residency_half_extent_native,
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
    coverage: Res<UsfScaleCoverageSnapshot>,
    mut residency_requests: ResMut<UsfResidencyRequestBuffer>,
    mut output: ResMut<VoxelRealizationDemandSnapshot>,
) {
    let mut next = VoxelRealizationDemandSnapshot::default();

    // Generic interest remains one canonical scope per source. Voxel-specific
    // refinement policy is read separately rather than smuggled into extra
    // generic demand scopes.
    let mut sources = Vec::<VoxelDemandSource>::new();
    for scope in spatial.iter() {
        let Ok(refinement) = voxel_sources.get(scope.source()) else {
            continue;
        };

        sources.push(VoxelDemandSource {
            scope,
            minimum_realization_scale: refinement.and_then(|value| value.minimum_scale()),
            refinement_half_extent_native: refinement
                .map(|value| value.half_extent_native()),
        });
    }

    for (world_entity, layer, manifestation, pinned) in &worlds {
        let scale = layer.scale();

        // Persistent capability-local residency is explicit voxel policy.
        if let Some(pinned) = pinned {
            let scope = SpatialDemandScope::at_scale(
                world_entity,
                scale,
                pinned.center(),
                pinned.half_extent_native(),
                pinned.priority(),
            );
            next.push(
                world_entity,
                scope,
                materialization_residency_extent(scope.half_extent_native()),
            );
        }

        if let Some(manifestation) = manifestation
            && let Ok((field, domain)) = celestial_authorities.get(manifestation.0)
        {
            if !domain.realizes(scale) {
                continue;
            }

            for source in sources.iter().copied() {
                let plan = realization_plan(source, *domain);
                let Some(step) = plan.step(scale) else {
                    continue;
                };

                if let Some(scope) = celestial_surface_demand(
                    *field,
                    *domain,
                    source.scope,
                    scale,
                    step.half_extent_native(),
                    step.priority(),
                ) && parent_realization_ready(
                    manifestation.0,
                    step.parent_scale(),
                    &coverage,
                    &scope.center(),
                ) {
                    next.push(
                        world_entity,
                        scope,
                        step.residency_half_extent_native(),
                    );
                }
            }
            continue;
        }

        // Standalone/non-celestial worlds consume interest only in their own
        // numerical chart. They do not inherit a made-up multi-scale spine.
        for source in sources.iter().copied() {
            if source.scope.scale() == scale {
                next.push(
                    world_entity,
                    source.scope,
                    materialization_residency_extent(
                        source.scope.half_extent_native(),
                    ),
                );
            }
        }
    }

    next.demands.sort_by_key(|demand| {
        (
            demand.target_world.to_bits(),
            demand.scope.source().to_bits(),
            Reverse(demand.scope.scale().exponent()),
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
            demand.residency_half_extent_native,
            scope.priority(),
        ));
    }

    if output.demands != next.demands {
        output.demands = next.demands;
    }
}

fn realization_plan(
    source: VoxelDemandSource,
    domain: VoxelScaleDomain,
) -> UsfRefinementPlan {
    let tip_extent = source
        .refinement_half_extent_native
        .unwrap_or(Vec3::splat(domain.local_patch_half_extent_native));

    UsfRefinementPlan::new(
        source.scope.scale(),
        source.minimum_realization_scale,
        domain.realization_slices(),
        tip_extent,
        source.scope.priority().saturating_add(1_000),
    )
    .with_residency_halo_native(Vec3::splat(
        MATERIALIZATION_CHUNK_SIZE as f32 * 0.5,
    ))
}

fn materialization_residency_extent(half_extent_native: Vec3) -> Vec3 {
    half_extent_native + Vec3::splat(MATERIALIZATION_CHUNK_SIZE as f32 * 0.5)
}

#[cfg(test)]
fn realization_half_extent_at_scale(
    source: VoxelDemandSource,
    domain: VoxelScaleDomain,
    target_scale: SpatialScale,
) -> Vec3 {
    realization_plan(source, domain)
        .step(target_scale)
        .expect("test target scale must participate in the refinement plan")
        .half_extent_native()
}

#[cfg(test)]
fn realization_parent_scale(
    domain: VoxelScaleDomain,
    target_scale: SpatialScale,
) -> Option<SpatialScale> {
    domain.realization_slices().next_coarser(target_scale)
}

#[cfg(test)]
fn realization_requests_scale(
    source_scale: SpatialScale,
    minimum_scale: Option<SpatialScale>,
    target_scale: SpatialScale,
) -> bool {
    UsfRefinementPlan::new(
        source_scale,
        minimum_scale,
        UsfChartMask::ALL,
        Vec3::ONE,
        0,
    )
    .requests_scale(target_scale)
}

fn parent_realization_ready(
    authority: Entity,
    parent_scale: Option<SpatialScale>,
    coverage: &UsfScaleCoverageSnapshot,
    center: &UsfPosition,
) -> bool {
    let Some(parent_scale) = parent_scale else {
        return true;
    };

    coverage.has_near_for_authority(
        authority,
        parent_scale,
        center,
        UsfScaleRoleMask::REALIZATION,
        0.0,
    )
}

fn celestial_surface_demand(
    field: CelestialVoxelField,
    domain: VoxelScaleDomain,
    source: SpatialDemandScope,
    target_scale: SpatialScale,
    half_extent_native: Vec3,
    priority: i32,
) -> Option<SpatialDemandScope> {
    let body = field.realization(target_scale);
    let activation = domain.refinement_activation_native;
    let search_bound =
        activation + half_extent_native.length() + MATERIALIZATION_CHUNK_SIZE as f32;

    let (center, _up, signed_clearance) =
        body.surface_near(&source.center(), search_bound)?;

    if signed_clearance.abs() > activation {
        return None;
    }

    Some(SpatialDemandScope::at_scale(
        source.source(),
        target_scale,
        center,
        half_extent_native,
        priority,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refinement_footprint_is_relative_to_requested_tip_scale() {
        let mut ecs = World::new();
        let source_entity = ecs.spawn_empty().id();
        let s0 = SpatialScale::ZERO;
        let s1 = SpatialScale::new(1).unwrap();
        let s2 = SpatialScale::new(2).unwrap();
        let s35 = SpatialScale::MAX;
        let extent = Vec3::new(64.0, 32.0, 64.0);
        let source = VoxelDemandSource {
            scope: SpatialDemandScope::at_scale(
                source_entity,
                s35,
                UsfPosition::zero(SpatialScale::MIN),
                extent,
                100,
            ),
            minimum_realization_scale: Some(s0),
            refinement_half_extent_native: Some(extent),
        };
        let domain = VoxelScaleDomain::contiguous(s0, SpatialScale::new(6).unwrap());

        assert_eq!(realization_half_extent_at_scale(source, domain, s0), extent);

        let at_s1 = realization_half_extent_at_scale(source, domain, s1);
        let at_s2 = realization_half_extent_at_scale(source, domain, s2);
        assert!((at_s1.x - 6.4).abs() < 1.0e-5);
        assert!((at_s1.y - 3.2).abs() < 1.0e-5);
        assert!((at_s2.x - 0.64).abs() < 1.0e-5);
        assert!((at_s2.y - 0.32).abs() < 1.0e-5);
    }

    #[test]
    fn realization_parent_chain_ends_at_coarsest_supported_slice() {
        let s0 = SpatialScale::ZERO;
        let s5 = SpatialScale::new(5).unwrap();
        let s6 = SpatialScale::new(6).unwrap();
        let domain = VoxelScaleDomain::contiguous(s0, s6);

        assert_eq!(realization_parent_scale(domain, s6), None);
        assert_eq!(realization_parent_scale(domain, s5), Some(s6));
        assert_eq!(
            realization_parent_scale(domain, s0),
            Some(SpatialScale::new(1).unwrap())
        );
    }

    #[test]
    fn ordinary_interest_realizes_its_current_scale_without_refinement() {
        let s3 = SpatialScale::new(3).unwrap();
        let s4 = SpatialScale::new(4).unwrap();
        let s5 = SpatialScale::new(5).unwrap();

        assert!(!realization_requests_scale(s4, None, s3));
        assert!(realization_requests_scale(s4, None, s4));
        assert!(!realization_requests_scale(s4, None, s5));
    }

    #[test]
    fn explicit_refinement_requests_the_supported_multiscale_ladder() {
        let s3 = SpatialScale::new(3).unwrap();
        let s4 = SpatialScale::new(4).unwrap();
        let s5 = SpatialScale::new(5).unwrap();

        assert!(!realization_requests_scale(s5, Some(s4), s3));
        assert!(realization_requests_scale(s5, Some(s4), s4));
        assert!(realization_requests_scale(s5, Some(s4), s5));
    }
}
