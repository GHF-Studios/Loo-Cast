//! Observation and diagnostic console commands for the current world view.
//!
//! ## Module map
//!
//! - `position`: Canonical position and authored landmark inspection.
//! - `presentation`: View, interaction and coverage diagnostic projection.
//! - `runtime`: Bounded navigation history and worker telemetry commands.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use super::super::{
    control::LocalControlSubject,
    locomotion::{ControlledSubjectLocomotion, MotionExecution},
    navigation::{NavigationAudit, NavigationFlightRecorder},
    world::UniverseLandmarkIndex,
};
use super::{controlled_semantic_entity, primary_view_context};
use crate::{
    console::{ConsoleCommandInvocation, ConsoleCommandResult},
    physics::topology::runtime_semantic_of_world,
    spatial::{
        UsfCapabilityRealization, UsfInteractionScaleAffinity, UsfPosition,
        UsfPresentationDomainProbe, UsfPrimaryInteractionSlice, UsfRuntimeChartState,
        UsfScaleCoverageSnapshot, UsfScaleLayer, UsfScalePresentation, UsfScaleRoleMask,
        UsfSceneryPresentation, UsfViewRenderAnchor,
    },
    view::PrimaryGameView,
    voxel::VoxelMaterializationTelemetry,
};
use bevy::prelude::*;
use std::collections::BTreeMap;

mod position;
mod presentation;
mod runtime;

pub(super) use position::{locate_command, where_command};
pub(super) use presentation::presentation_command;
pub(super) use runtime::{navtrace_command, voxelstream_command};
