core_mod_macros::reflect_extern_sub_module!(
    id = core_mod_api::usf,
    sub_modules = [pos, zone, realization_channels],
    traits = [],
    types = [],
    module_associated_functions = [],
);

pub mod pos;
pub mod realization_channels;
pub mod zone;
