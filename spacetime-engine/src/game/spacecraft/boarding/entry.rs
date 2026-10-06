//! Boarding selection and semantic control-transfer request.

use super::*;

const ENTER_DISTANCE_METRES: f32 = 12.0;

/// Boarding reach is physical SI distance even when the player and ship use
/// different runtime Scale Slices. A failed chart projection grants no reach.
fn within_boarding_reach(
    frame: &UsfRuntimeChartState,
    player_transform: &Transform,
    player_layer: &UsfScaleLayer,
    ship_transform: &Transform,
    ship_layer: &UsfScaleLayer,
) -> bool {
    let Ok(player_position) = frame
        .origin()
        .translated_at_scale(player_layer.scale(), player_transform.translation)
    else {
        return false;
    };
    let Ok(ship_position) = frame
        .origin()
        .translated_at_scale(ship_layer.scale(), ship_transform.translation)
    else {
        return false;
    };
    player_position
        .relative_at_scale_bounded_f64(
            &ship_position,
            SpatialScale::ZERO,
            f64::from(ENTER_DISTANCE_METRES) * 2.0,
        )
        .is_ok_and(|relative_metres| relative_metres.length() <= f64::from(ENTER_DISTANCE_METRES))
}

/// Boarding chooses a nearby landed ship and queues the semantic control
/// transfer; the controlled player's manifestation is hidden until disembark.
pub(in crate::game::spacecraft) fn handle_ship_entry(
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

        if !within_boarding_reach(
            &frame,
            player_transform,
            player_layer,
            ship_transform,
            ship_layer,
        ) {
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
