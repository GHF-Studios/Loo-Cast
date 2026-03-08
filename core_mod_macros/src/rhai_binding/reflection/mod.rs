//! # VibeCoding Warning
//! Beware: This module has basically been fully vibe coded! View at your own discretion!

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::{
    Attribute, Expr, ExprPath, Ident, ImplItem, ImplItemFn, Item, ItemFn, ItemImpl, ItemTrait, LitStr, Path, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

fn path_to_string(path: &Path) -> String {
    path.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<_>>().join("::")
}

fn expr_path_to_string(path: &ExprPath) -> String {
    path.path.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<_>>().join("::")
}

fn type_path_to_string(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(tp) => tp.path.segments.iter().map(|s| s.ident.to_string()).collect::<Vec<_>>().join("::"),
        _ => panic!("Expected a path type"),
    }
}

fn trait_impl_path_string(self_ty: &syn::Type, trait_path: &Path) -> String {
    format!("<{} as {}>", type_path_to_string(self_ty), path_to_string(trait_path))
}

fn stable_hash(input: &str) -> u64 {
    // FNV-1a 64-bit
    let mut hash: u64 = 0xcbf29ce484222325;
    for b in input.as_bytes() {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn make_marker_ident(kind: &str, key: &str) -> Ident {
    let h = stable_hash(&format!("{}::{}", kind, key));
    format_ident!("__RB_{}_{}", kind, h)
}

fn make_static_ident(kind: &str, key: &str) -> Ident {
    let h = stable_hash(&format!("{}::{}", kind, key));
    format_ident!("__RB_STATIC_{}_{}", kind, h)
}

fn qualify(base_module: &str, raw: &str) -> String {
    if raw.contains("::") || raw.starts_with('<') {
        raw.to_string()
    } else {
        format!("{}::{}", base_module, raw)
    }
}

fn path_lit(path: &str) -> syn::LitStr {
    syn::LitStr::new(path, Span::call_site())
}

fn parse_single_path_attr(attr: TokenStream) -> Path {
    syn::parse(attr).expect("Failed to parse attribute argument as Path")
}

fn parse_single_expr_path_attr(attr: TokenStream) -> ExprPath {
    syn::parse(attr).expect("Failed to parse attribute argument as ExprPath")
}

fn get_attr_path(attr: &Attribute) -> Option<Path> {
    attr.parse_args::<Path>().ok()
}

fn attr_name(attr: &Attribute) -> Option<String> {
    attr.path().segments.last().map(|s| s.ident.to_string())
}

fn is_reflect_marker_attr(attr: &Attribute) -> bool {
    matches!(
        attr_name(attr).as_deref(),
        Some("reflect_constructor_function" | "reflect_method_function" | "reflect_item_associated_function" | "reflect_module_associated_function")
    )
}

fn strip_reflect_marker_attrs(mut f: ImplItemFn) -> ImplItemFn {
    f.attrs.retain(|a| !is_reflect_marker_attr(a));
    f
}

fn parse_id_paths(input: ParseStream) -> syn::Result<Vec<Path>> {
    let content;
    syn::bracketed!(content in input);
    let elems = Punctuated::<Path, Token![,]>::parse_terminated(&content)?;
    Ok(elems.into_iter().collect())
}

fn parse_lit_strings(input: ParseStream) -> syn::Result<Vec<LitStr>> {
    let content;
    syn::bracketed!(content in input);
    let elems = Punctuated::<LitStr, Token![,]>::parse_terminated(&content)?;
    Ok(elems.into_iter().collect())
}

fn collect_generic_type_param_metadata(type_generics: &syn::Generics) -> (Vec<String>, Vec<Vec<String>>) {
    let mut param_names = Vec::<String>::new();
    let mut bounds_by_param = std::collections::BTreeMap::<String, std::collections::BTreeSet<String>>::new();

    for generic_param in &type_generics.params {
        if let syn::GenericParam::Type(type_param) = generic_param {
            let param_name = type_param.ident.to_string();
            param_names.push(param_name.clone());
            bounds_by_param.entry(param_name.clone()).or_default();

            for bound in &type_param.bounds {
                if let syn::TypeParamBound::Trait(trait_bound) = bound {
                    bounds_by_param.entry(param_name.clone()).or_default().insert(path_to_string(&trait_bound.path));
                }
            }
        }
    }

    if let Some(where_clause) = &type_generics.where_clause {
        for predicate in &where_clause.predicates {
            let syn::WherePredicate::Type(type_predicate) = predicate else {
                continue;
            };

            let syn::Type::Path(type_path) = &type_predicate.bounded_ty else {
                continue;
            };

            if type_path.qself.is_some() {
                continue;
            }

            let Some(single_segment) = type_path.path.get_ident() else {
                continue;
            };

            let param_name = single_segment.to_string();
            if !bounds_by_param.contains_key(&param_name) {
                continue;
            }

            for bound in &type_predicate.bounds {
                if let syn::TypeParamBound::Trait(trait_bound) = bound {
                    bounds_by_param.entry(param_name.clone()).or_default().insert(path_to_string(&trait_bound.path));
                }
            }
        }
    }

    let param_trait_bounds = param_names
        .iter()
        .map(|name| bounds_by_param.get(name).map(|set| set.iter().cloned().collect::<Vec<_>>()).unwrap_or_default())
        .collect::<Vec<_>>();

    (param_names, param_trait_bounds)
}

fn infer_trait_dyn_safety(item_trait: &ItemTrait) -> (bool, Vec<String>) {
    let mut notes = Vec::<String>::new();

    for bound in &item_trait.supertraits {
        if let syn::TypeParamBound::Trait(trait_bound) = bound {
            let bound_path = path_to_string(&trait_bound.path);
            if bound_path.ends_with("::Sized") || bound_path == "Sized" {
                notes.push("Trait has supertrait `Sized`, therefore it is not dyn-safe".to_string());
            }
        }
    }

    for item in &item_trait.items {
        let syn::TraitItem::Fn(method) = item else {
            continue;
        };

        let method_name = method.sig.ident.to_string();

        if !method.sig.generics.params.is_empty() {
            notes.push(format!("Method `{method_name}` has generic parameters, therefore it is not object-safe"));
        }

        let has_receiver = method.sig.receiver().is_some();
        if !has_receiver {
            notes.push(format!(
                "Method `{method_name}` has no receiver, therefore it is not object-safe unless explicitly constrained to `Self: Sized`"
            ));
        }

        for input in &method.sig.inputs {
            if let syn::FnArg::Typed(typed) = input {
                let type_string = quote! { #typed.ty }.to_string();
                if type_string.contains("Self") {
                    notes.push(format!("Method `{method_name}` input references `Self`, therefore it is not object-safe"));
                }
            }
        }

        if let syn::ReturnType::Type(_, ty) = &method.sig.output {
            let type_string = quote! { #ty }.to_string();
            if type_string.contains("Self") {
                notes.push(format!("Method `{method_name}` return type references `Self`, therefore it is not object-safe"));
            }
        }
    }

    (notes.is_empty(), notes)
}

struct ModuleMacroInput {
    id: Path,
    sub_modules: Vec<Path>,
    traits: Vec<Path>,
    types: Vec<Path>,
    module_associated_functions: Vec<Path>,
}

impl Parse for ModuleMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut id: Option<Path> = None;
        let mut sub_modules: Option<Vec<Path>> = None;
        let mut traits_: Option<Vec<Path>> = None;
        let mut types_: Option<Vec<Path>> = None;
        let mut module_associated_functions: Option<Vec<Path>> = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "id" | "id_path" => {
                    id = Some(input.parse::<Path>()?);
                }
                "sub_modules" => {
                    sub_modules = Some(parse_id_paths(input)?);
                }
                "traits" => {
                    traits_ = Some(parse_id_paths(input)?);
                }
                "types" => {
                    types_ = Some(parse_id_paths(input)?);
                }
                "module_associated_functions" => {
                    module_associated_functions = Some(parse_id_paths(input)?);
                }
                other => {
                    return Err(syn::Error::new(key.span(), format!("Unknown key '{other}'")));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        let id = id.ok_or_else(|| syn::Error::new(Span::call_site(), "Missing `id`/`id_path`"))?;

        Ok(Self {
            id,
            sub_modules: sub_modules.unwrap_or_default(),
            traits: traits_.unwrap_or_default(),
            types: types_.unwrap_or_default(),
            module_associated_functions: module_associated_functions.unwrap_or_default(),
        })
    }
}

struct TraitImplAttr {
    self_ty: syn::Type,
    trait_path: Path,
}

#[derive(Clone, Copy)]
enum ReflectTypeValueSemantics {
    Clone,
    Owned,
    Ref,
    Mut,
    ScopedOwned,
    ScopedRef,
    ScopedMut,
}

struct ReflectTypeAttr {
    type_path: Path,
    value_semantics: ReflectTypeValueSemantics,
}

fn parse_value_semantics_ident(value: &Ident) -> syn::Result<ReflectTypeValueSemantics> {
    match value.to_string().as_str() {
        "clone" | "clone_on_move" => Ok(ReflectTypeValueSemantics::Clone),
        "owned" | "persistent_own" => Ok(ReflectTypeValueSemantics::Owned),
        "ref" | "persistent_ref" => Ok(ReflectTypeValueSemantics::Ref),
        "mut" | "persistent_mut" => Ok(ReflectTypeValueSemantics::Mut),
        "scoped_owned" => Ok(ReflectTypeValueSemantics::ScopedOwned),
        "scoped_ref" => Ok(ReflectTypeValueSemantics::ScopedRef),
        "scoped_mut" => Ok(ReflectTypeValueSemantics::ScopedMut),
        other => Err(syn::Error::new(
            value.span(),
            format!("Unknown value semantics '{other}'. Expected one of: clone, owned, ref, mut, scoped_owned, scoped_ref, scoped_mut"),
        )),
    }
}

impl Parse for ReflectTypeAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let type_path: Path = input.parse()?;
        let mut value_semantics = ReflectTypeValueSemantics::Clone;

        if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
        }

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "value_semantics" | "semantics" => {
                    let value: Ident = input.parse()?;
                    value_semantics = parse_value_semantics_ident(&value)?;
                }
                other => {
                    return Err(syn::Error::new(key.span(), format!("Unknown key '{other}' in #[reflect_type(...)]")));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self { type_path, value_semantics })
    }
}

