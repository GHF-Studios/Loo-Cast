use bevy::{
    prelude::*,
    window::{CursorGrabMode, CursorOptions, WindowPlugin},
};

use spacetime_engine::{
    ecs::component_conflict::ComponentConflictPlugin,
    game::TestGamePlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_cursor_options: Some(CursorOptions {
                visible: false,
                grab_mode: CursorGrabMode::Locked,
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            ComponentConflictPlugin,
            TestGamePlugin,
        ))
        .run();
}