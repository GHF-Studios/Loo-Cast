use proc_macro2::{TokenStream as TokenStream2};
use quote::{quote, format_ident};
use syn::{
    parse_macro_input, Ident, Token, braced,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    Type, Path,
};

pub(super) struct TypedDagDef {
    nodes: Vec<TypedNodeDecl>,
}
impl Parse for TypedDagDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut nodes = Vec::new();
        while !input.is_empty() {
            nodes.push(input.parse()?);
        }
        Ok(TypedDagDef { nodes })
    }
}
impl TypedDagDef {
    pub fn generate(self) -> TokenStream2 {
        let enum_ident = format_ident!("TypedDagNode");
        
        let variants: Vec<_> = self.nodes.iter().map(|n| {
            let ident = &n.name;
            quote! { #ident(#ident) }
        }).collect();
    
        let from_impls: Vec<_> = self.nodes.iter().map(|n| {
            let ident = &n.name;
            quote! {
                impl From<#ident> for #enum_ident {
                    fn from(value: #ident) -> Self {
                        #enum_ident::#ident(value)
                    }
                }
            }
        }).collect();
    
        let enum_def = quote! {
            #[derive(Clone)]
            pub enum #enum_ident {
                #(#variants),*
            }
        
            #(#from_impls)*
        };
    
        let mut tokens = TokenStream2::new();
        tokens.extend(enum_def);
    
        for node in self.nodes {
            tokens.extend(node.generate());
        }
    
        tokens
    }
}

struct TypedNodeDecl {
    name: Ident,
    payload_ty: Type,
    parent_tys: Vec<Type>,
    child_tys: Vec<Type>,
}
impl Parse for TypedNodeDecl {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let mut payload_ty = None;
        let mut parent_tys = None;
        let mut child_tys = None;

        while !content.is_empty() {
            let key: Ident = content.parse()?;
            content.parse::<Token![:]>()?;

            match key.to_string().as_str() {
                "payload" => {
                    payload_ty = Some(content.parse()?);
                }
                "parent" => {
                    let list;
                    syn::bracketed!(list in content);
                    parent_tys = Some(Punctuated::<Type, Token![,]>::parse_terminated(&list)?
                        .into_iter()
                        .collect());
                }
                "children" => {
                    let list;
                    syn::bracketed!(list in content);
                    child_tys = Some(Punctuated::<Type, Token![,]>::parse_terminated(&list)?
                        .into_iter()
                        .collect());
                }
                other => {
                    return Err(syn::Error::new(key.span(), format!("Unknown key `{}`", other)));
                }
            }

            // Optionally consume trailing comma
            let _ = content.parse::<Token![,]>();
        }

        Ok(TypedNodeDecl {
            name,
            payload_ty: payload_ty.expect("Missing `payload`"),
            parent_tys: parent_tys.unwrap_or_else(Vec::new),
            child_tys: child_tys.unwrap_or_else(Vec::new),
        })
    }
}
impl TypedNodeDecl {
    fn generate(self) -> TokenStream2 {
        let struct_name = self.name;
        let payload_ty = self.payload_ty;

        let children = self.child_tys.into_iter().map(|ty| quote! { Box<#ty> });
        let parents = self.parent_tys;

        let child_tuple_ty = quote! { (#(#children),*) };
        let parent_tuple_ty = quote! { (#(#parents),*) };

        quote! {
            #[derive(Clone)]
            pub struct #struct_name {
                pub payload: #payload_ty,
                pub children: #child_tuple_ty,
            }

            impl crate::typed_dag::DagNode for #struct_name {
                type Parent = #parent_tuple_ty;
                type Children = #child_tuple_ty;
                type Payload = #payload_ty;

                fn payload(&self) -> &Self::Payload {
                    &self.payload
                }

                fn children(&self) -> &Self::Children {
                    &self.children
                }
            }

            impl crate::typed_dag::ConstructNode for #struct_name {
                fn new(payload: Self::Payload, children: Self::Children) -> Self {
                    Self { payload, children }
                }
            }

            impl crate::typed_dag::WithChildren for #struct_name {
                fn with_children(&self, children: Self::Children) -> Self {
                    Self {
                        payload: self.payload.clone(),
                        children,
                    }
                }
            }
        }
    }
}
