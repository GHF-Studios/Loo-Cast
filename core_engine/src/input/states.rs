use bevy::prelude::*;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash, States, Reflect)]
#[reflect(State)]
pub enum InputMode {
    #[default]
    Game,
    DebugSuite,
}
impl InputMode {
    #[inline]
    pub fn is_game(&self) -> bool {
        matches!(self, InputMode::Game)
    }

    #[inline]
    pub fn is_debug_suite(&self) -> bool {
        matches!(self, InputMode::DebugSuite)
    }
}