struct ExternTypeMacroInput {
    id: Path,
    rust_type: syn::Type,
    value_semantics: ReflectTypeValueSemantics,
    method_functions: Vec<Path>,
    constructor_functions: Vec<Path>,
    item_associated_functions: Vec<Path>,
    registrator: Option<Expr>,
}

impl Parse for ExternTypeMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut id: Option<Path> = None;
        let mut rust_type: Option<syn::Type> = None;
        let mut value_semantics = ReflectTypeValueSemantics::Clone;
        let mut method_functions: Vec<Path> = vec![];
        let mut constructor_functions: Vec<Path> = vec![];
        let mut item_associated_functions: Vec<Path> = vec![];
        let mut registrator: Option<Expr> = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "id" | "id_path" => {
                    id = Some(input.parse::<Path>()?);
                }
                "rust_type" | "type" => {
                    rust_type = Some(input.parse::<syn::Type>()?);
                }
                "value_semantics" | "semantics" => {
                    let value: Ident = input.parse()?;
                    value_semantics = parse_value_semantics_ident(&value)?;
                }
                "method_functions" => {
                    method_functions = parse_id_paths(input)?;
                }
                "constructor_functions" => {
                    constructor_functions = parse_id_paths(input)?;
                }
                "item_associated_functions" => {
                    item_associated_functions = parse_id_paths(input)?;
                }
                "registrator" => {
                    registrator = Some(input.parse::<Expr>()?);
                }
                other => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!(
                            "Unknown key '{other}' in reflect_extern_type!. Expected: id, rust_type, value_semantics, method_functions, constructor_functions, item_associated_functions, registrator"
                        ),
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        let id = id.ok_or_else(|| syn::Error::new(Span::call_site(), "Missing `id`/`id_path`"))?;
        let rust_type = rust_type.ok_or_else(|| syn::Error::new(Span::call_site(), "Missing `rust_type`/`type`"))?;

        Ok(Self {
            id,
            rust_type,
            value_semantics,
            method_functions,
            constructor_functions,
            item_associated_functions,
            registrator,
        })
    }
}

struct ExternFunctionMacroInput {
    id: Path,
    registrator: Expr,
}

impl Parse for ExternFunctionMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut id: Option<Path> = None;
        let mut registrator: Option<Expr> = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "id" | "id_path" => {
                    id = Some(input.parse::<Path>()?);
                }
                "registrator" => {
                    registrator = Some(input.parse::<Expr>()?);
                }
                other => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!("Unknown key '{other}' in extern function macro. Expected: id, registrator"),
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            id: id.ok_or_else(|| syn::Error::new(Span::call_site(), "Missing `id`/`id_path`"))?,
            registrator: registrator.ok_or_else(|| syn::Error::new(Span::call_site(), "Missing `registrator`"))?,
        })
    }
}

struct ExternTraitMacroInput {
    id: Path,
    trait_name: Option<LitStr>,
    trait_object_name: Option<LitStr>,
    trait_object_id: Option<Path>,
    super_traits: Vec<Path>,
    is_dyn_safe: Option<bool>,
    object_safety_notes: Vec<LitStr>,
}

struct GenericBoundSpec {
    param: Ident,
    traits: Vec<Path>,
}

impl Parse for GenericBoundSpec {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let param: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let traits = parse_id_paths(input)?;

        Ok(Self { param, traits })
    }
}

fn parse_generic_bound_specs(input: ParseStream) -> syn::Result<Vec<GenericBoundSpec>> {
    let content;
    syn::bracketed!(content in input);
    let elems = Punctuated::<GenericBoundSpec, Token![,]>::parse_terminated(&content)?;
    Ok(elems.into_iter().collect())
}

struct ExternGenericDefinitionMacroInput {
    id: LitStr,
    owner_kind: Ident,
    params: Vec<Ident>,
    bounds: Vec<GenericBoundSpec>,
    notes: Vec<LitStr>,
}

impl Parse for ExternGenericDefinitionMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut id: Option<LitStr> = None;
        let mut owner_kind: Option<Ident> = None;
        let mut params: Vec<Ident> = vec![];
        let mut bounds: Vec<GenericBoundSpec> = vec![];
        let mut notes: Vec<LitStr> = vec![];

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "id" => {
                    id = Some(input.parse::<LitStr>()?);
                }
                "owner_kind" => {
                    owner_kind = Some(input.parse::<Ident>()?);
                }
                "params" => {
                    let content;
                    syn::bracketed!(content in input);
                    let elems = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?;
                    params = elems.into_iter().collect();
                }
                "bounds" => {
                    bounds = parse_generic_bound_specs(input)?;
                }
                "notes" => {
                    notes = parse_lit_strings(input)?;
                }
                other => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!("Unknown key '{other}' in reflect_extern_generic_definition!. Expected: id, owner_kind, params, bounds, notes"),
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            id: id.ok_or_else(|| syn::Error::new(Span::call_site(), "Missing `id`"))?,
            owner_kind: owner_kind.ok_or_else(|| syn::Error::new(Span::call_site(), "Missing `owner_kind`"))?,
            params,
            bounds,
            notes,
        })
    }
}

struct ExternGenericInstantiationMacroInput {
    id: LitStr,
    generic_id: LitStr,
    type_arguments: Vec<Path>,
    concrete_item_path: LitStr,
    value_semantics: Option<ReflectTypeValueSemantics>,
}

impl Parse for ExternGenericInstantiationMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut id: Option<LitStr> = None;
        let mut generic_id: Option<LitStr> = None;
        let mut type_arguments: Vec<Path> = vec![];
        let mut concrete_item_path: Option<LitStr> = None;
        let mut value_semantics: Option<ReflectTypeValueSemantics> = None;

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "id" => {
                    id = Some(input.parse::<LitStr>()?);
                }
                "generic_id" => {
                    generic_id = Some(input.parse::<LitStr>()?);
                }
                "type_arguments" => {
                    type_arguments = parse_id_paths(input)?;
                }
                "concrete_item_path" => {
                    concrete_item_path = Some(input.parse::<LitStr>()?);
                }
                "value_semantics" => {
                    value_semantics = Some(parse_value_semantics_ident(&input.parse::<Ident>()?)?);
                }
                other => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!(
                            "Unknown key '{other}' in reflect_extern_generic_instantiation!. Expected: id, generic_id, type_arguments, concrete_item_path, value_semantics"
                        ),
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            id: id.ok_or_else(|| syn::Error::new(Span::call_site(), "Missing `id`"))?,
            generic_id: generic_id.ok_or_else(|| syn::Error::new(Span::call_site(), "Missing `generic_id`"))?,
            type_arguments,
            concrete_item_path: concrete_item_path.ok_or_else(|| syn::Error::new(Span::call_site(), "Missing `concrete_item_path`"))?,
            value_semantics,
        })
    }
}

impl Parse for ExternTraitMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut id: Option<Path> = None;
        let mut trait_name: Option<LitStr> = None;
        let mut trait_object_name: Option<LitStr> = None;
        let mut trait_object_id: Option<Path> = None;
        let mut super_traits: Vec<Path> = vec![];
        let mut is_dyn_safe: Option<bool> = None;
        let mut object_safety_notes: Vec<LitStr> = vec![];

        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match key.to_string().as_str() {
                "id" | "id_path" => {
                    id = Some(input.parse::<Path>()?);
                }
                "trait_name" => {
                    trait_name = Some(input.parse::<LitStr>()?);
                }
                "trait_object_name" => {
                    trait_object_name = Some(input.parse::<LitStr>()?);
                }
                "trait_object_id" => {
                    trait_object_id = Some(input.parse::<Path>()?);
                }
                "super_traits" => {
                    super_traits = parse_id_paths(input)?;
                }
                "is_dyn_safe" => {
                    is_dyn_safe = Some(input.parse::<syn::LitBool>()?.value());
                }
                "object_safety_notes" => {
                    object_safety_notes = parse_lit_strings(input)?;
                }
                other => {
                    return Err(syn::Error::new(
                        key.span(),
                        format!(
                            "Unknown key '{other}' in reflect_extern_trait!. Expected: id, trait_name, trait_object_name, trait_object_id, super_traits, is_dyn_safe, object_safety_notes"
                        ),
                    ));
                }
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            id: id.ok_or_else(|| syn::Error::new(Span::call_site(), "Missing `id`/`id_path`"))?,
            trait_name,
            trait_object_name,
            trait_object_id,
            super_traits,
            is_dyn_safe,
            object_safety_notes,
        })
    }
}

