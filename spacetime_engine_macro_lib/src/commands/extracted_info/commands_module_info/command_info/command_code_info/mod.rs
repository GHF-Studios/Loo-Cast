pub mod command_code_parameters_info;

use crate::commands::parsed_input::command_type::*;
use self::command_code_parameters_info::CommandCodeParametersInfo;
use super::CommandInfo;

#[derive(Clone)]
pub struct CommandCodeInfo {
    pub name: String,
    pub parameters_info: CommandCodeParametersInfo
}

impl CommandCodeInfo {
    pub fn extract(command_type: &CommandType, command_info: &CommandInfo) -> Self {
        let parameters_info = CommandCodeParametersInfo::extract(&command_type.code_type, &command_info.code_info);

        Self {
            name: command_info.code_info.name.clone(),
            parameters_info
        }
    }
}