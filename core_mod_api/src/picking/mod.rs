pub mod constants;
pub mod resources;
pub mod systems;

use bevy::picking::backend::prelude::*;
use bevy::prelude::*;
use resources::{SpritePickingMode, SpritePickingSettings};
use systems::{sprite_picking_backend, mouse_pick_events, spawn_mouse_pointers};

use crate::core::types::{Diegetic, Meta};

pub(crate) struct PickingPlugin;
impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SpritePickingSettings>()
            .add_systems(Startup, spawn_mouse_pointers)
            .add_systems(First, mouse_pick_events.in_set(PickSet::Input))
            .add_systems(
                PreUpdate,
                (
                    sprite_picking_backend::<Diegetic>,
                    sprite_picking_backend::<Meta>,
                    // log_hits::<Diegetic>,
                    // log_hits::<Meta>,
                )
                    .chain()
                    .in_set(PickSet::Backend),
            )
            .register_type::<SpritePickingMode>()
            .register_type::<SpritePickingSettings>();
    }
}

// fn log_hits<OC: crate::core::types::OntologicalContext>(mut hits: EventReader<PointerHits>) {
//     for hit_event in hits.read() {
//         if hit_event.pointer != OC::pointer_id() {
//             continue;
//         }
// 
//         warn!("🧩 {} PointerHits seen by BevyPicking: {:?}", OC::name(), hit_event);
//     }
// }
