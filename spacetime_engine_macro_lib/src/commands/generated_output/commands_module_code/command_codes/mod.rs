pub mod command_code;

pub struct CommandsModuleCommandCodes {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandsModuleCommandCodes {
    pub fn generate(commands_module_type: &CommandsModuleType) -> Self {
        todo!();
    }
}