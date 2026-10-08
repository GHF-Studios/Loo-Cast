//! Translate generic spatial intent into voxel-specific capability scopes.

use super::*;
use bevy::math::DVec3;

// Bound the view-owned surface footprint before sparse chunk traversal. This
// selects representation detail; it does not change semantic terrain or physics.
const VIEW_SURFACE_CHUNK_BUDGET: f64 = 512.0;

fn append_celestial_view_intents(
    output: &mut VoxelRealizationIntentSnapshot,
    authority: CelestialAuthority<'_>,
    view: &UsfViewDemand,
) {
    let radius = authority.field.conservative_outer_radius_metres();
    let Ok(observer) =
        view.anchor()
            .relative_at_scale_bounded_f64(authority.origin, SpatialScale::ZERO, f64::MAX)
    else {
        return;
    };
    let observer = observer + view.projection_eye_offset_metres();
    let distance = observer.length();
    if !distance.is_finite() || distance <= radius {
        return;
    }
    let outward = observer / distance;
    let cap_radius = radius * (1.0 - (radius / distance).powi(2)).max(0.0).sqrt();
    let cap_depth = radius * (1.0 - radius / distance);

    let selected = (SpatialScale::MIN.exponent()
        ..=authority.field.coarsest_detail_scale().exponent())
        .filter_map(SpatialScale::new)
        .filter(|scale| authority.domain.realizes(*scale))
        .find(|scale| {
            let edge = MATERIALIZATION_CHUNK_SIZE as f64 * scale.metres_per_native();
            (2.0 * cap_radius / edge).powi(2) <= VIEW_SURFACE_CHUNK_BUDGET
        })
        .or_else(|| {
            (SpatialScale::MIN.exponent()..=authority.field.coarsest_detail_scale().exponent())
                .rev()
                .filter_map(SpatialScale::new)
                .find(|scale| authority.domain.realizes(*scale))
        });
    let Some(selected) = selected else { return };

    // A tangent disk plus the sphere-cap depth, enclosed in a world-aligned
    // box. The view frustum and baseline surface shell prune it to sparse cells.
    let tangent_u = outward.any_orthonormal_vector();
    let tangent_v = outward.cross(tangent_u);
    let center_metres = outward * (radius - cap_depth * 0.5);
    let half_metres =
        cap_radius * (tangent_u.abs() + tangent_v.abs()) + outward.abs() * (cap_depth * 0.5);
    let focus = authority
        .origin
        .translated_metres_f64(outward * radius)
        .ok();
    let parent = selected
        .exponent()
        .checked_add(1)
        .and_then(SpatialScale::new)
        .filter(|scale| *scale != selected);
    for scale in Some(selected).into_iter().chain(parent) {
        if !authority.domain.realizes(scale) {
            continue;
        }
        let edge = MATERIALIZATION_CHUNK_SIZE as f64 * scale.metres_per_native();
        let Ok(center) = authority.origin.translated_metres_f64(center_metres) else {
            continue;
        };
        let half = half_metres + DVec3::splat(edge * 2.0);
        let half = Vec3::new(
            scale.metres_to_native_f64(half.x) as f32,
            scale.metres_to_native_f64(half.y) as f32,
            scale.metres_to_native_f64(half.z) as f32,
        );
        if !view.intersects_presentation_native_aabb(scale, &center, half) {
            continue;
        }
        let scope = SpatialDemandScope::at_scale(view.source(), scale, center, half, 0);
        output.push_intent(VoxelRealizationIntent {
            target: VoxelRealizationIntentTarget::Celestial(VoxelRealizationTarget::new(
                authority.entity,
                scale,
            )),
            scope,
            roles: presentation_roles(),
            view_source: Some(view.source()),
            residency_half_extent_native: materialization_residency_extent(half),
            priority_focus: focus,
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
    // Every demanded voxel Scale may present its own realized surface. Fine
    // presentation clips only the covered part of its coarser parent; the
    // selected interaction Scale alone owns physical collision and editing.
    let mut roles = presentation_roles();

    if target_scale != physical_target_scale {
        return roles;
    }

    if domain.collides(target_scale) {
        roles = roles.union(UsfScaleRoleMask::COLLISION);
    }
    if domain.editable(target_scale) {
        roles = roles.union(UsfScaleRoleMask::EDITING);
    }
    roles
}

/// Celestial scale realizations are created only for spatial/capability
/// interest. Demanded ancestor realizations provide coarse visual context.
fn collect_sources(
    spatial: &SpatialDemandSnapshot,
    voxel_sources: &Query<Option<&SpatialRefinementDemand>, With<VoxelMaterializationDemand>>,
) -> Vec<VoxelDemandSource> {
    let mut sources = Vec::new();
    for scope in spatial.iter() {
        let Ok(refinement) = voxel_sources.get(scope.source()) else {
            continue;
        };
        let minimum_realization_scale = refinement.and_then(|value| value.minimum_scale());
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
            refinement_half_extent_native: refinement
                .map(|value| value.half_extent_native_at(refinement_tip_scale)),
        });
    }

    sources
}

#[derive(Clone, Copy)]
struct CelestialAuthority<'a> {
    entity: Entity,
    origin: &'a UsfPosition,
    frame: &'a UsfSemanticFrame,
    field: &'a CelestialVoxelField,
    domain: VoxelScaleDomain,
}

