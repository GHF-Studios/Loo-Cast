use syn::{
    parse::{Parse, ParseStream}, spanned::Spanned, Ident, LitStr, Token, Type
};

#[derive(Clone)]
pub struct CommandOutputParameterType {
    pub parameter_name: LitStr,
    pub parameter_type: Type,
    pub interpolation: String
}

impl Parse for CommandOutputParameterType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parameter_label = input.parse::<Ident>()?;
        let parameter_name = parameter_label.to_string();
        let parameter_name = LitStr::new(&parameter_name, parameter_name.span());

        input.parse::<Token![:]>()?;

        let parameter_type: Type = input.parse()?;

        Ok(CommandOutputParameterType {
            parameter_name: parameter_name.clone(),
            parameter_type: parameter_type.clone(),
            interpolation: format!("{}: ({})", parameter_name.value(), quote::quote!{#parameter_type}.to_string())
        })
    }

}
