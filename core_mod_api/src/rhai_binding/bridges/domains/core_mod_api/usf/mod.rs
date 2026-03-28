core_mod_macros::reflect_extern_sub_module!(
    id = core_mod_api::usf,
    sub_modules = [phenomenon, pos, substrate, zone],
    traits = [],
    types = [],
    module_associated_functions = [],
);

pub mod phenomenon;
pub mod pos;
pub mod substrate;
pub mod zone;
