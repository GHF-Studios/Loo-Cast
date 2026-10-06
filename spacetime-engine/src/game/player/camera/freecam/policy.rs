//! Debug camera input and projection policies.

use crate::spatial::UsfViewDemandMode;
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum FreecamControlPolicy {
    /// Freecam consumes local movement/look intent; the gameplay subject stays still.
    #[default]
    Exclusive,
    /// Freecam moves while ordinary gameplay input is also delivered to the subject.
    Passthrough,
}

impl FreecamControlPolicy {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Passthrough => "passthrough",
        }
    }

    pub(crate) fn parse(raw: &str) -> Option<Self> {
        match raw.to_ascii_lowercase().as_str() {
            "exclusive" | "detached" | "blocked" => Some(Self::Exclusive),
            "passthrough" | "shared" => Some(Self::Passthrough),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum FreecamProjectionPolicy {
    /// The detached camera is a real presentation observer. USF projection
    /// remains active and is re-anchored from the freecam's canonical position.
    #[default]
    Follow,
    /// Freeze the USF presentation observer where it last was while the local
    /// camera flies through already-realized local geometry.
    Frozen,
    /// Disable the dedicated USF projection pass; inspect only local/raw scene state.
    Disabled,
}

impl FreecamProjectionPolicy {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Follow => "follow",
            Self::Frozen => "frozen",
            Self::Disabled => "disabled",
        }
    }

    pub(crate) fn parse(raw: &str) -> Option<Self> {
        match raw.to_ascii_lowercase().as_str() {
            "follow" | "observer" | "normal" => Some(Self::Follow),
            "frozen" | "freeze" => Some(Self::Frozen),
            "disabled" | "off" | "bypass" => Some(Self::Disabled),
            _ => None,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub(crate) struct DebugFreecam {
    enabled: bool,
    translation_speed_mps: f32,
    boost_multiplier: f32,
    control_policy: FreecamControlPolicy,
    projection_policy: FreecamProjectionPolicy,
    view_demand_mode: UsfViewDemandMode,
}

impl DebugFreecam {
    pub const DEFAULT_TRANSLATION_SPEED_MPS: f32 = 20.0;
    pub const DEFAULT_BOOST_MULTIPLIER: f32 = 8.0;

    pub(crate) const fn enabled(&self) -> bool {
        self.enabled
    }

    pub(crate) fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub(crate) const fn translation_speed_mps(&self) -> f32 {
        self.translation_speed_mps
    }

    pub(crate) fn set_translation_speed_mps(&mut self, value: f32) -> Result<(), String> {
        if !value.is_finite() || value < 0.0 {
            return Err("freecam translation speed must be finite and non-negative".to_string());
        }
        self.translation_speed_mps = value;
        Ok(())
    }

    pub(crate) const fn boost_multiplier(&self) -> f32 {
        self.boost_multiplier
    }

    pub(crate) fn set_boost_multiplier(&mut self, value: f32) -> Result<(), String> {
        if !value.is_finite() || value <= 0.0 {
            return Err("freecam boost multiplier must be finite and > 0".to_string());
        }
        self.boost_multiplier = value;
        Ok(())
    }

    pub(crate) const fn control_policy(&self) -> FreecamControlPolicy {
        self.control_policy
    }

    pub(crate) fn set_control_policy(&mut self, policy: FreecamControlPolicy) {
        self.control_policy = policy;
    }

    pub(crate) const fn projection_policy(&self) -> FreecamProjectionPolicy {
        self.projection_policy
    }

    pub(crate) fn set_projection_policy(&mut self, policy: FreecamProjectionPolicy) {
        self.projection_policy = policy;
    }

    pub(crate) const fn view_demand_mode(&self) -> UsfViewDemandMode {
        self.view_demand_mode
    }

    pub(crate) fn set_view_demand_mode(&mut self, mode: UsfViewDemandMode) {
        self.view_demand_mode = mode;
    }

    pub(crate) const fn consumes_gameplay_input(&self) -> bool {
        matches!(self.control_policy, FreecamControlPolicy::Exclusive)
    }
}

impl Default for DebugFreecam {
    fn default() -> Self {
        Self {
            enabled: false,
            translation_speed_mps: Self::DEFAULT_TRANSLATION_SPEED_MPS,
            boost_multiplier: Self::DEFAULT_BOOST_MULTIPLIER,
            control_policy: FreecamControlPolicy::Exclusive,
            projection_policy: FreecamProjectionPolicy::Follow,
            // Freeze presentation demand by default so flying the camera does not
            // heal/replace the scene being inspected. Dense physical demand is
            // player-owned and is never transferred to freecam at all.
            view_demand_mode: UsfViewDemandMode::Frozen,
        }
    }
}
