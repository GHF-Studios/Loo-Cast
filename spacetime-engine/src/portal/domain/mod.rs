//! Portal domain types and public mutation protocol.

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
