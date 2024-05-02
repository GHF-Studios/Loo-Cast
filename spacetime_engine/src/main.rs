extern crate spacetime_engine;

use bevy::log::*;
use bevy::prelude::*;
use bevy::app::AppExit;

// Module Functions
fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    asset_folder: "assets".to_string(),
                    ..default()
                })
                .set(LogPlugin {
                    level: bevy::log::Level::INFO,
                    ..default()
                }),
        )
        .run();
}