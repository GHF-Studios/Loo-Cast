//! Diagnostic presentation filter; never semantic authority.

use super::*;

/// Diagnostic filter for the two terrain presentation domains.
///
/// This changes presentation output only. It never changes semantic state,
/// capability coverage, residency, physics or interaction Scale Slice ownership.
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum UsfPresentationProbe {
    #[default]
    All,
    Physical,
    Context,
}

impl UsfPresentationProbe {
    pub const fn physical_enabled(self) -> bool {
        matches!(self, Self::All | Self::Physical)
    }

    pub const fn context_enabled(self) -> bool {
        matches!(self, Self::All | Self::Context)
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Physical => "physical",
            Self::Context => "context",
        }
    }
}
