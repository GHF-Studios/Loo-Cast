use syn::{
    parse::{Parse, ParseStream}, spanned::Spanned, Ident, LitStr, Token, Type
};
use quote::quote;

#[derive(Clone)]
pub struct CommandInputParameterType {
    pub parameter_name: LitStr,
    pub parameter_type: Type,
    pub field_declaration: proc_macro2::TokenStream,
    pub self_access: proc_macro2::TokenStream,
    pub interpolation: String
}

impl Parse for CommandInputParameterType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parameter_label = input.parse::<Ident>()?;
        let parameter_name = parameter_label.to_string();
        let parameter_name = LitStr::new(&parameter_name, parameter_name.span());

        input.parse::<Token![:]>()?;

        let parameter_type: Type = input.parse()?;

        Ok(CommandInputParameterType {
            parameter_name: parameter_name.clone(),
            parameter_type: parameter_type.clone(),
            field_declaration: quote! {
                pub #parameter_name: #parameter_type
            },
            self_access: quote! {
                self.#parameter_name
            },
            interpolation: quote! {
                #parameter_name: ({})
            }.to_string()
        })
    }
}
