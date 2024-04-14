use crate::commands::generated_output::commands_module_code::command_codes::command_code::CommandType;

pub struct CommandCodeImplDisplayCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandCodeImplDisplayCode {
    pub fn generate(command_type: &CommandType) -> Self {
        todo!();
    }
}