use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Block, Expr, Ident, Token, parse::{Parse, ParseStream}};

pub const SCALES: &[&str] = &[
    "ScaleQuettaMeter100000", "ScaleQuettaMeter10000", "ScaleQuettaMeter1000",
    "ScaleQuettaMeter100", "ScaleQuettaMeter10", "ScaleQuettaMeter1",
    "ScaleRonnaMeter100", "ScaleRonnaMeter10", "ScaleRonnaMeter1",
    "ScaleYottaMeter100", "ScaleYottaMeter10", "ScaleYottaMeter1",
    "ScaleZettaMeter100", "ScaleZettaMeter10", "ScaleZettaMeter1",
    "ScaleExaMeter100", "ScaleExaMeter10", "ScaleExaMeter1",
    "ScalePetaMeter100", "ScalePetaMeter10", "ScalePetaMeter1",
    "ScaleTeraMeter100", "ScaleTeraMeter10", "ScaleTeraMeter1",
    "ScaleGigaMeter100", "ScaleGigaMeter10", "ScaleGigaMeter1",
    "ScaleMegaMeter100", "ScaleMegaMeter10", "ScaleMegaMeter1",
    "ScaleKiloMeter100", "ScaleKiloMeter10", "ScaleKiloMeter1",
    "ScaleMeter100", "ScaleMeter10", "ScaleMeter1",
    "ScaleMilliMeter100", "ScaleMilliMeter10", "ScaleMilliMeter1",
    "ScaleMicroMeter100", "ScaleMicroMeter10", "ScaleMicroMeter1",
    "ScaleNanoMeter100", "ScaleNanoMeter10", "ScaleNanoMeter1",
    "ScalePicoMeter100", "ScalePicoMeter10", "ScalePicoMeter1",
    "ScaleFemtoMeter100", "ScaleFemtoMeter10", "ScaleFemtoMeter1",
    "ScaleAttoMeter100", "ScaleAttoMeter10", "ScaleAttoMeter1",
    "ScaleZeptoMeter100", "ScaleZeptoMeter10", "ScaleZeptoMeter1",
    "ScaleYoctoMeter100", "ScaleYoctoMeter10", "ScaleYoctoMeter1",
    "ScaleRontoMeter100", "ScaleRontoMeter10", "ScaleRontoMeter1",
    "ScaleQuectoMeter100", "ScaleQuectoMeter10", "ScaleQuectoMeter1",
    "ScaleQuectoMeter01", "ScaleQuectoMeter001", "ScaleQuectoMeter0001",
    "ScaleQuectoMeter00001", "ScaleQuectoMeter000001"
];

pub struct ScaleTypeGenericMatch {
    expr: Expr,
    block: Block,
    overrides: Vec<(Ident, Block)>,
}
impl Parse for ScaleTypeGenericMatch {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let expr: Expr = input.parse()?;
        let _ = input.parse::<Token![,]>()?;
        let block: Block = input.parse()?;

        let mut overrides = Vec::new();

        if input.is_empty() {
            return Ok(Self { expr, block, overrides });
        }

        input.parse::<Token![,]>()?;

        loop {
            let scale_ident: Ident = input.parse()?;
            if SCALES.iter().all(|&s| s != scale_ident.to_string().as_str()) {
                return Err(syn::Error::new(
                    scale_ident.span(), 
                    "Expected any of the following scale identifiers: \
                    ScaleQuettaMeter100000, ScaleQuettaMeter10000, ScaleQuettaMeter1000, \
                    ScaleQuettaMeter100, ScaleQuettaMeter10, ScaleQuettaMeter1, \
                    ScaleRonnaMeter100, ScaleRonnaMeter10, ScaleRonnaMeter1, \
                    ScaleYottaMeter100, ScaleYottaMeter10, ScaleYottaMeter1, \
                    ScaleZettaMeter100, ScaleZettaMeter10, ScaleZettaMeter1, \
                    ScaleExaMeter100, ScaleExaMeter10, ScaleExaMeter1, \
                    ScalePetaMeter100, ScalePetaMeter10, ScalePetaMeter1, \
                    ScaleTeraMeter100, ScaleTeraMeter10, ScaleTeraMeter1, \
                    ScaleGigaMeter100, ScaleGigaMeter10, ScaleGigaMeter1, \
                    ScaleMegaMeter100, ScaleMegaMeter10, ScaleMegaMeter1, \
                    ScaleKiloMeter100, ScaleKiloMeter10, ScaleKiloMeter1, \
                    ScaleMeter100, ScaleMeter10, ScaleMeter1, \
                    ScaleMilliMeter100, ScaleMilliMeter10, ScaleMilliMeter1, \
                    ScaleMicroMeter100, ScaleMicroMeter10, ScaleMicroMeter1, \
                    ScaleNanoMeter100, ScaleNanoMeter10, ScaleNanoMeter1, \
                    ScalePicoMeter100, ScalePicoMeter10, ScalePicoMeter1, \
                    ScaleFemtoMeter100, ScaleFemtoMeter10, ScaleFemtoMeter1, \
                    ScaleAttoMeter100, ScaleAttoMeter10, ScaleAttoMeter1, \
                    ScaleZeptoMeter100, ScaleZeptoMeter10, ScaleZeptoMeter1, \
                    ScaleYoctoMeter100, ScaleYoctoMeter10, ScaleYoctoMeter1, \
                    ScaleRontoMeter100, ScaleRontoMeter10, ScaleRontoMeter1, \
                    ScaleQuectoMeter100, ScaleQuectoMeter10, ScaleQuectoMeter1, \
                    ScaleQuectoMeter01, ScaleQuectoMeter001, ScaleQuectoMeter0001, \
                    ScaleQuectoMeter00001, ScaleQuectoMeter000001"
                ));
            }
            let _ = input.parse::<Token![=>]>()?;
            let override_block: Block = input.parse()?;
            overrides.push((scale_ident, override_block));

            if input.peek(Token![,]) {
                let _ = input.parse::<Token![,]>()?;
            }
            
            if input.is_empty() {
                break;
            }
        }

        Ok(Self { expr, block, overrides })
    }
}
impl ScaleTypeGenericMatch {
    pub fn generate(self) -> proc_macro2::TokenStream {
        let ScaleTypeGenericMatch { expr, block, overrides } = self;

        let fallback_arms = SCALES.iter().map(|variant| {
            let ident = format_ident!("{}", variant);
            quote! { Scale::#ident => { type __SCALE__ = #ident; #block } }
        });

        let override_arms = overrides.iter().map(|(pat, block)| {
            quote! { #pat => #block }
        });

        let expanded = quote! {
            match #expr {
                #(#override_arms,)*
                scale_type => match scale_type {
                    #(#fallback_arms,)*
                }
            }
        };

        expanded
    }
}
