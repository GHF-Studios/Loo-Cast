//! Embarked player presentation follows semantic constituency.

use super::*;

/// Player-body visibility is derived from semantic constituency, not from which
/// runtime subject currently owns local control.
///
/// This makes the projection total: stale `Visibility::Hidden` cannot survive a
/// completed disembark/control transfer merely because a spacecraft still exists.
pub(in crate::game::spacecraft) fn sync_embarked_player_visibility(
    ownership: UsfOwnershipQuery,
    constituents: Query<&UsfConstituentOf>,
    spacecraft: Query<(), With<Spacecraft>>,
    mut player: Query<(Entity, &mut Visibility), (With<Player>, Without<SpacecraftManifestation>)>,
) {
    let Ok((player_entity, mut visibility)) = player.single_mut() else {
        return;
    };
    let Some(player_semantic) = ownership.semantic_of(player_entity) else {
        return;
    };

    let embarked = constituents
        .get(player_semantic)
        .is_ok_and(|relationship| spacecraft.contains(relationship.0));
    let desired = if embarked {
        Visibility::Hidden
    } else {
        Visibility::Inherited
    };

    if *visibility != desired {
        *visibility = desired;
    }
}