fn append_celestial_intents(
    output: &mut VoxelRealizationIntentSnapshot,
    authority: CelestialAuthority<'_>,
    source: VoxelDemandSource,
    motions: &SpatialDemandMotionSnapshot,
    physical_target_scale: SpatialScale,
) {
    let Some(contact) = observe_celestial_contact(
        authority.origin,
        authority.frame,
        authority.field,
        source,
        motions,
    ) else {
        return;
    };

    let plan = realization_plan(source, authority.domain);
    for step in plan.steps_coarse_to_fine() {
        let scale = step.scale();
        let Some(scope) = celestial_contact_volume_demand(
            contact.boundary_center,
            contact.signed_clearance_metres,
            contact.outer_clearance_metres,
            contact.closing_speed_metres_per_second,
            source.scope,
            scale,
            step.half_extent_native(),
            step.priority(),
        ) else {
            continue;
        };

        // Demand tracks the current source immediately. Coverage/presentation
        // own make-before-break retention of the previous realized scope.
        output.push_intent(VoxelRealizationIntent {
            target: VoxelRealizationIntentTarget::Celestial(VoxelRealizationTarget::new(
                authority.entity,
                scale,
            )),
            scope,
            roles: roles_for_scale(authority.domain, scale, physical_target_scale),
            view_source: None,
            residency_half_extent_native: materialization_residency_extent(
                scope.half_extent_native(),
            ),
            priority_focus: priority_focus_within_scope(contact.boundary_center, scope, scale),
        });
    }
}

fn append_standalone_intents(
    output: &mut VoxelRealizationIntentSnapshot,
    realization_entity: Entity,
    scale: SpatialScale,
    sources: &[VoxelDemandSource],
) {
    for source in sources.iter().copied() {
        if source.scope.scale() == scale {
            output.push(
                VoxelRealizationIntentTarget::ExistingRealization(realization_entity),
                source.scope,
                full_runtime_roles(),
                None,
                materialization_residency_extent(source.scope.half_extent_native()),
                None,
            );
        }
    }
}

pub(in crate::voxel) fn collect_voxel_realization_intents(
    spatial: Res<SpatialDemandSnapshot>,
    views: Res<UsfViewDemandSnapshot>,
    motions: Res<SpatialDemandMotionSnapshot>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    voxel_sources: Query<Option<&SpatialRefinementDemand>, With<VoxelMaterializationDemand>>,
    standalone_realizations: Query<
        (Entity, &UsfScaleLayer, Option<&UsfLogicalRealizationOf>),
        With<VoxelScaleRealization>,
    >,
    celestial_authorities: Query<(
        Entity,
        &UsfPosition,
        &UsfSemanticFrame,
        &CelestialVoxelField,
        &VoxelScaleDomain,
    )>,
    mut residency_requests: ResMut<UsfResidencyRequests>,
    mut output: ResMut<VoxelRealizationIntentSnapshot>,
) {
    let mut next = VoxelRealizationIntentSnapshot::default();
    //
    // The destination must be able to build the capability that gates entry
    // into it. `scale()` is the committed outgoing chart; `target_scale()` is
    // the requested destination during a handoff and therefore owns prep work.
    let physical_target_scale = interaction.target_scale();

    let sources = collect_sources(&spatial, &voxel_sources);

    for (entity, origin, frame, field, domain) in &celestial_authorities {
        let authority = CelestialAuthority {
            entity,
            origin,
            frame,
            field,
            domain: *domain,
        };
        for source in sources.iter().copied() {
            append_celestial_intents(
                &mut next,
                authority,
                source,
                &motions,
                physical_target_scale,
            );
        }
        for view in views.iter() {
            append_celestial_view_intents(&mut next, authority, view);
        }
    }

    for (realization_entity, layer, logical_realization) in &standalone_realizations {
        if logical_realization.is_none() {
            append_standalone_intents(&mut next, realization_entity, layer.scale(), &sources);
        }
    }

    next.sort_for_publication();

    for intent in next.iter() {
        residency_requests.request(SpatialDemandScope::at_scale(
            intent.scope.source(),
            intent.scope.scale(),
            intent.scope.center(),
            intent.residency_half_extent_native,
            intent.scope.priority(),
        ));
    }

    output.replace_if_changed(next);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coarse_voxel_ancestor_presents_without_physical_authority() {
        let coarse = SpatialScale::new(2).unwrap();
        let physical = SpatialScale::new(1).unwrap();
        let domain = VoxelScaleDomain::contiguous(physical, coarse)
            .with_collision_slices(UsfScaleSliceMask::inclusive_range(physical, coarse))
            .with_editing_slices(UsfScaleSliceMask::inclusive_range(physical, coarse));

        let coarse_roles = roles_for_scale(domain, coarse, physical);
        assert!(coarse_roles.contains(UsfScaleRoleMask::REALIZATION));
        assert!(coarse_roles.contains(UsfScaleRoleMask::PRESENTATION));
        assert!(!coarse_roles.contains(UsfScaleRoleMask::COLLISION));
        assert!(!coarse_roles.contains(UsfScaleRoleMask::EDITING));

        let physical_roles = roles_for_scale(domain, physical, physical);
        assert!(physical_roles.contains(UsfScaleRoleMask::PRESENTATION));
        assert!(physical_roles.contains(UsfScaleRoleMask::COLLISION));
        assert!(physical_roles.contains(UsfScaleRoleMask::EDITING));
    }
}
