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
pub use traveler::{PortalSplitTraveler, PortalTraveler, PortalVelocity};

pub(crate) use config::MAX_VISUAL_RECURSION_DEPTH;
pub(crate) use face::{PortalFace, PortalSide};
pub(crate) use portal::PortalSupport;
pub(crate) use traveler::ActivePortalSplit;
