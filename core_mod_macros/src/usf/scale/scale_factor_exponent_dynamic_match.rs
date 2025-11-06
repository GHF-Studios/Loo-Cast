use quote::{format_ident, quote};
use syn::{Expr, Token, parse::{Parse, ParseStream}};

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

pub fn scale_factor_exponent(scale_str: &str) -> Result<i8, ()> {
    match scale_str {
        "ScaleQuettaMeter100000" => Ok(35),
        "ScaleQuettaMeter10000" => Ok(34),
        "ScaleQuettaMeter1000" => Ok(33),
        "ScaleQuettaMeter100" => Ok(32),
        "ScaleQuettaMeter10" => Ok(31),
        "ScaleQuettaMeter1" => Ok(30),
        "ScaleRonnaMeter100" => Ok(29),
        "ScaleRonnaMeter10" => Ok(28),
        "ScaleRonnaMeter1" => Ok(27),
        "ScaleYottaMeter100" => Ok(26),
        "ScaleYottaMeter10" => Ok(25),
        "ScaleYottaMeter1" => Ok(24),
        "ScaleZettaMeter100" => Ok(23),
        "ScaleZettaMeter10" => Ok(22),
        "ScaleZettaMeter1" => Ok(21),
        "ScaleExaMeter100" => Ok(20),
        "ScaleExaMeter10" => Ok(19),
        "ScaleExaMeter1" => Ok(18),
        "ScalePetaMeter100" => Ok(17),
        "ScalePetaMeter10" => Ok(16),
        "ScalePetaMeter1" => Ok(15),
        "ScaleTeraMeter100" => Ok(14),
        "ScaleTeraMeter10" => Ok(13),
        "ScaleTeraMeter1" => Ok(12),
        "ScaleGigaMeter100" => Ok(11),
        "ScaleGigaMeter10" => Ok(10),
        "ScaleGigaMeter1" => Ok(9),
        "ScaleMegaMeter100" => Ok(8),
        "ScaleMegaMeter10" => Ok(7),
        "ScaleMegaMeter1" => Ok(6),
        "ScaleKiloMeter100" => Ok(5),
        "ScaleKiloMeter10" => Ok(4),
        "ScaleKiloMeter1" => Ok(3),
        "ScaleMeter100" => Ok(2),
        "ScaleMeter10" => Ok(1),
        "ScaleMeter1" => Ok(0),
        "ScaleMilliMeter100" => Ok(-1),
        "ScaleMilliMeter10" => Ok(-2),
        "ScaleMilliMeter1" => Ok(-3),
        "ScaleMicroMeter100" => Ok(-4),
        "ScaleMicroMeter10" => Ok(-5),
        "ScaleMicroMeter1" => Ok(-6),
        "ScaleNanoMeter100" => Ok(-7),
        "ScaleNanoMeter10" => Ok(-8),
        "ScaleNanoMeter1" => Ok(-9),
        "ScalePicoMeter100" => Ok(-10),
        "ScalePicoMeter10" => Ok(-11),
        "ScalePicoMeter1" => Ok(-12),
        "ScaleFemtoMeter100" => Ok(-13),
        "ScaleFemtoMeter10" => Ok(-14),
        "ScaleFemtoMeter1" => Ok(-15),
        "ScaleAttoMeter100" => Ok(-16),
        "ScaleAttoMeter10" => Ok(-17),
        "ScaleAttoMeter1" => Ok(-18),
        "ScaleZeptoMeter100" => Ok(-19),
        "ScaleZeptoMeter10" => Ok(-20),
        "ScaleZeptoMeter1" => Ok(-21),
        "ScaleYoctoMeter100" => Ok(-22),
        "ScaleYoctoMeter10" => Ok(-23),
        "ScaleYoctoMeter1" => Ok(-24),
        "ScaleRontoMeter100" => Ok(-25),
        "ScaleRontoMeter10" => Ok(-26),
        "ScaleRontoMeter1" => Ok(-27),
        "ScaleQuectoMeter100" => Ok(-28),
        "ScaleQuectoMeter10" => Ok(-29),
        "ScaleQuectoMeter1" => Ok(-30),
        "ScaleQuectoMeter01" => Ok(-31),
        "ScaleQuectoMeter001" => Ok(-32),
        "ScaleQuectoMeter0001" => Ok(-33),
        "ScaleQuectoMeter00001" => Ok(-34),
        "ScaleQuectoMeter000001" => Ok(-35),
        _ => Err(()),
    }
}

pub struct ScaleFactorExponentDynamicMatch {
    value_expr: Expr,
    case_expr: Expr,
    fallback_case_expr: Expr,
}
impl Parse for ScaleFactorExponentDynamicMatch {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let value_expr: Expr = input.parse()?;
        let _ = input.parse::<Token![,]>()?;
        let case_expr: Expr = input.parse()?;
        let _ = input.parse::<Token![,]>()?;
        let fallback_case_expr: Expr = input.parse()?;

        Ok(Self { value_expr, case_expr, fallback_case_expr })
    }
}
impl ScaleFactorExponentDynamicMatch {
    pub fn generate(self) -> proc_macro2::TokenStream {
        let ScaleFactorExponentDynamicMatch { value_expr, case_expr, fallback_case_expr } = self;

        let scales = SCALES.iter().map(|scale| {
                let scale_factor_exponent = scale_factor_exponent(scale).unwrap();
                let scale_ident = format_ident!("{}", scale);

                quote! { #scale_factor_exponent => { const __SCALE__: Scale = Scale::#scale_ident; #case_expr } }
        }).collect::<Vec<proc_macro2::TokenStream>>();

        let expanded = quote! {
            match #value_expr {
                i8::MIN..=-36_i8 => #fallback_case_expr,
                #(#scales,)*
                36_i8..=i8::MAX => #fallback_case_expr
            }
        };

        expanded
    }
}
