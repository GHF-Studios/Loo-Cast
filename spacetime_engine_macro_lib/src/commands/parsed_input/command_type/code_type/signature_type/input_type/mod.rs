use syn::{
    parse::{Parse, ParseStream}, Ident
};

#[derive(Clone)]
pub struct CommandCodeSignatureInputType;

impl Parse for CommandCodeSignatureInputType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input_type_label = input.parse::<Ident>()?;
        let span = input_type_label.span();
        let input_type_label = input_type_label.to_string();

        if input_type_label != "input" {
            return Err(syn::Error::new(span, "Expected 'input' Label"));
        }

        Ok(CommandCodeSignatureInputType {})
    }
}
