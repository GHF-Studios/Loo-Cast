use bevy::ecs::entity::Entity;
use core_mod_macros::export_static;
use std::sync::Mutex;

export_static!(self, crate::core_mod_api::entity::statics::ENTITY_RESERVATION_BUFFER: Mutex<Vec<Entity>> = Mutex::new(Vec::new()));
