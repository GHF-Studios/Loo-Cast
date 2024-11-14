use std::any::TypeId;
use crate::*;
use super::wrappers::CoreCommandTypeRegistry;

pub const CORE_TYPE_BINDING: TypeBinding = TypeBinding {
    type_name: "core",
    type_id: TypeId::of::<Core>(),
    type_pre_setup: |hierarchy| {
        
    },
    type_setup: |hierarchy| {

    },
    type_post_setup: |hierarchy| {

    },
};