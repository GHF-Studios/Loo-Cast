use crate::commands::generated_output::commands_module_code::command_codes::command_code::CommandType;

pub struct CommandCodeImplCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandCodeImplCode {
    pub fn generate(command_type: &CommandType) -> Self {
        todo!();
    }
}