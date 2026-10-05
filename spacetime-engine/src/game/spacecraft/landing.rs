//! Hull-based landing opportunity and resolved placement.

use super::*;

#[derive(Component, Debug, Default, Clone, Copy)]
pub(crate) struct SpacecraftLandingSolution {
    resolved: Option<(SpatialScale, Vec3, Quat)>,
}

impl SpacecraftLandingSolution {
    fn clear(&mut self) {
        self.resolved = None;
    }
    fn set(&mut self, scale: SpatialScale, translation: Vec3, rotation: Quat) {
        self.resolved = Some((scale, translation, rotation));
    }
    pub(super) fn at_scale(self, scale: SpatialScale) -> Option<(Vec3, Quat)> {
        let (s, t, r) = self.resolved?;
        (s == scale).then_some((t, r))
    }
}

pub(super) fn detect_landing(
    spatial_query: SpatialQuery,
    physics_charts: UsfPhysicsSlices,
    mut ships: Query<
        (
            Entity,
            &Transform,
            &UsfScaleLayer,
            &DetailedBodyScale,
            &CharacterLocomotionFrame,
            &CharacterMovementConfig,
            &Collider,
            Option<&KinematicQueryExclusions>,
            &UsfCanonicalMotion,
            &ControlledSubjectLocomotion,
            &FlightSafetyProfile,
            &FlightContactState,
            &mut FlightLandingOpportunity,
            &mut SpacecraftLandingSolution,
        ),
        (
            With<SpacecraftManifestation>,
            With<LocalControlSubject>,
            Without<Player>,
        ),
    >,
) {
    let Ok((
        entity,
        transform,
        layer,
        detailed,
        locomotion_frame,
        movement,
        collider,
        exclusions,
        motion,
        locomotion,
        safety,
        contact,
        mut opportunity,
        mut solution,
    )) = ships.single_mut()
    else {
        return;
    };

    opportunity.set_available(false);
    solution.clear();

    if contact.is_landed()
        || layer.scale() != detailed.0
        || locomotion.regime() != LocomotionRegime::LocalFlight
        || motion.speed_metres_per_second() > safety.preferred_contact_speed_metres_per_second()
    {
        return;
    }

    let up = locomotion_frame.up();
    let up_si = DVec3::new(f64::from(up.x), f64::from(up.y), f64::from(up.z));
    if motion.velocity_metres_per_second().dot(up_si)
        > LANDING_SEPARATION_SPEED_EPSILON_METRES_PER_SECOND
    {
        return;
    }

    let Ok(direction) = Dir3::new(-up) else {
        return;
    };
    let lift = layer
        .scale()
        .metres_to_native_f32(LANDING_PROBE_LIFT_METRES);
    let probe_origin = transform.translation + up * lift;
    let max_distance = layer
        .scale()
        .metres_to_native_f32(LANDING_PROBE_LIFT_METRES + LANDING_PROBE_METRES);
    let filter = physics_charts.filter_for_scale(
        layer.scale(),
        std::iter::once(entity).chain(exclusions.into_iter().flat_map(|items| items.iter())),
    );
    let config = ShapeCastConfig {
        max_distance,
        ignore_origin_penetration: false,
        ..default()
    };
    let aligned = locomotion_frame.aligned_rotation(transform.rotation);
    let Some(hit) =
        spatial_query.cast_shape(collider, probe_origin, aligned, direction, &config, &filter)
    else {
        return;
    };
    if hit.normal1.dot(up) < movement.min_ground_dot {
        return;
    }

    let settled = probe_origin - up * hit.distance.max(0.0);
    opportunity.set_available(true);
    solution.set(layer.scale(), settled, aligned);
}
