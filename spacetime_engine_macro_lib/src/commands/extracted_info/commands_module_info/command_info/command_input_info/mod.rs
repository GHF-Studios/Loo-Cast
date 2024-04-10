pub mod command_input_parameters_info;

use crate::commands::parsed_input::{command_type::*};

use self::{command_input_parameters_info::CommandInputParametersInfo, input_type::CommandInputType};

#[derive(Clone)] 
pub struct CommandInputInfo {
    pub parameters_info: CommandInputParametersInfo
}

impl CommandInputInfo {
    pub fn extract(command_input_type: CommandInputType) -> Self {
        Self {
            parameters_info: CommandInputParametersInfo::extract(command_input_type)
        }
    }
}