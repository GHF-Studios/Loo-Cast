use bevy::ecs::entity::Entity;
use once_cell::sync::OnceCell;
use std::sync::Mutex;

use crate::statics::get_ref;

pub fn init_entity_reservation_buffer() -> Mutex<Vec<Entity>> {
    Mutex::new(Vec::new())
}

pub fn entity_reservation_buffer() -> &'static Mutex<Vec<Entity>> {
    get_ref("entity_reservation_buffer")
}
