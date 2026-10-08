//! Translate generic spatial intent into voxel-specific capability scopes.

use super::*;

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
    pinned: Option<&VoxelPinnedMaterializationDemand>,
    sources: &[VoxelDemandSource],
) {
    if let Some(pinned) = pinned {
        let scope = SpatialDemandScope::at_scale(
            realization_entity,
            scale,
            pinned.center(),
            pinned.half_extent_native(),
            pinned.priority(),
        );
        output.push(
            VoxelRealizationIntentTarget::ExistingRealization(realization_entity),
            scope,
            presentation_roles(),
            None,
            materialization_residency_extent(scope.half_extent_native()),
            None,
        );
    }

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
    motions: Res<SpatialDemandMotionSnapshot>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    voxel_sources: Query<Option<&SpatialRefinementDemand>, With<VoxelMaterializationDemand>>,
    standalone_realizations: Query<
        (
            Entity,
            &UsfScaleLayer,
            Option<&UsfLogicalRealizationOf>,
            Option<&VoxelPinnedMaterializationDemand>,
        ),
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
    }

    for (realization_entity, layer, logical_realization, pinned) in &standalone_realizations {
        if logical_realization.is_none() {
            append_standalone_intents(
                &mut next,
                realization_entity,
                layer.scale(),
                pinned,
                &sources,
            );
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
