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
    ecs::{UsfAuthorityPartitionOf, UsfLogicalRealizationOf},
    usf::USF_CHILD_CHUNKS_PER_AXIS,
    spatial::{
        SpatialDemandScope, SpatialDemandSnapshot, SpatialRefinementDemand, SpatialScale,
        UsfChartMask, UsfChunkAddress, UsfPosition, UsfRefinementPlan,
        UsfResidencyRequestBuffer, UsfScaleCoverageSnapshot, UsfScaleLayer,
        UsfScaleRoleMask, UsfViewDemandSnapshot,
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

    pub const fn local_patch_half_extent_native(self) -> f32 {
        self.local_patch_half_extent_native
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

/// One capability-specific reason for a voxel materialization scope to exist.
///
/// View and physical demand share geometry/cache machinery without sharing
/// capability authority.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::voxel) struct VoxelRealizationScope {
    scope: SpatialDemandScope,
    roles: UsfScaleRoleMask,
    view_source: Option<Entity>,
    residency_half_extent_native: Vec3,
}

impl VoxelRealizationScope {
    pub(in crate::voxel) fn new(
        scope: SpatialDemandScope,
        roles: UsfScaleRoleMask,
    ) -> Self {
        Self {
            scope,
            roles,
            view_source: None,
            residency_half_extent_native:
                materialization_residency_extent(scope.half_extent_native()),
        }
    }

    pub(in crate::voxel) const fn with_view_source(mut self, source: Entity) -> Self {
        self.view_source = Some(source);
        self
    }

    pub(in crate::voxel) const fn scope(self) -> SpatialDemandScope {
        self.scope
    }

    pub(in crate::voxel) const fn roles(self) -> UsfScaleRoleMask {
        self.roles
    }

    pub(in crate::voxel) const fn view_source(self) -> Option<Entity> {
        self.view_source
    }

    pub(in crate::voxel) const fn residency_half_extent_native(self) -> Vec3 {
        self.residency_half_extent_native
    }
}

#[cfg(test)]
impl From<SpatialDemandScope> for VoxelRealizationScope {
    fn from(scope: SpatialDemandScope) -> Self {
        Self::new(scope, full_runtime_roles())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct VoxelRealizationDemand {
    target_world: Entity,
    scope: SpatialDemandScope,
    roles: UsfScaleRoleMask,
    view_source: Option<Entity>,
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
    pub(in crate::voxel) fn requests_for(
        &self,
        world: Entity,
    ) -> impl Iterator<Item = VoxelRealizationScope> + '_ {
        self.demands
            .iter()
            .filter(move |demand| demand.target_world == world)
            .map(|demand| VoxelRealizationScope {
                scope: demand.scope,
                roles: demand.roles,
                view_source: demand.view_source,
                residency_half_extent_native: demand.residency_half_extent_native,
            })
    }

    fn push(
        &mut self,
        target_world: Entity,
        scope: SpatialDemandScope,
        roles: UsfScaleRoleMask,
        view_source: Option<Entity>,
        residency_half_extent_native: Vec3,
    ) {
        self.demands.push(VoxelRealizationDemand {
            target_world,
            scope,
            roles,
            view_source,
            residency_half_extent_native,
        });
    }
}

fn presentation_roles() -> UsfScaleRoleMask {
    UsfScaleRoleMask::REALIZATION.union(UsfScaleRoleMask::PRESENTATION)
}

fn full_runtime_roles() -> UsfScaleRoleMask {
    presentation_roles()
        .union(UsfScaleRoleMask::COLLISION)
        .union(UsfScaleRoleMask::EDITING)
}

fn roles_for_scale(domain: VoxelScaleDomain, scale: SpatialScale) -> UsfScaleRoleMask {
    let mut roles = presentation_roles();
    if domain.collides(scale) {
        roles = roles.union(UsfScaleRoleMask::COLLISION);
    }
    if domain.editable(scale) {
        roles = roles.union(UsfScaleRoleMask::EDITING);
    }
    roles
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
            Option<&UsfLogicalRealizationOf>,
            Option<&VoxelPinnedDemand>,
        ),
        With<VoxelWorld>,
    >,
    authority_partitions: Query<&UsfAuthorityPartitionOf>,
    celestial_authorities: Query<(&CelestialVoxelField, &VoxelScaleDomain)>,
    coverage: Res<UsfScaleCoverageSnapshot>,
    view_demands: Res<UsfViewDemandSnapshot>,
    mut residency_requests: ResMut<UsfResidencyRequestBuffer>,
    mut output: ResMut<VoxelRealizationDemandSnapshot>,
) {
    // Previous accepted scopes are branch-transaction state. If the desired
    // child moves into a parent branch that is not ready yet, the previous child
    // remains demanded instead of disappearing.
    let previous = output.demands.clone();
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

    for (world_entity, layer, logical_realization, pinned) in &worlds {
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
            // Permanent whole-body bootstrap is presentation ancestry,
            // not a request for far-field physics/editing.
            next.push(
                world_entity,
                scope,
                presentation_roles(),
                None,
                materialization_residency_extent(scope.half_extent_native()),
            );
        }

        if let Some(logical_realization) = logical_realization
            && let Ok(partition) = authority_partitions.get(logical_realization.0)
            && let Ok((field, domain)) = celestial_authorities.get(partition.0)
        {
            if !domain.realizes(scale) {
                continue;
            }

            for source in sources.iter().copied() {
                let plan = realization_plan(source, *domain);
                let Some(step) = plan.step(scale) else {
                    continue;
                };

                let candidate = celestial_surface_demand(
                    *field,
                    *domain,
                    source.scope,
                    scale,
                    step.half_extent_native(),
                    step.priority(),
                )
                .map(|scope| VoxelRealizationDemand {
                    target_world: world_entity,
                    scope,
                    roles: roles_for_scale(*domain, scale),
                    view_source: None,
                    residency_half_extent_native: step.residency_half_extent_native(),
                });
                let parent_ready = candidate.is_some_and(|candidate| {
                    parent_realization_ready(
                        partition.0,
                        step.parent_scale(),
                        &coverage,
                        &candidate.scope.center(),
                    )
                });
                let previous_branch = previous.iter().copied().find(|demand| {
                    demand.target_world == world_entity
                        && demand.scope.source() == source.scope.source()
                });

                if let Some(demand) = select_refinement_branch_demand(
                    candidate,
                    parent_ready,
                    previous_branch,
                ) {
                    next.demands.push(demand);
                }
            }

            // Observer presentation is a separate capability reason. Keeping a
            // fixed native aperture makes physical reach grow one decade per
            // coarser slice; streaming applies materialization-level frustum
            // and screen-significance rejection.
            for view in view_demands.iter() {
                if !view.requests_scale(scale) {
                    continue;
                }

                let half_extent_native =
                    observer_presentation_half_extent_native(*domain);
                let source_scope = SpatialDemandScope::at_scale(
                    view.source(),
                    scale,
                    view.anchor(),
                    half_extent_native,
                    500,
                );

                let candidate = celestial_surface_demand(
                    *field,
                    *domain,
                    source_scope,
                    scale,
                    half_extent_native,
                    500,
                )
                .map(|scope| VoxelRealizationDemand {
                    target_world: world_entity,
                    scope,
                    roles: presentation_roles(),
                    view_source: Some(view.source()),
                    residency_half_extent_native:
                        materialization_residency_extent(half_extent_native),
                });

                let parent_scale = domain.realization_slices().next_coarser(scale);
                let parent_ready = candidate.is_some_and(|candidate| {
                    parent_realization_ready(
                        partition.0,
                        parent_scale,
                        &coverage,
                        &candidate.scope.center(),
                    )
                });
                let previous_branch = previous.iter().copied().find(|demand| {
                    demand.target_world == world_entity
                        && demand.scope.source() == view.source()
                        && demand.view_source == Some(view.source())
                });

                if let Some(demand) = select_refinement_branch_demand(
                    candidate,
                    parent_ready,
                    previous_branch,
                ) {
                    next.demands.push(demand);
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
                    full_runtime_roles(),
                    None,
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
            demand.roles.bits(),
            demand.view_source.map(Entity::to_bits).unwrap_or(0),
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

fn select_refinement_branch_demand(
    candidate: Option<VoxelRealizationDemand>,
    parent_ready: bool,
    previous: Option<VoxelRealizationDemand>,
) -> Option<VoxelRealizationDemand> {
    match candidate {
        Some(candidate) if parent_ready => Some(candidate),
        Some(_) => previous,
        None => None,
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

/// Native observer aperture for one contextual Scale Slice.
///
/// A coarser slice needs only enough native reach to overlap the next-finer
/// working window; physical reach grows by the decimal Scale Stack ratio.
/// Keep at least one materialization radius so alignment cannot collapse the
/// contextual aperture to a single fragile boundary cell.
fn observer_presentation_half_extent_native(domain: VoxelScaleDomain) -> Vec3 {
    let inherited_finer_reach =
        domain.local_patch_half_extent_native() / USF_CHILD_CHUNKS_PER_AXIS as f32;
    Vec3::splat(
        inherited_finer_reach.max(MATERIALIZATION_CHUNK_SIZE as f32),
    )
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
    child_center: &UsfPosition,
) -> bool {
    let Some(parent_scale) = parent_scale else {
        return true;
    };

    // Parent-before-child refinement follows canonical ancestry. Different
    // scale realizations are approximations and are not required to place their
    // geometric surfaces at the same point. The only valid prerequisite is that
    // the exact canonical parent context for this child branch already has
    // realized capability from the same semantic authority.
    let Ok(parent_context) =
        UsfChunkAddress::containing(*child_center, parent_scale)
    else {
        return false;
    };

    coverage.has_in_context_for_authority(
        authority,
        parent_context,
        UsfScaleRoleMask::REALIZATION,
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
    fn make_before_break_parent_block_keeps_previous_child_demand() {
        let mut ecs = World::new();
        let source = ecs.spawn_empty().id();
        let target_world = ecs.spawn_empty().id();
        let scale = SpatialScale::ZERO;
        let previous = VoxelRealizationDemand {
            target_world,
            roles: presentation_roles().union(UsfScaleRoleMask::COLLISION),
            view_source: None,
            scope: SpatialDemandScope::at_scale(
                source,
                scale,
                UsfPosition::zero(scale),
                Vec3::splat(32.0),
                10,
            ),
            residency_half_extent_native: Vec3::splat(37.0),
        };
        let candidate = VoxelRealizationDemand {
            target_world,
            roles: presentation_roles().union(UsfScaleRoleMask::COLLISION),
            view_source: None,
            scope: SpatialDemandScope::at_scale(
                source,
                scale,
                UsfPosition::zero(scale)
                    .translated_native(Vec3::new(40.0, 0.0, 0.0))
                    .unwrap(),
                Vec3::splat(32.0),
                10,
            ),
            residency_half_extent_native: Vec3::splat(37.0),
        };

        assert_eq!(
            select_refinement_branch_demand(Some(candidate), false, Some(previous)),
            Some(previous),
        );
        assert_eq!(
            select_refinement_branch_demand(Some(candidate), true, Some(previous)),
            Some(candidate),
        );
        assert_eq!(
            select_refinement_branch_demand(None, false, Some(previous)),
            None,
        );
    }

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
