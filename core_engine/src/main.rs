use core_lib::*;
use core_lib::config::statics::CONFIG;
use core_lib::core::constants::{CLI_LOG_FILTER, ENABLE_BACKTRACE};
use core_lib::core::types::ShortTime;
use core_lib::logging::tracing::types::LogTreeTracingLayer;

use bevy::app::PluginGroupBuilder;
use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin};
use bevy::log::{error, info, info_span, LogPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_egui::EguiPlugin;
use bevy_rapier2d::prelude::*;
use iyes_perf_ui::prelude::*;
use libloading::Library;
use std::path::PathBuf;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{EnvFilter, Layer};

fn main() {
    setup_tracing();
    let _span = info_span!("main").entered();
    configure_low_level_stuff();
    let bevy_plugins = configure_third_party_plugins();
    let mut app = configure_app(bevy_plugins);
    load_core_mod(&mut app);
    run_app(app);
}

fn setup_tracing() {
    if !CONFIG.get::<bool>("log/tracing/enabled") {
        return;
    }

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_timer(ShortTime)
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_ansi(true)
        .with_filter(EnvFilter::new(CLI_LOG_FILTER));

    let subscriber = tracing_subscriber::registry()
        .with(LogTreeTracingLayer)
        .with(fmt_layer)
        .with(console_subscriber::spawn());

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");
}

fn configure_low_level_stuff() {
    info!("Configuring low-level stuff");

    std::panic::set_hook(Box::new(|panic_info| {
        error!("{}", panic_info);
    }));
    std::env::set_var("RUST_BACKTRACE", if ENABLE_BACKTRACE { "1" } else { "0" });
}

fn configure_third_party_plugins() -> PluginGroupBuilder {
    info!("Configuring bevy's DefaultPlugins");

    DefaultPlugins
        .build()
        // Basic Bevy Plugins
        .disable::<LogPlugin>()
        .set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoVsync,
                title: "Loo Cast".to_string(),
                ..default()
            }),
            ..default()
        })
        // Diagnostics Plugins
        .add(FrameTimeDiagnosticsPlugin::default())
        .add(EntityCountDiagnosticsPlugin)
        .add(SystemInformationDiagnosticsPlugin)
        // Ui Plugins
        .add(EguiPlugin::default())

    // Picking Plugins
    //.add(PickingPlugin::default())
    //.add(MeshPickingPlugin)
    //.add(UiPickingPlugin)
    //.add(SpritePickingPlugin)
}

fn configure_app(bevy_plugins: PluginGroupBuilder) -> App {
    info!("Building App...");

    let mut app = App::new();
    app.add_plugins(bevy_plugins)
        .add_plugins(PerfUiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default());

    unsafe {
        let profile = std::env::var("PROFILE").unwrap(); // "debug" or "release"
        let lib_path = format!("target/{}/core_mod{}", profile, std::env::consts::DLL_SUFFIX);
        let lib = libloading::Library::new(lib_path).unwrap();
        let func: libloading::Symbol<unsafe extern "C" fn(&mut App)> =
            lib.get(b"init_mod").unwrap();
        func(&mut app);
    }

    app
}

fn load_core_mod(app: &mut App) {
    let exe_dir = std::env::current_exe()
        .expect("failed to get exe path")
        .parent()
        .expect("exe has no parent")
        .to_path_buf();

    let mut lib_path = exe_dir;
    lib_path.push(format!("core_mod{}", std::env::consts::DLL_SUFFIX));

    unsafe {
        let lib = Library::new(lib_path).unwrap();
        let func: libloading::Symbol<unsafe extern "C" fn(&mut App)> =
            lib.get(b"init_mod").unwrap();
        func(app);
    }
}

fn run_app(mut app: App) {
    info!("Running App...");

    app.run();
}
