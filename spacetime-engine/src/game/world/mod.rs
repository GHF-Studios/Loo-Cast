//! Loo Cast world bootstrap and environment composition.

mod lighting;
mod procedural;
mod selection;

use bevy::prelude::*;

pub(in crate::game) use procedural::UniverseLandmarkIndex;

#[derive(States, Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameWorld {
    #[default]
    Selection,
    Playground,
    Procedural,
}

pub(super) struct GameWorldPlugin;

impl Plugin for GameWorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameWorld>();

        lighting::configure(app);
        selection::configure(app);
        procedural::configure(app);
    }
}
