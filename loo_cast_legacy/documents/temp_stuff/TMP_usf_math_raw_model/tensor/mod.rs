#![allow(dead_code)]

/// Rank-3 tensor aliases for repr unions and operand variants.
pub mod aliases;
/// Normal rank-3 tensor representations and contracts.
pub mod normal;
/// Shared rank-3 tensor contracts consumed by both normal and USF tensor surfaces.
pub mod shared;
/// USF rank-3 tensor representations and cross-scale contracts.
pub mod usf;
