//! Disembark transaction after a walkable pose is proven.

use super::*;

pub(in crate::game::spacecraft) fn handle_ship_exit(
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
            &mut FlightActuation,
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
            mut player_actuation,
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
        player_actuation.set_thrusters_enabled(false);
        player_actuation.set_rcs_enabled(false);

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
