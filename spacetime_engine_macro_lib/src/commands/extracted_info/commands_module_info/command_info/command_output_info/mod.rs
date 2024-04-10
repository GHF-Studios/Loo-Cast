pub mod command_output_parameters_info;

use crate::commands::parsed_input::command_type::*;
use self::command_output_parameters_info::*;
use super::CommandInfo;

#[derive(Clone)]
pub struct CommandOutputInfo {
    pub parameters_info: CommandOutputParametersInfo
}

impl CommandOutputInfo {
    pub fn extract(command_output_type: &CommandOutputType) -> Self {
        Self {
            parameters_info: CommandOutputParametersInfo::extract(command_output_type)
        }
    }
}