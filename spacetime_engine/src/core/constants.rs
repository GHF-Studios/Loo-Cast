use std::any::TypeId;
use crate::*;
use crate::command::structs::Command;
use crate::operation::structs::Operation;

pub const RESERVED_STRING_IDS: [&str; 1] = ["root"];
pub const RESERVED_NUMERIC_IDS: [i32; 1] = [0];
pub const TYPE_BINDINGS: [TypeBinding; 3] = [
    TypeBinding {
        type_name: "core",
        type_id: TypeId::of::<Core>(),
        type_pre_setup: |type_, hierarchy| {
            
        },
        type_setup: |type_, hierarchy| {

        },
        type_post_setup: |type_, hierarchy| {

        },
    },
    TypeBinding {
        type_name: "operation",
        type_id: TypeId::of::<Operation>(),
        type_pre_setup: |type_, hierarchy| {

        },
        type_setup: |type_, hierarchy| {

        },
        type_post_setup: |type_, hierarchy| {

        },
    },
    TypeBinding {
        type_name: "command",
        type_id: TypeId::of::<Command>(),
        type_pre_setup: |type_, hierarchy| {

        },
        type_setup: |type_, hierarchy| {

        },
        type_post_setup: |type_, hierarchy| {

        },
    },
];