//! Controls whether camera observation updates presentation demand.

use bevy::prelude::*;

/// Runtime policy for sparse presentation/view demand.
///
/// This is presentation interest only. Dense physical/collision/editing demand
/// remains owned by its explicit capability/spatial demand sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UsfViewDemandMode {
    /// Recompute demand from the current observer each frame when it changes.
    #[default]
    Live,
    /// Preserve the last captured demand while the observer camera moves.
    Frozen,
    /// Publish no active presentation-view demand.
    Disabled,
}

impl UsfViewDemandMode {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Frozen => "frozen",
            Self::Disabled => "disabled",
        }
    }

    pub fn parse(raw: &str) -> Option<Self> {
        match raw.to_ascii_lowercase().as_str() {
            "live" | "follow" => Some(Self::Live),
            "frozen" | "freeze" => Some(Self::Frozen),
            "disabled" | "off" | "none" => Some(Self::Disabled),
            _ => None,
        }
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct UsfViewDemandPolicy {
    mode: UsfViewDemandMode,
}

impl Default for UsfViewDemandPolicy {
    fn default() -> Self {
        Self {
            mode: UsfViewDemandMode::Live,
        }
    }
}

impl UsfViewDemandPolicy {
    pub const fn mode(self) -> UsfViewDemandMode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: UsfViewDemandMode) {
        self.mode = mode;
    }
}
