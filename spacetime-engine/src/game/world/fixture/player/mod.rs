//! Prepare the locally controlled subject for the authored celestial fixture.
//!
//! World bootstrap authors semantic/canonical position. Runtime `Transform` is
//! always derived from canonical position plus the subject's current Scale Slice.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::{
    ecs::UsfOwnershipQuery,
    game::{
        control::LocalControlSubject,
        flight::FlightContactState,
        locomotion::{
            ControlledSubjectLocomotion, LocomotionInhibition,
            LocomotionInhibitionReason,
        },
    },
    physics::{
        PhysicalBoxHull,
        character::{CharacterControlFrame, CharacterLocomotionFrame},
    },
    portal::PortalTraveler,
    voxel::CelestialVoxelField,
    spatial::{
        SpatialRefinementDemand, UsfCanonicalMotion, UsfInteractionScaleAffinity,
        UsfPosition, UsfScaleLayer, UsfSemanticFrame, UsfScaleRoleMask,
        UsfSpatialFrame, UsfSpatialTransition,
        UsfSpatialTransitionQueue, UsfTransitionVelocity,
    },
};

use super::{FixtureArrivalSite, spawn::resolve_good_spawn};

const FIXTURE_SPAWN_GAP_METRES: f32 = 0.75;
const FIXTURE_SPACECRAFT_AIR_GAP_METRES: f32 = 25.0;

pub(super) fn prepare_controlled_subject(
    arrival_site: Res<FixtureArrivalSite>,
    frame: Res<UsfSpatialFrame>,
    ownership: UsfOwnershipQuery,
    mut transitions: ResMut<UsfSpatialTransitionQueue>,
    mut positions: ParamSet<(
        Query<&mut UsfPosition>,
        Query<(&UsfPosition, &UsfSemanticFrame, &CelestialVoxelField)>,
    )>,
    subject: Single<
        (
            Entity,
            &UsfScaleLayer,
            &UsfInteractionScaleAffinity,
            &mut Transform,
            &mut PortalTraveler,
            &mut LinearVelocity,
            &mut UsfCanonicalMotion,
            &mut ControlledSubjectLocomotion,
            Option<&mut FlightContactState>,
            Option<&mut LocomotionInhibition>,
            &mut CharacterControlFrame,
            &mut CharacterLocomotionFrame,
            &PhysicalBoxHull,
            &mut SpatialRefinementDemand,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (
        realization,
        layer,
        affinity,
        mut transform,
        mut traveler,
        mut velocity,
        mut motion,
        mut locomotion,
        mut flight_contact,
        mut inhibition,
        mut control,
        mut locomotion_frame,
        hull,
        mut refinement,
    ) = subject.into_inner();

    let Some(authored_site) = arrival_site.site() else {
        error!("fixture bootstrap has no authored body-surface arrival hint");
        return;
    };

    let Ok((body_origin, body_frame, field)) =
        positions.p1().get(authored_site.body()).map(|(origin, frame, field)| {
            (*origin, *frame, *field)
        })
    else {
        error!(
            body = ?authored_site.body(),
            "fixture bootstrap arrival body semantic field is unavailable"
        );
        return;
    };

    let Some(site) = resolve_good_spawn(
        authored_site,
        body_origin,
        body_frame,
        field,
        *hull,
    ) else {
        error!(
            body = ?authored_site.body(),
            "fixture bootstrap could not resolve a safe canonical spawn near the authored hint"
        );
        return;
    };

    let Some(semantic_entity) = ownership.semantic_of(realization) else {
        error!(
            realization = ?realization,
            "fixture bootstrap subject has no semantic USF owner"
        );
        return;
    };

    locomotion_frame.up = site.up();
    let aligned = locomotion_frame.aligned_rotation(transform.rotation);
    transform.rotation = aligned;
    control.snap_to(aligned);

    // Generic oriented-body support radius, in metres.
    let support_metres = hull.projection_radius_metres(aligned, site.up());
    let spawn_gap_metres = if flight_contact.is_some() {
        FIXTURE_SPACECRAFT_AIR_GAP_METRES
    } else {
        FIXTURE_SPAWN_GAP_METRES
    };
    let clearance_metres = support_metres + spawn_gap_metres;
    let clearance_native = site.scale().metres_to_native_f32(clearance_metres);

    let Ok(canonical) = site
        .surface()
        .translated_at_scale(site.scale(), site.up() * clearance_native)
    else {
        error!("fixture bootstrap could not offset its body-surface arrival site");
        return;
    };

    let Ok(runtime_position) = canonical.relative_at_scale_bounded(
        frame.origin(),
        layer.scale(),
        f32::MAX,
    ) else {
        error!(
            subject_scale = %layer.scale(),
            "canonical fixture bootstrap spawn could not project into subject runtime chart"
        );
        return;
    };

    let mut semantic_positions = positions.p0();
    let Ok(mut semantic) = semantic_positions.get_mut(semantic_entity) else {
        error!(
            subject = ?semantic_entity,
            "controlled subject semantic position is unavailable during fixture bootstrap"
        );
        return;
    };

    *semantic = canonical;
    transform.translation = runtime_position;

    // Relocate in the current bootstrap chart so demand is centered on the
    // destination immediately. Interaction itself is coverage-gated below.
    traveler.commit_position(runtime_position);

    // fixture-bootstrap-uses-controlled-affinity-v1
    //
    // The site owns canonical arrival POSITION. The controlled manifestation's
    // affinity owns interaction SCALE. Runtime evidence showed a flight-capable
    // S+1 subject first requesting +1, then this fixture path forcing +0 and
    // requeueing that wrong one-shot transition forever.
    let interaction_scale = affinity.scale();
    let required_roles = affinity.required_roles();

    refinement.request_through(interaction_scale);

    let coverage_radius_metres =
        hull.half_extents_metres().length() + spawn_gap_metres;
    let coverage_radius_native =
        interaction_scale.metres_to_native_f32(coverage_radius_metres);

    let mut transition = UsfSpatialTransition::new(
        semantic_entity,
        canonical,
        UsfTransitionVelocity::Zero,
    )
    .with_scale(interaction_scale)
    // View framing is presentation policy; the authored site may still seed it.
    .with_view_exponent(f32::from(site.scale().exponent()));

    if !required_roles.is_empty() {
        transition = transition.requiring_coverage_from(
            site.body(),
            required_roles,
            coverage_radius_native,
        );
    }

    transitions.request(transition);

    velocity.0 = Vec3::ZERO;
    motion.stop();
    locomotion.request_automatic();

    // Safe fixture spacecraft begin clear of terrain and immediately usable.
    if let Some(contact) = flight_contact.as_deref_mut() {
        contact.launch();
        locomotion.set_thrusters_enabled(true);
        locomotion.set_rcs_enabled(true);
    }
    if let Some(inhibition) = inhibition.as_deref_mut() {
        inhibition.set(LocomotionInhibitionReason::SurfaceContact, false);
    }

    info!(
        subject = ?semantic_entity,
        body = ?site.body(),
        bootstrap_scale = %layer.scale(),
        site_scale = %site.scale(),
        interaction_scale = %interaction_scale,
        interaction_required_roles = required_roles.bits(),
        support_metres,
        gap_metres = spawn_gap_metres,
        runtime = ?runtime_position,
        "prepared coverage-gated canonical body-surface arrival"
    );
}
