pub mod command_input_info;
pub mod command_output_info;
pub mod command_error_info;
pub mod command_code_info;

use crate::commands::parsed_input::commands_type::CommandsModuleType;

use self::command_input_info::*;
use self::command_output_info::*;
use self::command_error_info::*;
use self::command_code_info::*;

use super::CommandsModuleInfo;

#[derive(Clone)]
pub struct CommandInfo {
    pub name: String,
    pub input_info: CommandInputInfo,
    pub output_info: CommandOutputInfo,
    pub error_info: CommandErrorInfo,
    pub code_info: CommandCodeInfo
}

impl CommandInfo {
    pub fn extract(_commands_module_type: CommandsModuleType, commands_module_info: CommandsModuleInfo) -> Vec<Self> {
        commands_module_info.command_infos.iter().map(|command_info| {
            Self {
                name: command_info.name.clone(),
                input_info: command_info.input_info.clone(),
                output_info: command_info.output_info.clone(),
                error_info: command_info.error_info.clone(),
                code_info: command_info.code_info.clone()
            }
        }).collect()
    }
}