impl ReflectTypeValueSemantics {
    fn as_tokens(self) -> TokenStream2 {
        match self {
            ReflectTypeValueSemantics::Clone => {
                quote! { crate::rhai_binding::value_semantics::modes::TypeValueSemantics::Clone }
            }
            ReflectTypeValueSemantics::Owned => {
                quote! { crate::rhai_binding::value_semantics::modes::TypeValueSemantics::Owned }
            }
            ReflectTypeValueSemantics::Ref => {
                quote! { crate::rhai_binding::value_semantics::modes::TypeValueSemantics::Ref }
            }
            ReflectTypeValueSemantics::Mut => {
                quote! { crate::rhai_binding::value_semantics::modes::TypeValueSemantics::Mut }
            }
            ReflectTypeValueSemantics::ScopedOwned => {
                quote! { crate::rhai_binding::value_semantics::modes::TypeValueSemantics::ScopedOwned }
            }
            ReflectTypeValueSemantics::ScopedRef => {
                quote! { crate::rhai_binding::value_semantics::modes::TypeValueSemantics::ScopedRef }
            }
            ReflectTypeValueSemantics::ScopedMut => {
                quote! { crate::rhai_binding::value_semantics::modes::TypeValueSemantics::ScopedMut }
            }
        }
    }

    fn as_const_tokens(self) -> TokenStream2 {
        match self {
            ReflectTypeValueSemantics::Clone => {
                quote! { crate::rhai_binding::value_semantics::modes::consts::CLONE }
            }
            ReflectTypeValueSemantics::Owned => {
                quote! { crate::rhai_binding::value_semantics::modes::consts::OWNED }
            }
            ReflectTypeValueSemantics::Ref => {
                quote! { crate::rhai_binding::value_semantics::modes::consts::REF }
            }
            ReflectTypeValueSemantics::Mut => {
                quote! { crate::rhai_binding::value_semantics::modes::consts::MUT }
            }
            ReflectTypeValueSemantics::ScopedOwned => {
                quote! { crate::rhai_binding::value_semantics::modes::consts::SCOPED_OWNED }
            }
            ReflectTypeValueSemantics::ScopedRef => {
                quote! { crate::rhai_binding::value_semantics::modes::consts::SCOPED_REF }
            }
            ReflectTypeValueSemantics::ScopedMut => {
                quote! { crate::rhai_binding::value_semantics::modes::consts::SCOPED_MUT }
            }
        }
    }
}

impl Parse for TraitImplAttr {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<Token![<]>()?;
        let self_ty: syn::Type = input.parse()?;
        input.parse::<Token![as]>()?;
        let trait_path: Path = input.parse()?;
        input.parse::<Token![>]>()?;

        if !input.is_empty() {
            return Err(input.error("Unexpected trailing tokens in trait-impl path"));
        }

        Ok(Self { self_ty, trait_path })
    }
}

fn parse_trait_impl_attr(attr: TokenStream) -> TraitImplAttr {
    syn::parse(attr).expect("Failed to parse attribute argument as `<Type as Trait>`")
}

fn get_attr_trait_impl_path(attr: &Attribute) -> Option<String> {
    attr.parse_args::<TraitImplAttr>()
        .ok()
        .map(|parsed| trait_impl_path_string(&parsed.self_ty, &parsed.trait_path))
}

fn generate_module_metadata(input: ModuleMacroInput, top_level: bool) -> TokenStream2 {
    let id_string = path_to_string(&input.id);
    let marker_ident = make_marker_ident(if top_level { "TOP" } else { "SUB" }, &id_string);
    let static_ident = make_static_ident(if top_level { "TOP" } else { "SUB" }, &id_string);

    let sub_modules: Vec<String> = input.sub_modules.iter().map(path_to_string).map(|p| qualify(&id_string, &p)).collect();
    let traits_: Vec<String> = input.traits.iter().map(path_to_string).map(|p| qualify(&id_string, &p)).collect();
    let types_: Vec<String> = input.types.iter().map(path_to_string).map(|p| qualify(&id_string, &p)).collect();
    let module_fns: Vec<String> = input
        .module_associated_functions
        .iter()
        .map(path_to_string)
        .map(|p| qualify(&id_string, &p))
        .collect();

    // One type-binding module per type by default.
    let type_binding_modules = types_.clone();

    let id_lit = path_lit(&id_string);
    let sub_module_lits: Vec<_> = sub_modules.iter().map(|s| path_lit(s)).collect();
    let trait_lits: Vec<_> = traits_.iter().map(|s| path_lit(s)).collect();
    let type_lits: Vec<_> = types_.iter().map(|s| path_lit(s)).collect();
    let type_binding_lits: Vec<_> = type_binding_modules.iter().map(|s| path_lit(s)).collect();
    let module_fn_lits: Vec<_> = module_fns.iter().map(|s| path_lit(s)).collect();

    if top_level {
        quote! {
            #[allow(non_upper_case_globals)]
            static #static_ident: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::module::TopLevelModuleMetadata>
                = crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#marker_ident as crate::rhai_binding::meta::generic::module::TopLevelModuleDynamicTypedMetadata>::from_comptime_to_runtime(&#marker_ident, &#marker_ident))
                );
            inventory::submit!(crate::rhai_binding::meta::registry::TopLevelModuleMetadataEntry(&#static_ident));

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            struct #marker_ident;

            impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #marker_ident {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }

            impl crate::rhai_binding::meta::generic::module::NativeModuleConstDynMetadata for #marker_ident {
                fn traits(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::trait_path::TraitPath>> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#trait_lits.into()),*]))
                }
                fn types(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::type_path::TypePath>> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#type_lits.into()),*]))
                }
                fn inherent_impls(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::impl_path::InherentImplPath>> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![]))
                }
                fn trait_impls(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::impl_path::TraitImplPath>> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![]))
                }
            }

            impl crate::rhai_binding::meta::generic::module::TopLevelModuleConstDynMetadata for #marker_ident {
                fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::module_path::TopLevelModulePath> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #id_lit.into()))
                }
                fn sub_modules(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::module_path::SubModulePath>> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#sub_module_lits.into()),*]))
                }
                fn type_binding_modules(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::module_path::TypeBindingModulePath>> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#type_binding_lits.into()),*]))
                }
                fn module_associated_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::ModuleAssociatedFunctionPath>> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#module_fn_lits.into()),*]))
                }
            }

            impl crate::rhai_binding::meta::generic::module::TopLevelModuleDynamicTypedMetadata for #marker_ident {}
        }
    } else {
        quote! {
            #[allow(non_upper_case_globals)]
            static #static_ident: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::module::SubModuleMetadata>
                = crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#marker_ident as crate::rhai_binding::meta::generic::module::SubModuleDynamicTypedMetadata>::from_comptime_to_runtime(&#marker_ident, &#marker_ident))
                );
            inventory::submit!(crate::rhai_binding::meta::registry::SubModuleMetadataEntry(&#static_ident));

            #[allow(non_camel_case_types)]
            #[derive(Clone, PartialEq, Eq, Hash)]
            struct #marker_ident;

            impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #marker_ident {
                fn raw_rust_module_path(&self) -> &'static str { module_path!() }
            }

            impl crate::rhai_binding::meta::generic::module::NativeModuleConstDynMetadata for #marker_ident {
                fn traits(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::trait_path::TraitPath>> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#trait_lits.into()),*]))
                }
                fn types(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::type_path::TypePath>> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#type_lits.into()),*]))
                }
                fn inherent_impls(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::impl_path::InherentImplPath>> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![]))
                }
                fn trait_impls(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::impl_path::TraitImplPath>> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![]))
                }
            }

            impl crate::rhai_binding::meta::generic::module::SubModuleConstDynMetadata for #marker_ident {
                fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::module_path::SubModulePath> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #id_lit.into()))
                }
                fn sub_modules(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::module_path::SubModulePath>> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#sub_module_lits.into()),*]))
                }
                fn type_binding_modules(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::module_path::TypeBindingModulePath>> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#type_binding_lits.into()),*]))
                }
                fn module_associated_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::ModuleAssociatedFunctionPath>> {
                    crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#module_fn_lits.into()),*]))
                }
            }

            impl crate::rhai_binding::meta::generic::module::SubModuleDynamicTypedMetadata for #marker_ident {}
        }
    }
}

pub fn reflect_top_level_module(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ModuleMacroInput);
    generate_module_metadata(input, true).into()
}

pub fn reflect_sub_module(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ModuleMacroInput);
    generate_module_metadata(input, false).into()
}

pub fn reflect_extern_top_level_module(input: TokenStream) -> TokenStream {
    reflect_top_level_module(input)
}

pub fn reflect_extern_sub_module(input: TokenStream) -> TokenStream {
    reflect_sub_module(input)
}

