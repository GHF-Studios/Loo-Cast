use crate::bevy::ecs::query::With;
use crate::bevy::prelude::{Entity as BevyEntity, World as BevyWorld};
use crate::player::components::Player;
use crate::rhai_binding::runtime::ecs::dispatch_policy::submit_query_dispatch_entry;
use crate::rhai_binding::runtime::ecs::system::query::bindings::types::Query;
use crate::rhai_binding::runtime::ecs::system::query::internals::types::{QueryDispatchAccess, QueryDispatchTerm};

pub const QUERY_SIG__ENTITY: &str = "QUERY_SIG__ENTITY";
pub const QUERY_SIG__ENTITY__WITH_PLAYER: &str = "QUERY_SIG__ENTITY__WITH_PLAYER";

pub const TYPE_PATH__ENTITY: &str = "bevy::ecs::entity::Entity";
pub const TYPE_PATH__PLAYER: &str = "core_mod_api::player::components::Player";

const QUERY_DATA__ENTITY: &[QueryDispatchTerm] = &[QueryDispatchTerm {
    type_id: TYPE_PATH__ENTITY,
    access: QueryDispatchAccess::Value,
}];

fn dispatch_query_sig_entity(world: &mut BevyWorld) -> Query {
    let mut query = world.query::<BevyEntity>();
    let values = query.iter(&*world).map(rhai::Dynamic::from).collect();
    Query::from_values(values)
}

fn dispatch_query_sig_entity_with_player(world: &mut BevyWorld) -> Query {
    let mut query = world.query_filtered::<BevyEntity, With<Player>>();
    let values = query.iter(&*world).map(rhai::Dynamic::from).collect();
    Query::from_values(values)
}

submit_query_dispatch_entry!(
    signature_id = QUERY_SIG__ENTITY,
    data_terms = QUERY_DATA__ENTITY,
    filter_with = &[],
    filter_without = &[],
    dispatch = dispatch_query_sig_entity,
);

submit_query_dispatch_entry!(
    signature_id = QUERY_SIG__ENTITY__WITH_PLAYER,
    data_terms = QUERY_DATA__ENTITY,
    filter_with = &[TYPE_PATH__PLAYER],
    filter_without = &[],
    dispatch = dispatch_query_sig_entity_with_player,
);
