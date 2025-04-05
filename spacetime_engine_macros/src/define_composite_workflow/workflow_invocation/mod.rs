use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, Expr, Ident, Token, Result, braced};
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{Attribute, ExprPath, ExprStruct};

pub struct WorkflowInvocation(pub proc_macro2::TokenStream);

impl Parse for WorkflowInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        let content: proc_macro2::TokenStream = input.parse()?;
        Ok(WorkflowInvocation(content))
    }
}

impl WorkflowInvocation {
    pub fn generate(self) -> proc_macro2::TokenStream {
        self.0
    }
}
