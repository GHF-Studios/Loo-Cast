core_engine_macros::reflect_extern_sub_module!(
    id = core_mod_api::usf,
    sub_modules = [pos, zone, output_channels],
    traits = [],
    types = [],
    module_associated_functions = [],
);

pub mod output_channels;
pub mod pos;
pub mod zone;
