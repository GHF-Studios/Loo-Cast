use quote::quote;
use syn::Ident;

pub struct CommandInputStructCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandInputStructCode {
    pub fn generate(
        command_input_name: Ident,
        command_input_parameter_declarations: proc_macro2::TokenStream
    ) -> Self {
        let tokens = quote! {
            pub struct #command_input_name {
                #command_input_parameter_declarations
            }
        };

        Self {
            tokens
        }
    }
}