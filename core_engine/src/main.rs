extern crate core_engine;

use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::log::{LogPlugin, info, error, info_span};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_egui::EguiPlugin;
use bevy_rapier2d::prelude::*;
use core_engine::log::statics::LOG_TREE_HANDLE;
use core_engine::log::types::LogTreeTracingLayer;
use core_engine::*;
use iyes_perf_ui::prelude::*;
use tracing_subscriber::{EnvFilter, Layer};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::fmt::{self, format::FmtSpan, time::FormatTime};
use std::time::{SystemTime, UNIX_EPOCH};

const ENABLE_BACKTRACE: bool = true;
const CLI_LOG_FILTER: &str = "warn,core_engine=warn,core_engine_macros=warn";

struct ShortTime;
impl FormatTime for ShortTime {
    fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();

        let millis = now.as_millis() % 1000;
        let secs = now.as_secs() % 60;
        let mins = (now.as_secs() / 60) % 60;

        write!(w, "T+ {:02}m:{:02}s.{:03}ms", mins, secs, millis)
    }
}

fn main() {
    let log_tree_tracing_layer = LogTreeTracingLayer { handle: LOG_TREE_HANDLE.clone() };

    let subscriber = tracing_subscriber::registry()
        .with(log_tree_tracing_layer)
        .with(
            fmt::layer()
                .with_timer(ShortTime)
                .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
                .with_ansi(true)
                .with_filter(EnvFilter::new(CLI_LOG_FILTER)),
        );

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");

    let span = info_span!("main", on = true);
    let _guard = span.enter();

    info!("Configuring low-level stuff");

    std::panic::set_hook(Box::new(|panic_info| {
        error!("{}", panic_info);
    }));
    std::env::set_var("RUST_BACKTRACE", if ENABLE_BACKTRACE { "1" } else { "0" });

    info!("Configuring Bevy's DefaultPlugins");

    let bevy_plugins = DefaultPlugins.build()
        .disable::<LogPlugin>()
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

    info!("Building App...");

    let mut app = App::new();
    app
        .add_plugins(bevy_plugins)
        .add_plugins(PerfUiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(SpacetimeEngineCorePlugins)
        .add_plugins(SpacetimeEngineWorkflowPlugins);

    info!("Running App...");

    app.run();
}
