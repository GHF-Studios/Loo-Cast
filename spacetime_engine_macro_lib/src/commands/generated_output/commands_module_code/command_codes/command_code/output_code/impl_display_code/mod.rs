use syn::Ident;
use quote::quote;

pub struct CommandOutputImplDisplayCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandOutputImplDisplayCode {
    pub fn generate(
        command_output_name: Ident,
        command_output_parameters_interpolation: String,
        command_output_parameter_self_accesses: proc_macro2::TokenStream
    ) -> Self {
        let tokens = quote! {
            impl std::fmt::Display for #command_output_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                    write!(f, #command_output_parameters_interpolation, #command_output_parameter_self_accesses)
                }
            }
        };

        Self {
            tokens
        }
    }
}