use bevy::ecs::entity::Entity;

use super::statics::entity_reservation_buffer;

pub fn get_reserved_entity() -> Entity {
    let mut queue = entity_reservation_buffer().lock().unwrap();
    queue.pop().expect("Entity reservation buffer exhausted")
}
