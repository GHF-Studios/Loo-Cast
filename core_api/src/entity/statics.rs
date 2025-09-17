use core_api_macros::export_static;
use bevy::ecs::entity::Entity;
use std::sync::Mutex;

export_static!(self, crate::entity::statics::ENTITY_RESERVATION_BUFFER: Mutex<Vec<Entity>> = Mutex::new(Vec::new()));