use bevy::prelude::*;

use super::types::{PauseState, StepConfig};

#[derive(Resource, Clone, Default, Debug, Reflect)]
#[reflect(Resource)]
pub struct TimeInfo {
    pub pause_state: PauseState,
    pub step_config: StepConfig,
}

#[derive(Resource, Clone, Default, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub struct VirtualPaused(pub bool);