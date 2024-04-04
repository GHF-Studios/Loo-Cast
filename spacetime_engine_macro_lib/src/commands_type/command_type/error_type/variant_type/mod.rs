use syn::{
    parse::{Parse, ParseStream}, spanned::Spanned, Ident, LitStr
};

#[derive(Clone)]
pub struct CommandErrorVariantType {
    pub variant_name: LitStr
}

impl Parse for CommandErrorVariantType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variant_label = input.parse::<Ident>()?;
        let variant_name = variant_label.to_string();
        let variant_name = LitStr::new(&variant_name, variant_name.span());

        Ok(CommandErrorVariantType {
            variant_name
        })
    }
}
