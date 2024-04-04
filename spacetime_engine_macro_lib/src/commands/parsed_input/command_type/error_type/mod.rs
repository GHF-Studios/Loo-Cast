pub mod variant_type;

use variant_type::*;
use syn::{
    parse::{Parse, ParseStream}, punctuated::Punctuated, Ident, Token
};

#[derive(Clone)]
pub struct CommandErrorType {
    pub variant_types: Vec<CommandErrorVariantType>
}

impl Parse for CommandErrorType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let error_label = input.parse::<Ident>()?;
        let span = error_label.span();
        let error_label = error_label.to_string();

        if error_label != "Error" {
            return Err(syn::Error::new(span, "Expected 'Error' Label"));
        }

        let content;
        syn::braced!(content in input);

        if content.is_empty() {
            return Ok(CommandErrorType {
                variant_types: Vec::new()
            });
        }

        let parsed_variants: Punctuated<CommandErrorVariantType, Token![,]> = Punctuated::parse_terminated(&content)?;

        Ok(CommandErrorType {
            variant_types: parsed_variants.into_iter().collect()
        })
    }
}
