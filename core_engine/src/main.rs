use core_lib::*;
use core_lib::config::statics::config;
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
use libloading::{Library, Symbol};
use std::path::PathBuf;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::{EnvFilter, Layer};
use core_runtime::{CoreRuntimeApi, build_runtime_api};

fn main() {
    setup_tracing();
    let _span = info_span!("main").entered();
    configure_low_level_stuff();
    let bevy_plugins = configure_third_party_plugins();
    let mut app = configure_app(bevy_plugins);
    run_app(app);
}

fn setup_tracing() {
    if !config().get::<bool>("log/tracing/enabled") {
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
        .add(PerfUiPlugin)
        // Physics Plugins
        .add(RapierPhysicsPlugin::<NoUserData>::default())

    // Picking Plugins
    //.add(PickingPlugin::default())
    //.add(MeshPickingPlugin)
    //.add(UiPickingPlugin)
    //.add(SpritePickingPlugin)
}

fn configure_app(third_party_plugins: PluginGroupBuilder) -> App {
    info!("Building App...");

    let mut app = App::new();
    app
        .add_plugins(third_party_plugins)
        .add_plugins(CoreLibPluginGroup);

    load_core_mod(&mut app);
    
    app
}

fn load_core_mod(app: &mut App) {
    let exe_dir = std::env::current_exe()
        .expect("failed to get exe path")
        .parent()
        .unwrap()
        .to_path_buf();

    let lib_path = exe_dir.join(format!("core_mod{}", std::env::consts::DLL_SUFFIX));
    // println!("🔍 Trying to load core_mod from: {}", lib_path.display());

    unsafe {
        let runtime_api = build_runtime_api();
        let lib = Library::new(&lib_path)?;
        let init_mod: Symbol<unsafe extern "C" fn(&mut App, *mut CoreRuntimeApi)> = lib.get(b"init_mod")?;
        init_mod(app, &runtime_api as *const _ as *mut _);
    }
}

fn run_app(mut app: App) {
    info!("Running App...");

    app.run();
}
