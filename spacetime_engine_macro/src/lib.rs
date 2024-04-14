use spacetime_engine_macro_lib::commands::generated_output::commands_module_code::CommandsModuleCode;
use spacetime_engine_macro_lib::commands::parsed_input::commands_type::*;
use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn define_commands_module(tokens: TokenStream) -> TokenStream {
    let commands_module_type = parse_macro_input!(tokens as CommandsModuleType);

    let commands_module_code = CommandsModuleCode::generate(&commands_module_type);

    commands_module_code.tokens.into()
}