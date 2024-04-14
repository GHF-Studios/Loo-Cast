use crate::commands::generated_output::commands_module_code::command_codes::command_code::CommandType;

pub struct CommandInputImplCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandInputImplCode {
    pub fn generate(command_type: &CommandType) -> Self {
        todo!();
    }
}