pub fn reflect_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_parsed = syn::parse::<ReflectTypeAttr>(attr).expect("Failed to parse #[reflect_type(...)] arguments");
    let type_path = attr_parsed.type_path;
    let type_path_string = path_to_string(&type_path);
    let type_path_lit = path_lit(&type_path_string);
    let value_semantics = attr_parsed.value_semantics.as_tokens();
    let value_semantics_const = attr_parsed.value_semantics.as_const_tokens();

    let item_parsed = parse_macro_input!(item as Item);
    let (item_tokens, type_ident, type_generics) = match &item_parsed {
        Item::Struct(s) => (quote! { #s }, s.ident.clone(), s.generics.clone()),
        Item::Enum(e) => (quote! { #e }, e.ident.clone(), e.generics.clone()),
        _ => {
            return syn::Error::new(Span::call_site(), "`#[reflect_type(..)]` only supports structs and enums")
                .to_compile_error()
                .into();
        }
    };
    let (impl_generics, ty_generics, where_clause) = type_generics.split_for_impl();
    let type_reflection_target = quote! { #type_ident #ty_generics };
    let type_registrator = if type_generics.params.is_empty() {
        quote! {
            crate::utils::clone_closure::CloneClosure::new(self.id_path().get().type_name().clone(), |name, parent_module| {
                parent_module.set_custom_type::<#type_ident>(&name);
            })
        }
    } else {
        quote! {
            crate::utils::clone_closure::CloneClosure::new(self.id_path().get().type_name().clone(), |_name, _parent_module| {
                // Generic declarations are reflected as metadata only.
                // Concrete monomorphized registrations are intentionally explicit.
            })
        }
    };
    let (generic_param_names_raw, generic_param_trait_bounds_raw) = collect_generic_type_param_metadata(&type_generics);
    let generic_param_name_lits: Vec<syn::LitStr> = generic_param_names_raw.iter().map(|name| path_lit(name)).collect();
    let generic_param_trait_bounds_tokens: Vec<TokenStream2> = generic_param_trait_bounds_raw
        .iter()
        .map(|bounds| {
            let bound_lits: Vec<syn::LitStr> = bounds.iter().map(|bound| path_lit(bound)).collect();
            quote! { vec![#(#bound_lits.into()),*] }
        })
        .collect();
    let generic_definition_id_expr = if generic_param_names_raw.is_empty() {
        quote! { None }
    } else {
        let generic_id = format!("{}<{}>", type_path_string, generic_param_names_raw.join(","));
        let generic_id_lit = path_lit(&generic_id);
        quote! { Some(rhai::ImmutableString::from(#generic_id_lit)) }
    };

    let type_marker = make_marker_ident("TYPE", &type_path_string);
    let type_static = make_static_ident("TYPE", &type_path_string);
    let binding_marker = make_marker_ident("TYPE_BINDING", &type_path_string);
    let binding_static = make_static_ident("TYPE_BINDING", &type_path_string);

    quote! {
        #item_tokens

        #[allow(non_upper_case_globals)]
        static #type_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::type_::TypeMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#type_marker as crate::rhai_binding::meta::generic::type_::TypeDynamicTypedMetadata>::from_comptime_to_runtime(&#type_marker, &#type_marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::TypeMetadataEntry(&#type_static));

        #[allow(non_upper_case_globals)]
        static #binding_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::module::TypeBindingModuleMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#binding_marker as crate::rhai_binding::meta::generic::module::TypeBindingModuleDynamicTypedMetadata>::from_comptime_to_runtime(&#binding_marker, &#binding_marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::TypeBindingModuleMetadataEntry(&#binding_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #type_marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #type_marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::abstract_::trait_identity::GetTypeId for #type_marker {
            const TYPE_ID: &'static str = #type_path_lit;
        }

        impl #impl_generics crate::rhai_binding::meta::abstract_::trait_identity::GetTypeId for #type_reflection_target #where_clause {
            const TYPE_ID: &'static str = #type_path_lit;
        }

        impl #impl_generics crate::rhai_binding::value_semantics::modes::GetTypeValueSemantics for #type_reflection_target #where_clause {
            const VALUE_SEMANTICS: crate::rhai_binding::value_semantics::modes::TypeValueSemantics = #value_semantics;
        }

        impl #impl_generics crate::rhai_binding::value_semantics::modes::HasTypeValueSemanticsConst<{ #value_semantics_const }> for #type_reflection_target #where_clause {}

        impl crate::rhai_binding::meta::generic::type_::TypeConstDynMetadata for #type_marker {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::type_path::TypePath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #type_path_lit.into()))
            }

            fn registrator(self) -> crate::utils::clone_closure::CloneClosure<rhai::ImmutableString, &'static mut rhai::Module, (), fn(rhai::ImmutableString, &mut rhai::Module)> {
                #type_registrator
            }

            fn method_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::MethodFunctionPath>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![]))
            }

            fn value_semantics(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::value_semantics::modes::TypeValueSemantics> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #value_semantics))
            }

            fn generic_definition_id(&self) -> crate::utils::clone_lazy::CloneLazy<Option<rhai::ImmutableString>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #generic_definition_id_expr))
            }

            fn generic_param_names(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<rhai::ImmutableString>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#generic_param_name_lits.into()),*]))
            }

            fn generic_param_trait_bounds(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<Vec<crate::rhai_binding::path::trait_path::TraitPath>>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#generic_param_trait_bounds_tokens),*]))
            }

            fn generic_instantiation_args(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<Vec<crate::rhai_binding::path::type_path::TypePath>>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![]))
            }
        }

        impl crate::rhai_binding::meta::generic::type_::TypeDynamicTypedMetadata for #type_marker {}

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #binding_marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #binding_marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::generic::module::TypeBindingModuleConstDynMetadata for #binding_marker {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::module_path::TypeBindingModulePath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #type_path_lit.into()))
            }

            fn item_associated_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::ItemAssociatedFunctionPath>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![]))
            }

            fn constructor_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::ConstructorFunctionPath>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![]))
            }

            fn method_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::MethodFunctionPath>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![]))
            }
        }

        impl crate::rhai_binding::meta::generic::module::TypeBindingModuleDynamicTypedMetadata for #binding_marker {}
    }
    .into()
}

pub fn reflect_extern_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExternTypeMacroInput);

    let type_path_string = path_to_string(&input.id);
    let type_path_lit = path_lit(&type_path_string);
    let value_semantics = input.value_semantics.as_tokens();
    let value_semantics_const = input.value_semantics.as_const_tokens();
    let rust_type = input.rust_type;

    let type_marker = make_marker_ident("TYPE", &type_path_string);
    let type_static = make_static_ident("TYPE", &type_path_string);
    let binding_marker = make_marker_ident("TYPE_BINDING", &type_path_string);
    let binding_static = make_static_ident("TYPE_BINDING", &type_path_string);

    let method_lits: Vec<_> = input.method_functions.iter().map(path_to_string).map(|s| path_lit(&s)).collect();
    let ctor_lits: Vec<_> = input.constructor_functions.iter().map(path_to_string).map(|s| path_lit(&s)).collect();
    let item_lits: Vec<_> = input.item_associated_functions.iter().map(path_to_string).map(|s| path_lit(&s)).collect();

    let type_registrator = if let Some(registrator) = input.registrator {
        quote! {
            crate::utils::clone_closure::CloneClosure::new(self.id_path().get().type_name().clone(), |name, parent_module| {
                (#registrator)(name, parent_module);
            })
        }
    } else {
        quote! {
            crate::utils::clone_closure::CloneClosure::new(self.id_path().get().type_name().clone(), |name, parent_module| {
                parent_module.set_custom_type::<#rust_type>(&name);
            })
        }
    };

    quote! {
        #[allow(non_upper_case_globals)]
        static #type_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::type_::TypeMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#type_marker as crate::rhai_binding::meta::generic::type_::TypeDynamicTypedMetadata>::from_comptime_to_runtime(&#type_marker, &#type_marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::TypeMetadataEntry(&#type_static));

        #[allow(non_upper_case_globals)]
        static #binding_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::module::TypeBindingModuleMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#binding_marker as crate::rhai_binding::meta::generic::module::TypeBindingModuleDynamicTypedMetadata>::from_comptime_to_runtime(&#binding_marker, &#binding_marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::TypeBindingModuleMetadataEntry(&#binding_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #type_marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #type_marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::abstract_::trait_identity::GetTypeId for #type_marker {
            const TYPE_ID: &'static str = #type_path_lit;
        }

        impl crate::rhai_binding::meta::abstract_::trait_identity::GetTypeId for #rust_type {
            const TYPE_ID: &'static str = #type_path_lit;
        }

        impl crate::rhai_binding::value_semantics::modes::GetTypeValueSemantics for #rust_type {
            const VALUE_SEMANTICS: crate::rhai_binding::value_semantics::modes::TypeValueSemantics = #value_semantics;
        }

        impl crate::rhai_binding::value_semantics::modes::HasTypeValueSemanticsConst<{ #value_semantics_const }> for #rust_type {}

        impl crate::rhai_binding::meta::generic::type_::TypeConstDynMetadata for #type_marker {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::type_path::TypePath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #type_path_lit.into()))
            }

            fn registrator(self) -> crate::utils::clone_closure::CloneClosure<rhai::ImmutableString, &'static mut rhai::Module, (), fn(rhai::ImmutableString, &mut rhai::Module)> {
                #type_registrator
            }

            fn method_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::MethodFunctionPath>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#method_lits.into()),*]))
            }

            fn value_semantics(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::value_semantics::modes::TypeValueSemantics> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #value_semantics))
            }

            fn generic_definition_id(&self) -> crate::utils::clone_lazy::CloneLazy<Option<rhai::ImmutableString>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| None))
            }

            fn generic_param_names(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<rhai::ImmutableString>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![]))
            }

            fn generic_param_trait_bounds(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<Vec<crate::rhai_binding::path::trait_path::TraitPath>>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![]))
            }

            fn generic_instantiation_args(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<Vec<crate::rhai_binding::path::type_path::TypePath>>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![]))
            }
        }

        impl crate::rhai_binding::meta::generic::type_::TypeDynamicTypedMetadata for #type_marker {}

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #binding_marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #binding_marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::generic::module::TypeBindingModuleConstDynMetadata for #binding_marker {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::module_path::TypeBindingModulePath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #type_path_lit.into()))
            }

            fn item_associated_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::ItemAssociatedFunctionPath>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#item_lits.into()),*]))
            }

            fn constructor_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::ConstructorFunctionPath>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#ctor_lits.into()),*]))
            }

            fn method_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::MethodFunctionPath>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#method_lits.into()),*]))
            }
        }

        impl crate::rhai_binding::meta::generic::module::TypeBindingModuleDynamicTypedMetadata for #binding_marker {}
    }
    .into()
}

