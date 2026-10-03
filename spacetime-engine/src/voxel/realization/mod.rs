//! Multiscale voxel realization policy.
// target-scale-preparation-bootstrap-deadlock-v1
// interaction-terrain-readiness-megapass-v1
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
    ecs::UsfLogicalRealizationOf,
    spatial::{
        SpatialDemandScope, SpatialDemandSnapshot, SpatialRefinementDemand, SpatialScale,
        UsfChartMask, UsfPosition, UsfPrimaryInteractionSlice, UsfRefinementPlan,
        UsfResidencyRequestBuffer, UsfScaleLayer, UsfScaleRoleMask, UsfSemanticFrame,
    },
};

use super::{
    CelestialVoxelField, CelestialVoxelRealizationRegistry,
    MATERIALIZATION_CHUNK_SIZE, VoxelMaterializationDemand,
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

/// Semantic celestial realization identity. This target can exist before its
/// scale-local `VoxelWorld`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::voxel) struct VoxelRealizationTarget {
    authority: Entity,
    scale: SpatialScale,
}

impl VoxelRealizationTarget {
    pub(in crate::voxel) const fn new(authority: Entity, scale: SpatialScale) -> Self {
        Self { authority, scale }
    }

    pub(in crate::voxel) const fn authority(self) -> Entity {
        self.authority
    }

