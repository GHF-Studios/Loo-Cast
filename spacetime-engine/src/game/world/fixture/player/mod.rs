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
        flight::{FlightControlCommand, FlightControlRequest},
        locomotion::{
            ControlledSubjectLocomotion, LocomotionInhibition, LocomotionInhibitionReason,
        },
    },
    physics::{
        PhysicalBoxHull,
        character::{CharacterControlFrame, CharacterLocomotionFrame},
    },
    portal::PortalTraveler,
    spatial::{
        SpatialRefinementDemand, SpatialScale, UsfCanonicalMotion, UsfInteractionScaleAffinity,
        UsfPosition, UsfRuntimeChartState, UsfScaleLayer, UsfSemanticFrame, UsfSpatialTransition,
        UsfSpatialTransitions, UsfTransitionVelocity,
    },
    voxel::CelestialVoxelField,
};

use super::{BodySurfaceSite, FixtureArrivalSite, spawn::resolve_good_spawn};

const FIXTURE_SPAWN_GAP_METRES: f32 = 0.75;
const FIXTURE_SPACECRAFT_AIR_GAP_METRES: f32 = 25.0;

/// Canonical placement is derived from the authored surface and physical hull;
/// the runtime point is only its projection in the current bootstrap chart.
struct FixtureArrivalPlacement {
    site: BodySurfaceSite,
    canonical: UsfPosition,
    runtime_position: Vec3,
    support_metres: f32,
    gap_metres: f32,
}

enum FixtureArrivalError {
    Offset,
    Projection,
}

impl FixtureArrivalPlacement {
    fn resolve(
        site: BodySurfaceSite,
        hull: PhysicalBoxHull,
        aligned: Quat,
        spacecraft: bool,
        chart_origin: &UsfPosition,
        subject_scale: SpatialScale,
    ) -> Result<Self, FixtureArrivalError> {
        let support_metres = hull.projection_radius_metres(aligned, site.up());
        let gap_metres = if spacecraft {
            FIXTURE_SPACECRAFT_AIR_GAP_METRES
        } else {
            FIXTURE_SPAWN_GAP_METRES
        };
        let clearance_native = site
            .scale()
            .metres_to_native_f32(support_metres + gap_metres);
        let canonical = site
            .surface()
            .translated_at_scale(site.scale(), site.up() * clearance_native)
            .map_err(|_| FixtureArrivalError::Offset)?;
        let runtime_position = canonical
            .relative_at_scale_bounded(chart_origin, subject_scale, f32::MAX)
            .map_err(|_| FixtureArrivalError::Projection)?;
        Ok(Self {
            site,
            canonical,
            runtime_position,
            support_metres,
            gap_metres,
        })
    }

    /// The site chooses position and initial view scale. The controlled
    /// manifestation's affinity owns interaction scale and coverage roles.
    /// Using the site's scale here previously requeued an S0 transition for
    /// an S+1 flight subject indefinitely.
    fn transition(
        &self,
        subject: Entity,
        affinity: &UsfInteractionScaleAffinity,
        hull: PhysicalBoxHull,
    ) -> UsfSpatialTransition {
        let interaction_scale = affinity.scale();
        let mut transition =
            UsfSpatialTransition::new(subject, self.canonical, UsfTransitionVelocity::Zero)
                .with_scale(interaction_scale)
                .with_view_exponent(f32::from(self.site.scale().exponent()));
        let roles = affinity.required_roles();
        if !roles.is_empty() {
            let coverage_metres = hull.half_extents_metres().length() + self.gap_metres;
            transition = transition.requiring_coverage_from(
                self.site.body(),
                roles,
                interaction_scale.metres_to_native_f32(coverage_metres),
            );
        }
        transition
    }
}

pub(super) fn prepare_controlled_subject(
    arrival_site: Res<FixtureArrivalSite>,
    frame: Res<UsfRuntimeChartState>,
    ownership: UsfOwnershipQuery,
    mut transitions: ResMut<UsfSpatialTransitions>,
    mut flight_requests: MessageWriter<FlightControlRequest>,
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

    let Ok((body_origin, body_frame, field)) = positions
        .p1()
        .get(authored_site.body())
        .map(|(origin, frame, field)| (*origin, *frame, *field))
    else {
        error!(
            body = ?authored_site.body(),
            "fixture bootstrap arrival body semantic field is unavailable"
        );
        return;
    };

    let Some(site) = resolve_good_spawn(authored_site, body_origin, body_frame, field, *hull)
    else {
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

    let placement = match FixtureArrivalPlacement::resolve(
        site,
        *hull,
        aligned,
        flight_contact.is_some(),
        frame.origin(),
        layer.scale(),
    ) {
        Ok(placement) => placement,
        Err(FixtureArrivalError::Offset) => {
            error!("fixture bootstrap could not offset its body-surface arrival site");
            return;
        }
        Err(FixtureArrivalError::Projection) => {
            error!(
                subject_scale = %layer.scale(),
                "canonical fixture bootstrap spawn could not project into subject runtime chart"
            );
            return;
        }
    };
    let canonical = placement.canonical;
    let runtime_position = placement.runtime_position;

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

    let interaction_scale = affinity.scale();
    let required_roles = affinity.required_roles();

    refinement.request_through(interaction_scale);
    transitions.relocate(placement.transition(semantic_entity, affinity, *hull));

    velocity.0 = Vec3::ZERO;
    motion.stop();
    locomotion.request_automatic();

    // Safe fixture spacecraft begin clear of terrain and immediately usable.
    if let Some(contact) = flight_contact.as_deref_mut() {
        contact.launch();
        flight_requests.write(FlightControlRequest::new(
            realization,
            FlightControlCommand::SetMainPropulsion(true),
        ));
        flight_requests.write(FlightControlRequest::new(
            realization,
            FlightControlCommand::SetReactionControl(true),
        ));
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
        support_metres = placement.support_metres,
        gap_metres = placement.gap_metres,
        runtime = ?runtime_position,
        "prepared coverage-gated canonical body-surface arrival"
    );
}
