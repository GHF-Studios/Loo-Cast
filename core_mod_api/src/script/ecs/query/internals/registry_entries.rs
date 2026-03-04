use crate::bevy::ecs::query::With;
use crate::bevy::prelude::{Entity as BevyEntity, World as BevyWorld};
use crate::player::components::Player;
use crate::script::ecs::query::bindings::types::{Query, QUERY_FILTER_NONE_ID};
use crate::script::ecs::query::internals::types::QueryDispatchEntry;

pub const QUERY_DATA_ENTITY_ID: &str = "ecs::entities::Entity";
pub const QUERY_FILTER_WITH_PLAYER_ID: &str = "player::components::Player";

fn dispatch_entities(world: &mut BevyWorld) -> Query {
    let mut query = world.query::<BevyEntity>();
    let values = query.iter(&*world).map(rhai::Dynamic::from).collect();
    Query { values }
}

fn dispatch_entities_with_player(world: &mut BevyWorld) -> Query {
    let mut query = world.query_filtered::<BevyEntity, With<Player>>();
    let values = query.iter(&*world).map(rhai::Dynamic::from).collect();
    Query { values }
}

inventory::submit! {
    QueryDispatchEntry {
        data_id: QUERY_DATA_ENTITY_ID,
        filter_id: QUERY_FILTER_NONE_ID,
        dispatch: dispatch_entities,
    }
}

inventory::submit! {
    QueryDispatchEntry {
        data_id: QUERY_DATA_ENTITY_ID,
        filter_id: QUERY_FILTER_WITH_PLAYER_ID,
        dispatch: dispatch_entities_with_player,
    }
}
