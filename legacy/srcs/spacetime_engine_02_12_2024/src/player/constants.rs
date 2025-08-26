use std::any::TypeId;
use structs::TypeBinding;
use crate::*;

pub const PLAYER_TYPE_BINDING: TypeBinding = TypeBinding {
    type_name: "player",
    type_id: TypeId::of::<Player>(),
    type_pre_setup: |hierarchy| {
        
    },
    type_setup: |hierarchy| {

    },
    type_post_setup: |hierarchy| {

    },
};