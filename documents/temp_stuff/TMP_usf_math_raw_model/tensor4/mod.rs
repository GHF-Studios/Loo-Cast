#![allow(dead_code)]

/// Rank-4 tensor aliases for domain unions and operand variants.
pub mod aliases;
/// Normal rank-4 tensor representations and contracts.
pub mod normal;
/// Shared rank-4 tensor contracts consumed by both normal and USF tensor surfaces.
pub mod shared;
/// USF rank-4 tensor representations and cross-scale contracts.
pub mod usf;
