use syn::Ident;
use quote::quote;

pub struct CommandCodeImplDisplayCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandCodeImplDisplayCode {
    pub fn generate(
        command_code_name: Ident,
        command_code_parameters_interpolation: String
    ) -> Self {
        let tokens = quote! {
            impl std::fmt::Display for #command_code_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                    write!(f, #command_code_parameters_interpolation)
                }
            }
        };

        Self {
            tokens
        }
    }
}