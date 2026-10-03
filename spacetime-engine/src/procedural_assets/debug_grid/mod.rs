//! Analytical development-grid procedural asset contract.
//!
//! analytical-procedural-debug-grid-v1
//!
//! The debug grid is no longer a finite bitmap. Voxel presentation evaluates
//! it analytically in the fragment shader from metric-stable UV coordinates.
//! One UV unit historically represented two S0 metres; preserving that contract
//! keeps existing mesh UV generation coherent while allowing each realization
//! to supply its physical metres-per-UV-unit conversion explicitly.

/// Physical metres represented by one existing terrain UV unit at S0.
pub(crate) const DEBUG_GRID_BASE_UV_METRES_PER_UNIT: f32 = 2.0;
