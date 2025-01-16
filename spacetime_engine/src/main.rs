extern crate spacetime_engine;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, EntityCountDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use iyes_perf_ui::prelude::*;
use bevy::log::{Level, LogPlugin};
use bevy_rapier2d::prelude::*;
use spacetime_engine::*;

// Maybe add object pooling for chunks?
// Make it so chunks can exist in an intermediate state for several frames, so that we can implement deferred/non-blocking/asynchronous/parallelized/hardware-accelerated chunk spawning
// Add actual noise generation to chunks to procedurally generate a set of random positions to spawn test objects at to test the deferred loading of chunks
// Chunks should not be forced to load in a single frame. Let's instead focus on keeping the experience smooth

const ENABLE_BACKTRACE: bool = false;
const REROUTE_LOGS_TO_FILE: bool = false;
const LOG_LEVEL: Level = Level::INFO;
const LOG_FILTER: &str = "info,spacetime_engine=debug";

fn main() {
    std::env::set_var(
        "RUST_BACKTRACE", 
        if ENABLE_BACKTRACE { "1" } else { "0" }
    );

    let default_bevy_plugins = if REROUTE_LOGS_TO_FILE {
        // Redirect logs to a file
        let file_appender = tracing_appender::rolling::daily("logs", "bevy_log.txt");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
        // Configure the logger
        tracing_subscriber::fmt()
            .with_env_filter(LOG_FILTER)
            .with_writer(non_blocking)
            .init();

        DefaultPlugins.build()
    } else {
        DefaultPlugins
            .set(LogPlugin {
                filter: LOG_FILTER.into(),
                level: LOG_LEVEL,
                ..Default::default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    present_mode: PresentMode::AutoNoVsync,
                    title: "Loo Cast".to_string(),
                    ..default()
                }),
                ..default()
            })
            .add(FrameTimeDiagnosticsPlugin)
            .add(EntityCountDiagnosticsPlugin)
            .add(SystemInformationDiagnosticsPlugin)
    };

    App::new()
        .add_plugins(default_bevy_plugins)
        .add_plugins(PerfUiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(SpacetimeEnginePlugins)
        .run();
}
