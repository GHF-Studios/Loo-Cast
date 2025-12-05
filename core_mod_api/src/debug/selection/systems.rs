use bevy::prelude::*;

use crate::core::components::Meta;
use crate::picking::constants::NO_HIT_SENTINEL;
use crate::render::resources::PrimaryWindowUiState;

#[tracing::instrument(skip_all)]
pub(super) fn handle_selection(
    mut events: EventReader<Pointer<Click>>,
    selectable_query: Query<Entity, (With<Sprite>, Without<Meta<Sprite>>)>,
    mut debug_suite_ui_state: ResMut<PrimaryWindowUiState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let selected = &mut debug_suite_ui_state.selected_entities;

    if !events.is_empty() {
        // warn!("Handling {} pointer click events (this should be 1) when clicking once", events.len());
    }

    if events.len() > 1 {
        warn!("Multiple pointer click events detected in a single frame; this may indicate an issue with input handling.\nEvents: '{:?}'", events.read().collect::<Vec<_>>());
    }

    for event in events.read() {
        if event.target == NO_HIT_SENTINEL {
            warn!("Clicked on empty space; clearing selection.");
            selected.clear();
            continue;
        }

        if selectable_query.get(event.target).is_err() {
            warn!("Tried to select non-existent/incompatible entity: {:?}", event.target);
            continue;
        }

        warn!("Selecting entity: {:?}", event.target);

        let ctrl = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);

        if ctrl {
            if selected.contains(event.target) {
                selected.retain(|e| e != event.target);
            } else {
                selected.select_maybe_add(event.target, true);
            }
        } else {
            selected.select_replace(event.target);
        }
    }
}
