pub mod components;
pub(in crate) mod systems;

use bevy::prelude::*;

pub(in crate) struct TranslationPlugin;

impl Plugin for TranslationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, systems::update);
    }
}