use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Attribute, Data, DataStruct, DeriveInput, Error, Fields, Ident, Lit, Meta, MetaList, MetaNameValue, Result, Type
};

pub fn impl_dag_node(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match expand_dag_node(&input) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn expand_dag_node(input: &DeriveInput) -> Result<TokenStream> {
    let struct_name = &input.ident;
    let attrs = &input.attrs;
    let data = &input.data;

    // Parse dag(parent = ..., children = (...)) attribute
    let (parent_ty, children_ty) = parse_dag_attributes(attrs)?;

    // Extract payload and children field names
    let (payload_ty, payload_field, children_field) = extract_payload_and_children(data)?;

    let dag_node_impl = quote! {
        impl DagNode for #struct_name {
            type Parent = #parent_ty;
            type Children = #children_ty;
            type Payload = #payload_ty;

            fn payload(&self) -> &Self::Payload {
                &self.#payload_field
            }

            fn children(&self) -> &Self::Children {
                &self.#children_field
            }
        }
    };

    let construct_node_impl = quote! {
        impl ConstructNode for #struct_name {
            fn new(payload: Self::Payload, children: Self::Children) -> Self {
                Self {
                    #payload_field: payload,
                    #children_field: children,
                }
            }
        }
    };

    let with_children_impl = quote! {
        impl WithChildren for #struct_name {
            fn with_children(&self, children: Self::Children) -> Self {
                Self {
                    #payload_field: self.#payload_field.clone(),
                    #children_field: children,
                }
            }
        }
    };

    Ok(quote! {
        #dag_node_impl
        #construct_node_impl
        #with_children_impl
    })
}

fn parse_dag_attributes(attrs: &[Attribute]) -> Result<(Type, Type)> {
    for attr in attrs {
        if attr.path().is_ident("dag") {
            return attr.parse_args_with(|parser: syn::parse::ParseStream| {
                let mut parent_ty: Option<Type> = None;
                let mut children_ty: Option<Type> = None;

                while !parser.is_empty() {
                    let ident: Ident = parser.parse()?;
                    parser.parse::<syn::Token![=]>()?;

                    if ident == "parent" {
                        let ty: Type = parser.parse()?;
                        parent_ty = Some(ty);
                    } else if ident == "children" {
                        let ty: Type = parser.parse()?;
                        children_ty = Some(ty);
                    } else {
                        return Err(Error::new(ident.span(), "Unknown key in #[dag(...)]"));
                    }

                    if parser.peek(syn::Token![,]) {
                        parser.parse::<syn::Token![,]>()?;
                    }
                }

                match (parent_ty, children_ty) {
                    (Some(parent), Some(children)) => Ok((parent, children)),
                    _ => Err(Error::new(Span::call_site(), "Expected both `parent = ...` and `children = ...`")),
                }
            }).map_err(|e| Error::new(attr.span(), e.to_string()));
        }
    }

    // Only reached if no #[dag(...)] was found at all
    Err(Error::new(Span::call_site(), "Missing #[dag(...)] attribute"))
}

fn extract_payload_and_children(data: &Data) -> Result<(Type, Ident, Ident)> {
    let Data::Struct(DataStruct { fields: Fields::Named(fields), .. }) = data else {
        return Err(Error::new(Span::call_site(), "Only named-field structs are supported"));
    };

    let mut payload_field = None;
    let mut children_field = None;

    for field in &fields.named {
        if field.ident.as_ref().unwrap() == "children" {
            children_field = Some((field.ident.clone().unwrap(), field.ty.clone()));
        } else if payload_field.is_none() {
            // First non-children field is treated as payload
            payload_field = Some((field.ident.clone().unwrap(), field.ty.clone()));
        }
    }

    match (payload_field, children_field) {
        (Some((payload_ident, payload_ty)), Some((children_ident, _))) => {
            Ok((payload_ty, payload_ident, children_ident))
        }
        _ => Err(Error::new(Span::call_site(), "Struct must have at least one non-children field and one field named 'children'")),
    }
}
