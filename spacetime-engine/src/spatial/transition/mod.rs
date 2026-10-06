//! First-class canonical spatial transitions.
//!
//! Runtime transforms are projections into one bounded active chart. They are
//! never authoritative universe coordinates. Ordinary movement is committed to
//! canonical [`UsfPosition`] first; scale changes and discontinuous relocation
//! then rebuild the runtime chart from canonical state.

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
    UsfSpatialTransitionCause, UsfSpatialTransitionQueue, UsfTransitionVelocity,
};
