mod config;
mod face;
mod portal;
mod traveler;

pub use config::{
    PortalConfig,
    PortalEndpointConfig,
};

pub use face::PortalSidedness;

pub use portal::{
    Portal,
    PortalPair,
    PortalView,
};

pub use traveler::{
    PortalTraveler,
    PortalVelocity,
};

pub(crate) use config::MAX_VISUAL_RECURSION_DEPTH;

pub(crate) use face::{
    PortalEndpoint,
    PortalFace,
    PortalSide,
};