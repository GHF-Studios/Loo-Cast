use syn::Ident;
use quote::quote;

pub struct CommandErrorEnumCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandErrorEnumCode {
    pub fn generate(
        command_error_name: Ident,
        command_error_variants_interpolation: proc_macro2::TokenStream
    ) -> Self {
        let tokens = quote! {
            pub enum #command_error_name {
                #command_error_variants_interpolation
            }
        };

        Self {
            tokens
        }
    }
}