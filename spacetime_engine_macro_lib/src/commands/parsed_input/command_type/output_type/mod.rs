pub mod parameter_type;

use parameter_type::*;
use syn::{
    parse::{Parse, ParseStream}, punctuated::Punctuated, Ident, Token
};

#[derive(Clone)]
pub struct CommandOutputType {
    pub parameter_types: Vec<CommandOutputParameterType>,
    pub interpolation: String
}

impl Parse for CommandOutputType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let output_label = input.parse::<Ident>()?;
        let span = output_label.span();
        let output_label = output_label.to_string();

        if output_label != "Output" {
            return Err(syn::Error::new(span, "Expected 'Output' Label"));
        }

        let content;
        syn::braced!(content in input);

        if content.is_empty() {
            return Ok(CommandOutputType {
                parameter_types: Vec::new(),
                interpolation: String::new()
            });
        }

        let parsed_parameters: Punctuated<CommandOutputParameterType, Token![,]> = Punctuated::parse_terminated(&content)?;

        Ok(CommandOutputType {
            parameter_types: parsed_parameters.clone().into_iter().collect(),
            interpolation: format!("parameter_types: ({})", parsed_parameters.iter().map(|parameter_type| parameter_type.interpolation.clone()).collect::<Vec<String>>().join(", "))
        })
    }
}
