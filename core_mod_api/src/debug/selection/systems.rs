use bevy::prelude::*;

use crate::render::resources::PrimaryWindowUiState;

#[tracing::instrument(skip_all)]
pub(super) fn handle_selection(
    mut events: EventReader<Pointer<Click>>,
    selectable_query: Query<Entity, With<Sprite>>,
    mut debug_suite_ui_state: ResMut<PrimaryWindowUiState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let selected = &mut debug_suite_ui_state.selected_entities;
    
    if !events.is_empty() {
        warn!("Handling {} pointer click events (this should be 1) when clicking once", events.len());
    }

    if events.len() > 1 {
        warn!("Multiple pointer click events detected in a single frame; this may indicate an issue with input handling.");
        warn!("Events: {:?}", events.read().collect::<Vec<_>>());
    }

    for event in events.read() {
        if selectable_query.get(event.target).is_err() {
            continue;
        }

        let shift = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);

        if shift {
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