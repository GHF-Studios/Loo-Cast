use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    Attribute, Data, DeriveInput, Expr, ExprRange, Fields, Generics, Ident, LitStr, Result, Type,
    spanned::Spanned,
};

/// The Cargo package name of the runtime crate.
///
/// This must match `[package].name` in the runtime crate's Cargo.toml.
const SPACETIME_ENGINE_CRATE: &str = "spacetime-engine";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AccessMode {
    ReadOnly,
    Direct,
    Validated,
    Transactional,
    Command,
}

impl AccessMode {
    fn variant(self, spacetime_engine: &TokenStream) -> TokenStream {
        let variant = match self {
            Self::ReadOnly => quote!(ReadOnly),
            Self::Direct => quote!(Direct),
            Self::Validated => quote!(Validated),
            Self::Transactional => quote!(Transactional),
            Self::Command => quote!(Command),
        };
        quote!(#spacetime_engine::devtools::InspectAccess::#variant)
    }
}

#[derive(Default)]
struct TypeAttributes {
    label: Option<LitStr>,
}

#[derive(Default)]
struct FieldAttributes {
    skip: bool,
    id: Option<LitStr>,
    label: Option<LitStr>,
    symbol: Option<LitStr>,
    unit: Option<LitStr>,
    hint: Option<LitStr>,
    role: Option<LitStr>,
    widget: Option<LitStr>,
    access: Option<AccessMode>,
    speed: Option<Expr>,
    range: Option<ExprRange>,
    slider: bool,
}

struct InspectField {
    ident: Ident,
    ty: Type,
    attributes: FieldAttributes,
}

pub struct Inspect {
    ident: Ident,
    label: LitStr,
    fields: Vec<InspectField>,
}

impl Inspect {
    pub fn parse(input: TokenStream) -> Result<Self> {
        let input = syn::parse2::<DeriveInput>(input)?;
        ensure_non_generic(&input.generics)?;

        let ident = input.ident.clone();
        let type_attributes = parse_type_attributes(&input.attrs)?;
        let label = type_attributes
            .label
            .unwrap_or_else(|| LitStr::new(&ident.to_string(), ident.span()));

        let fields = match input.data {
            Data::Struct(item) => match item.fields {
                Fields::Named(fields) => fields
                    .named
                    .into_iter()
                    .map(|field| {
                        let span = field.span();
                        let ident = field.ident.ok_or_else(|| {
                            syn::Error::new(span, "Inspect requires named struct fields")
                        })?;
                        let attributes = parse_field_attributes(&field.attrs)?;
                        Ok(InspectField {
                            ident,
                            ty: field.ty,
                            attributes,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?,
                fields => {
                    return Err(syn::Error::new_spanned(
                        fields,
                        "Inspect can currently only be derived for structs with named fields",
                    ));
                }
            },
            Data::Enum(item) => {
                return Err(syn::Error::new_spanned(
                    item.enum_token,
                    "Inspect can currently only be derived for structs with named fields",
                ));
            }
            Data::Union(item) => {
                return Err(syn::Error::new_spanned(
                    item.union_token,
                    "Inspect cannot be derived for unions",
                ));
            }
        };

        Ok(Self {
            ident,
            label,
            fields,
        })
    }

    pub fn generate(self) -> TokenStream {
        let Self {
            ident,
            label,
            fields,
        } = self;

        let spacetime_engine = match spacetime_engine_path() {
            Ok(path) => path,
            Err(error) => return error.into_compile_error(),
        };

        let visible_fields = fields
            .iter()
            .filter(|field| !field.attributes.skip)
            .collect::<Vec<_>>();

        let metadata = visible_fields
            .iter()
            .map(|field| field_metadata(field, &spacetime_engine));

        let immutable_visits = visible_fields.iter().enumerate().map(|(index, field)| {
            let field_ident = &field.ident;
            quote! {
                visitor.field(&metadata.fields[#index], &self.#field_ident);
            }
        });

        let mutable_visits = visible_fields.iter().enumerate().map(|(index, field)| {
            let field_ident = &field.ident;
            if field.attributes.access == Some(AccessMode::Direct) {
                quote! {
                    visitor.direct(&metadata.fields[#index], &mut self.#field_ident);
                }
            } else {
                quote! {
                    visitor.read_only(&metadata.fields[#index], &self.#field_ident);
                }
            }
        });

        quote! {
            impl #spacetime_engine::devtools::Inspect for #ident {
                fn inspect_type_metadata() -> &'static #spacetime_engine::devtools::InspectTypeMetadata {
                    static FIELDS: &[#spacetime_engine::devtools::InspectFieldMetadata] = &[
                        #(#metadata),*
                    ];
                    static METADATA: #spacetime_engine::devtools::InspectTypeMetadata =
                        #spacetime_engine::devtools::InspectTypeMetadata {
                            rust_name: concat!(module_path!(), "::", stringify!(#ident)),
                            label: #label,
                            fields: FIELDS,
                        };
                    &METADATA
                }

                fn visit_inspect_fields(
                    &self,
                    visitor: &mut dyn #spacetime_engine::devtools::InspectFieldVisitor,
                ) {
                    let metadata = <Self as #spacetime_engine::devtools::Inspect>::inspect_type_metadata();
                    #(#immutable_visits)*
                }

                fn visit_inspect_fields_mut(
                    &mut self,
                    visitor: &mut dyn #spacetime_engine::devtools::InspectFieldVisitorMut,
                ) {
                    let metadata = <Self as #spacetime_engine::devtools::Inspect>::inspect_type_metadata();
                    #(#mutable_visits)*
                }
            }
        }
    }
}

fn field_metadata(field: &InspectField, spacetime_engine: &TokenStream) -> TokenStream {
    let ident = &field.ident;
    let ty = &field.ty;
    let attributes = &field.attributes;

    let rust_name = LitStr::new(&ident.to_string(), ident.span());
    let id = attributes
        .id
        .clone()
        .unwrap_or_else(|| LitStr::new(&ident.to_string(), ident.span()));
    let label = attributes
        .label
        .clone()
        .unwrap_or_else(|| LitStr::new(&ident.to_string(), ident.span()));
    let access = attributes
        .access
        .unwrap_or(AccessMode::ReadOnly)
        .variant(spacetime_engine);

    let symbol = option_string(&attributes.symbol);
    let hint = option_string(&attributes.hint);
    let role = option_string(&attributes.role);
    let unit = attributes.unit.as_ref().map_or_else(
        || quote!(None),
        |unit| quote!(Some(#spacetime_engine::devtools::InspectUnit(#unit))),
    );
    let widget = attributes.widget.as_ref().map_or_else(
        || quote!(None),
        |widget| quote!(Some(#spacetime_engine::devtools::InspectWidgetId(#widget))),
    );
    let number_input = number_input(attributes, spacetime_engine);

    quote! {
        #spacetime_engine::devtools::InspectFieldMetadata {
            id: #spacetime_engine::devtools::InspectFieldId(#id),
            rust_name: #rust_name,
            rust_type_name: stringify!(#ty),
            label: #label,
            symbol: #symbol,
            access: #access,
            unit: #unit,
            hint: #hint,
            role: #role,
            widget: #widget,
            number_input: #number_input,
        }
    }
}

fn option_string(value: &Option<LitStr>) -> TokenStream {
    value
        .as_ref()
        .map_or_else(|| quote!(None), |value| quote!(Some(#value)))
}

fn number_input(attributes: &FieldAttributes, spacetime_engine: &TokenStream) -> TokenStream {
    if attributes.speed.is_none() && attributes.range.is_none() && !attributes.slider {
        return quote!(None);
    }

    let speed = attributes
        .speed
        .as_ref()
        .map_or_else(|| quote!(0.1_f64), |speed| quote!((#speed) as f64));

    let (minimum, maximum) = attributes.range.as_ref().map_or_else(
        || (quote!(None), quote!(None)),
        |range| {
            let minimum = range
                .start
                .as_ref()
                .map_or_else(|| quote!(None), |value| quote!(Some((#value) as f64)));
            let maximum = range
                .end
                .as_ref()
                .map_or_else(|| quote!(None), |value| quote!(Some((#value) as f64)));
            (minimum, maximum)
        },
    );
    let slider = attributes.slider;

    quote! {
        Some(#spacetime_engine::devtools::InspectNumberInput {
            speed: #speed,
            minimum: #minimum,
            maximum: #maximum,
            slider: #slider,
        })
    }
}

fn parse_type_attributes(attributes: &[Attribute]) -> Result<TypeAttributes> {
    let mut result = TypeAttributes::default();
    for attribute in attributes
        .iter()
        .filter(|attribute| attribute.path().is_ident("inspect"))
    {
        attribute.parse_nested_meta(|meta| {
            if meta.path.is_ident("label") {
                set_once(
                    &mut result.label,
                    meta.value()?.parse()?,
                    meta.path.span(),
                    "label",
                )
            } else {
                Err(meta
                    .error("unsupported #[inspect(...)] type option; expected `label = \"...\"`"))
            }
        })?;
    }
    Ok(result)
}

fn parse_field_attributes(attributes: &[Attribute]) -> Result<FieldAttributes> {
    let mut result = FieldAttributes::default();

    for attribute in attributes
        .iter()
        .filter(|attribute| attribute.path().is_ident("inspect"))
    {
        attribute.parse_nested_meta(|meta| {
            if meta.path.is_ident("skip") {
                result.skip = true;
                return Ok(());
            }
            if meta.path.is_ident("slider") {
                result.slider = true;
                return Ok(());
            }

            for (name, mode) in [
                ("read_only", AccessMode::ReadOnly),
                ("direct", AccessMode::Direct),
                ("validated", AccessMode::Validated),
                ("transactional", AccessMode::Transactional),
                ("command", AccessMode::Command),
            ] {
                if meta.path.is_ident(name) {
                    if let Some(previous) = result.access {
                        if previous != mode {
                            return Err(
                                meta.error("only one inspection access mode may be specified")
                            );
                        }
                    }
                    result.access = Some(mode);
                    return Ok(());
                }
            }

            macro_rules! string_value {
                ($name:literal, $target:expr) => {
                    if meta.path.is_ident($name) {
                        let value: LitStr = meta.value()?.parse()?;
                        return set_once($target, value, meta.path.span(), $name);
                    }
                };
            }

            string_value!("id", &mut result.id);
            string_value!("label", &mut result.label);
            string_value!("symbol", &mut result.symbol);
            string_value!("unit", &mut result.unit);
            string_value!("hint", &mut result.hint);
            string_value!("role", &mut result.role);
            string_value!("widget", &mut result.widget);

            if meta.path.is_ident("speed") {
                let value: Expr = meta.value()?.parse()?;
                return set_once(&mut result.speed, value, meta.path.span(), "speed");
            }
            if meta.path.is_ident("range") {
                let value: ExprRange = meta.value()?.parse()?;
                if value.start.is_none() || value.end.is_none() {
                    return Err(meta.error("inspection ranges require both a minimum and maximum"));
                }
                return set_once(&mut result.range, value, meta.path.span(), "range");
            }

            Err(meta.error(
                "unsupported #[inspect(...)] field option; expected skip, read_only, direct, \
                 validated, transactional, command, id, label, symbol, unit, hint, role, \
                 widget, speed, range, or slider",
            ))
        })?;
    }

    if result.skip {
        let has_other_options = result.id.is_some()
            || result.label.is_some()
            || result.symbol.is_some()
            || result.unit.is_some()
            || result.hint.is_some()
            || result.role.is_some()
            || result.widget.is_some()
            || result.access.is_some()
            || result.speed.is_some()
            || result.range.is_some()
            || result.slider;
        if has_other_options {
            return Err(syn::Error::new(
                Span::call_site(),
                "#[inspect(skip)] cannot be combined with other inspection options",
            ));
        }
    }

    Ok(result)
}

fn set_once<T>(slot: &mut Option<T>, value: T, span: Span, name: &str) -> Result<()> {
    if slot.is_some() {
        return Err(syn::Error::new(
            span,
            format!("duplicate #[inspect] option `{name}`"),
        ));
    }
    *slot = Some(value);
    Ok(())
}

fn ensure_non_generic(generics: &Generics) -> Result<()> {
    if generics.params.is_empty() {
        return Ok(());
    }

    Err(syn::Error::new_spanned(
        generics,
        "Inspect can currently only be derived for non-generic structs; implement Inspect manually for generic types",
    ))
}

fn spacetime_engine_path() -> Result<TokenStream> {
    match crate_name(SPACETIME_ENGINE_CRATE) {
        Ok(FoundCrate::Itself) => Ok(quote!(crate)),
        Ok(FoundCrate::Name(name)) => {
            let ident = Ident::new(&name, Span::call_site());
            Ok(quote!(::#ident))
        }
        Err(error) => Err(syn::Error::new(
            Span::call_site(),
            format!("could not resolve runtime crate `{SPACETIME_ENGINE_CRATE}`: {error}"),
        )),
    }
}
