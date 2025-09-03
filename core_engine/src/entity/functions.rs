use bevy::ecs::entity::Entity;

use super::statics::ENTITY_RESERVATION_BUFFER;

pub fn get_reserved_entity() -> Entity {
    let mut queue = ENTITY_RESERVATION_BUFFER.lock().unwrap();
    queue.pop().expect("Entity reservation buffer exhausted")
}
