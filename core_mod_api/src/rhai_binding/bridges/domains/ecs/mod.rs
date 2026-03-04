core_mod_macros::reflect_top_level_module!(
    id = ecs,
    sub_modules = [world, commands, entities],
    traits = [],
    types = [],
    module_associated_functions = [],
);

pub mod commands;
pub mod entities;
pub mod world;
