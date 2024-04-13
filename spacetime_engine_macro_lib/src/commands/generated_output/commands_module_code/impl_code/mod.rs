pub mod command_request_function_codes;

use quote::quote;
use syn::Ident;
use crate::commands::parsed_input::commands_type::CommandsModuleType;
use self::command_request_function_codes::CommandsModuleCommandRequestFunctionCodes;

pub struct CommandsModuleImplCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandsModuleImplCode {
    pub fn generate(commands_module_type: &CommandsModuleType) -> Self {
        let commands_module_name = commands_module_type.module_id.value() + "Commands";
        let commands_module_name = Ident::new(&commands_module_name, commands_module_type.module_id.span());

        let commands_module_command_request_function_codes = CommandsModuleCommandRequestFunctionCodes::generate(commands_module_type).tokens;

        let tokens = quote! {
            impl #commands_module_name {
                #commands_module_command_request_function_codes
            }
        };

        Self {
            tokens
        }
    }
}