use syn::{spanned::Spanned, Ident, LitStr};
use quote::quote;
use crate::commands::parsed_input::command_type::error_type::*;
use super::CommandErrorInfo;

#[derive(Clone)]
pub struct CommandErrorVariantsInfo {
    pub variant_declarations: proc_macro2::TokenStream,
    pub interpolations: String
}

impl CommandErrorVariantsInfo {
    pub fn extract(command_error_type: &CommandErrorType, command_error_info: &CommandErrorInfo) -> Self {
        let command_error_name = &command_error_info.name;
        let command_error_name = Ident::new(command_error_name, command_error_name.span());

        let error_variant_infos: Vec<LitStr> = command_error_type.variant_types.iter().map(|variant_type| {
            variant_type.variant_name.clone()
        }).collect();

        let mut variant_declarations = quote! {};
        let mut interpolations = quote! {};
        let mut first = true;

        for variant_name in error_variant_infos.clone() {
            let variant_name = Ident::new(&variant_name.value(), variant_name.span());

            if !first {
                variant_declarations = quote! {
                    #variant_declarations, 
                };
                interpolations = quote! {
                    #interpolations, 
                };
            } else {
                first = false;
            }

            let error_variant_display_string = command_error_name.to_string() + "::" + &variant_name.to_string();
            let error_variant_display_string = LitStr::new(
                &error_variant_display_string, 
                error_variant_display_string.span()
            );

            variant_declarations = quote! {
                #variant_declarations
                #variant_name
            };
            interpolations = quote! {
                #interpolations
                #command_error_name::#variant_name => {
                    return write!(f, #error_variant_display_string);
                }
            };
        }

        Self {
            variant_declarations,
            interpolations: interpolations.to_string()
        }
    }
}