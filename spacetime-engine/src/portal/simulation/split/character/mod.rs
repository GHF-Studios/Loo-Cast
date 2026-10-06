//! Character-specific portal split lifecycle.
//!
//! The character adapter owns kinematic query exclusions, character ground/control
//! state, destination-space remainder simulation and character peer colliders.
//! Candidate selection and aperture geometry remain shared split policy.
//!
//! ## Module map
//!
//! - `materialization`: Character authority/peer collider materialization.
//! - `preparation`: Predictive character split activation before motor simulation.
//! - `resolution`: Character split resolution after ordinary motor movement.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use bevy::prelude::*;

use crate::{physics::character::CharacterLocomotionFrame, portal::PortalSplitTraveler};

use super::retire_split_partition;

mod materialization;
mod preparation;
mod resolution;

pub(crate) use materialization::materialize_portal_splits;
pub(crate) use preparation::prepare_portal_splits;
pub(crate) use resolution::resolve_portal_splits;

fn finish_character_split(
    commands: &mut Commands,
    split: &mut PortalSplitTraveler,
    body: &mut Transform,
    locomotion_frame: Option<&CharacterLocomotionFrame>,
) {
    if retire_split_partition(commands, split).is_some() {
        if let Some(frame) = locomotion_frame {
            body.rotation = frame.aligned_rotation(body.rotation);
        }
    }
}
