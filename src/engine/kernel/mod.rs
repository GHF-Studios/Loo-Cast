// Modules
pub mod background;
pub mod camera;
pub mod game;
pub mod iteration_test;
pub mod math;
pub mod player;
pub mod save_game;
pub mod ui;
pub mod universe;

// Internal imports
use background::BackgroundPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use iteration_test::IterationTestPlugin;
use player::PlayerPlugin;
use save_game::SaveGamePlugin;
use ui::UIPlugin;
use universe::UniversePlugin;

// External imports
use bevy::{app::PluginGroupBuilder, prelude::*};
use bevy_rapier2d::prelude::*;

// Static variables

// Constant variables

// Types

// Enums
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    SaveGamesMenu,
    CreateSaveGameMenu,
    Game,
}

// Structs
pub struct LooCastBasePlugins;

pub struct RapierPlugins;

// Implementations
impl PluginGroup for LooCastBasePlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group
            .add(BackgroundPlugin)
            .add(CameraPlugin)
            .add(GamePlugin)
            .add(IterationTestPlugin)
            .add(PlayerPlugin)
            .add(SaveGamePlugin)
            .add(UIPlugin)
            .add(UniversePlugin);

        group
    }
}

impl PluginGroup for RapierPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group
            .add(RapierDebugRenderPlugin::default())
            .add(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

        group
    }
}

// Module Functions
pub(in crate) fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // Rapier Plugins
        .add_plugins(RapierPlugins)
        // States
        .add_state::<AppState>()
        // Loo Cast Base Plugins
        .add_plugins(LooCastBasePlugins)
        // Run
        .run();
}