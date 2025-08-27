use bevy::prelude::*;

use crate::config::statics::CONFIG;
use super::statics::ENTITY_RESERVATION_BUFFER;

pub(super) fn fill_entity_reservation_buffer(mut commands: Commands) {
    let mut queue = ENTITY_RESERVATION_BUFFER.lock().unwrap();
    while queue.len() < CONFIG.get::<usize>("entity/reservation_buffer_size") {
        let reserved_entity_name = Name::new("RESERVED_ENTITY");
        queue.push(commands.spawn(reserved_entity_name).id())
    }
}
