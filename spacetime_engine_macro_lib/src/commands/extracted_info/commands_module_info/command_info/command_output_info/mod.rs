pub mod command_output_parameters_info;

use crate::commands::parsed_input::command_type::*;
use self::command_output_parameters_info::*;
use super::CommandInfo;

#[derive(Clone)]
pub struct CommandOutputInfo {
    pub name: String,
    pub parameters_info: CommandOutputParametersInfo
}

impl CommandOutputInfo {
    pub fn extract(command_type: &CommandType, command_info: &CommandInfo) -> Self {
        let parameters_info = CommandOutputParametersInfo::extract(&command_type.output_type, &command_info.output_info);

        Self {
            name: command_info.output_info.name.clone(),
            parameters_info
        }
    }
}