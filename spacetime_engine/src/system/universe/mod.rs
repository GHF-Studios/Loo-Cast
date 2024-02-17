use bevy::prelude::*;

pub struct UniversePlugin;

impl Plugin for UniversePlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup Systems
            .add_systems(PreStartup, UniverseManager::pre_startup)
            .add_systems(Startup, UniverseManager::startup);
    }
}

#[derive(Resource, Default)]
pub struct UniverseManager {
}

impl UniverseManager {
    fn pre_startup(mut commands: Commands) {
        info!("Pre-Starting universe Manager...");

        commands.insert_resource(UniverseManager::default());

        info!("Pre-Started universe Manager.");
    }

    fn startup(commands: Commands) {
        info!("Starting universe Manager...");

        info!("Started universe Manager.");
    }
}