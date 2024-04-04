pub mod parameter_type;

use parameter_type::*;
use syn::{
    parse::{Parse, ParseStream}, punctuated::Punctuated, Ident, Token
};

#[derive(Clone)]
pub struct CommandInputType {
    pub parameter_types: Vec<CommandInputParameterType>
}

impl Parse for CommandInputType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input_label = input.parse::<Ident>()?;
        let span = input_label.span();
        let input_label = input_label.to_string();

        if input_label != "Input" {
            return Err(syn::Error::new(span, "Expected 'Input' Label"));
        }

        let content;
        syn::braced!(content in input);

        if content.is_empty() {
            return Ok(CommandInputType {
                parameter_types: Vec::new()
            });
        }

        let parsed_parameters: Punctuated<CommandInputParameterType, Token![,]> = Punctuated::parse_terminated(&content)?;

        Ok(CommandInputType {
            parameter_types: parsed_parameters.into_iter().collect()
        })
    }
}
