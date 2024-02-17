// Modules
pub mod camera;
pub mod game;
pub mod math;
pub mod new_ui;
pub mod ui;
pub mod universe;

// Local imports

// Internal imports
use camera::CameraPlugin;
use game::GamePlugin;
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
    Main,
    Games,
    CreateGame,
    Game,
}

// Structs
#[derive(Resource, Default)]
pub struct SystemManager {}

pub struct SystemPlugins;

pub struct RapierPlugins;

// Implementations
impl SystemManager {}

impl PluginGroup for SystemPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group
            // Internal Modules
            .add(CameraPlugin)
            .add(GamePlugin)
            .add(UIPlugin)
            .add(UniversePlugin)
            // External Modules
            .add(RapierDebugRenderPlugin::default())
            .add(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

        group
    }
}

// Module Functions
