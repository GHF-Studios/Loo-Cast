use bevy::ecs::entity::Entity;
use std::sync::Mutex;

pub(super) static ENTITY_RESERVATION_BUFFER: once_cell::sync::Lazy<Mutex<Vec<Entity>>> = once_cell::sync::Lazy::new(|| Mutex::new(Vec::new()));

pub fn reserve_entity_id() -> Entity {
    let mut queue = ENTITY_RESERVATION_BUFFER.lock().unwrap();
    queue.pop().expect("Entity reservation buffer exhausted")
}