pub fn reflect_extern_module_associated_function(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExternFunctionMacroInput);
    let id_path_string = path_to_string(&input.id);
    let id_lit = path_lit(&id_path_string);
    let fn_name = id_path_string
        .rsplit("::")
        .next()
        .expect("Module-associated function id path must include a function segment");
    let fn_name_lit = path_lit(fn_name);
    let registrator = input.registrator;

    let marker = make_marker_ident("MODULE_FN", &id_path_string);
    let marker_static = make_static_ident("MODULE_FN", &id_path_string);

    quote! {
        #[allow(non_upper_case_globals)]
        static #marker_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::function::ModuleAssociatedFunctionMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#marker as crate::rhai_binding::meta::generic::function::ModuleAssociatedFunctionDynamicTypedMetadata>::from_comptime_to_runtime(&#marker, &#marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::ModuleAssociatedFunctionMetadataEntry(&#marker_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::generic::function::ModuleAssociatedFunctionConstDynMetadata for #marker {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::function_path::ModuleAssociatedFunctionPath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #id_lit.into()))
            }
            fn registrator(self) -> crate::utils::clone_closure::CloneClosure<rhai::ImmutableString, &'static mut rhai::Module, (), fn(rhai::ImmutableString, &mut rhai::Module)> {
                crate::utils::clone_closure::CloneClosure::new(#fn_name_lit.into(), |name, parent_module| {
                    (#registrator)(name, parent_module);
                })
            }
        }

        impl crate::rhai_binding::meta::generic::function::ModuleAssociatedFunctionDynamicTypedMetadata for #marker {}
    }
    .into()
}

pub fn reflect_extern_constructor_function(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExternFunctionMacroInput);
    let id_path_string = path_to_string(&input.id);
    let id_lit = path_lit(&id_path_string);
    let fn_name = id_path_string
        .rsplit("::")
        .next()
        .expect("Constructor function id path must include a function segment");
    let fn_name_lit = path_lit(fn_name);
    let registrator = input.registrator;

    let marker = make_marker_ident("CTOR_FN", &id_path_string);
    let marker_static = make_static_ident("CTOR_FN", &id_path_string);

    quote! {
        #[allow(non_upper_case_globals)]
        static #marker_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::function::ConstructorFunctionMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#marker as crate::rhai_binding::meta::generic::function::ConstructorFunctionDynamicTypedMetadata>::from_comptime_to_runtime(&#marker, &#marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::ConstructorFunctionMetadataEntry(&#marker_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::generic::function::ConstructorFunctionConstDynMetadata for #marker {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::function_path::ConstructorFunctionPath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #id_lit.into()))
            }
            fn registrator(self) -> crate::utils::clone_closure::CloneClosure<rhai::ImmutableString, &'static mut rhai::Module, (), fn(rhai::ImmutableString, &mut rhai::Module)> {
                crate::utils::clone_closure::CloneClosure::new(#fn_name_lit.into(), |name, parent_module| {
                    (#registrator)(name, parent_module);
                })
            }
        }

        impl crate::rhai_binding::meta::generic::function::ConstructorFunctionDynamicTypedMetadata for #marker {}
    }
    .into()
}

pub fn reflect_extern_method_function(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExternFunctionMacroInput);
    let id_path_string = path_to_string(&input.id);
    let id_lit = path_lit(&id_path_string);
    let fn_name = id_path_string.rsplit("::").next().expect("Method id path must include a function segment");
    let fn_name_lit = path_lit(fn_name);
    let registrator = input.registrator;

    let marker = make_marker_ident("METHOD_FN", &id_path_string);
    let marker_static = make_static_ident("METHOD_FN", &id_path_string);

    quote! {
        #[allow(non_upper_case_globals)]
        static #marker_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::function::MethodFunctionMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#marker as crate::rhai_binding::meta::generic::function::MethodFunctionDynamicTypedMetadata>::from_comptime_to_runtime(&#marker, &#marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::MethodFunctionMetadataEntry(&#marker_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::generic::function::MethodFunctionConstDynMetadata for #marker {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::function_path::MethodFunctionPath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #id_lit.into()))
            }
            fn registrator(self) -> crate::utils::clone_closure::CloneClosure<rhai::ImmutableString, &'static mut rhai::Engine, (), fn(rhai::ImmutableString, &mut rhai::Engine)> {
                crate::utils::clone_closure::CloneClosure::new(#fn_name_lit.into(), |name, engine| {
                    (#registrator)(name, engine);
                })
            }
        }

        impl crate::rhai_binding::meta::generic::function::MethodFunctionDynamicTypedMetadata for #marker {}
    }
    .into()
}

pub fn reflect_extern_item_associated_function(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExternFunctionMacroInput);
    let id_path_string = path_to_string(&input.id);
    let id_lit = path_lit(&id_path_string);
    let fn_name = id_path_string
        .rsplit("::")
        .next()
        .expect("Item associated function id path must include a function segment");
    let fn_name_lit = path_lit(fn_name);
    let registrator = input.registrator;

    let marker = make_marker_ident("ITEM_FN", &id_path_string);
    let marker_static = make_static_ident("ITEM_FN", &id_path_string);

    quote! {
        #[allow(non_upper_case_globals)]
        static #marker_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::function::ItemAssociatedFunctionMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#marker as crate::rhai_binding::meta::generic::function::ItemAssociatedFunctionDynamicTypedMetadata>::from_comptime_to_runtime(&#marker, &#marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::ItemAssociatedFunctionMetadataEntry(&#marker_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::generic::function::ItemAssociatedFunctionConstDynMetadata for #marker {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::function_path::ItemAssociatedFunctionPath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #id_lit.into()))
            }
            fn registrator(self) -> crate::utils::clone_closure::CloneClosure<rhai::ImmutableString, &'static mut rhai::Module, (), fn(rhai::ImmutableString, &mut rhai::Module)> {
                crate::utils::clone_closure::CloneClosure::new(#fn_name_lit.into(), |name, parent_module| {
                    (#registrator)(name, parent_module);
                })
            }
        }

        impl crate::rhai_binding::meta::generic::function::ItemAssociatedFunctionDynamicTypedMetadata for #marker {}
    }
    .into()
}

