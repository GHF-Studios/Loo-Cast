pub mod command_error_variants_info;

use crate::commands::parsed_input::command_type::*;
use self::command_error_variants_info::CommandErrorVariantsInfo;
use super::CommandInfo;

#[derive(Clone)]
pub struct CommandErrorInfo {
    pub name: String,
    pub variants_info: CommandErrorVariantsInfo
}

impl CommandErrorInfo {
    pub fn extract(command_type: &CommandType, command_info: &CommandInfo) -> Self {
        let variants_info = CommandErrorVariantsInfo::extract(&command_type.error_type, &command_info.error_info);

        Self {
            name: command_info.error_info.name.clone(),
            variants_info
        }
    }
}