use syn::Ident;
use quote::quote;

pub struct CommandInputImplDisplayCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandInputImplDisplayCode {
    pub fn generate(
        command_input_name: Ident,
        command_input_parameters_interpolation: String,
        command_input_parameter_self_accesses: proc_macro2::TokenStream
    ) -> Self {
        let tokens = quote! {
            impl std::fmt::Display for #command_input_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                    write!(f, #command_input_parameters_interpolation, #command_input_parameter_self_accesses)
                }
            }
        };

        Self {
            tokens
        }
    }
}