use proc_macro::TokenStream;
use std::str::FromStr;

/// Canonical `core` constants trait path.
const CORE_CONSTANT_TRAITS: &[&str] = &["crate::math::scalar::constants::core::CoreConstants"];

/// Canonical range-sample constants module paths.
const RANGE_SAMPLE_MODULES: &[&str] = &[
    "positive_decillions",
    "positive_nonillions",
    "positive_octillions",
    "positive_septillions",
    "positive_sextillions",
    "positive_quintillions",
    "positive_quadrillions",
    "positive_trillions",
    "positive_billions",
    "positive_millions",
    "positive_thousands",
    "positive_peanuts",
    "positive_one_over_peanuts",
    "positive_one_over_thousands",
    "positive_one_over_millions",
    "positive_one_over_billions",
    "positive_one_over_trillions",
    "positive_one_over_quadrillions",
    "positive_one_over_quintillions",
    "positive_one_over_sextillions",
    "positive_one_over_septillions",
    "positive_one_over_octillions",
    "positive_one_over_nonillions",
    "positive_one_over_decillions",
    "negative_decillion",
    "negative_nonillion",
    "negative_octillion",
    "negative_septillion",
    "negative_sextillion",
    "negative_quintillion",
    "negative_quadrillion",
    "negative_trillion",
    "negative_billion",
    "negative_million",
    "negative_thousand",
    "negative_peanuts",
    "negative_one_over_peanuts",
    "negative_one_over_thousands",
    "negative_one_over_millions",
    "negative_one_over_billions",
    "negative_one_over_trillions",
    "negative_one_over_quadrillions",
    "negative_one_over_quintillions",
    "negative_one_over_sextillions",
    "negative_one_over_septillions",
    "negative_one_over_octillions",
    "negative_one_over_nonillions",
    "negative_one_over_decillions",
];

/// Emits `compile_error!` tokens with an escaped message.
fn compile_error_tokens(message: &str) -> TokenStream {
    let escaped = message.replace('\\', "\\\\").replace('"', "\\\"");
    TokenStream::from_str(&format!("compile_error!(\"{escaped}\");")).expect("compile_error! construction should always parse")
}

/// Parses generated source into a token stream, returning compile-error tokens on failure.
fn parse_tokens(source: &str) -> TokenStream {
    match TokenStream::from_str(source) {
        Ok(tokens) => tokens,
        Err(err) => compile_error_tokens(&format!("base_mod_macros parse failure: {err}")),
    }
}

/// Converts a `snake_case` module segment to `PascalCase`.
fn module_to_pascal(module: &str) -> String {
    let mut out = String::new();
    for segment in module.split('_') {
        let mut chars = segment.chars();
        if let Some(first) = chars.next() {
            out.push(first.to_ascii_uppercase());
            out.push_str(chars.as_str());
        }
    }
    out
}

/// Resolves the range-sample trait identifier for a module.
fn range_sample_trait_name(module: &str) -> String {
    format!("{}RangeSampleConstants", module_to_pascal(module))
}

/// Resolves the fully-qualified range-sample trait path for a module.
fn range_sample_trait_path(module: &str) -> String {
    format!("crate::math::scalar::constants::range_sample::{module}::{}", range_sample_trait_name(module))
}

/// Returns all fully-qualified scalar constants trait paths (core + range-sample).
fn all_constant_trait_paths() -> Vec<String> {
    let mut trait_paths: Vec<String> = CORE_CONSTANT_TRAITS.iter().map(ToString::to_string).collect();
    trait_paths.extend(RANGE_SAMPLE_MODULES.iter().map(|module| range_sample_trait_path(module)));
    trait_paths
}

/// Parses an optional trait name argument from the macro input.
fn parse_requested_trait_name(input: TokenStream) -> String {
    let requested_name = input.to_string();
    if requested_name.trim().is_empty() {
        "ScalarConstants".to_string()
    } else {
        requested_name.trim().to_string()
    }
}

/// Generates a master trait that blankets all scalar constants fragment traits.
pub(crate) fn declare_scalar_constants_trait(input: TokenStream) -> TokenStream {
    let trait_name = parse_requested_trait_name(input);
    let bounds = all_constant_trait_paths();
    let bounds_joined = bounds.join("\n    + ");
    let out = format!(
        "pub trait {trait_name}: {bounds_joined} {{}}

impl<T> {trait_name} for T
where
    T: {bounds_joined}
{{}}
"
    );

    parse_tokens(&out)
}

/// Generates impl blocks for every scalar constants fragment trait on a concrete type.
pub(crate) fn impl_scalar_constants_for(input: TokenStream) -> TokenStream {
    let ty = input.to_string();
    if ty.trim().is_empty() {
        return compile_error_tokens("impl_scalar_constants_for! expects a type, e.g. impl_scalar_constants_for!(UsfScalar)");
    }

    let mut out = String::new();
    for trait_path in all_constant_trait_paths() {
        out.push_str(&format!("impl {trait_path} for {ty} {{}}\n"));
    }
    parse_tokens(&out)
}
