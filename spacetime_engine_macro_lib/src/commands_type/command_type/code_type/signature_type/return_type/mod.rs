pub mod output_type;
pub mod result_type;

use output_type::*;
use result_type::*;
use syn::{
    parenthesized, parse::{Parse, ParseStream}, token::Paren, Ident, Token
};

#[derive(Clone)]
pub enum CommandCodeSignatureReturnType {
    OutputType(CommandCodeSignatureOutputType),
    ResultType(CommandCodeSignatureResultType),
}

impl Parse for CommandCodeSignatureReturnType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![-]>()?;
        input.parse::<Token![>]>()?;

        if input.peek(Paren) {
            let _content;
            parenthesized!(_content in input);

            Ok(Self::OutputType(CommandCodeSignatureOutputType::UnitType))
        } else if input.fork().parse::<Ident>().is_ok() {
            let return_type_label = input.parse::<Ident>()?;
            let span = return_type_label.span();
            let return_type_label = return_type_label.to_string();
    
            match return_type_label.as_str() {
                "Output" => {
                    let output_type = CommandCodeSignatureOutputType::OutputType;

                    return Ok(Self::OutputType(output_type))
                }
                "Result" => {
                    input.parse::<Token![<]>()?;

                    let result_type = input.parse::<CommandCodeSignatureResultType>()?;

                    input.parse::<Token![>]>()?;

                    return Ok(Self::ResultType(result_type));
                }
                _ => {
                    return Err(syn::Error::new(span, "Expected 'Output' Label or 'Result' Label"));
                }
            }
        } else {
            Err(syn::Error::new(input.span(), "Expected 'Output' Label or Unit Label or 'Result' Label"))
        }
    }
}
