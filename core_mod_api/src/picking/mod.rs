pub mod systems;

use bevy::prelude::*;
use bevy::picking::backend::prelude::*;
use systems::sprite_picking;

pub(crate) struct PickingPlugin;
impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(PreUpdate, sprite_picking.in_set(PickSet::Backend));
    }
}
