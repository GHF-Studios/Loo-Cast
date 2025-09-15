use bevy::prelude::*;

use crate::config::statics::config;
use crate::debug::observers::on_click_select;

use super::statics::entity_reservation_buffer;

pub(super) fn fill_entity_reservation_buffer(mut commands: Commands) {
    let mut queue = entity_reservation_buffer().lock().unwrap();
    while queue.len() < config().get::<usize>("entity/reservation_buffer_size") {
        let reserved_entity_name = Name::new("RESERVED_ENTITY");
        queue.push(commands.spawn(reserved_entity_name).observe(on_click_select).id())
    }
}
