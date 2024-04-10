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
    pub fn extract(command_type: &CommandType) -> Self {
        let name = command_type.name().to_string();
        let input_info = CommandInputInfo::extract(command_type.input_type());
        let output_info = CommandOutputInfo::extract(command_type.output_type());
        let error_info = CommandErrorInfo::extract(command_type.error_type());
        let code_info = CommandCodeInfo::extract(command_type.code_type());
        CommandInfo {
            name,
            input_info,
            output_info,
            error_info,
            code_info
        }
    }
}