pub fn reflect_extern_trait(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExternTraitMacroInput);

    let trait_path_string = path_to_string(&input.id);
    let trait_path_lit = path_lit(&trait_path_string);

    let default_trait_name = trait_path_string
        .rsplit("::")
        .next()
        .expect("Trait path must have at least one segment")
        .to_string();
    let trait_name_lit = input.trait_name.unwrap_or_else(|| path_lit(&default_trait_name));

    let default_trait_object_name = format!("{default_trait_name}TraitObject");
    let trait_object_name_lit = input.trait_object_name.unwrap_or_else(|| path_lit(&default_trait_object_name));

    let default_module_path = trait_path_string.rsplit_once("::").map(|(m, _)| m.to_string()).unwrap_or_default();
    let default_trait_object_id = if default_module_path.is_empty() {
        default_trait_object_name
    } else {
        format!("{default_module_path}::{default_trait_object_name}")
    };
    let trait_object_id_string = input.trait_object_id.as_ref().map(path_to_string).unwrap_or(default_trait_object_id);
    let trait_object_id_lit = path_lit(&trait_object_id_string);
    let super_trait_lits: Vec<LitStr> = input.super_traits.iter().map(path_to_string).map(|s| path_lit(&s)).collect();
    let is_dyn_safe = input.is_dyn_safe.unwrap_or(true);
    let object_safety_note_lits: Vec<LitStr> = input.object_safety_notes.clone();

    let marker_ident = make_marker_ident("TRAIT", &trait_path_string);
    let marker_static = make_static_ident("TRAIT", &trait_path_string);
    let object_marker_ident = make_marker_ident("TRAIT_OBJECT", &trait_path_string);
    let object_marker_static = make_static_ident("TRAIT_OBJECT", &trait_path_string);

    quote! {
        #[allow(non_upper_case_globals)]
        static #marker_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::trait_::TraitMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#marker_ident as crate::rhai_binding::meta::generic::trait_::TraitDynamicTypedMetadata>::from_comptime_to_runtime(&#marker_ident, &#marker_ident))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::TraitMetadataEntry(&#marker_static));

        #[allow(non_upper_case_globals)]
        static #object_marker_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::trait_::TraitObjectMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#object_marker_ident as crate::rhai_binding::meta::generic::trait_::TraitObjectDynamicTypedMetadata>::from_comptime_to_runtime(&#object_marker_ident, &#object_marker_ident))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::TraitObjectMetadataEntry(&#object_marker_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #marker_ident;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #marker_ident {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::abstract_::trait_identity::DynGetTraitName for #marker_ident {
            fn trait_name(&self) -> &'static str { #trait_name_lit }
        }

        impl crate::rhai_binding::meta::abstract_::trait_identity::GetTraitId for #marker_ident {
            const TRAIT_ID: &'static str = #trait_path_lit;
        }

        impl crate::rhai_binding::meta::generic::trait_::TraitConstDynMetadata for #marker_ident {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::trait_path::TraitPath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #trait_path_lit.into()))
            }
            fn super_traits(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::trait_path::TraitPath>> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#super_trait_lits.into()),*])
                )
            }
            fn is_dyn_safe(&self) -> crate::utils::clone_lazy::CloneLazy<bool> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| #is_dyn_safe)
                )
            }
            fn object_safety_notes(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<rhai::ImmutableString>> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#object_safety_note_lits.into()),*])
                )
            }
        }

        impl crate::rhai_binding::meta::generic::trait_::TraitDynamicTypedMetadata for #marker_ident {}

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #object_marker_ident;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #object_marker_ident {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::abstract_::trait_identity::DynGetTraitObjectName for #object_marker_ident {
            fn trait_object_name(&self) -> &'static str { #trait_object_name_lit }
        }

        impl crate::rhai_binding::meta::abstract_::trait_identity::GetTraitObjectId for #marker_ident {
            const TRAIT_OBJECT_ID: &'static str = #trait_object_id_lit;
        }

        impl crate::rhai_binding::meta::generic::trait_::TraitObjectConstDynMetadata for #object_marker_ident {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::trait_path::TraitPath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #trait_path_lit.into()))
            }
        }

        impl crate::rhai_binding::meta::generic::trait_::TraitObjectDynamicTypedMetadata for #object_marker_ident {}
    }
    .into()
}

pub fn reflect_extern_generic_definition(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExternGenericDefinitionMacroInput);

    let generic_id_lit = input.id;
    let owner_kind = match input.owner_kind.to_string().as_str() {
        "type" => quote! { crate::rhai_binding::meta::generic::generic_::GenericOwnerKind::Type },
        "r#type" => quote! { crate::rhai_binding::meta::generic::generic_::GenericOwnerKind::Type },
        "function" => quote! { crate::rhai_binding::meta::generic::generic_::GenericOwnerKind::Function },
        "method" => quote! { crate::rhai_binding::meta::generic::generic_::GenericOwnerKind::Method },
        other => {
            return syn::Error::new(
                input.owner_kind.span(),
                format!("Unknown owner_kind '{other}'. Expected one of: type, function, method"),
            )
            .to_compile_error()
            .into();
        }
    };
    let param_lits: Vec<LitStr> = input.params.iter().map(|param| path_lit(&param.to_string())).collect();
    let note_lits: Vec<LitStr> = input.notes.clone();

    let mut bounds_by_param = std::collections::BTreeMap::<String, Vec<LitStr>>::new();
    for param in &input.params {
        bounds_by_param.insert(param.to_string(), Vec::new());
    }
    for bound_spec in input.bounds {
        let param_name = bound_spec.param.to_string();
        if !bounds_by_param.contains_key(&param_name) {
            return syn::Error::new(bound_spec.param.span(), format!("Bound specified for unknown generic parameter '{param_name}'"))
                .to_compile_error()
                .into();
        }
        let bound_lits = bound_spec.traits.iter().map(path_to_string).map(|s| path_lit(&s)).collect::<Vec<_>>();
        bounds_by_param.entry(param_name).or_default().extend(bound_lits);
    }
    let bound_vec_tokens: Vec<TokenStream2> = input
        .params
        .iter()
        .map(|param| {
            let param_name = param.to_string();
            let bound_lits = bounds_by_param.get(&param_name).cloned().unwrap_or_default();
            quote! { vec![#(#bound_lits.into()),*] }
        })
        .collect();

    let marker = make_marker_ident("GENERIC_DEF", &generic_id_lit.value());
    let marker_static = make_static_ident("GENERIC_DEF", &generic_id_lit.value());

    quote! {
        #[allow(non_upper_case_globals)]
        static #marker_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::generic_::GenericDefinitionMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#marker as crate::rhai_binding::meta::generic::generic_::GenericDefinitionDynamicTypedMetadata>::from_comptime_to_runtime(&#marker, &#marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::GenericDefinitionMetadataEntry(&#marker_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::generic::generic_::GenericDefinitionConstDynMetadata for #marker {
            fn id(&self) -> crate::utils::clone_lazy::CloneLazy<rhai::ImmutableString> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| rhai::ImmutableString::from(#generic_id_lit))
                )
            }
            fn owner_kind(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::generic::generic_::GenericOwnerKind> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| #owner_kind)
                )
            }
            fn params(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<rhai::ImmutableString>> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#param_lits.into()),*])
                )
            }
            fn param_trait_bounds(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<Vec<crate::rhai_binding::path::trait_path::TraitPath>>> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#bound_vec_tokens),*])
                )
            }
            fn notes(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<rhai::ImmutableString>> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#note_lits.into()),*])
                )
            }
        }

        impl crate::rhai_binding::meta::generic::generic_::GenericDefinitionDynamicTypedMetadata for #marker {}
    }
    .into()
}

pub fn reflect_extern_generic_instantiation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ExternGenericInstantiationMacroInput);

    let instantiation_id_lit = input.id;
    let generic_id_lit = input.generic_id;
    let type_argument_lits: Vec<LitStr> = input.type_arguments.iter().map(path_to_string).map(|s| path_lit(&s)).collect();
    let concrete_item_path_lit = input.concrete_item_path;
    let value_semantics_expr = if let Some(semantics) = input.value_semantics {
        let semantics_tokens = semantics.as_tokens();
        quote! { Some(#semantics_tokens) }
    } else {
        quote! { None }
    };

    let marker = make_marker_ident("GENERIC_INST", &instantiation_id_lit.value());
    let marker_static = make_static_ident("GENERIC_INST", &instantiation_id_lit.value());

    quote! {
        #[allow(non_upper_case_globals)]
        static #marker_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::generic_::GenericInstantiationMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#marker as crate::rhai_binding::meta::generic::generic_::GenericInstantiationDynamicTypedMetadata>::from_comptime_to_runtime(&#marker, &#marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::GenericInstantiationMetadataEntry(&#marker_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::generic::generic_::GenericInstantiationConstDynMetadata for #marker {
            fn id(&self) -> crate::utils::clone_lazy::CloneLazy<rhai::ImmutableString> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| rhai::ImmutableString::from(#instantiation_id_lit))
                )
            }
            fn generic_id(&self) -> crate::utils::clone_lazy::CloneLazy<rhai::ImmutableString> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| rhai::ImmutableString::from(#generic_id_lit))
                )
            }
            fn type_arguments(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::type_path::TypePath>> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#type_argument_lits.into()),*])
                )
            }
            fn concrete_item_path(&self) -> crate::utils::clone_lazy::CloneLazy<rhai::ImmutableString> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| rhai::ImmutableString::from(#concrete_item_path_lit))
                )
            }
            fn value_semantics(&self) -> crate::utils::clone_lazy::CloneLazy<Option<crate::rhai_binding::value_semantics::modes::TypeValueSemantics>> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| #value_semantics_expr)
                )
            }
        }

        impl crate::rhai_binding::meta::generic::generic_::GenericInstantiationDynamicTypedMetadata for #marker {}
    }
    .into()
}

