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