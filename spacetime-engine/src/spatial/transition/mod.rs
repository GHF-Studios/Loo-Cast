//! First-class canonical spatial transitions.
//!
//! Runtime transforms are projections into one bounded active chart. They are
//! never authoritative universe coordinates. Ordinary movement is committed to
//! canonical [`UsfPosition`] first; scale changes and discontinuous relocation
//! then rebuild the runtime chart from canonical state.
//!
//! ## Integration
//!
//! Callers submit canonical relocation or interaction requirements. The transition facility owns
//! queueing, coverage admission, and runtime projection after canonical state changes.
//!
//! ## Module map
//!
//! - `admission`: Capability evidence and backend vetoes for interaction-chart handoffs.
//! - `apply`: Apply admitted canonical transitions to runtime chart participants.
//! - `intent`: Canonical relocation requests, continuous interaction requirements and their
//!   queue.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use std::collections::{HashMap, HashSet, VecDeque};

use avian3d::prelude::{LinearVelocity, Position};
use bevy::prelude::*;

use crate::ecs::{UsfLogicalRealizationOf, UsfOwnershipQuery};

use super::{
    SpatialScale, UsfCanonicalMotion, UsfInteractionProjection, UsfPosition,
    UsfPrimaryInteractionSlice, UsfRuntimeChartState, UsfScaleCoverageSnapshot, UsfScaleLayer,
    UsfScaleRoleMask, UsfSpatialAnchor, UsfViewContext, UsfViewRenderAnchor,
};

mod admission;
mod apply;
mod intent;

pub use admission::UsfInteractionHandoffGuards;
pub(in crate::spatial) use admission::reset_interaction_handoff_guards;
use admission::{InteractionHandoffWaitFingerprint, ResolvedTransition};
pub(super) use apply::apply_spatial_transitions;
use intent::TransitionCoverageRequirement;
pub use intent::{
    UsfInteractionRequirement, UsfSpatialTransition, UsfSpatialTransitionApplied,
    UsfSpatialTransitionCause, UsfSpatialTransitions, UsfTransitionVelocity,
};
