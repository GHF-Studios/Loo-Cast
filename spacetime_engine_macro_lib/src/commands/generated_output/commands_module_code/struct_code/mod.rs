use quote::quote;
use syn::Ident;
use crate::commands::parsed_input::commands_type::CommandsModuleType;

pub struct CommandsModuleStructCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandsModuleStructCode {
    pub fn generate(commands_module_type: &CommandsModuleType) -> Self {
        let commands_module_name = commands_module_type.module_id.value() + "Commands";
        let commands_module_name = Ident::new(&commands_module_name, commands_module_type.module_id.span());

        let tokens = quote! {
            pub struct #commands_module_name {}
        };

        Self {
            tokens
        }
    }
}