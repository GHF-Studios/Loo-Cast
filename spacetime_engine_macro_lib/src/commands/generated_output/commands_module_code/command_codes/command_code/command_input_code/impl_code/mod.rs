use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;
use super::CommandInputParametersInfo;

pub struct CommandInputImplCode {
    pub tokens: TokenStream,
}

impl CommandInputImplCode {
    pub fn generate(
        command_input_name: Ident,
        command_input_parameters_info: &CommandInputParametersInfo
    ) -> Self {
        let input_parameter_self_accesses = &command_input_parameters_info.self_accesses;
        let input_parameter_interpolations = &command_input_parameters_info.interpolations;

        let tokens = quote! {
            impl std::fmt::Display for #command_input_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                    write!(f, #input_parameter_interpolations, #input_parameter_self_accesses)
                }
            }
        };

        Self {
            tokens
        }
    }
}