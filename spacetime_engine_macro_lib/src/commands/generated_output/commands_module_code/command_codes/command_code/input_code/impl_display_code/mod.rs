use crate::commands::generated_output::commands_module_code::command_codes::command_code::CommandType;

pub struct CommandInputImplDisplayCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandInputImplDisplayCode {
    pub fn generate(command_type: &CommandType) -> Self {
        todo!();
    }
}