/// Scalar-focused aliases, including USF/normal union wrappers.
pub mod aliases;
/// Canonical decimal-digit carrier and scientific-literal parsing scaffolding.
pub(crate) mod digits;
/// Normal scalar representations and normal-space scalar contracts.
pub mod normal;
/// Shared scalar contracts consumed by both normal and USF scalar surfaces.
pub mod shared;
/// USF scalar representations and cross-scale scalar contracts.
pub mod usf;
