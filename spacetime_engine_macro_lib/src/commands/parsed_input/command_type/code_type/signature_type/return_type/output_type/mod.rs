use syn::{
    braced, parse::{Parse, ParseStream}, token::Brace, Ident
};

#[derive(Clone)]
pub enum CommandCodeSignatureOutputType {
    UnitType,
    OutputType,
}

impl Parse for CommandCodeSignatureOutputType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        if input.peek(Brace) {
            let _content;
            braced!(_content in input);

            Ok(CommandCodeSignatureOutputType::UnitType)
        } else if input.fork().parse::<Ident>().is_ok() {
            let output_type_label = input.parse::<Ident>()?;
            let span = output_type_label.span();
            let output_type_label = output_type_label.to_string();
    
            if output_type_label != "Output" {
                return Err(syn::Error::new(span, "Expected 'Output' Label"));
            }

            Ok(CommandCodeSignatureOutputType::OutputType)
        } else {
            Err(syn::Error::new(input.span(), "Expected 'Output' Label or Unit Label"))
        }
    }
}
