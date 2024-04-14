use syn::Ident;
use quote::quote;

pub struct CommandOutputStructCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandOutputStructCode {
    pub fn generate(
        command_output_name: Ident,
        command_output_parameter_declarations: proc_macro2::TokenStream
    ) -> Self {
        let tokens = quote! {
            pub struct #command_output_name {
                #command_output_parameter_declarations
            }
        };

        Self {
            tokens
        }
    }
}