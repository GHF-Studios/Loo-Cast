mod sub_macro;
mod workflow_id;
mod workflow_invocation;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, Expr, Ident, Token, Result, braced};
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;

use sub_macro::*;
use workflow_id::IdMacro;
use workflow_invocation::WorkflowMacro;

pub struct CompositeWorkflow;

impl Parse for CompositeWorkflow {
    fn parse(input: ParseStream) -> Result<Self> {
        let input: TokenStream = input.parse()?;
        let input = SubMacro::WorkflowId.expand_in(input);
        let input = SubMacro::WorkflowInvocation.expand_in(input);

        Ok(CompositeWorkflow)
    }
}

impl CompositeWorkflow {
    pub fn generate(self) -> TokenStream {
        quote! {}
    }
}
