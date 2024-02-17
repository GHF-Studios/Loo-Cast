use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app
            // Startup Systems
            .add_systems(PreStartup, CommandsManager::pre_startup)
            .add_systems(Startup, CommandsManager::startup)
            .add_systems(PostStartup, CommandsManager::post_startup)
            // Update Systems
            .add_systems(Update, CommandsManager::process_ecs_world_command_requests)
            .add_systems(Update, CommandsManager::process_ecs_entity_command_requests);
    }
}

#[derive(Resource, Default)]
pub struct CommandsManager {
    pub ecs_world_commands: Vec<Box<dyn 'static + Send + Sync + ECSWorldCommand>>,
    pub ecs_entity_commands: Vec<(Box<dyn 'static + Send + Sync + ECSEntityCommand>, Entity)>,

}

impl CommandsManager {
    fn pre_startup(mut commands: Commands) {
        info!("Pre-Starting commands Manager...");

        commands.insert_resource(CommandsManager::default());

        info!("Pre-Started commands Manager.");
    }

    fn startup() {
        info!("Starting commands Manager...");

        info!("Started commands Manager.");
    }

    fn post_startup() {
        info!("Post-Starting commands Manager...");

        info!("Post-Started commands Manager.");
    }

    fn shutdown() {
        info!("Shutting down commands Manager...");

        info!("Shut down commands Manager.");
    }

    pub fn request_ecs_world_command<T: 'static + Send + Sync + ECSWorldCommand>(&mut self, command: T) {
        trace!("Requesting ecs world command...");

        self.ecs_world_commands.push(Box::new(command));

        trace!("Requested ecs world command.");
    }

    pub fn request_ecs_entity_command<T: 'static + Send + Sync + ECSEntityCommand>(&mut self, command: T, entity: Entity) {
        trace!("Requesting ecs entity command...");

        self.ecs_entity_commands.push((Box::new(command), entity));

        trace!("Requested ecs entity command.");
    }

    fn process_ecs_world_command_requests(mut world_commands: Commands, commands_manager: ResMut<CommandsManager>) {
        trace!("Processing ecs world command requests...");

        for command in commands_manager.ecs_world_commands.iter() {
            command.execute(&mut world_commands);
        }

        trace!("Processed ecs world command requests.");
    }

    fn process_ecs_entity_command_requests(mut world_commands: Commands, mut commands_manager: ResMut<CommandsManager>) {
        trace!("Processing ecs entity command requests...");

        while let Some((command, entity_id)) = commands_manager.ecs_entity_commands.pop() {
            command.execute(&mut world_commands.entity(entity_id));
        }

        trace!("Processed ecs entity command requests.");
    }
}

pub trait ECSWorldCommand {
    fn execute(&self, commands: &mut Commands);
}

pub trait ECSEntityCommand {
    fn execute(&self, entity_commands: &mut EntityCommands);
}