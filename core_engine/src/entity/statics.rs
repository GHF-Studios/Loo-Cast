use bevy::ecs::entity::Entity;
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub(super) static ENTITY_RESERVATION_BUFFER: Lazy<Mutex<Vec<Entity>>> = Lazy::new(|| Mutex::new(Vec::new()));
