//! Compatibility composition for test-game legacy observability.
//!
//! Look selection is now owned by `game::devtools`; this module only mirrors the
//! new focus into `DebugContext` until Stage 3 migrates old world visualization.

use bevy::prelude::*;

use crate::{
    devtools::{DeveloperFocus, DeveloperSet},
    observability::{DebugContext, DebugSelection, ObservabilitySet},
};

use super::player::PlayerCamera;

pub struct TestGameObservabilityPlugin;

impl Plugin for TestGameObservabilityPlugin {
    fn build(&self, app: &mut App) {
        super::portal::observability::configure(app);
        super::thermal::observability::configure(app);

        app.add_systems(
            PostUpdate,
            (choose_player_debug_observer, sync_legacy_debug_selection)
                .chain()
                .in_set(ObservabilitySet::Prepare)
                .after(DeveloperSet::ResolveFocus),
        );
    }
}

fn choose_player_debug_observer(
    cameras: Query<Entity, With<PlayerCamera>>,
    mut context: ResMut<DebugContext>,
) {
    context.observer = cameras.iter().next();
}

fn sync_legacy_debug_selection(
    focus: Res<DeveloperFocus>,
    mut context: ResMut<DebugContext>,
) {
    let Some(target) = focus.current() else {
        context.clear_selection();
        return;
    };

    context.set_selection(DebugSelection {
        entity: target.spatial_entity,
        semantic_entity: target.semantic_entity,
        world_position: target.hit.position,
        distance_meters: target.hit.distance_meters,
    });
}
