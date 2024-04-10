pub mod command_info;

use self::command_info::*;

#[derive(Clone)]
pub struct CommandsModuleInfo {
    pub name: String,
    pub command_infos: Vec<CommandInfo>
}

impl CommandsModuleInfo {
    pub fn extract(commands_module_type: &CommandsModuleType) -> Self {
        let name = commands_module_type.name().to_string();
        let command_infos = commands_module_type.commands().iter().map(|command_type| {
            CommandInfo::extract(command_type)
        }).collect();
        CommandsModuleInfo {
            name,
            command_infos
        }
    }
}