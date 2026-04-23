use crate::bevy::prelude::*;

use super::resources::StartupFinished;

pub fn run_after_startup_finished(startup_finished: Option<Res<StartupFinished>>) -> bool {
    startup_finished.is_some()
}
