pub mod command_info;

use self::command_info::*;

#[derive(Clone)]
pub struct CommandsModuleInfo {
    pub name: String,
    pub command_infos: Vec<CommandInfo>
}