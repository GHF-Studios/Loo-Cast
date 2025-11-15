pub mod constants;
pub mod resources;
pub mod systems;

use bevy::prelude::*;
use bevy::picking::backend::prelude::*;
use resources::{SpritePickingMode, SpritePickingSettings};
use systems::{spawn_mouse_pointer, mouse_pick_events, sprite_picking_backend};

pub(crate) struct PickingPlugin;
impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SpritePickingSettings>()
            .add_systems(Startup, spawn_mouse_pointer)
            .add_systems(First, mouse_pick_events.in_set(PickSet::Input))
            .add_systems(PreUpdate, sprite_picking_backend.in_set(PickSet::Backend))
            .register_type::<SpritePickingMode>()
            .register_type::<SpritePickingSettings>();
    }
}
