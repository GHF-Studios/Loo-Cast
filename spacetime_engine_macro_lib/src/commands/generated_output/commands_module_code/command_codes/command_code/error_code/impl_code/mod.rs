use crate::commands::generated_output::commands_module_code::command_codes::command_code::CommandType;

pub struct CommandErrorImplCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandErrorImplCode {
    pub fn generate(command_type: &CommandType) -> Self {
        todo!();
    }
}