//! Enter/exit control transactions and embarked presentation state.

use super::*;

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
        ship_locomotion.set_thrusters_enabled(false);
        ship_locomotion.set_rcs_enabled(false);
        return;
    }

    if ship_contact.is_landed()
        && input.gameplay_active()
        && input.just_pressed(PlayerAction::TakeOff)
    {
        ship_contact.launch();
        ship_inhibition.set(LocomotionInhibitionReason::SurfaceContact, false);
        ship_locomotion.request_regime(LocomotionRegime::LocalFlight);
        ship_locomotion.set_thrusters_enabled(true);
        ship_locomotion.set_rcs_enabled(true);
        ship_velocity.0 = ship_frame.up() * ship_layer.scale().metres_to_native_f32(5.0);
        ship_motion.set_from_native_velocity(ship_layer.scale(), ship_velocity.0);
        return;
    }
}

pub(super) fn handle_ship_exit(
    input: Res<PlayerInputFrame>,
    frame: Res<UsfRuntimeChartState>,
    spatial_query: SpatialQuery,
    physics_charts: UsfPhysicsSlices,
    ownership: UsfOwnershipQuery,
    mut commands: Commands,
    mut control_transfers: MessageWriter<LocalControlTransferRequest>,
    player: Single<
        (
            Entity,
            &mut Transform,
            &UsfScaleLayer,
            &mut Visibility,
            &mut SpatialDemandSource,
            &mut LocomotionEnabled,
            &mut ControlledSubjectLocomotion,
            &mut TravelPace,
            &mut CharacterControlFrame,
            &mut CharacterLocomotionFrame,
            &PhysicalBoxHull,
            &CharacterMovementConfig,
            Option<&KinematicQueryExclusions>,
            &mut PortalTraveler,
        ),
        (With<Player>, Without<SpacecraftManifestation>),
    >,
    mut semantic_positions: Query<&mut UsfPosition>,
    mut controlled_ship: Query<
        (
            Entity,
            &Transform,
            &UsfScaleLayer,
            &CharacterLocomotionFrame,
            &PhysicalBoxHull,
            &mut SpatialDemandSource,
            &FlightContactState,
        ),
        (
            With<SpacecraftManifestation>,
            With<LocalControlSubject>,
            Without<Player>,
        ),
    >,
) {
    if let Ok((
        ship_entity,
        ship_transform,
        ship_layer,
        ship_frame,
        ship_hull,
        mut ship_demand,
        ship_contact,
    )) = controlled_ship.single_mut()
    {
        if !ship_contact.is_landed()
            || !input.gameplay_active()
            || !input.just_pressed(PlayerAction::Interact)
        {
            return;
        }

        let (
            player_entity,
            mut player_transform,
            _player_layer,
            mut player_visibility,
            mut player_demand,
            mut player_enabled,
            mut player_locomotion,
            mut player_pace,
            mut player_control,
            mut player_frame,
            player_hull,
            player_movement,
            player_exclusions,
            mut player_traveler,
        ) = player.into_inner();

        // Vehicle exit is a real standing-pose query. The player does not
        // inherit the ship-center altitude: search for walkable support beside
        // the landed hull using the player's own detailed body.
        let exit_frame = CharacterLocomotionFrame {
            up: ship_frame.up(),
        };
        let aligned_player = exit_frame.aligned_rotation(ship_transform.rotation);

        let Some(exit_local) = resolve_disembark_pose(
            &spatial_query,
            &physics_charts,
            ship_entity,
            &ship_transform,
            ship_layer,
            ship_frame,
            ship_hull,
            player_entity,
            player_hull,
            player_movement,
            player_exclusions,
            aligned_player,
        ) else {
            return;
        };
        let Ok(exit_semantic) = frame
            .origin()
            .translated_at_scale(ship_layer.scale(), exit_local)
        else {
            return;
        };

        let Some(player_semantic) = ownership.semantic_of(player_entity) else {
            return;
        };
        let Ok(mut semantic) = semantic_positions.get_mut(player_semantic) else {
            return;
        };
        *semantic = exit_semantic;
        *player_frame = exit_frame;
        player_transform.translation = exit_local;

        // Vehicle exit is a pose transaction. The radial control frame and
        // standing support pose were resolved above before control changes.
        player_transform.rotation = aligned_player;
        player_control.snap_to(aligned_player);

        // Scale-Slice reassignment is part of the deferred control/handoff
        // transaction. The action system only needs to read the current player
        // layer, which keeps its collision-query SystemParams alias-safe.
        commands
            .entity(player_entity)
            .insert(UsfScaleLayer::new(ship_layer.scale()));
        player_traveler.commit_position(exit_local);
        *player_visibility = Visibility::Inherited;
        player_demand.set_enabled(true);
        player_enabled.0 = true;
        // Vehicle test pace is controller state, not character locomotion
        // policy. Reset it on disembark so 2^N ship tuning never leaks into
        // normal walking.
        *player_pace = TravelPace::default();
        player_locomotion.request_automatic();
        player_locomotion.set_thrusters_enabled(false);
        player_locomotion.set_rcs_enabled(false);

        ship_demand.set_enabled(false);

        commands
            .entity(player_semantic)
            .remove::<UsfConstituentOf>();

        control_transfers.write(LocalControlTransferRequest::new(
            player_semantic,
            player_entity,
        ));
        return;
    }
}

