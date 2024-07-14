use bevy::prelude::*;
use super::constants::*;
use super::components::*;
use super::structs::PlayerID;

// TODO: Revamp this completely: Request a new chunk loader entity (which will be available once it's starting chunk has been loaded) 
//       and attach a Player component and any other relevant components to it.
//       This will allow the player to be spawned in a chunk that is already loaded.
// Do this as well for regular chunk actor entities.
// But think about this some more and in regards to chunk actors as opposed to chunk loaders and shit.
pub(super) fn new_player_entity(
    commands: &mut Commands,
    player_id: PlayerID,
    player_world_position: Vec2
) -> Entity {
    let player_entity = commands
        .spawn(Transform::from_translation(Vec3::new(player_world_position.x, player_world_position.y, PLAYER_Z_INDEX)))
        .insert(Player { id: player_id, create_chunk_actor_request_ids: Vec::new() })
        .id();

    player_entity
}

pub(super) fn promote__player_entity(
    commands: &mut Commands, 
    player_id: PlayerID, 
    target_entity_reference: Entity,
    ineligible_entity_query_0: &mut Query<Entity, Without<Transform>>,
    ineligible_entity_query_1: &mut Query<Entity, With<Player>>,
    eligible_entity_query: &mut Query<Entity, (With<Transform>, Without<Player>)>,
) -> Result<Entity, Entity> {
    if let Ok(_) = ineligible_entity_query_0.get(target_entity_reference) {
        error!("Entity '{:?}' does not have a Transform component!", target_entity_reference);

        return Err(target_entity_reference);
    };

    if let Ok(_) = ineligible_entity_query_1.get(target_entity_reference) {
        error!("Entity '{:?}' already has a Player component!", target_entity_reference);

        return Err(target_entity_reference);
    };

    if let Ok(eligible_entity) = eligible_entity_query.get_mut(target_entity_reference) {
        return Ok(commands.entity(eligible_entity).insert(Player { id: player_id, create_chunk_actor_request_ids: Vec::new() }).id());
    } else {
        error!("Entity does not exist or does not have a Transform component.");

        return Err(target_entity_reference);
    };
}