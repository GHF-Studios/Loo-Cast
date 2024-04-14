pub mod enum_code;
pub mod impl_code;

use crate::commands::generated_output::commands_module_code::command_codes::command_code::CommandType;
use self::enum_code::*;
use self::impl_code::*;

pub struct CommandErrorCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandErrorCode {
    pub fn generate(command_type: &CommandType) -> Self {
        todo!();
    }
}