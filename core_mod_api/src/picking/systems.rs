use bevy::input::ButtonState;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::ecs::query::QuerySingleError;
use bevy::picking::pointer::{Location, PointerAction, PointerButton, PointerId, PointerInput, PointerLocation, PointerPress};
use bevy::picking::backend::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::{PrimaryWindow, WindowEvent, WindowRef};

use crate::camera::components::MainCamera;
use crate::debug::resources::DebugSuiteUiState;
use crate::player::components::Player;
use crate::reflect::functions::get_struct_field_mut;

use super::constants::MOUSE_POINTER_ID;

pub(super) fn spawn_mouse_pointer(mut commands: Commands) {
    commands.spawn(MOUSE_POINTER_ID);
}

/// Sends mouse pointer events to *`hopefully`* be processed by the core plugin
/// This silently early-returns if any other non-primary windows (or not window at all) are detected
#[tracing::instrument(skip_all)]
pub(super) fn mouse_pick_events(
    mut window_events: EventReader<WindowEvent>,
    mut pointer_event_writer: EventWriter<PointerInput>,
    mut cursor_last: Local<Vec2>,
    mut pointers: Query<(&PointerId, &mut PointerLocation, &mut PointerPress)>,
    primary_window: Query<(Entity, &Window), With<PrimaryWindow>>,
    ui_state: Res<DebugSuiteUiState>,
) {
    if !ui_state.enabled || window_events.is_empty() {
        return;
    }

    let Ok((primary_window_entity, primary_window)) = primary_window.single() else { return };
    let Some(cursor_pos) = primary_window.cursor_position() else { return };
    let Some(viewport) = ui_state.viewport_rect_precision_proxy else { return };

    // Only inject pointer if it's within the egui image viewport
    if !viewport.contains(egui::Pos2::new(cursor_pos.x, cursor_pos.y)) {
        return;
    }

    let mut pointer_events: Vec<PointerInput> = Vec::with_capacity(window_events.len());

    for window_event in window_events.read() {
        match window_event {
            WindowEvent::CursorMoved(event) => {
                let location = Location {
                    target: match RenderTarget::Window(WindowRef::Primary)
                        .normalize(Some(primary_window_entity))
                    {
                        Some(target) => target,
                        None => continue,
                    },
                    position: event.position,
                };
                let pointer_input = PointerInput::new(
                    MOUSE_POINTER_ID,
                    location,
                    PointerAction::Move {
                        delta: event.position - *cursor_last,
                    },
                );
                pointer_events.push(pointer_input.clone());
                *cursor_last = event.position;
            }
            WindowEvent::MouseButtonInput(input) => {
                let location = Location {
                    target: match RenderTarget::Window(WindowRef::Primary)
                        .normalize(Some(primary_window_entity))
                    {
                        Some(target) => target,
                        None => continue,
                    },
                    position: *cursor_last,
                };
                let button = match input.button {
                    MouseButton::Left => PointerButton::Primary,
                    MouseButton::Right => PointerButton::Secondary,
                    MouseButton::Middle => PointerButton::Middle,
                    MouseButton::Other(_) | MouseButton::Back | MouseButton::Forward => continue,
                };
                let action = match input.state {
                    ButtonState::Pressed => PointerAction::Press(button),
                    ButtonState::Released => PointerAction::Release(button),
                };
                let pointer_input = PointerInput::new(MOUSE_POINTER_ID, location, action);
                pointer_events.push(pointer_input.clone());
            }
            WindowEvent::MouseWheel(event) => {
                let MouseWheel { unit, x, y, window: _ } = *event;
                let location = Location {
                    target: match RenderTarget::Window(WindowRef::Primary)
                        .normalize(Some(primary_window_entity))
                    {
                        Some(target) => target,
                        None => continue,
                    },
                    position: *cursor_last,
                };
                let action = PointerAction::Scroll { x, y, unit };
                let pointer_input = PointerInput::new(MOUSE_POINTER_ID, location, action);
                pointer_events.push(pointer_input.clone());
            }
            _ => {}
        }
    }

    for event in pointer_events.into_iter() {
        match event.action {
            PointerAction::Press(ref button) => {
                pointers
                    .iter_mut()
                    .for_each(|(pointer_id, _, mut pointer)| {
                        if *pointer_id == event.pointer_id {
                            match button {
                                PointerButton::Primary => *get_struct_field_mut(&mut *pointer, "primary") = true,
                                PointerButton::Secondary => *get_struct_field_mut(&mut *pointer, "secondary") = true,
                                PointerButton::Middle => *get_struct_field_mut(&mut *pointer, "middle") = true,
                            }
                        }
                    });
            }
            PointerAction::Release(ref button) => {
                pointers
                    .iter_mut()
                    .for_each(|(pointer_id, _, mut pointer)| {
                        if *pointer_id == event.pointer_id {
                            match button {
                                PointerButton::Primary => *get_struct_field_mut(&mut *pointer, "primary") = false,
                                PointerButton::Secondary => *get_struct_field_mut(&mut *pointer, "secondary") = false,
                                PointerButton::Middle => *get_struct_field_mut(&mut *pointer, "middle") = false,
                            }
                        }
                    });
            }
            PointerAction::Move { .. } => {
                pointers.iter_mut().for_each(|(id, mut pointer, _)| {
                    if *id == event.pointer_id {
                        pointer.location = Some(event.location.to_owned());
                    }
                });
            }
            _ => {}
        }

        pointer_event_writer.write(event);
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn sprite_picking_backend(
    pointers: Query<(&PointerId, &PointerLocation)>,
    main_camera_query: Query<(Entity, &Camera), With<MainCamera>>,
    player_query: Query<(Entity, &GlobalTransform), With<Player>>,
    mut output: EventWriter<PointerHits>,
) {
    let (pointer_id, _) = match pointers.iter().find(|(p_id, _)| **p_id == MOUSE_POINTER_ID) {
        Some(value) => value,
        None => {
            warn!("Pointer not found");
            return
        }
    };

    let (main_camera_entity, main_camera) = match main_camera_query.single() {
        Ok(value) => value,
        Err(err) => match err {
            QuerySingleError::NoEntities(_) => {
                warn!("No main camera found");
                return
            },
            QuerySingleError::MultipleEntities(_) => panic!("Multiple MainCameras not supported!"),
        }
    };
    let (player_entity, player_transform) = match player_query.single() {
        Ok(value) => value,
        Err(err) => match err {
            QuerySingleError::NoEntities(_) => {
                warn!("No player found");
                return
            },
            QuerySingleError::MultipleEntities(_) => panic!("Multiple Players not supported!"),
        },
    };

    let picks = vec![(
        player_entity,
        HitData::new(
            main_camera_entity,
            0.0,
            Some(Vec3::ZERO),
            Some(*player_transform.back()),
        ),
    )];

    let order = main_camera.order as f32;
    output.write(PointerHits::new(*pointer_id, picks, order));
}