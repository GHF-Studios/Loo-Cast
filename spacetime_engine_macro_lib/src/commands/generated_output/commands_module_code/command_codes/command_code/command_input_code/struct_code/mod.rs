use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
use super::CommandInputParametersInfo;

pub struct CommandInputStructCode {
    pub tokens: TokenStream,
}

impl CommandInputStructCode {
    pub fn generate(
        command_input_name: Ident,
        command_input_parameters_info: &CommandInputParametersInfo
    ) -> Self {
        let input_parameter_field_declarations = &command_input_parameters_info.field_declarations;

        let tokens = quote! {
            pub struct #command_input_name {
                #input_parameter_field_declarations
            }
        };

        Self {
            tokens
        }
    }
}