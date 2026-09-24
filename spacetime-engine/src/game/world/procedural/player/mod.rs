//! Prepare the locally controlled subject for the procedural root world.
//!
//! World bootstrap authors semantic/canonical position. Runtime `Transform` is
//! always derived from canonical position plus the subject's current Scale Slice.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    ecs::UsfManifestationOf,
    game::{
        control::LocalControlSubject,
        locomotion::ControlledSubjectLocomotion,
    },
    physics::{
        PhysicalBoxHull,
        character::{CharacterControlFrame, CharacterLocomotionFrame},
    },
    portal::PortalTraveler,
    spatial::{
        SpatialRefinementDemand, UsfCanonicalMotion, UsfPosition, UsfScaleLayer,
        UsfScaleRoleMask, UsfSpatialFrame, UsfSpatialTransition,
        UsfSpatialTransitionQueue, UsfTransitionVelocity,
    },
};

use super::ProceduralArrivalSite;

const PROCEDURAL_SPAWN_GAP_METRES: f32 = 0.75;

pub(super) fn prepare_controlled_subject(
    arrival_site: Res<ProceduralArrivalSite>,
    frame: Res<UsfSpatialFrame>,
    mut transitions: ResMut<UsfSpatialTransitionQueue>,
    mut semantic_positions: Query<&mut UsfPosition>,
    subject: Single<
        (
            &UsfManifestationOf,
            &UsfScaleLayer,
            &mut Transform,
            &mut PortalTraveler,
            &mut LinearVelocity,
            &mut UsfCanonicalMotion,
            &mut ControlledSubjectLocomotion,
            &mut CharacterControlFrame,
            &mut CharacterLocomotionFrame,
            &PhysicalBoxHull,
            &mut SpatialRefinementDemand,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (
        manifestation,
        layer,
        mut transform,
        mut traveler,
        mut velocity,
        mut motion,
        mut locomotion,
        mut control,
        mut locomotion_frame,
        hull,
        mut refinement,
    ) = subject.into_inner();

    let Some(site) = arrival_site.site() else {
        error!("procedural bootstrap has no resolved body-surface arrival site");
        return;
    };

    locomotion_frame.up = site.up();
    let aligned = locomotion_frame.aligned_rotation(transform.rotation);
    transform.rotation = aligned;
    control.snap_to(aligned);

    // Generic oriented-body support radius, in metres.
    let support_metres = hull.projection_radius_metres(aligned, site.up());
    let clearance_metres = support_metres + PROCEDURAL_SPAWN_GAP_METRES;
    let clearance_native = site.scale().metres_to_native_f32(clearance_metres);

    let Ok(canonical) = site
        .surface()
        .translated_at_scale(site.scale(), site.up() * clearance_native)
    else {
        error!("procedural bootstrap could not offset its body-surface arrival site");
        return;
    };

    let Ok(runtime_position) = canonical.relative_at_scale_bounded(
        frame.origin(),
        layer.scale(),
        f32::MAX,
    ) else {
        error!(
            subject_scale = %layer.scale(),
            "canonical procedural bootstrap spawn could not project into subject runtime chart"
        );
        return;
    };

    let Ok(mut semantic) = semantic_positions.get_mut(manifestation.0) else {
        error!(
            subject = ?manifestation.0,
            "controlled subject semantic position is unavailable during procedural bootstrap"
        );
        return;
    };

    *semantic = canonical;
    transform.translation = runtime_position;

    // Relocate in the current bootstrap chart so demand is centered on the
    // destination immediately. Interaction itself is coverage-gated below.
    traveler.commit_position(runtime_position);

    // Bootstrap is not an interstellar approach. Realize the destination site
    // immediately and jump directly to its physical interaction chart only when
    // the selected body's own collision/realization coverage exists there.
    refinement.request_through(site.scale());

    let coverage_radius_metres =
        hull.half_extents_metres().length() + PROCEDURAL_SPAWN_GAP_METRES;
    let coverage_radius_native =
        site.scale().metres_to_native_f32(coverage_radius_metres);

    transitions.request(
        UsfSpatialTransition::new(
            manifestation.0,
            canonical,
            UsfTransitionVelocity::Zero,
        )
        .with_scale(site.scale())
        .with_view_exponent(f32::from(site.scale().exponent()))
        .requiring_coverage_from(
            site.body(),
            UsfScaleRoleMask::REALIZATION.union(UsfScaleRoleMask::COLLISION),
            coverage_radius_native,
        ),
    );

    velocity.0 = Vec3::ZERO;
    motion.stop();
    locomotion.request_automatic();
    locomotion.set_thrusters_enabled(false);

    info!(
        subject = ?manifestation.0,
        body = ?site.body(),
        bootstrap_scale = %layer.scale(),
        site_scale = %site.scale(),
        support_metres,
        gap_metres = PROCEDURAL_SPAWN_GAP_METRES,
        runtime = ?runtime_position,
        "prepared coverage-gated canonical body-surface arrival"
    );
}
