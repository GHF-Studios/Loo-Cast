use bevy::prelude::*;

use crate::core::components::Meta;
use crate::picking::constants::{DIEGETIC_MOUSE_POINTER_ID, NO_HIT_SENTINEL};
use crate::render::resources::PrimaryWindowUiState;

#[tracing::instrument(skip_all)]
pub(super) fn handle_selection(
    mut events: EventReader<Pointer<Click>>,
    selectable_query: Query<Entity, (With<Sprite>, Without<Meta<Sprite>>)>,
    mut debug_suite_ui_state: ResMut<PrimaryWindowUiState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let selected = &mut debug_suite_ui_state.selected_entities;

    for event in events.read() {
        if event.pointer_id != DIEGETIC_MOUSE_POINTER_ID {
            continue;
        }

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
