use crate::bevy::ecs::query::With;
use crate::bevy::prelude::{Entity as BevyEntity, World as BevyWorld};
use crate::player::components::Player;
use crate::script::ecs::query::bindings::types::Query;
use crate::script::ecs::query::internals::types::{QueryDispatchAccess, QueryDispatchEntry, QueryDispatchTerm};

pub const QUERY_SIG__ENTITY: &str = "QUERY_SIG__ENTITY";
pub const QUERY_SIG__ENTITY__WITH_PLAYER: &str = "QUERY_SIG__ENTITY__WITH_PLAYER";

pub const TYPE_PATH__ENTITY: &str = "ecs::entities::Entity";
pub const TYPE_PATH__PLAYER: &str = "player::components::Player";

const QUERY_DATA__ENTITY: &[QueryDispatchTerm] = &[QueryDispatchTerm {
    type_id: TYPE_PATH__ENTITY,
    access: QueryDispatchAccess::Value,
}];

fn dispatch_query_sig_entity(world: &mut BevyWorld) -> Query {
    let mut query = world.query::<BevyEntity>();
    let values = query.iter(&*world).map(rhai::Dynamic::from).collect();
    Query { values }
}

fn dispatch_query_sig_entity_with_player(world: &mut BevyWorld) -> Query {
    let mut query = world.query_filtered::<BevyEntity, With<Player>>();
    let values = query.iter(&*world).map(rhai::Dynamic::from).collect();
    Query { values }
}

inventory::submit! {
    QueryDispatchEntry {
        signature_id: QUERY_SIG__ENTITY,
        data_terms: QUERY_DATA__ENTITY,
        filter_with: &[],
        filter_without: &[],
        dispatch: dispatch_query_sig_entity,
    }
}

inventory::submit! {
    QueryDispatchEntry {
        signature_id: QUERY_SIG__ENTITY__WITH_PLAYER,
        data_terms: QUERY_DATA__ENTITY,
        filter_with: &[TYPE_PATH__PLAYER],
        filter_without: &[],
        dispatch: dispatch_query_sig_entity_with_player,
    }
}
