// Modules

// Local imports

// Internal imports
use crate::system::game::SimulationState;
use crate::system::player::Player;
use crate::system::AppState;

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
pub struct LocalEntityPositionText;

#[derive(Component)]
pub struct AbsoluteLocalChunkPositionText;

#[derive(Component)]
pub struct ApparentLocalChunkPositionText;

#[derive(Component)]
pub struct CurrentScaleIndexText;

#[derive(Component)]
pub struct GlobalChunkPositionText;

#[derive(Resource)]
pub struct IterationTestManager {}

// Implementations
impl Plugin for IterationTestPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(IterationTestManager {})
            // Enter Systems
            .add_systems(OnEnter(AppState::Game), IterationTestManager::initialize)
            // Update Systems
            .add_systems(
                Update,
                (
                    IterationTestManager::update_scene_position_system,
                    IterationTestManager::update_absolute_local_chunk_position_system,
                    IterationTestManager::update_apparent_local_chunk_position_system,
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
                            "Local Entity Position: ",
                            TextStyle {
                                font: asset_server
                                    .load("loo_cast_base_mod/resources/fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::from_style(TextStyle {
                            font: asset_server
                                .load("loo_cast_base_mod/resources/fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        }),
                    ]),
                    LocalEntityPositionText,
                ));
                parent.spawn((
                    TextBundle::from_sections([
                        TextSection::new(
                            "Absolute Local Chunk Position: ",
                            TextStyle {
                                font: asset_server
                                    .load("loo_cast_base_mod/resources/fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::from_style(TextStyle {
                            font: asset_server
                                .load("loo_cast_base_mod/resources/fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        }),
                    ]),
                    AbsoluteLocalChunkPositionText,
                ));
                parent.spawn((
                    TextBundle::from_sections([
                        TextSection::new(
                            "Apparent Local Chunk Position: ",
                            TextStyle {
                                font: asset_server
                                    .load("loo_cast_base_mod/resources/fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::from_style(TextStyle {
                            font: asset_server
                                .load("loo_cast_base_mod/resources/fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::WHITE,
                        }),
                    ]),
                    ApparentLocalChunkPositionText,
                ));
                parent.spawn((
                    TextBundle::from_sections([TextSection::new(
                        "Current Scale Index: 0",
                        TextStyle {
                            font: asset_server
                                .load("loo_cast_base_mod/resources/fonts/FiraSans-Bold.ttf"),
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
                                font: asset_server
                                    .load("loo_cast_base_mod/resources/fonts/FiraSans-Bold.ttf"),
                                font_size: 20.0,
                                color: Color::WHITE,
                            },
                        ),
                        TextSection::from_style(TextStyle {
                            font: asset_server
                                .load("loo_cast_base_mod/resources/fonts/FiraSans-Bold.ttf"),
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
        debug_text_panel_query: Query<bevy::prelude::Entity, With<DebugTextPanel>>,
    ) {
        if let Ok(entity) = debug_text_panel_query.get_single() {
            commands.entity(entity).despawn();
        }
    }

    fn update_scene_position_system(
        mut text_query: Query<&mut Text, With<LocalEntityPositionText>>,
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

    fn update_absolute_local_chunk_position_system(
        mut text_query: Query<&mut Text, With<AbsoluteLocalChunkPositionText>>,
        player_query: Query<&Transform, With<Player>>,
    ) {
        for mut text in &mut text_query {
            if let Ok(player_transform) = player_query.get_single() {
                let local_entity_pos: LocalEntityPos = player_transform.translation.into();
                let absolute_local_chunk_pos: AbsoluteLocalChunkPos = local_entity_pos.into();
                text.sections[1].value = format!(
                    "x: {:.2}, y: {:.2}",
                    absolute_local_chunk_pos.x, absolute_local_chunk_pos.y
                );
            }
        }
    }

    fn update_apparent_local_chunk_position_system(
        mut text_query: Query<&mut Text, With<ApparentLocalChunkPositionText>>,
        player_query: Query<&Transform, With<Player>>,
    ) {
        for mut text in &mut text_query {
            if let Ok(player_transform) = player_query.get_single() {
                let local_entity_pos: LocalEntityPos = player_transform.translation.into();
                let apparent_local_chunk_pos: ApparentLocalChunkPos = local_entity_pos.into();
                text.sections[1].value = format!(
                    "x: {:.2}, y: {:.2}",
                    apparent_local_chunk_pos.x, apparent_local_chunk_pos.y
                );
            }
        }
    }

    fn update_global_chunk_position_system(
        mut text_query: Query<&mut Text, With<GlobalChunkPositionText>>,
        player_query: Query<&Transform, With<Player>>,
    ) {
        for mut text in &mut text_query {
            if let Ok(_player_transform) = player_query.get_single() {
                let global_chunk_pos = vec![(0, 0)];
                text.sections[1].value = format!("{:?}", global_chunk_pos);
            }
        }
    }
}

// Module Functions
