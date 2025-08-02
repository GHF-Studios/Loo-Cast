extern crate core_engine;

use bevy::app::PluginGroupBuilder;
use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::log::{error, info, info_span, LogPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_egui::EguiPlugin;
use bevy_rapier2d::prelude::*;
use core_engine::constants::{CLI_LOG_FILTER, ENABLE_BACKTRACE};
use core_engine::log::tracing::types::LogTreeTracingLayer;
use core_engine::types::ShortTime;
use core_engine::*;
use iyes_perf_ui::prelude::*;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{EnvFilter, Layer};

fn main() {
    setup_tracing();
    let _span = info_span!("main").entered();
    configure_low_level_stuff();
    let bevy_plugins = configure_bevy_default_plugins();
    let app = configure_app(bevy_plugins);
    run_app(app);
}

fn setup_tracing() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_timer(ShortTime)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_ansi(true)
        .with_filter(EnvFilter::new(CLI_LOG_FILTER));

    let subscriber = tracing_subscriber::registry().with(LogTreeTracingLayer).with(fmt_layer);

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");
}

fn configure_low_level_stuff() {
    info!("Configuring low-level stuff");

    std::panic::set_hook(Box::new(|panic_info| {
        error!("{}", panic_info);
    }));
    std::env::set_var("RUST_BACKTRACE", if ENABLE_BACKTRACE { "1" } else { "0" });
}

fn configure_bevy_default_plugins() -> PluginGroupBuilder {
    info!("Configuring bevy's DefaultPlugins");

    DefaultPlugins
        .build()
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
        .add(EguiPlugin)
}

fn configure_app(bevy_plugins: PluginGroupBuilder) -> App {
    info!("Building App...");

    let mut app = App::new();
    app.add_plugins(bevy_plugins)
        .add_plugins(PerfUiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(SpacetimeEngineCorePlugins)
        .add_plugins(SpacetimeEngineWorkflowPlugins);

    app
}

fn run_app(mut app: App) {
    info!("Running App...");

    app.run();
}
