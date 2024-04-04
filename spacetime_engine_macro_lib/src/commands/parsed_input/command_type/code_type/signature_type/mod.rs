pub mod input_type;
pub mod return_type;

use input_type::*;
use return_type::*;
use syn::{
    parse::{Parse, ParseStream}, Ident, Token
};

#[derive(Clone)]
pub struct CommandCodeSignature {
    pub input_type: Option<CommandCodeSignatureInputType>,
    pub return_type: CommandCodeSignatureReturnType,
}

impl Parse for CommandCodeSignature {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let code_label = input.parse::<Ident>()?;
        let span = code_label.span();
        let code_label = code_label.to_string();

        if code_label != "Code" {
            return Err(syn::Error::new(span, "Expected 'Code' Label"));
        }

        input.parse::<Token![|]>()?;

        let mut input_type = None;
        if input.fork().parse::<Ident>().is_ok() {
            input_type = Some(input.parse::<CommandCodeSignatureInputType>()?);
        }

        input.parse::<Token![|]>()?;

        let return_type = input.parse::<CommandCodeSignatureReturnType>()?;

        Ok(CommandCodeSignature {
            input_type,
            return_type
        })
    }

}
