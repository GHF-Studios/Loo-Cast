//! Composition of test-game-specific developer tools.

use bevy::prelude::*;

use crate::observability::{DebugContext, ObservabilitySet};

use super::player::PlayerCamera;

pub struct TestGameObservabilityPlugin;

impl Plugin for TestGameObservabilityPlugin {
    fn build(&self, app: &mut App) {
        super::portal::observability::configure(app);
        super::thermal::observability::configure(app);

        app.add_systems(
            PostUpdate,
            choose_player_debug_observer.in_set(ObservabilitySet::Prepare),
        );
    }
}

fn choose_player_debug_observer(
    cameras: Query<Entity, With<PlayerCamera>>,
    mut context: ResMut<DebugContext>,
) {
    context.observer = cameras.iter().next();
}
