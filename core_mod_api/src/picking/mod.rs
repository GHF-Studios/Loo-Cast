pub mod constants;
pub mod resources;
pub mod systems;

use bevy::picking::backend::prelude::*;
use bevy::prelude::*;
use resources::{SpritePickingMode, SpritePickingSettings};
use systems::{diegetic_sprite_picking_backend, mouse_pick_events, spawn_mouse_pointer};

pub(crate) struct PickingPlugin;
impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpritePickingSettings>()
            .add_systems(Startup, spawn_mouse_pointer)
            .add_systems(First, mouse_pick_events.in_set(PickSet::Input))
            .add_systems(
                PreUpdate,
                (
                    diegetic_sprite_picking_backend,
                    // log_hits
                )
                    .chain()
                    .in_set(PickSet::Backend),
            )
            .register_type::<SpritePickingMode>()
            .register_type::<SpritePickingSettings>();
    }
}

// fn log_hits(mut hits: EventReader<PointerHits>) {
// if !hits.is_empty()  {
// warn!("🧩 PointerHits seen by BevyPicking: {:?}", hits.read().collect::<Vec<_>>());
// }
// }
