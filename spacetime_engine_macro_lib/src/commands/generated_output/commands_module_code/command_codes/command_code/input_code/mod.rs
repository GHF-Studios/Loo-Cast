pub mod struct_code;
pub mod impl_code;

use crate::commands::generated_output::commands_module_code::command_codes::command_code::CommandType;
use self::struct_code::*;
use self::impl_code::*;

pub struct CommandInputCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandInputCode {
    pub fn generate(command_type: &CommandType) -> Self {
        todo!();
    }
}