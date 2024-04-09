pub mod command_input_parameters_info;

use crate::commands::parsed_input::command_type::*;
use self::command_input_parameters_info::*;
use super::CommandInfo;

#[derive(Clone)]
pub struct CommandInputInfo {
    pub name: String,
    pub parameters_info: CommandInputParametersInfo
}

impl CommandInputInfo {
    pub fn extract(command_info: &CommandInfo) -> Self {
        let parameters_info = CommandInputParametersInfo::extract(&command_type.input_type, &command_info.input_info);

        Self {
            name: command_info.input_info.name.clone(),
            parameters_info
        }
    }
}