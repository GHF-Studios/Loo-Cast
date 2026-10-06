//! Derived collision-space realization.
//!
//! Higher-level topology mechanics publish [`CollisionStencil`] intent against
//! immutable [`CollisionClipSource`] geometry. This domain owns the disposable
//! Avian collider realization and rebuilds it only when effective topology changes.
//!
//! ## Module map
//!
//! - `csg`: Pure convex clipping/decomposition used by collision topology.
//! - `source`: Immutable collision geometry that can be rebuilt after topology changes.
//! - `state`: Authored stencil state, change index and effective-topology fingerprint.
//! - `stencil_fit`: Rectangular subtractive-stencil fitting against immutable collision sources.
//! - `systems`: ECS reconciliation of dirty clip hosts with the physics collider.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod csg;
mod source;
mod state;
mod stencil_fit;
mod systems;

pub use source::CollisionClipSource;
pub use stencil_fit::{
    RectangularStencilFit, fit_rectangular_stencil, supports_rectangular_stencil,
};

pub use state::CollisionStencil;
use state::{AppliedCollisionTopology, CollisionTopologyState, topology_fingerprint};
pub(super) use systems::rebuild_clipped_colliders;
