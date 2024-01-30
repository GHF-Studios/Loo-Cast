use bevy::prelude::*;
use bevy::app::*;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup_test)
            .add_systems(Update, (update_test, shutdown_test));
    }
}

fn startup_test() {
    println!("Hello, world!");
}

fn update_test() {
    println!("Update!");
}

fn shutdown_test(mut app_exit_event_reader: EventReader<AppExit>) {
    if app_exit_event_reader.iter().next().is_some() {
        println!("Goodbye, world!");
    }
}