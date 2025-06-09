extern crate core_engine;

use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::log::{Level, LogPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_rapier2d::prelude::*;
use core_engine::*;
use iyes_perf_ui::prelude::*;
use bevy_egui::EguiPlugin;

const ENABLE_BACKTRACE: bool = true;
const REROUTE_LOGS_TO_FILE: bool = false;
const LOG_LEVEL: Level = Level::INFO;
const LOG_FILTER: &str = "info,core_engine=debug";

fn main() {
    std::env::set_var("RUST_BACKTRACE", if ENABLE_BACKTRACE { "1" } else { "0" });

    let mut bevy_plugins = DefaultPlugins.build();

    if REROUTE_LOGS_TO_FILE {
        // Redirect logs to a file
        let file_appender = tracing_appender::rolling::daily("logs", "bevy_log.txt");
        let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

        // Configure the logger
        tracing_subscriber::fmt().with_env_filter(LOG_FILTER).with_writer(non_blocking).init();
    } else {
        bevy_plugins = bevy_plugins.set(LogPlugin {
            filter: LOG_FILTER.into(),
            level: LOG_LEVEL,
            ..Default::default()
        });
    };

    bevy_plugins = bevy_plugins
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
        .add(EguiPlugin);

    App::new()
        .add_plugins(bevy_plugins)
        .add_plugins(PerfUiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(SpacetimeEngineCorePlugins)
        .add_plugins(SpacetimeEngineWorkflowPlugins)
        .run();
}
