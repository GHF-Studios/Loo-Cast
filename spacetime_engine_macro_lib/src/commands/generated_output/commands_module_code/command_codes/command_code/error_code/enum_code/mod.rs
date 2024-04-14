use crate::commands::generated_output::commands_module_code::command_codes::command_code::CommandType;

pub struct CommandErrorEnumCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandErrorEnumCode {
    pub fn generate(command_type: &CommandType) -> Self {
        todo!();
    }
}