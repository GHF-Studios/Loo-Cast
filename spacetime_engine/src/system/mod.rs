// Modules
pub mod camera;
pub mod math;
pub mod ui;
pub mod new_ui;

// Local imports

// Internal imports
use camera::CameraPlugin;
use ui::UIPlugin;

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
    GamesMenu,
    CreateGameMenu,
    Game,
}

// Structs
#[derive(Resource, Default)]
pub struct SystemManager {
}

pub struct SystemPlugins;

pub struct RapierPlugins;

// Implementations
impl SystemManager {
}

impl PluginGroup for SystemPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();

        group = group
            // Internal Modules
            .add(CameraPlugin)
            .add(UIPlugin)
            // External Modules
            .add(RapierDebugRenderPlugin::default())
            .add(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));

        group
    }
}

// Module Functions
