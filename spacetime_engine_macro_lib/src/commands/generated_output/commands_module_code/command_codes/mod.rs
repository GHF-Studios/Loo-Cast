pub mod command_code;

use quote::quote;

use crate::commands::parsed_input::commands_type::CommandsModuleType;

use self::command_code::CommandsModuleCommandCode;

pub struct CommandsModuleCommandCodes {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandsModuleCommandCodes {
    pub fn generate(commands_module_type: &CommandsModuleType) -> Self {
        let mut tokens = quote! {};
        let mut first = true;
        for command_type in &commands_module_type.command_types.values {
            let command_code = CommandsModuleCommandCode::generate(command_type).tokens;

            if !first {
                tokens = quote! {
                    #tokens

                    #command_code
                }
            } else {
                first = false;

                tokens = quote! {
                    #command_code
                }
            }
        }

        Self {
            tokens
        }
    }
}