pub fn reflect_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    let trait_path = parse_single_path_attr(attr);
    let trait_path_string = path_to_string(&trait_path);
    let trait_path_lit = path_lit(&trait_path_string);

    let item_trait = parse_macro_input!(item as ItemTrait);
    let trait_ident = item_trait.ident.clone();
    let trait_name_lit = path_lit(&trait_ident.to_string());
    let trait_module_path = trait_path_string.rsplit_once("::").map(|(m, _)| m.to_string()).unwrap_or_default();
    let super_trait_lits: Vec<LitStr> = item_trait
        .supertraits
        .iter()
        .filter_map(|bound| {
            let syn::TypeParamBound::Trait(trait_bound) = bound else {
                return None;
            };
            let raw = path_to_string(&trait_bound.path);
            Some(path_lit(&qualify(&trait_module_path, &raw)))
        })
        .collect();
    let (is_dyn_safe, object_safety_notes_raw) = infer_trait_dyn_safety(&item_trait);
    let object_safety_note_lits: Vec<LitStr> = object_safety_notes_raw.iter().map(|note| path_lit(note)).collect();

    let marker_ident = make_marker_ident("TRAIT", &trait_path_string);
    let marker_static = make_static_ident("TRAIT", &trait_path_string);

    let object_marker_ident = make_marker_ident("TRAIT_OBJECT", &trait_path_string);
    let object_marker_static = make_static_ident("TRAIT_OBJECT", &trait_path_string);

    let module_path = trait_path_string.rsplit_once("::").map(|(m, _)| m.to_string()).unwrap_or_else(|| String::new());
    let trait_object_id = if module_path.is_empty() {
        format!("{}TraitObject", trait_ident)
    } else {
        format!("{}::{}TraitObject", module_path, trait_ident)
    };
    let trait_object_id_lit = path_lit(&trait_object_id);
    let trait_object_name_lit = path_lit(&format!("{}TraitObject", trait_ident));

    quote! {
        #item_trait

        #[allow(non_upper_case_globals)]
        static #marker_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::trait_::TraitMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#marker_ident as crate::rhai_binding::meta::generic::trait_::TraitDynamicTypedMetadata>::from_comptime_to_runtime(&#marker_ident, &#marker_ident))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::TraitMetadataEntry(&#marker_static));

        #[allow(non_upper_case_globals)]
        static #object_marker_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::trait_::TraitObjectMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#object_marker_ident as crate::rhai_binding::meta::generic::trait_::TraitObjectDynamicTypedMetadata>::from_comptime_to_runtime(&#object_marker_ident, &#object_marker_ident))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::TraitObjectMetadataEntry(&#object_marker_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #marker_ident;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #marker_ident {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::abstract_::trait_identity::DynGetTraitName for #marker_ident {
            fn trait_name(&self) -> &'static str { #trait_name_lit }
        }

        impl crate::rhai_binding::meta::abstract_::trait_identity::GetTraitId for #marker_ident {
            const TRAIT_ID: &'static str = #trait_path_lit;
        }

        impl crate::rhai_binding::meta::generic::trait_::TraitConstDynMetadata for #marker_ident {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::trait_path::TraitPath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #trait_path_lit.into()))
            }
            fn super_traits(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::trait_path::TraitPath>> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#super_trait_lits.into()),*])
                )
            }
            fn is_dyn_safe(&self) -> crate::utils::clone_lazy::CloneLazy<bool> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| #is_dyn_safe)
                )
            }
            fn object_safety_notes(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<rhai::ImmutableString>> {
                crate::utils::clone_lazy::CloneLazy::new(
                    crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#object_safety_note_lits.into()),*])
                )
            }
        }

        impl crate::rhai_binding::meta::generic::trait_::TraitDynamicTypedMetadata for #marker_ident {}

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #object_marker_ident;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #object_marker_ident {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::abstract_::trait_identity::DynGetTraitObjectName for #object_marker_ident {
            fn trait_object_name(&self) -> &'static str { #trait_object_name_lit }
        }

        impl crate::rhai_binding::meta::abstract_::trait_identity::GetTraitObjectId for #marker_ident {
            const TRAIT_OBJECT_ID: &'static str = #trait_object_id_lit;
        }

        impl crate::rhai_binding::meta::generic::trait_::TraitObjectConstDynMetadata for #object_marker_ident {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::trait_path::TraitPath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #trait_path_lit.into()))
            }
        }

        impl crate::rhai_binding::meta::generic::trait_::TraitObjectDynamicTypedMetadata for #object_marker_ident {}
    }
    .into()
}

fn generate_module_associated_function_metadata(id_path_string: String, fn_ident: &Ident) -> TokenStream2 {
    let id_lit = path_lit(&id_path_string);
    let fn_name_lit = path_lit(&fn_ident.to_string());

    let marker = make_marker_ident("MODULE_FN", &id_path_string);
    let marker_static = make_static_ident("MODULE_FN", &id_path_string);

    quote! {
        #[allow(non_upper_case_globals)]
        static #marker_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::function::ModuleAssociatedFunctionMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#marker as crate::rhai_binding::meta::generic::function::ModuleAssociatedFunctionDynamicTypedMetadata>::from_comptime_to_runtime(&#marker, &#marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::ModuleAssociatedFunctionMetadataEntry(&#marker_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::generic::function::ModuleAssociatedFunctionConstDynMetadata for #marker {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::function_path::ModuleAssociatedFunctionPath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #id_lit.into()))
            }
            fn registrator(self) -> crate::utils::clone_closure::CloneClosure<rhai::ImmutableString, &'static mut rhai::Module, (), fn(rhai::ImmutableString, &mut rhai::Module)> {
                crate::utils::clone_closure::CloneClosure::new(#fn_name_lit.into(), |name, parent_module| {
                    rhai::FuncRegistration::new(name).set_into_module(parent_module, #fn_ident);
                })
            }
        }

        impl crate::rhai_binding::meta::generic::function::ModuleAssociatedFunctionDynamicTypedMetadata for #marker {}
    }
}

pub fn reflect_module_associated_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_path = parse_single_expr_path_attr(attr);
    let item_fn = parse_macro_input!(item as ItemFn);
    let fn_ident = item_fn.sig.ident.clone();

    let mut id_path = expr_path_to_string(&attr_path);
    if !id_path.ends_with(&format!("::{}", fn_ident)) && id_path != fn_ident.to_string() {
        id_path = format!("{}::{}", id_path, fn_ident);
    }

    let meta = generate_module_associated_function_metadata(id_path, &fn_ident);
    quote! {
        #item_fn
        #meta
    }
    .into()
}

fn generate_constructor_metadata(id_path_string: String, fn_name: &str, function_expr: TokenStream2) -> TokenStream2 {
    let id_lit = path_lit(&id_path_string);
    let fn_name_lit = path_lit(fn_name);

    let marker = make_marker_ident("CTOR_FN", &id_path_string);
    let marker_static = make_static_ident("CTOR_FN", &id_path_string);

    quote! {
        #[allow(non_upper_case_globals)]
        static #marker_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::function::ConstructorFunctionMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#marker as crate::rhai_binding::meta::generic::function::ConstructorFunctionDynamicTypedMetadata>::from_comptime_to_runtime(&#marker, &#marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::ConstructorFunctionMetadataEntry(&#marker_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::generic::function::ConstructorFunctionConstDynMetadata for #marker {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::function_path::ConstructorFunctionPath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #id_lit.into()))
            }
            fn registrator(self) -> crate::utils::clone_closure::CloneClosure<rhai::ImmutableString, &'static mut rhai::Module, (), fn(rhai::ImmutableString, &mut rhai::Module)> {
                crate::utils::clone_closure::CloneClosure::new(#fn_name_lit.into(), |name, parent_module| {
                    rhai::FuncRegistration::new(name).set_into_module(parent_module, #function_expr);
                })
            }
        }

        impl crate::rhai_binding::meta::generic::function::ConstructorFunctionDynamicTypedMetadata for #marker {}
    }
}

fn generate_method_metadata(id_path_string: String, fn_name: &str, function_expr: TokenStream2) -> TokenStream2 {
    let id_lit = path_lit(&id_path_string);
    let fn_name_lit = path_lit(fn_name);

    let marker = make_marker_ident("METHOD_FN", &id_path_string);
    let marker_static = make_static_ident("METHOD_FN", &id_path_string);

    quote! {
        #[allow(non_upper_case_globals)]
        static #marker_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::function::MethodFunctionMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#marker as crate::rhai_binding::meta::generic::function::MethodFunctionDynamicTypedMetadata>::from_comptime_to_runtime(&#marker, &#marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::MethodFunctionMetadataEntry(&#marker_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::generic::function::MethodFunctionConstDynMetadata for #marker {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::function_path::MethodFunctionPath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #id_lit.into()))
            }
            fn registrator(self) -> crate::utils::clone_closure::CloneClosure<rhai::ImmutableString, &'static mut rhai::Engine, (), fn(rhai::ImmutableString, &mut rhai::Engine)> {
                crate::utils::clone_closure::CloneClosure::new(#fn_name_lit.into(), |name, engine| {
                    engine.register_fn(name, #function_expr);
                })
            }
        }

        impl crate::rhai_binding::meta::generic::function::MethodFunctionDynamicTypedMetadata for #marker {}
    }
}

fn generate_item_assoc_metadata(id_path_string: String, fn_name: &str, function_expr: TokenStream2) -> TokenStream2 {
    let id_lit = path_lit(&id_path_string);
    let fn_name_lit = path_lit(fn_name);

    let marker = make_marker_ident("ITEM_FN", &id_path_string);
    let marker_static = make_static_ident("ITEM_FN", &id_path_string);

    quote! {
        #[allow(non_upper_case_globals)]
        static #marker_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::function::ItemAssociatedFunctionMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#marker as crate::rhai_binding::meta::generic::function::ItemAssociatedFunctionDynamicTypedMetadata>::from_comptime_to_runtime(&#marker, &#marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::ItemAssociatedFunctionMetadataEntry(&#marker_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::generic::function::ItemAssociatedFunctionConstDynMetadata for #marker {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::function_path::ItemAssociatedFunctionPath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #id_lit.into()))
            }
            fn registrator(self) -> crate::utils::clone_closure::CloneClosure<rhai::ImmutableString, &'static mut rhai::Module, (), fn(rhai::ImmutableString, &mut rhai::Module)> {
                crate::utils::clone_closure::CloneClosure::new(#fn_name_lit.into(), |name, parent_module| {
                    rhai::FuncRegistration::new(name).set_into_module(parent_module, #function_expr);
                })
            }
        }

        impl crate::rhai_binding::meta::generic::function::ItemAssociatedFunctionDynamicTypedMetadata for #marker {}
    }
}

