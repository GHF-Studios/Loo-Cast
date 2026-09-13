//! Compatibility composition for test-game legacy observability.
//!
//! Focus and observer ownership now live in `game::devtools`. This module only
//! mirrors the new developer context into `DebugContext` for legacy subsystems
//! that have not reached their migration stage yet.

use bevy::prelude::*;

use crate::{
    devtools::{DeveloperFocus, DeveloperSet, DeveloperView},
    observability::{DebugContext, DebugSelection, ObservabilitySet},
};

pub struct TestGameObservabilityPlugin;

impl Plugin for TestGameObservabilityPlugin {
    fn build(&self, app: &mut App) {
        super::portal::observability::configure(app);
        super::thermal::observability::configure(app);

        app.add_systems(
            PostUpdate,
            sync_legacy_debug_context
                .in_set(ObservabilitySet::Prepare)
                .after(DeveloperSet::ResolveFocus),
        );
    }
}

fn sync_legacy_debug_context(
    focus: Res<DeveloperFocus>,
    view: Res<DeveloperView>,
    mut context: ResMut<DebugContext>,
) {
    context.observer = view.observer();

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
