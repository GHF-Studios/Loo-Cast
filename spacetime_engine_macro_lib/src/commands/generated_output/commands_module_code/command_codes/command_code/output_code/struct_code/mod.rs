use crate::commands::generated_output::commands_module_code::command_codes::command_code::CommandType;

pub struct CommandCodeStructCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandCodeStructCode {
    pub fn generate(command_type: &CommandType) -> Self {
        todo!();
    }
}