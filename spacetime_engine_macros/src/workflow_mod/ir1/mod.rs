pub(in super) mod core_function;
pub(in super) mod core_type;
pub(in super) mod stage;
pub(in super) mod use_statement;
pub(in super) mod user_item;
pub(in crate) mod workflow;

pub(in super) mod kw {
    syn::custom_keyword!(Input);
    syn::custom_keyword!(State);
    syn::custom_keyword!(Output);
    syn::custom_keyword!(Error);
    syn::custom_keyword!(Result);
    syn::custom_keyword!(Outcome);
    syn::custom_keyword!(input);
    syn::custom_keyword!(state);
    syn::custom_keyword!(output);
    syn::custom_keyword!(error);
    syn::custom_keyword!(world);
    syn::custom_keyword!(name);
    syn::custom_keyword!(workflows);
    syn::custom_keyword!(user_imports);
    syn::custom_keyword!(user_items);
    syn::custom_keyword!(stages);
    syn::custom_keyword!(core_types);
    syn::custom_keyword!(core_functions);
}