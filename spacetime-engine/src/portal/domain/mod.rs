//! Portal domain types and public mutation protocol.
//!
//! ## Module map
//!
//! - `command`: Public mutation commands for the persistent portal pair.
//! - `config`: Configuration for portal-domain behavior and supported portal states.
//! - `face`: Directed portal faces.
//! - `portal`: Physical portal endpoints and pair identity.
//! - `traveler`: Track portal travelers and the state of ordinary or split traversal.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod command;
mod config;
mod face;
mod portal;
mod traveler;

pub use command::PortalCommand;
pub use config::{PortalConfig, PortalEndpointConfig};
pub use face::PortalSidedness;
pub use portal::{Portal, PortalActive, PortalEndpoint, PortalPair, PortalView};
pub use traveler::{PortalRigidSplitBody, PortalSplitTraveler, PortalTraveler, PortalVelocity};

pub(super) use face::{PortalFace, PortalSide};
pub(super) use portal::PortalSupport;
pub(super) use traveler::ActivePortalSplit;
