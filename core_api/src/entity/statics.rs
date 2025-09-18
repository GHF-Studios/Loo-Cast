use bevy::ecs::entity::Entity;
use core_api_macros::export_static;
use std::sync::Mutex;

export_static!(self, crate::entity::statics::ENTITY_RESERVATION_BUFFER: Mutex<Vec<Entity>> = Mutex::new(Vec::new()));
