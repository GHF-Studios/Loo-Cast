//! Maintain the presentation scene and surfaces for portal views.
//!
//! ## Module map
//!
//! - `frame`: Visible frame geometry around the aperture.
//! - `setup`: Creation of the physical demo pair and its visual infrastructure.
//! - `surface`: Directed aperture surfaces.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod frame;
mod setup;
mod surface;

pub use setup::setup_portals;

pub use surface::{spawn_portal_surface, spawn_terminal_surface};
