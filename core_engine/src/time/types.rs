use bevy::prelude::*;

#[derive(Clone, Default, Debug, PartialEq, Eq, Reflect)]
pub enum PauseState {
    #[default]
    Running,
    Paused,
    Step,
}
impl PauseState {
    pub fn is_paused(&self) -> bool {
        matches!(self, PauseState::Paused | PauseState::Step)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Reflect)]
pub enum StepConfig {
    Cycles(u32),
    Seconds(f32),
}
impl Default for StepConfig {
    fn default() -> Self {
        StepConfig::Cycles(1)
    }
}