/// Boarding chooses a nearby landed ship and queues the semantic control
/// transfer; the controlled player's manifestation is hidden until disembark.
pub(super) fn handle_ship_entry(
    input: Res<PlayerInputFrame>,
    frame: Res<UsfRuntimeChartState>,
    ownership: UsfOwnershipQuery,
    mut commands: Commands,
    mut control_transfers: MessageWriter<LocalControlTransferRequest>,
    player: Single<
        (
            Entity,
            &Transform,
            &UsfScaleLayer,
            &mut Visibility,
            &mut SpatialDemandSource,
            &mut LocomotionEnabled,
        ),
        (With<Player>, Without<SpacecraftManifestation>),
    >,
    player_controlled: Query<(), (With<Player>, With<LocalControlSubject>)>,
    mut ships: Query<
        (
            Entity,
            &Transform,
            &UsfScaleLayer,
            &FlightContactState,
            &mut SpatialDemandSource,
        ),
        (
            With<SpacecraftManifestation>,
            Without<LocalControlSubject>,
            Without<Player>,
        ),
    >,
) {
    if !input.gameplay_active() || !input.just_pressed(PlayerAction::Interact) {
        return;
    }

    let (
        player_entity,
        player_transform,
        player_layer,
        mut player_visibility,
        mut player_demand,
        mut player_enabled,
    ) = player.into_inner();

    if !player_controlled.contains(player_entity) {
        return;
    }

    let Some(player_semantic) = ownership.semantic_of(player_entity) else {
        return;
    };

    for (ship_entity, ship_transform, ship_layer, ship_contact, mut ship_demand) in &mut ships {
        if !ship_contact.is_landed() {
            continue;
        }

        // Player and ship may intentionally inhabit different interaction
        // Scales. Compare poses through the common canonical frame in SI.
        let Ok(player_position) = frame
            .origin()
            .translated_at_scale(player_layer.scale(), player_transform.translation)
        else {
            continue;
        };
        let Ok(ship_position) = frame
            .origin()
            .translated_at_scale(ship_layer.scale(), ship_transform.translation)
        else {
            continue;
        };
        let Ok(relative_metres) = player_position.relative_at_scale_bounded_f64(
            &ship_position,
            SpatialScale::ZERO,
            f64::from(ENTER_DISTANCE_METRES) * 2.0,
        ) else {
            continue;
        };
        if relative_metres.length() > f64::from(ENTER_DISTANCE_METRES) {
            continue;
        }

        let Some(ship_semantic) = ownership.semantic_of(ship_entity) else {
            continue;
        };
        *player_visibility = Visibility::Hidden;
        player_demand.set_enabled(false);
        player_enabled.0 = false;
        ship_demand.set_enabled(true);

        commands
            .entity(player_entity)
            .remove::<Collider>()
            .remove::<CharacterMotor>();
        commands
            .entity(player_semantic)
            .insert(UsfConstituentOf(ship_semantic));

        control_transfers.write(LocalControlTransferRequest::new(
            player_semantic,
            ship_entity,
        ));
        return;
    }
}

