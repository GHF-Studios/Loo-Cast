use std::any::TypeId;
use crate::*;

pub const COMMAND_TYPE_BINDING: TypeBinding = TypeBinding {
    type_name: "command",
    type_id: TypeId::of::<Command>(),
    type_pre_setup: |hierarchy| {
        
    },
    type_setup: |hierarchy| {

    },
    type_post_setup: |hierarchy| {

    },
};