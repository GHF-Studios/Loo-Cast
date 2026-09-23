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
    portal::PortalTraveler,
    spatial::{UsfCanonicalMotion, UsfPosition, UsfScaleLayer, UsfSpatialFrame},
};

use super::landmarks::UniverseLandmarkIndex;

const PROCEDURAL_SPAWN_CLEARANCE_METRES: f32 = 12.0;

pub(super) fn prepare_controlled_subject(
    landmarks: Res<UniverseLandmarkIndex>,
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
        hull,
    ) = subject.into_inner();

    let Some(landmark) = landmarks.find("earth-surface").into_iter().next() else {
        error!("procedural bootstrap could not resolve the canonical earth-surface landmark");
        return;
    };

    let half_height = hull
        .map(|hull| hull.size().y * 0.5)
        .unwrap_or(0.0);
    let clearance = PROCEDURAL_SPAWN_CLEARANCE_METRES + half_height;
    let Ok(canonical) = landmark
        .arrival
        .translated_at_scale(landmark.display_scale, Vec3::Y * clearance)
    else {
        error!("procedural bootstrap spawn could not offset its canonical landmark");
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
    traveler.commit_position(runtime_position);
    velocity.0 = Vec3::ZERO;
    motion.stop();
    locomotion.request_automatic();
    locomotion.set_thrusters_enabled(false);

    info!(
        subject = ?manifestation.0,
        subject_scale = %layer.scale(),
        authored_scale = %landmark.display_scale,
        runtime = ?runtime_position,
        "prepared controlled subject from canonical procedural spawn"
    );
}
