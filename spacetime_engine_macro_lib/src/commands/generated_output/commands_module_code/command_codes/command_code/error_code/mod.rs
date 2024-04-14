pub mod enum_code;
pub mod impl_display_code;

use quote::quote;
use syn::{spanned::Spanned, Ident, LitStr};
use crate::commands::generated_output::commands_module_code::command_codes::command_code::CommandType;
use self::enum_code::*;
use self::impl_display_code::*;

pub struct CommandErrorCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandErrorCode {
    pub fn generate(command_type: &CommandType) -> Self {
        let command_id = command_type.command_id.value().to_string();
        let command_error_name = command_id.clone() + "CommandError";
        let command_error_name = Ident::new(&command_error_name, command_error_name.span());

        let command_error_type = &command_type.error_type;
        let error_variant_infos: Vec<LitStr> = command_error_type.variant_types.iter().map(|variant_type| {
            variant_type.variant_name.clone()
        }).collect();
        let mut command_error_variants_declaration = quote! {};
        let mut command_error_variants_interpolation = quote! {};
        let mut first = true;
        for variant_name in error_variant_infos.clone() {
            let variant_name = Ident::new(&variant_name.value(), variant_name.span());

            if !first {
                command_error_variants_declaration = quote! {
                    #command_error_variants_declaration, 
                };
                command_error_variants_interpolation = quote! {
                    #command_error_variants_interpolation, 
                };
            } else {
                first = false;
            }

            let error_variant_display_string = command_error_name.to_string() + "::" + &variant_name.to_string();
            let error_variant_display_string = LitStr::new(
                &error_variant_display_string, 
                error_variant_display_string.span()
            );

            command_error_variants_declaration = quote! {
                #command_error_variants_declaration
                #variant_name
            };
            command_error_variants_interpolation = quote! {
                #command_error_variants_interpolation
                #command_error_name::#variant_name => {
                    return write!(f, #error_variant_display_string);
                }
            };
        }

        let command_error_enum_code = CommandErrorEnumCode::generate(
            command_error_name.clone(),
            command_error_variants_declaration
        ).tokens;

        let command_error_impl_display_code = CommandErrorImplDisplayCode::generate(
            command_error_name,
            command_error_variants_interpolation
        ).tokens;

        let tokens = quote! {
            #command_error_enum_code
            #command_error_impl_display_code
        };

        Self {
            tokens
        }
    }
}