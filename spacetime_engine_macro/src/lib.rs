use proc_macro::TokenStream;
use syn::parse_macro_input;
use spacetime_engine_macro_lib::commands::generated_output::commands_module_code::*;
use spacetime_engine_macro_lib::commands::parsed_input::commands_module_type::*;
use spacetime_engine_macro_lib::systems::generated_output::systems_module_code::*;
use spacetime_engine_macro_lib::systems::parsed_input::systems_type::*;
use spacetime_engine_macro_lib::primitives::generated_output::primitives_module_code::*;
use spacetime_engine_macro_lib::primitives::parsed_input::primitives_type::*;
use spacetime_engine_macro_lib::components::generated_output::components_module_code::*;
use spacetime_engine_macro_lib::components::parsed_input::components_type::*;
use spacetime_engine_macro_lib::archetypes::generated_output::archetypes_module_code::*;
use spacetime_engine_macro_lib::archetypes::parsed_input::archetypes_type::*;
use spacetime_engine_macro_lib::events::generated_output::events_module_code::*;
use spacetime_engine_macro_lib::events::parsed_input::events_type::*;
use spacetime_engine_macro_lib::resources::generated_output::resources_module_code::*;
use spacetime_engine_macro_lib::resources::parsed_input::resources_type::*;
use spacetime_engine_macro_lib::states::generated_output::states_module_code::*;
use spacetime_engine_macro_lib::states::parsed_input::states_type::*;

#[proc_macro]
pub fn define_commands_module(tokens: TokenStream) -> TokenStream {
    let commands_module_type = parse_macro_input!(tokens as CommandsModuleType);

    let commands_module_code = CommandsModuleCode::generate(&commands_module_type);

    commands_module_code.tokens.into()
}

#[proc_macro]
pub fn define_systems_module(tokens: TokenStream) -> TokenStream {
    let systems_module_type = parse_macro_input!(tokens as SystemsModuleType);

    let systems_module_code = SystemsModuleCode::generate(&systems_module_type);

    systems_module_code.tokens.into()
}

#[proc_macro]
pub fn define_primitives_module(tokens: TokenStream) -> TokenStream {
    let primitives_module_type = parse_macro_input!(tokens as PrimitivesModuleType);

    let primitives_module_code = PrimitivesModuleCode::generate(&primitives_module_type);

    primitives_module_code.tokens.into()
}

#[proc_macro]
pub fn define_components_module(tokens: TokenStream) -> TokenStream {
    let components_module_type = parse_macro_input!(tokens as ComponentsModuleType);

    let components_module_code = ComponentsModuleCode::generate(&components_module_type);

    components_module_code.tokens.into()
}

#[proc_macro]
pub fn defin_archetypes_module(tokens: TokenStream) -> TokenStream {
    let archetypes_module_type = parse_macro_input!(tokens as ArchetypesModuleType);

    let archetypes_module_code = ArchetypesModuleCode::generate(&archetypes_module_type);

    archetypes_module_code.tokens.into()
}

#[proc_macro]
pub fn define_events_module(tokens: TokenStream) -> TokenStream {
    let events_module_type = parse_macro_input!(tokens as EventsModuleType);

    let events_module_code = EventsModuleCode::generate(&events_module_type);

    events_module_code.tokens.into()
}

#[proc_macro]
pub fn define_resources_module(tokens: TokenStream) -> TokenStream {
    let resources_module_type = parse_macro_input!(tokens as ResourcesModuleType);

    let resources_module_code = ResourcesModuleCode::generate(&resources_module_type);

    resources_module_code.tokens.into()
}

#[proc_macro]
pub fn define_states_module(tokens: TokenStream) -> TokenStream {
    let states_module_type = parse_macro_input!(tokens as StatesModuleType);

    let states_module_code = StatesModuleCode::generate(&states_module_type);

    states_module_code.tokens.into()
}