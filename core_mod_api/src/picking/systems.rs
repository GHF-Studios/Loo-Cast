use bevy::input::ButtonState;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy::ecs::query::QuerySingleError;
use bevy::picking::pointer::{Location, PointerAction, PointerButton, PointerId, PointerInput, PointerLocation,};
use bevy::picking::backend::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::window::{PrimaryWindow, WindowEvent, WindowRef};

use crate::camera::components::MainCamera;
use crate::debug::resources::DebugSuiteUiState;
use crate::player::components::Player;

/// Sends mouse pointer events to *`hopefully`* be processed by the core plugin
/// This silently early-returns if any other non-primary windows (or not window at all) are detected
pub(super) fn mouse_pick_events(
    mut window_events: EventReader<WindowEvent>,
    mut pointer_events: EventWriter<PointerInput>,
    mut cursor_last: Local<Vec2>,
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
    println!("6: viewport.contains(egui::Pos2::new(cursor_pos.x, cursor_pos.y))");

    for window_event in window_events.read() {
        println!("7: window_event");
        match window_event {
            // Handle cursor movement events
            WindowEvent::CursorMoved(event) => {
                println!("8: CursorMoved");
                let location = Location {
                    target: match RenderTarget::Window(WindowRef::Primary)
                        .normalize(Some(primary_window_entity))
                    {
                        Some(target) => target,
                        None => continue,
                    },
                    position: event.position,
                };
                println!("9: CursorMoved");
                pointer_events.write(PointerInput::new(
                    PointerId::Mouse,
                    location,
                    PointerAction::Move {
                        delta: event.position - *cursor_last,
                    },
                ));
                *cursor_last = event.position;
            }
            // Handle mouse button press events
            WindowEvent::MouseButtonInput(input) => {
                println!("8: MouseButtonInput");
                let location = Location {
                    target: match RenderTarget::Window(WindowRef::Primary)
                        .normalize(Some(primary_window_entity))
                    {
                        Some(target) => target,
                        None => continue,
                    },
                    position: *cursor_last,
                };
                println!("9: MouseButtonInput");
                let button = match input.button {
                    MouseButton::Left => PointerButton::Primary,
                    MouseButton::Right => PointerButton::Secondary,
                    MouseButton::Middle => PointerButton::Middle,
                    MouseButton::Other(_) | MouseButton::Back | MouseButton::Forward => continue,
                };
                println!("10: MouseButtonInput");
                let action = match input.state {
                    ButtonState::Pressed => PointerAction::Press(button),
                    ButtonState::Released => PointerAction::Release(button),
                };
                pointer_events.write(PointerInput::new(PointerId::Mouse, location, action));
            }
            WindowEvent::MouseWheel(event) => {
                println!("8: MouseWheel");
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
                println!("9: MouseWheel");

                let action = PointerAction::Scroll { x, y, unit };

                pointer_events.write(PointerInput::new(PointerId::Mouse, location, action));
            }
            _ => {}
        }
    }
}

pub(super) fn sprite_picking_backend(
    pointers: Query<(&PointerId, &PointerLocation)>,
    main_camera_query: Query<(Entity, &Camera), With<MainCamera>>,
    player_query: Query<(Entity, &GlobalTransform), With<Player>>,
    mut output: EventWriter<PointerHits>,
) {
    let (pointer_id, _) = match pointers.single() {
        Ok(value) => value,
        Err(err) => match err {
            QuerySingleError::NoEntities(_) => {
                warn!("No pointer found");
                return
            },
            QuerySingleError::MultipleEntities(_) => panic!("Multiple Pointers not supported!"),
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