/// Prove a walkable player-hull pose before disembark changes semantic or
/// manifestation state. A missing/non-walkable hit leaves control on the ship.
fn resolve_disembark_pose(
    spatial_query: &SpatialQuery,
    physics_charts: &UsfPhysicsSlices,
    ship_entity: Entity,
    ship_transform: &Transform,
    ship_layer: &UsfScaleLayer,
    ship_frame: &CharacterLocomotionFrame,
    ship_hull: &PhysicalBoxHull,
    player_entity: Entity,
    player_hull: &PhysicalBoxHull,
    player_movement: &CharacterMovementConfig,
    player_exclusions: Option<&KinematicQueryExclusions>,
    aligned_player: Quat,
) -> Option<Vec3> {
    let exit_offset =
        ship_transform.rotation * Vec3::X * ship_layer.scale().metres_to_native_f32(4.0);
    let exit_column = ship_transform.translation + exit_offset;

    let ship_support_metres =
        ship_hull.projection_radius_metres(ship_transform.rotation, ship_frame.up());
    let player_support_metres =
        player_hull.projection_radius_metres(aligned_player, ship_frame.up());
    let probe_lift_metres = player_support_metres + LANDING_PROBE_METRES;
    let probe_distance_metres =
        ship_support_metres + player_support_metres + LANDING_PROBE_METRES * 2.0;

    let exit_probe_start =
        exit_column + ship_frame.up() * ship_layer.scale().metres_to_native_f32(probe_lift_metres);
    let exit_direction = Dir3::new(-ship_frame.up()).ok()?;
    let exit_filter = physics_charts.filter_for_scale(
        ship_layer.scale(),
        std::iter::once(player_entity)
            .chain(std::iter::once(ship_entity))
            .chain(player_exclusions.into_iter().flat_map(|items| items.iter())),
    );
    let exit_config = ShapeCastConfig {
        max_distance: ship_layer
            .scale()
            .metres_to_native_f32(probe_distance_metres),
        ignore_origin_penetration: true,
        ..default()
    };
    let player_collider = player_hull.collider(ship_layer.scale());
    let exit_hit = spatial_query.cast_shape(
        &player_collider,
        exit_probe_start,
        aligned_player,
        exit_direction,
        &exit_config,
        &exit_filter,
    )?;
    (exit_hit.normal1.dot(ship_frame.up()) >= player_movement.min_ground_dot)
        .then_some(exit_probe_start - ship_frame.up() * exit_hit.distance.max(0.0))
}

pub(super) fn enforce_embarked_player_hidden(
    ownership: UsfOwnershipQuery,
    constituents: Query<&UsfConstituentOf>,
    controlled_ship: Query<Entity, (With<SpacecraftManifestation>, With<LocalControlSubject>)>,
    mut player: Query<(Entity, &mut Visibility), (With<Player>, Without<SpacecraftManifestation>)>,
) {
    let Ok(ship_entity) = controlled_ship.single() else {
        return;
    };
    let Some(ship_semantic) = ownership.semantic_of(ship_entity) else {
        return;
    };
    let Ok((player_entity, mut visibility)) = player.single_mut() else {
        return;
    };
    let Some(player_semantic) = ownership.semantic_of(player_entity) else {
        return;
    };

    let embarked = constituents
        .get(player_semantic)
        .is_ok_and(|relationship| relationship.0 == ship_semantic);
    if embarked && *visibility != Visibility::Hidden {
        *visibility = Visibility::Hidden;
    }
}
