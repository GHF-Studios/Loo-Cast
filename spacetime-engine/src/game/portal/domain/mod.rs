mod config;
mod portal;
mod traveler;

pub use config::{
    PortalConfig,
    PortalEndpointConfig,
};
pub use portal::{
    Portal,
    PortalPair,
    PortalSidedness,
    PortalView,
};
pub use traveler::{
    PortalTraveler,
    PortalVelocity,
};

pub(crate) use config::MAX_VISUAL_RECURSION_DEPTH;
pub(crate) use portal::{
    PortalEndpoint,
    PortalSide,
};
