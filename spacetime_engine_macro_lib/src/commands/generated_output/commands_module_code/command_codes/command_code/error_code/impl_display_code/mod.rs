use syn::Ident;
use quote::quote;

pub struct CommandErrorImplDisplayCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandErrorImplDisplayCode {
    pub fn generate(
        command_error_name: Ident,
        command_error_variants_interpolation: proc_macro2::TokenStream
    ) -> Self {
        let tokens = quote! {
            impl std::fmt::Display for #command_error_name {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
                    match *self {
                        #command_error_variants_interpolation
                    }
                }
            }
        };

        Self {
            tokens
        }
    }
}