pub mod struct_code;
pub mod impl_code;
pub mod command_codes;

use quote::quote;
use crate::commands::parsed_input::commands_type::CommandsModuleType;

use self::{command_codes::CommandsModuleCommandCodes, impl_code::CommandsModuleImplCode, struct_code::CommandsModuleStructCode};

pub struct CommandsModuleCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandsModuleCode {
    pub fn generate(commands_module_type: &CommandsModuleType) -> Self {
        let commands_module_struct_code = CommandsModuleStructCode::generate(commands_module_type).tokens;
        let commands_module_impl_code = CommandsModuleImplCode::generate(commands_module_type).tokens;
        let commands_module_command_codes = CommandsModuleCommandCodes::generate(commands_module_type).tokens;

        let tokens = quote! {
            #commands_module_struct_code
            #commands_module_impl_code
            #commands_module_command_codes
        };

        Self {
            tokens
        }
    }
}