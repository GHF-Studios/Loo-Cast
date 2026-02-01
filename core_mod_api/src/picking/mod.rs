pub mod constants;
pub mod resources;
pub mod systems;

use bevy::picking::backend::prelude::*;
use bevy::prelude::*;
use resources::{SpritePickingMode, SpritePickingSettings};
use systems::{mouse_pick_messages, spawn_mouse_pointers, sprite_picking_backend};

pub(crate) struct PickingPlugin;
impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpritePickingSettings>()
            .add_systems(Startup, spawn_mouse_pointers)
            .add_systems(First, mouse_pick_messages.in_set(PickSet::Input))
            .add_systems(
                PreUpdate,
                (
                    sprite_picking_backend,
                    // log_hits::<crate::core::types::Diegetic>,
                    // log_hits::<crate::core::types::Meta>,
                )
                    .chain()
                    .in_set(PickSet::Backend),
            )
            .register_type::<SpritePickingMode>()
            .register_type::<SpritePickingSettings>();
    }
}

// fn log_hits<OC: crate::core::types::OntologicalContext>(mut hits: MessageReader<PointerHits>) {
//     for hit_message in hits.read() {
//         if hit_message.pointer != OC::pointer_id() {
//             continue;
//         }
//
//         warn!("🧩 {} PointerHits seen by BevyPicking: {:?}", OC::name(), hit_message);
//     }
// }
