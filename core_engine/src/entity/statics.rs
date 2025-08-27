use bevy::ecs::entity::Entity;
use std::sync::Mutex;
use once_cell::sync::Lazy;

pub(super) static ENTITY_RESERVATION_BUFFER: Lazy<Mutex<Vec<Entity>>> = Lazy::new(|| Mutex::new(Vec::new()));