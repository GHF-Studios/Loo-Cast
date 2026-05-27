pub mod constants;
pub mod rand;

/// Scalar-focused aliases, including USF/normal union wrappers.
pub mod aliases;
/// Canonical fixed-width decimal parts and shared digit buffers.
pub mod decimal_parts;
/// Canonical decimal-digit carrier and scientific-literal parsing scaffolding.
pub mod digits;
/// Normal scalar representations and normal-space scalar contracts.
pub mod normal;
/// Shared scalar contracts consumed by both normal and USF scalar surfaces.
pub mod shared;
/// USF scalar representations and cross-scale scalar contracts.
pub mod usf;