    pub(in crate::voxel) const fn scale(self) -> SpatialScale {
        self.scale
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum VoxelRealizationIntentTarget {
    ExistingWorld(Entity),
    Celestial(VoxelRealizationTarget),
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct VoxelRealizationIntent {
    target: VoxelRealizationIntentTarget,
    scope: SpatialDemandScope,
    roles: UsfScaleRoleMask,
    view_source: Option<Entity>,
    residency_half_extent_native: Vec3,
}

#[derive(Resource, Debug, Default)]
pub(in crate::voxel) struct VoxelRealizationIntentSnapshot {
    intents: Vec<VoxelRealizationIntent>,
}

impl VoxelRealizationIntentSnapshot {
    pub(in crate::voxel) fn celestial_targets(
        &self,
    ) -> impl Iterator<Item = VoxelRealizationTarget> + '_ {
        self.intents.iter().filter_map(|intent| match intent.target {
            VoxelRealizationIntentTarget::Celestial(target) => Some(target),
            VoxelRealizationIntentTarget::ExistingWorld(_) => None,
        })
    }

    fn push(
        &mut self,
        target: VoxelRealizationIntentTarget,
        scope: SpatialDemandScope,
        roles: UsfScaleRoleMask,
        view_source: Option<Entity>,
        residency_half_extent_native: Vec3,
    ) {
        self.intents.push(VoxelRealizationIntent {
            target,
            scope,
            roles,
            view_source,
            residency_half_extent_native,
        });
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
        // Demands are sorted by target_world during publication. Binary-search
        // the world's contiguous run instead of rescanning every demand once
        // for every resident VoxelWorld.
        let world_bits = world.to_bits();
        let start = self
            .demands
            .partition_point(|demand| demand.target_world.to_bits() < world_bits);
        let end = self.demands[start..]
            .partition_point(|demand| demand.target_world == world)
            + start;

        self.demands[start..end]
            .iter()
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

// interaction-scale-dense-role-ownership-v1
// prepare-target-before-commit-v1
fn roles_for_scale(
    domain: VoxelScaleDomain,
    target_scale: SpatialScale,
    physical_target_scale: SpatialScale,
) -> UsfScaleRoleMask {
    // Decimal USF Scale is an interaction/numerical domain, not graphical LOD.
    // Binary voxel resolution owns automatic visual refinement.
    //
    // Keep ancestor/future Scale worlds as reusable REALIZATION context only.
    // The controlled source Scale is the one dense world allowed to own local
    // presentation, collision and editing capability.
    let mut roles = UsfScaleRoleMask::REALIZATION;

    if target_scale != physical_target_scale {
        return roles;
    }

    roles = roles.union(UsfScaleRoleMask::PRESENTATION);
    if domain.collides(target_scale) {
        roles = roles.union(UsfScaleRoleMask::COLLISION);
    }
    if domain.editable(target_scale) {
        roles = roles.union(UsfScaleRoleMask::EDITING);
    }
    roles
}


pub(super) fn collect_voxel_realization_intent(
    spatial: Res<SpatialDemandSnapshot>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    voxel_sources: Query<
        Option<&SpatialRefinementDemand>,
        With<VoxelMaterializationDemand>,
    >,
    standalone_worlds: Query<
        (
            Entity,
            &UsfScaleLayer,
            Option<&UsfLogicalRealizationOf>,
            Option<&VoxelPinnedDemand>,
        ),
        With<VoxelWorld>,
    >,
    celestial_authorities: Query<(
        Entity,
        &UsfPosition,
        &UsfSemanticFrame,
        &CelestialVoxelField,
        &VoxelScaleDomain,
    )>,
    mut residency_requests: ResMut<UsfResidencyRequestBuffer>,
    mut output: ResMut<VoxelRealizationIntentSnapshot>,
) {
    let mut next = VoxelRealizationIntentSnapshot::default();
    // target-scale-preparation-bootstrap-deadlock-v1
    //
    // The destination must be able to build the capability that gates entry
    // into it. `scale()` is the committed outgoing chart; `target_scale()` is
    // the requested destination during a handoff and therefore owns prep work.
    let physical_target_scale = interaction.target_scale();

    let mut sources = Vec::<VoxelDemandSource>::new();
    for scope in spatial.iter() {
        let Ok(refinement) = voxel_sources.get(scope.source()) else {
            continue;
        };
        sources.push(VoxelDemandSource {
            scope,
            minimum_realization_scale: refinement.and_then(|value| value.minimum_scale()),
            refinement_half_extent_native: refinement.map(|value| value.half_extent_native()),
        });
    }

    for (authority, body_origin, body_frame, field, domain) in &celestial_authorities {
        // Whole-body context is owned by regional planetary presentation.
        // Dense celestial worlds are created only from explicit spatial/capability
        // demand. Camera/view visibility never manufactures voxel worlds.

        for source in sources.iter().copied() {
            let plan = realization_plan(source, *domain);
            for step in plan.steps_coarse_to_fine() {
                let scale = step.scale();
                let target = VoxelRealizationTarget::new(authority, scale);
                let candidate = celestial_surface_demand(
                    *body_origin,
                    *body_frame,
                    *field,
                    *domain,
                    source.scope,
                    scale,
                    step.half_extent_native(),
                    step.priority(),
                )
                .map(|scope| VoxelRealizationIntent {
                    target: VoxelRealizationIntentTarget::Celestial(target),
                    scope,
                    roles: roles_for_scale(*domain, scale, physical_target_scale),
                    view_source: None,
                    residency_half_extent_native: step.residency_half_extent_native(),
                });

                // current-location-refinement-demand-v1
                //
                // Demand location follows the source immediately. The previous
                // implementation retained the previous child scope until parent
                // coverage was ready at the new location. At high speed that
                // pins fine terrain behind the loader and makes it jump forward
                // only after the parent catches up.
                //
                // Make-before-break belongs to presentation ownership. It must
                // not falsify current spatial demand.
                if let Some(intent) = candidate {
                    next.intents.push(intent);
                }
            }
        }

    }

    // Standalone voxel worlds keep their direct world-targeted path.
    for (world_entity, layer, logical_realization, pinned) in &standalone_worlds {
        if logical_realization.is_some() {
            continue;
        }
        let scale = layer.scale();

        if let Some(pinned) = pinned {
            let scope = SpatialDemandScope::at_scale(
                world_entity,
                scale,
                pinned.center(),
                pinned.half_extent_native(),
                pinned.priority(),
            );
            next.push(
                VoxelRealizationIntentTarget::ExistingWorld(world_entity),
                scope,
                presentation_roles(),
                None,
                materialization_residency_extent(scope.half_extent_native()),
            );
        }

        for source in sources.iter().copied() {
            if source.scope.scale() == scale {
                next.push(
                    VoxelRealizationIntentTarget::ExistingWorld(world_entity),
                    source.scope,
                    full_runtime_roles(),
                    None,
                    materialization_residency_extent(source.scope.half_extent_native()),
                );
            }
        }
    }

    next.intents.sort_by_key(|intent| {
        let (kind, owner, scale) = match intent.target {
            VoxelRealizationIntentTarget::ExistingWorld(world) => {
                (0_u8, world.to_bits(), intent.scope.scale().exponent())
            }
            VoxelRealizationIntentTarget::Celestial(target) => {
                (1_u8, target.authority().to_bits(), target.scale().exponent())
            }
        };
        (
            kind,
            owner,
            intent.scope.source().to_bits(),
            Reverse(scale),
            intent.roles.bits(),
            intent.view_source.map(Entity::to_bits).unwrap_or(0),
        )
    });

    for intent in &next.intents {
        residency_requests.request(SpatialDemandScope::at_scale(
            intent.scope.source(),
            intent.scope.scale(),
            intent.scope.center(),
            intent.residency_half_extent_native,
            intent.scope.priority(),
        ));
    }

    if output.intents != next.intents {
        output.intents = next.intents;
    }
}

/// Resolve authority+Scale intent to the disposable world entity consumed by
/// the existing dense materialization backend.
pub(super) fn resolve_voxel_realization_demand(
    intents: Res<VoxelRealizationIntentSnapshot>,
    registry: Res<CelestialVoxelRealizationRegistry>,
    mut output: ResMut<VoxelRealizationDemandSnapshot>,
) {
    let mut next = VoxelRealizationDemandSnapshot::default();

    for intent in &intents.intents {
        let target_world = match intent.target {
            VoxelRealizationIntentTarget::ExistingWorld(world) => Some(world),
            VoxelRealizationIntentTarget::Celestial(target) => registry.world_for(target),
        };
        let Some(target_world) = target_world else {
            continue;
        };

        next.push(
            target_world,
            intent.scope,
            intent.roles,
            intent.view_source,
            intent.residency_half_extent_native,
        );
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

fn celestial_surface_demand(
    body_origin: UsfPosition,
    body_frame: UsfSemanticFrame,
    field: CelestialVoxelField,
    domain: VoxelScaleDomain,
    source: SpatialDemandScope,
    target_scale: SpatialScale,
    half_extent_native: Vec3,
    priority: i32,
) -> Option<SpatialDemandScope> {
    let body = field.realization(body_origin, body_frame, target_scale);
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
    fn refinement_demand_tracks_the_current_location_immediately() {
        let mut ecs = World::new();
        let source = ecs.spawn_empty().id();
        let scale = SpatialScale::ZERO;
        let previous_center = UsfPosition::zero(scale);
        let candidate_center = previous_center
            .translated_native(Vec3::new(400.0, 0.0, 0.0))
            .unwrap();

        assert_ne!(previous_center, candidate_center);

        let candidate = SpatialDemandScope::at_scale(
            source,
            scale,
            candidate_center,
            Vec3::splat(32.0),
            10,
        );

        // Demand itself always describes the current location. Presentation
        // continuity is owned elsewhere.
        assert_eq!(candidate.center(), candidate_center);
    }

    #[test]
    fn authority_scale_intent_survives_without_world() {
        let mut ecs = World::new();
        let authority = ecs.spawn_empty().id();
        let source = ecs.spawn_empty().id();
        let scale = SpatialScale::ZERO;
        let target = VoxelRealizationTarget::new(authority, scale);
        let scope = SpatialDemandScope::at_scale(
            source,
            scale,
            UsfPosition::zero(scale),
            Vec3::splat(8.0),
            10,
        );

        let mut snapshot = VoxelRealizationIntentSnapshot::default();
        snapshot.push(
            VoxelRealizationIntentTarget::Celestial(target),
            scope,
            presentation_roles(),
            None,
            materialization_residency_extent(scope.half_extent_native()),
        );

        assert_eq!(snapshot.celestial_targets().collect::<Vec<_>>(), vec![target]);
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
    fn pending_target_can_receive_physical_roles_before_commit() {
        let s0 = SpatialScale::ZERO;
        let bootstrap = SpatialScale::MAX;
        let physical = UsfChartMask::ALL;
        let domain = VoxelScaleDomain::contiguous(SpatialScale::MIN, SpatialScale::MAX)
            .with_collision_slices(physical)
            .with_editing_slices(physical);

        // Runtime may still be committed to bootstrap S+35. The role helper is
        // intentionally fed the *pending target* S0 by the collector.
        let target_roles = roles_for_scale(domain, s0, s0);
        assert!(target_roles.contains(UsfScaleRoleMask::PRESENTATION));
        assert!(target_roles.contains(UsfScaleRoleMask::COLLISION));

        let outgoing_roles = roles_for_scale(domain, bootstrap, s0);
        assert_eq!(outgoing_roles, UsfScaleRoleMask::REALIZATION);
    }

    #[test]
    fn only_interaction_scale_requests_dense_physical_presentation() {
        let s0 = SpatialScale::ZERO;
        let s1 = SpatialScale::new(1).unwrap();
        let s3 = SpatialScale::new(3).unwrap();
        let s5 = SpatialScale::new(5).unwrap();
        let s6 = SpatialScale::new(6).unwrap();
        let physical = UsfChartMask::inclusive_range(s0, s6);
        let domain = VoxelScaleDomain::contiguous(s0, s6)
            .with_collision_slices(physical)
            .with_editing_slices(physical);

        for context in [s0, s1, s5] {
            let roles = roles_for_scale(domain, context, s3);
            assert!(roles.contains(UsfScaleRoleMask::REALIZATION));
            assert!(!roles.contains(UsfScaleRoleMask::PRESENTATION));
            assert!(!roles.contains(UsfScaleRoleMask::COLLISION));
            assert!(!roles.contains(UsfScaleRoleMask::EDITING));
        }

        let interaction = roles_for_scale(domain, s3, s3);
        assert!(interaction.contains(UsfScaleRoleMask::REALIZATION));
        assert!(interaction.contains(UsfScaleRoleMask::PRESENTATION));
        assert!(interaction.contains(UsfScaleRoleMask::COLLISION));
        assert!(interaction.contains(UsfScaleRoleMask::EDITING));
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
