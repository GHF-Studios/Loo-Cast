//! Embarked player presentation follows semantic constituency.

use super::*;

pub(in crate::game::spacecraft) fn enforce_embarked_player_hidden(
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
