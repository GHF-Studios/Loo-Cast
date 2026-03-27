use crate::bevy::prelude::*;

use crate::core::components::Meta;
use crate::player::components::PlayerVisual3dLink;
use crate::picking::constants::{DIEGETIC_MOUSE_POINTER_ID, NO_HIT_SENTINEL};
use crate::render::components::{LogicProxy, RenderProxy};
use crate::render::resources::PrimaryWindowUiState;

#[tracing::instrument(skip_all)]
pub(super) fn handle_selection(
    mut messages: MessageReader<Pointer<Click>>,
    selectable_query: Query<Entity, (Or<(With<Sprite>, With<Mesh3d>)>, Without<Meta<Sprite>>, Without<Meta<Mesh3d>>)>,
    render_proxy_query: Query<&RenderProxy>,
    logic_proxy_query: Query<&LogicProxy>,
    player_visual_link_query: Query<&PlayerVisual3dLink>,
    child_of_query: Query<&ChildOf>,
    mut debug_suite_ui_state: ResMut<PrimaryWindowUiState>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let remap_pick_targets = debug_suite_ui_state.remap_pick_targets_to_source_entities;
    let selected = &mut debug_suite_ui_state.selected_entities;

    for message in messages.read() {
        if message.pointer_id != DIEGETIC_MOUSE_POINTER_ID {
            continue;
        }

        let clicked_target = message.event_target();

        if clicked_target == NO_HIT_SENTINEL {
            warn!("Clicked on empty space; clearing selection.");
            selected.clear();
            continue;
        }

        let target = if remap_pick_targets {
            remap_pick_target_to_source_entity(
                clicked_target,
                &render_proxy_query,
                &logic_proxy_query,
                &player_visual_link_query,
                &child_of_query,
            )
        } else {
            clicked_target
        };

        if remap_pick_targets && target != clicked_target {
            warn!("Remapped picked target {:?} -> {:?}", clicked_target, target);
        }

        if selectable_query.get(target).is_err() {
            warn!("Tried to select non-existent/incompatible entity: {:?}", target);
            continue;
        }

        warn!("Selecting entity: {:?}", target);

        let ctrl = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);

        if ctrl {
            if selected.contains(target) {
                selected.retain(|e| e != target);
            } else {
                selected.select_maybe_add(target, true);
            }
        } else {
            selected.select_replace(target);
        }
    }
}

fn remap_pick_target_to_source_entity(
    picked_entity: Entity,
    render_proxy_query: &Query<&RenderProxy>,
    logic_proxy_query: &Query<&LogicProxy>,
    player_visual_link_query: &Query<&PlayerVisual3dLink>,
    child_of_query: &Query<&ChildOf>,
) -> Entity {
    if let Ok(render_proxy) = render_proxy_query.get(picked_entity) {
        return render_proxy.source;
    }
    if let Ok(logic_proxy) = logic_proxy_query.get(picked_entity) {
        return logic_proxy.source;
    }

    let mut current = picked_entity;
    for _ in 0..32 {
        let Ok(child_of) = child_of_query.get(current) else {
            break;
        };
        let parent = child_of.parent();

        if let Ok(render_proxy) = render_proxy_query.get(parent) {
            return render_proxy.source;
        }
        if let Ok(logic_proxy) = logic_proxy_query.get(parent) {
            return logic_proxy.source;
        }
        if let Ok(visual_link) = player_visual_link_query.get(parent)
            && visual_link.entity == current
        {
            return parent;
        }

        current = parent;
    }

    picked_entity
}
