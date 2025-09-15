use crate::statics::get_ref;
use bevy::ecs::entity::Entity;
use std::sync::Mutex;

/// Registry key: "entity_reservation_buffer"
pub fn entity_reservation_buffer() -> &'static Mutex<Vec<Entity>> {
    get_ref("entity_reservation_buffer")
}
