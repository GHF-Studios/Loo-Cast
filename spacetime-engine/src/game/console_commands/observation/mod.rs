//! Observation and diagnostic console commands for the current world view.

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
        UsfCapabilityRealization, UsfInteractionScaleAffinity, UsfPosition, UsfPresentationProbe,
        UsfPrimaryInteractionSlice, UsfRuntimeChartState, UsfScaleCoverageSnapshot, UsfScaleLayer,
        UsfScalePresentation, UsfScaleRoleMask, UsfSceneryPresentation, UsfViewRenderAnchor,
    },
    view::PrimaryGameView,
    voxel::VoxelStreamingTelemetry,
};
use bevy::prelude::*;
use std::collections::BTreeMap;

mod position;
mod presentation;
mod runtime;

pub(super) use position::{locate_command, where_command};
pub(super) use presentation::presentation_command;
pub(super) use runtime::{navtrace_command, voxelstream_command};
