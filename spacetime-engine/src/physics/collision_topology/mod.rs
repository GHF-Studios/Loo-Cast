//! Runtime collision-space modification.
//!
//! Higher-level mechanics publish [`CollisionStencil`] components. Hosts keep
//! immutable [`CollisionClipSource`] geometry. This module rebuilds the actual
//! Avian collider only when the effective stencil set changes.

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
