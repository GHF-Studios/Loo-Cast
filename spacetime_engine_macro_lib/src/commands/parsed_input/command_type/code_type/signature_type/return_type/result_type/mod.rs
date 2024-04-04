use super::output_type::*;
use syn::{
    parenthesized, parse::{Parse, ParseStream}, token::Paren, Ident, Token
};

#[derive(Clone)]
pub struct CommandCodeSignatureResultType {
    pub output_type: CommandCodeSignatureOutputType,
}

impl Parse for CommandCodeSignatureResultType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let output_type;
        if input.peek(Paren) {
            let _content;
            parenthesized!(_content in input);

            output_type = CommandCodeSignatureOutputType::UnitType;
        } else if input.fork().parse::<Ident>().is_ok() {
            let output_type_label = input.parse::<Ident>()?;
            let span = output_type_label.span();
            let output_type_label = output_type_label.to_string();

            if output_type_label != "Output" {
                return Err(syn::Error::new(span, "Expected 'Output' Label"));
            }

            output_type = CommandCodeSignatureOutputType::OutputType;
        } else {
            return Err(syn::Error::new(input.span(), "Expected 'Output' Label or Unit Label"))
        }

        input.parse::<Token![,]>()?;

        let error_parameter_label = input.parse::<Ident>()?;
        let span = error_parameter_label.span();
        let error_parameter_label = error_parameter_label.to_string();

        if error_parameter_label != "Error" {
            return Err(syn::Error::new(span, "Expected 'Error' Label"));
        }

        Ok(CommandCodeSignatureResultType {
            output_type
        })
    }
}
