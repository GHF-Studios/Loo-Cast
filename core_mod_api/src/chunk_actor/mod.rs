pub mod components;

use bevy::prelude::*;

use components::ChunkActor;

use crate::core_mod_macros::configure_app_with_all_scales;
use crate::usf::scale::*;

pub(crate) struct ChunkActorPlugin;
impl Plugin for ChunkActorPlugin {
    fn build(&self, app: &mut App) {
        configure_app_with_all_scales!(
            { .register_type::<ChunkActor<__S__>>() },
        );
    }
}
