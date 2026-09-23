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
        locomotion::{ControlledSubjectHull, ControlledSubjectLocomotion},
    },
    physics::character::{CharacterControlFrame, CharacterLocomotionFrame},
    portal::PortalTraveler,
    spatial::{UsfCanonicalMotion, UsfPosition, UsfScaleLayer, UsfSpatialFrame},
};

use super::ProceduralArrivalSite;

const PROCEDURAL_SPAWN_CLEARANCE_METRES: f32 = 12.0;

pub(super) fn prepare_controlled_subject(
    arrival_site: Res<ProceduralArrivalSite>,
    frame: Res<UsfSpatialFrame>,
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
            Option<&ControlledSubjectHull>,
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
    ) = subject.into_inner();

    let Some(site) = arrival_site.site() else {
        error!("procedural bootstrap has no resolved body-surface arrival site");
        return;
    };

    let half_height = hull
        .map(|hull| hull.size().y * 0.5)
        .unwrap_or(0.0);
    let clearance_metres = PROCEDURAL_SPAWN_CLEARANCE_METRES + half_height;
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

    // Arrival is a physical pose transaction. Seed local gravity/control basis
    // from the selected surface normal before the first movement/grounding tick.
    locomotion_frame.up = site.up();
    let aligned = locomotion_frame.aligned_rotation(transform.rotation);
    transform.rotation = aligned;
    control.snap_to(aligned);

    traveler.commit_position(runtime_position);
    velocity.0 = Vec3::ZERO;
    motion.stop();
    locomotion.request_automatic();
    locomotion.set_thrusters_enabled(false);

    info!(
        subject = ?manifestation.0,
        body = ?site.body(),
        subject_scale = %layer.scale(),
        site_scale = %site.scale(),
        runtime = ?runtime_position,
        "prepared controlled subject from canonical body-surface arrival site"
    );
}
