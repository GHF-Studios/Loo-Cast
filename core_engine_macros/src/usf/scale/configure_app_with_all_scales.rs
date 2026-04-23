use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::{Result, Token};

pub struct AppConfigInput {
    blocks: Vec<TokenStream>,
}

impl Parse for AppConfigInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut blocks = Vec::new();
        while !input.is_empty() {
            let content;
            syn::braced!(content in input);
            let tokens: TokenStream = content.parse()?;
            blocks.push(tokens);
            let _ = input.parse::<Token![,]>();
        }
        Ok(Self { blocks })
    }
}

impl AppConfigInput {
    pub fn generate(self) -> TokenStream {
        const SCALE_NAMES: [&str; 71] = [
            "ScaleQuectoMeter000001",
            "ScaleQuectoMeter00001",
            "ScaleQuectoMeter0001",
            "ScaleQuectoMeter001",
            "ScaleQuectoMeter01",
            "ScaleQuectoMeter1",
            "ScaleQuectoMeter10",
            "ScaleQuectoMeter100",
            "ScaleRontoMeter1",
            "ScaleRontoMeter10",
            "ScaleRontoMeter100",
            "ScaleYoctoMeter1",
            "ScaleYoctoMeter10",
            "ScaleYoctoMeter100",
            "ScaleZeptoMeter1",
            "ScaleZeptoMeter10",
            "ScaleZeptoMeter100",
            "ScaleAttoMeter1",
            "ScaleAttoMeter10",
            "ScaleAttoMeter100",
            "ScaleFemtoMeter1",
            "ScaleFemtoMeter10",
            "ScaleFemtoMeter100",
            "ScalePicoMeter1",
            "ScalePicoMeter10",
            "ScalePicoMeter100",
            "ScaleNanoMeter1",
            "ScaleNanoMeter10",
            "ScaleNanoMeter100",
            "ScaleMicroMeter1",
            "ScaleMicroMeter10",
            "ScaleMicroMeter100",
            "ScaleMilliMeter1",
            "ScaleMilliMeter10",
            "ScaleMilliMeter100",
            "ScaleMeter1",
            "ScaleMeter10",
            "ScaleMeter100",
            "ScaleKiloMeter1",
            "ScaleKiloMeter10",
            "ScaleKiloMeter100",
            "ScaleMegaMeter1",
            "ScaleMegaMeter10",
            "ScaleMegaMeter100",
            "ScaleGigaMeter1",
            "ScaleGigaMeter10",
            "ScaleGigaMeter100",
            "ScaleTeraMeter1",
            "ScaleTeraMeter10",
            "ScaleTeraMeter100",
            "ScalePetaMeter1",
            "ScalePetaMeter10",
            "ScalePetaMeter100",
            "ScaleExaMeter1",
            "ScaleExaMeter10",
            "ScaleExaMeter100",
            "ScaleZettaMeter1",
            "ScaleZettaMeter10",
            "ScaleZettaMeter100",
            "ScaleYottaMeter1",
            "ScaleYottaMeter10",
            "ScaleYottaMeter100",
            "ScaleRonnaMeter1",
            "ScaleRonnaMeter10",
            "ScaleRonnaMeter100",
            "ScaleQuettaMeter1",
            "ScaleQuettaMeter10",
            "ScaleQuettaMeter100",
            "ScaleQuettaMeter1000",
            "ScaleQuettaMeter10000",
            "ScaleQuettaMeter100000",
        ];
        let scale_idents: [Ident; 71] = SCALE_NAMES.map(|scale_name| Ident::new(scale_name, proc_macro2::Span::call_site()));

        let mut blocks = Vec::new();

        for scale_ident in scale_idents {
            let mut chain = quote! { app };
            for block in &self.blocks {
                let block_str = block.to_string();
                let replaced_str = block_str.replace("__S__", &scale_ident.to_string());
                let replaced_tokens: TokenStream = replaced_str.parse().expect("Failed to parse substituted block");

                chain = quote! {
                    #chain
                    #replaced_tokens
                };
            }

            blocks.push(quote! {
                {
                    #chain;
                }
            });
        }

        quote! {
            #(#blocks)*
        }
    }
}
