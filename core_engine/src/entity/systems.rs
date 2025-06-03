use bevy::prelude::*;

use crate::{config::statics::CONFIG, entity::functions::ENTITY_RESERVATION_BUFFER};

pub(in super) fn fill_entity_reservation_buffer(mut commands: Commands) {
    let mut queue = ENTITY_RESERVATION_BUFFER.lock().unwrap();
    while queue.len() < CONFIG.get::<usize>("entity/reservation_buffer_size") {
        queue.push(commands.spawn(()).id())
    }
}