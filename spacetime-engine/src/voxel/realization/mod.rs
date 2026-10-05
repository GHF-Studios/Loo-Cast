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
    ecs::UsfLogicalRealizationOf,
    spatial::{
        SpatialDemandMotionSnapshot, SpatialDemandScope, SpatialDemandSnapshot,
        SpatialRefinementDemand, SpatialScale,
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
// Speed extends physical preparation only when it closes on the nearest
// canonical terrain boundary. It never selects decimal interaction Scale.
const EXTERIOR_CONTACT_PREPARATION_SECONDS: f64 = 1.5;
const EXTERIOR_CONTACT_GUARD_CHUNKS: f32 = 2.0;

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
    // Canonical nearest semantic boundary used ONLY for reconstructible work
    // ordering. It never grants capability or semantic authority.
    priority_focus: Option<UsfPosition>,
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
            priority_focus: None,
        }
    }

    pub(in crate::voxel) const fn with_view_source(mut self, source: Entity) -> Self {
        self.view_source = Some(source);
        self
    }

    pub(in crate::voxel) const fn with_priority_focus(
        mut self,
        focus: UsfPosition,
    ) -> Self {
        self.priority_focus = Some(focus);
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

    pub(in crate::voxel) const fn priority_focus(self) -> Option<UsfPosition> {
        self.priority_focus
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
    priority_focus: Option<UsfPosition>,
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
        priority_focus: Option<UsfPosition>,
    ) {
        self.intents.push(VoxelRealizationIntent {
            target,
            scope,
            roles,
            view_source,
            residency_half_extent_native,
            priority_focus,
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
    priority_focus: Option<UsfPosition>,
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
                priority_focus: demand.priority_focus,
            })
    }

    fn push(
        &mut self,
        target_world: Entity,
        scope: SpatialDemandScope,
        roles: UsfScaleRoleMask,
        view_source: Option<Entity>,
        residency_half_extent_native: Vec3,
        priority_focus: Option<UsfPosition>,
    ) {
        self.demands.push(VoxelRealizationDemand {
            target_world,
            scope,
            roles,
            view_source,
            residency_half_extent_native,
            priority_focus,
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
    motions: Res<SpatialDemandMotionSnapshot>,
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
        let minimum_realization_scale =
            refinement.and_then(|value| value.minimum_scale());
        let refinement_tip_scale = minimum_realization_scale
            .filter(|requested| *requested < scope.scale())
            .unwrap_or(scope.scale());

        sources.push(VoxelDemandSource {
            scope,
            minimum_realization_scale,
            // UsfRefinementPlan expects its footprint in TIP-native units.
            // During bootstrap the source may still be S+35 while the requested
            // tip is S0, so converting at source Scale would collapse the
            // physical S0 footprint almost to zero.
            refinement_half_extent_native: refinement.map(|value| {
                value.half_extent_native_at(refinement_tip_scale)
            }),
        });
    }

    for (authority, body_origin, body_frame, field, domain) in &celestial_authorities {
        // Whole-body visual context is owned by the Cartesian binary
        // presentation hierarchy. Dense celestial worlds exist only from
        // explicit spatial/capability demand.
        //
        //
        // Local volumetric ownership follows the source, while nearby boundary
        // contact must be prepared before landing/handoff. Full-SDF clearance
        // finds cave/terrain boundaries. Outer-shell clearance distinguishes
        // positive-SDF cave air from actual planetary exterior.
        for source in sources.iter().copied() {
            let Ok(source_local_metres) =
                body_frame.world_to_local_metres(
                    body_origin,
                    &source.scope.center(),
                    SpatialScale::ZERO,
                    f64::MAX,
                )
            else {
                continue;
            };
            let Some(signed_clearance_metres) =
                field.signed_distance_local_metres(source_local_metres)
            else {
                continue;
            };
            let outer_clearance_metres = field
                .outer_signed_distance_local_metres(source_local_metres)
                .unwrap_or(signed_clearance_metres);
            let boundary_center = field
                .boundary_near(
                    body_origin,
                    *body_frame,
                    &source.scope.center(),
                    f64::MAX,
                )
                .map(|(boundary, _)| boundary);

            //
            // Compute only the component of canonical velocity that closes the
            // distance to the nearest full-SDF boundary. Tangential or outward
            // motion must not retain a huge old ground patch.
            let closing_speed_metres_per_second = boundary_center
                .and_then(|boundary| {
                    boundary
                        .relative_at_scale_bounded_f64(
                            &source.scope.center(),
                            SpatialScale::ZERO,
                            f64::MAX,
                        )
                        .ok()
                })
                .map_or(0.0, |toward_boundary_metres| {
                    let distance = toward_boundary_metres.length();
                    if !distance.is_finite() || distance <= f64::EPSILON {
                        return 0.0;
                    }
                    let direction = toward_boundary_metres / distance;
                    motions
                        .velocity_metres_per_second(source.scope.source())
                        .dot(direction)
                        .max(0.0)
                });

            let plan = realization_plan(source, *domain);
            for step in plan.steps_coarse_to_fine() {
                let scale = step.scale();
                let target = VoxelRealizationTarget::new(authority, scale);
                let candidate = celestial_contact_volume_demand(
                    boundary_center,
                    signed_clearance_metres,
                    outer_clearance_metres,
                    closing_speed_metres_per_second,
                    source.scope,
                    scale,
                    step.half_extent_native(),
                    step.priority(),
                )
                .map(|scope| {
                    let priority_focus = boundary_center.filter(|boundary| {
                        let bound = scope.half_extent_native().length()
                            + MATERIALIZATION_CHUNK_SIZE as f32 * 2.0
                            + 1.0;
                        boundary
                            .relative_at_scale_bounded(
                                &scope.center(),
                                scale,
                                bound,
                            )
                            .ok()
                            .is_some_and(|delta| {
                                let half = scope.half_extent_native();
                                delta.x.abs() <= half.x + 0.001
                                    && delta.y.abs() <= half.y + 0.001
                                    && delta.z.abs() <= half.z + 0.001
                            })
                    });

                    VoxelRealizationIntent {
                        target: VoxelRealizationIntentTarget::Celestial(target),
                        scope,
                        roles: roles_for_scale(
                            *domain,
                            scale,
                            physical_target_scale,
                        ),
                        view_source: None,
                        residency_half_extent_native:
                            materialization_residency_extent(
                                scope.half_extent_native(),
                            ),
                        priority_focus,
                    }
                });

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
                None,
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
                    None,
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
            intent.priority_focus,
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




fn corridor_scope_between(
    source: SpatialDemandScope,
    boundary: UsfPosition,
    target_scale: SpatialScale,
    base_half_extent_native: Vec3,
    priority: i32,
    maximum_corridor_native: f32,
) -> Option<SpatialDemandScope> {
    let delta = boundary
        .relative_at_scale_bounded(
            &source.center(),
            target_scale,
            maximum_corridor_native.max(1.0),
        )
        .ok()?;
    let midpoint = source
        .center()
        .translated_at_scale(target_scale, delta * 0.5)
        .ok()?;
    let half_extent =
        base_half_extent_native + delta.abs() * 0.5;

    Some(SpatialDemandScope::at_scale(
        source.source(),
        target_scale,
        midpoint,
        half_extent,
        priority,
    ))
}

fn celestial_contact_volume_demand(
    boundary_center: Option<UsfPosition>,
    signed_clearance_metres: f64,
    outer_clearance_metres: f64,
    closing_speed_metres_per_second: f64,
    source: SpatialDemandScope,
    target_scale: SpatialScale,
    half_extent_native: Vec3,
    priority: i32,
) -> Option<SpatialDemandScope> {
    if !signed_clearance_metres.is_finite()
        || !outer_clearance_metres.is_finite()
    {
        return None;
    }

    let metres_per_native = target_scale.metres_per_native();

    //
    // The old 8192-native activation radius belonged to a world where dense
    // terrain also carried far visual context. Binary presentation owns that
    // now. Dense exterior terrain owns only near and predicted physical contact.
    let footprint_native =
        half_extent_native.length()
            + MATERIALIZATION_CHUNK_SIZE as f32
                * EXTERIOR_CONTACT_GUARD_CHUNKS;
    let local_contact_horizon_metres =
        f64::from(footprint_native) * metres_per_native;
    let predictive_contact_horizon_metres =
        closing_speed_metres_per_second.max(0.0)
            * EXTERIOR_CONTACT_PREPARATION_SECONDS;
    let exterior_limit_metres =
        local_contact_horizon_metres
            + predictive_contact_horizon_metres;

    // Full SDF can be positive inside a cave. Only outer-shell clearance proves
    // the observer is truly outside the planetary volume.
    if outer_clearance_metres > exterior_limit_metres {
        return None;
    }

    let local_scope = SpatialDemandScope::at_scale(
        source.source(),
        target_scale,
        source.center(),
        half_extent_native,
        priority,
    );

    let Some(boundary) = boundary_center else {
        return Some(local_scope);
    };

    // This corridor is contact-preparation, not a giant deep-volume prism.
    let corridor_native =
        (footprint_native * 2.0)
            .max(MATERIALIZATION_CHUNK_SIZE as f32 * 4.0);
    let corridor_metres =
        f64::from(corridor_native) * metres_per_native;

    if signed_clearance_metres.abs() <= corridor_metres {
        if let Some(scope) = corridor_scope_between(
            source,
            boundary,
            target_scale,
            half_extent_native,
            priority,
            corridor_native + footprint_native,
        ) {
            return Some(scope);
        }
    }

    if outer_clearance_metres > 0.0 {
        // Outside: prepare actual terrain, not an all-air cube around the view.
        Some(SpatialDemandScope::at_scale(
            source.source(),
            target_scale,
            boundary,
            half_extent_native,
            priority,
        ))
    } else {
        // Inside the body envelope, including cave voids, stay subject-local.
        Some(local_scope)
    }
}
