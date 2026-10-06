//! Hull-based landing opportunity and resolved placement.

use super::*;

const LANDING_PROBE_LIFT_METRES: f32 = LANDING_PROBE_METRES;
const LANDING_SEPARATION_SPEED_EPSILON_METRES_PER_SECOND: f64 = 0.25;

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
            &LocomotionCapabilities,
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
        capabilities,
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
        || locomotion.regime() != LocomotionRegime::SpacecraftFlight
        || !capabilities.landing()
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

/// Landing and launch mutate the controlled ship's physical and semantic pose
/// before any boarding control transfer can be requested this frame.
pub(super) fn handle_landing_actions(
    input: Res<PlayerInputFrame>,
    frame: Res<UsfRuntimeChartState>,
    ownership: UsfOwnershipQuery,
    mut semantic_positions: Query<&mut UsfPosition>,
    mut controlled_ship: Query<
        (
            Entity,
            &mut Transform,
            &UsfScaleLayer,
            &CharacterLocomotionFrame,
            &mut ControlledSubjectLocomotion,
            &mut FlightActuation,
            &mut LinearVelocity,
            &mut UsfCanonicalMotion,
            &mut LocomotionInhibition,
            &mut FlightContactState,
            &FlightLandingOpportunity,
            &SpacecraftLandingSolution,
            &mut PortalTraveler,
        ),
        (
            With<SpacecraftManifestation>,
            With<LocalControlSubject>,
            Without<Player>,
        ),
    >,
) {
    let Ok((
        ship_entity,
        mut ship_transform,
        ship_layer,
        ship_frame,
        mut ship_locomotion,
        mut ship_actuation,
        mut ship_velocity,
        mut ship_motion,
        mut ship_inhibition,
        mut ship_contact,
        ship_landing,
        ship_landing_solution,
        mut ship_traveler,
    )) = controlled_ship.single_mut()
    else {
        return;
    };
    if !ship_contact.is_landed()
        && input.gameplay_active()
        && input.just_pressed(PlayerAction::ToggleLanding)
        && ship_landing.available()
        && let Some((settled_translation, aligned)) =
            ship_landing_solution.at_scale(ship_layer.scale())
    {
        let Ok(settled_semantic) = frame
            .origin()
            .translated_at_scale(ship_layer.scale(), settled_translation)
        else {
            return;
        };
        let Some(semantic_ship) = ownership.semantic_of(ship_entity) else {
            return;
        };
        let Ok(mut semantic_position) = semantic_positions.get_mut(semantic_ship) else {
            return;
        };

        ship_transform.translation = settled_translation;
        ship_transform.rotation = aligned;
        ship_traveler.commit_position(settled_translation);
        *semantic_position = settled_semantic;
        ship_velocity.0 = Vec3::ZERO;
        ship_motion.stop();
        ship_contact.land();
        ship_inhibition.set(LocomotionInhibitionReason::SurfaceContact, true);
        ship_locomotion.request_automatic();
        ship_actuation.set_thrusters_enabled(false);
        ship_actuation.set_rcs_enabled(false);
        return;
    }

    if ship_contact.is_landed()
        && input.gameplay_active()
        && input.just_pressed(PlayerAction::TakeOff)
    {
        ship_contact.launch();
        ship_inhibition.set(LocomotionInhibitionReason::SurfaceContact, false);
        ship_locomotion.request_regime(LocomotionRegime::SpacecraftFlight);
        ship_actuation.set_thrusters_enabled(true);
        ship_actuation.set_rcs_enabled(true);
        ship_velocity.0 = ship_frame.up() * ship_layer.scale().metres_to_native_f32(5.0);
        ship_motion.set_from_native_velocity(ship_layer.scale(), ship_velocity.0);
        return;
    }
}
