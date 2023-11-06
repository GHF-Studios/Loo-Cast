// Modules

// Local imports

// Internal imports
use crate::game::SimulationState;
use crate::player::Player;
use crate::AppState;

// External imports
use bevy::prelude::*;

// Static variables

// Constant variables

// Types

// Enums

// Structs
pub struct IterationTestPlugin;

#[derive(Component)]
pub struct DebugTextPanel;

#[derive(Component)]
pub struct ScenePositionText;

#[derive(Component)]
pub struct LocalChunkPositionText;

#[derive(Component)]
pub struct CurrentScaleIndexText;

#[derive(Component)]
pub struct GlobalChunkPositionText;

#[derive(Resource)]
pub struct IterationTestManager {
    origin_shift_x: i32,
    origin_shift_y: i32,
    current_scale_index: u8,
}

// Implementations
impl Plugin for IterationTestPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IterationTestManager {
            origin_shift_x: 0,
            origin_shift_y: 0,
            current_scale_index: 0,
        })
        // Enter Systems
        .add_systems(OnEnter(AppState::Game), IterationTestManager::initialize)
        // Update Systems
        .add_systems(
            Update,
            (
                IterationTestManager::update_scene_position_system,
                IterationTestManager::update_local_chunk_position_system,
                IterationTestManager::update_global_chunk_position_system,
            )
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),
        )
        // Exit Systems
        .add_systems(OnExit(AppState::Game), IterationTestManager::terminate);
    }
}

impl IterationTestManager {
    fn initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        row_gap: Val::Px(8.0),
                        column_gap: Val::Px(8.0),
                        ..default()
                    },
                    ..default()
                },
                DebugTextPanel {},
            ))
            .with_children(|parent| {
                parent.spawn((
                    TextBundle::from_sections([
                        TextSection::new(
                            "Scene Position: ",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::from_style(TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        }),
                    ]),
                    ScenePositionText,
                ));
                parent.spawn((
                    TextBundle::from_sections([
                        TextSection::new(
                            "Local Chunk Position: ",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::from_style(TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        }),
                    ]),
                    LocalChunkPositionText,
                ));
                parent.spawn((
                    TextBundle::from_sections([TextSection::new(
                        "Current Scale Index: 0",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    )]),
                    CurrentScaleIndexText,
                ));
                parent.spawn((
                    TextBundle::from_sections([
                        TextSection::new(
                            "Global Chunk Position: ",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::from_style(TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        }),
                    ]),
                    GlobalChunkPositionText,
                ));
            });
    }

    fn terminate(
        mut commands: Commands,
        debug_text_panel_query: Query<Entity, With<DebugTextPanel>>,
    ) {
        if let Ok(entity) = debug_text_panel_query.get_single() {
            commands.entity(entity).despawn();
        }
    }

    fn update_scene_position_system(
        mut text_query: Query<&mut Text, With<ScenePositionText>>,
        player_query: Query<&Transform, With<Player>>,
    ) {
        for mut text in &mut text_query {
            if let Ok(player_transform) = player_query.get_single() {
                text.sections[1].value = format!(
                    "x: {:.2}, y: {:.2}",
                    player_transform.translation.x, player_transform.translation.y
                );
            }
        }
    }

    fn update_local_chunk_position_system(
        mut text_query: Query<&mut Text, With<LocalChunkPositionText>>,
        player_query: Query<&Transform, With<Player>>,
    ) {
        for mut text in &mut text_query {
            if let Ok(player_transform) = player_query.get_single() {
                let local_chunk_pos = local_chunk_pos_from_entity_pos(
                    (
                        player_transform.translation.x,
                        player_transform.translation.y,
                    ),
                    crate::universe::chunk::CHUNK_SIZE,
                );
                text.sections[1].value =
                    format!("x: {:.2}, y: {:.2}", local_chunk_pos.0, local_chunk_pos.1);
            }
        }
    }

    fn update_global_chunk_position_system(
        mut text_query: Query<&mut Text, With<GlobalChunkPositionText>>,
        player_query: Query<&Transform, With<Player>>,
    ) {
        for mut text in &mut text_query {
            if let Ok(player_transform) = player_query.get_single() {
                let global_chunk_pos = get_global_chunk_pos(
                    (
                        player_transform.translation.x,
                        player_transform.translation.y,
                    ),
                    crate::universe::chunk::CHUNK_SIZE,
                    0,
                );
                text.sections[1].value = format!("{:?}", global_chunk_pos);
            }
        }
    }
}

// Module Functions
pub fn local_chunk_pos_from_entity_pos(scene_pos: (f32, f32), chunk_size: u16) -> (i32, i32) {
    let half_chunk = (chunk_size as f32) / 2.0;
    let x = ((scene_pos.0 + half_chunk) / chunk_size as f32).floor() as i32;
    let y = ((scene_pos.1 + half_chunk) / chunk_size as f32).floor() as i32;

    (x, y)
}

pub fn entity_pos_from_local_chunk_pos(local_chunk_pos: (i32, i32), chunk_size: u16) -> (f32, f32) {
    let x = local_chunk_pos.0 as f32 * chunk_size as f32;
    let y = local_chunk_pos.1 as f32 * chunk_size as f32;

    (x, y)
}

pub fn get_global_chunk_pos(
    scene_pos: (f32, f32),
    chunk_size: u16,
    current_scale_index: u8,
) -> Vec<(u8, u8)> {
    vec![(0, 0)]
}
