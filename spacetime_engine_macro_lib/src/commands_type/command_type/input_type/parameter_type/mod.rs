use syn::{
    parse::{Parse, ParseStream}, spanned::Spanned, Ident, LitStr, Token
};

#[derive(Clone)]
pub struct CommandInputParameterType {
    pub parameter_name: LitStr,
    pub parameter_type: syn::Type
}

impl Parse for CommandInputParameterType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parameter_label = input.parse::<Ident>()?;
        let parameter_name = parameter_label.to_string();
        let parameter_name = LitStr::new(&parameter_name, parameter_name.span());

        input.parse::<Token![:]>()?;

        let parameter_type = input.parse()?;

        Ok(CommandInputParameterType {
            parameter_name,
            parameter_type
        })
    }
}
