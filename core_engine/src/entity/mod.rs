pub mod functions;
pub mod systems;

use bevy::prelude::*;
use systems::fill_entity_reservation_buffer;

pub(crate) struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, fill_entity_reservation_buffer);
    }
}