pub fn reflect_inherent_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_impl = parse_macro_input!(item as ItemImpl);

    if item_impl.trait_.is_some() {
        return syn::Error::new(Span::call_site(), "`#[reflect_inherent_impl]` expects an inherent impl block")
            .to_compile_error()
            .into();
    }

    let self_ty = (*item_impl.self_ty).clone();
    let self_ty_string = if attr.is_empty() {
        type_path_to_string(&self_ty)
    } else {
        path_to_string(&parse_single_path_attr(attr))
    };
    let impl_id_lit = path_lit(&self_ty_string);

    let mut ctor_paths: Vec<syn::LitStr> = Vec::new();
    let mut method_paths: Vec<syn::LitStr> = Vec::new();
    let mut item_paths: Vec<syn::LitStr> = Vec::new();
    let mut function_meta: Vec<TokenStream2> = Vec::new();

    let mut new_items = Vec::with_capacity(item_impl.items.len());

    for item in item_impl.items.into_iter() {
        match item {
            ImplItem::Fn(mut f) => {
                let fn_ident = f.sig.ident.clone();

                for attr in &f.attrs {
                    match attr_name(attr).as_deref() {
                        Some("reflect_constructor_function") => {
                            let base = get_attr_path(attr).map(|p| path_to_string(&p)).unwrap_or_else(|| self_ty_string.clone());
                            let id_path = format!("{}::{}", base, fn_ident);
                            ctor_paths.push(path_lit(&id_path));
                            function_meta.push(generate_constructor_metadata(id_path, &fn_ident.to_string(), quote! { #self_ty::#fn_ident }));
                        }
                        Some("reflect_method_function") => {
                            let base = get_attr_path(attr).map(|p| path_to_string(&p)).unwrap_or_else(|| self_ty_string.clone());
                            let id_path = format!("{}::{}", base, fn_ident);
                            method_paths.push(path_lit(&id_path));
                            function_meta.push(generate_method_metadata(id_path, &fn_ident.to_string(), quote! { #self_ty::#fn_ident }));
                        }
                        Some("reflect_item_associated_function") => {
                            let base = get_attr_path(attr).map(|p| path_to_string(&p)).unwrap_or_else(|| self_ty_string.clone());
                            let id_path = format!("{}::{}", base, fn_ident);
                            item_paths.push(path_lit(&id_path));
                            function_meta.push(generate_item_assoc_metadata(id_path, &fn_ident.to_string(), quote! { #self_ty::#fn_ident }));
                        }
                        _ => {}
                    }
                }

                f = strip_reflect_marker_attrs(f);
                new_items.push(ImplItem::Fn(f));
            }
            other => new_items.push(other),
        }
    }

    item_impl.items = new_items;

    let impl_marker = make_marker_ident("INHERENT_IMPL", &self_ty_string);
    let impl_static = make_static_ident("INHERENT_IMPL", &self_ty_string);

    quote! {
        #item_impl

        #(#function_meta)*

        #[allow(non_upper_case_globals)]
        static #impl_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::impl_::InherentImplMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#impl_marker as crate::rhai_binding::meta::generic::impl_::InherentImplDynamicTypedMetadata>::from_comptime_to_runtime(&#impl_marker, &#impl_marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::InherentImplMetadataEntry(&#impl_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #impl_marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #impl_marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::generic::impl_::InherentImplConstDynMetadata for #impl_marker {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::impl_path::InherentImplPath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #impl_id_lit.into()))
            }
            fn constructor_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::ConstructorFunctionPath>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#ctor_paths.into()),*]))
            }
            fn method_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::MethodFunctionPath>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#method_paths.into()),*]))
            }
            fn item_associated_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::ItemAssociatedFunctionPath>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#item_paths.into()),*]))
            }
        }

        impl crate::rhai_binding::meta::generic::impl_::InherentImplDynamicTypedMetadata for #impl_marker {}
    }
    .into()
}

pub fn reflect_trait_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut item_impl = parse_macro_input!(item as ItemImpl);

    let Some((_, trait_path, _)) = &item_impl.trait_ else {
        return syn::Error::new(Span::call_site(), "`#[reflect_trait_impl]` expects a trait impl block")
            .to_compile_error()
            .into();
    };

    let self_ty = (*item_impl.self_ty).clone();
    let default_self_ty_string = type_path_to_string(&self_ty);
    let default_trait_path_string = path_to_string(trait_path);

    let (self_ty_string, trait_path_string, impl_path_string) = if attr.is_empty() {
        let impl_path_string = trait_impl_path_string(&self_ty, trait_path);
        (default_self_ty_string, default_trait_path_string, impl_path_string)
    } else {
        let parsed = parse_trait_impl_attr(attr);
        let self_ty_string = type_path_to_string(&parsed.self_ty);
        let trait_path_string = path_to_string(&parsed.trait_path);
        let impl_path_string = trait_impl_path_string(&parsed.self_ty, &parsed.trait_path);
        (self_ty_string, trait_path_string, impl_path_string)
    };
    let impl_id_lit = path_lit(&impl_path_string);

    let mut item_paths: Vec<syn::LitStr> = Vec::new();
    let mut function_meta: Vec<TokenStream2> = Vec::new();

    let mut new_items = Vec::with_capacity(item_impl.items.len());
    for item in item_impl.items.into_iter() {
        match item {
            ImplItem::Fn(mut f) => {
                let fn_ident = f.sig.ident.clone();

                for attr in &f.attrs {
                    if matches!(attr_name(attr).as_deref(), Some("reflect_item_associated_function")) {
                        let base = get_attr_trait_impl_path(attr)
                            .or_else(|| get_attr_path(attr).map(|p| path_to_string(&p)))
                            .unwrap_or_else(|| format!("<{} as {}>", self_ty_string, trait_path_string));
                        let id_path = format!("{}::{}", base, fn_ident);
                        item_paths.push(path_lit(&id_path));
                        function_meta.push(generate_item_assoc_metadata(
                            id_path,
                            &fn_ident.to_string(),
                            quote! { <#self_ty as #trait_path>::#fn_ident },
                        ));
                    }
                }

                f = strip_reflect_marker_attrs(f);
                new_items.push(ImplItem::Fn(f));
            }
            other => new_items.push(other),
        }
    }

    item_impl.items = new_items;

    let impl_marker = make_marker_ident("TRAIT_IMPL", &impl_path_string);
    let impl_static = make_static_ident("TRAIT_IMPL", &impl_path_string);

    quote! {
        #item_impl

        #(#function_meta)*

        #[allow(non_upper_case_globals)]
        static #impl_static: crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::meta::monomorphized::impl_::TraitImplMetadata>
            = crate::utils::clone_lazy::CloneLazy::new(
                crate::utils::clone_closure::CloneClosure::new((), |(), ()| <#impl_marker as crate::rhai_binding::meta::generic::impl_::TraitImplDynamicTypedMetadata>::from_comptime_to_runtime(&#impl_marker, &#impl_marker))
            );
        inventory::submit!(crate::rhai_binding::meta::registry::TraitImplMetadataEntry(&#impl_static));

        #[allow(non_camel_case_types)]
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct #impl_marker;

        impl crate::rhai_binding::meta::generic::abstract_primitive::ConstDynMetadata for #impl_marker {
            fn raw_rust_module_path(&self) -> &'static str { module_path!() }
        }

        impl crate::rhai_binding::meta::generic::impl_::TraitImplConstDynMetadata for #impl_marker {
            fn id_path(&self) -> crate::utils::clone_lazy::CloneLazy<crate::rhai_binding::path::impl_path::TraitImplPath> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| #impl_id_lit.into()))
            }
            fn constructor_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::ConstructorFunctionPath>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![]))
            }
            fn method_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::MethodFunctionPath>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![]))
            }
            fn item_associated_functions(&self) -> crate::utils::clone_lazy::CloneLazy<Vec<crate::rhai_binding::path::function_path::ItemAssociatedFunctionPath>> {
                crate::utils::clone_lazy::CloneLazy::new(crate::utils::clone_closure::CloneClosure::new((), |_, _| vec![#(#item_paths.into()),*]))
            }
        }

        impl crate::rhai_binding::meta::generic::impl_::TraitImplDynamicTypedMetadata for #impl_marker {}
    }
    .into()
}

// Marker attributes used by `reflect_inherent_impl` / `reflect_trait_impl`.
// They are intentionally pass-through to keep usage ergonomic.
pub fn reflect_item_associated_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

pub fn reflect_constructor_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

pub fn reflect_method_function(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
