// Modules

// Local imports

// Internal imports
use crate::kernel::manager::*;
use crate::system::game::SimulationState;
use crate::system::player::Player;
use crate::system::universe::chunk::pos::*;
use crate::system::universe::entity::pos::*;
use crate::system::AppState;

// External imports
use bevy::prelude::*;
use lazy_static::*;
use std::sync::{Arc, Mutex};

// Static variables
lazy_static! {
    pub static ref TEST_MANAGER: Arc<Mutex<TestManager>> = Arc::new(Mutex::new(TestManager::new()));
}

// Constant variables

// Types

// Enums

// Structs
pub enum TestState {
    Stopped,
    Running
}

pub struct TestPlugin;

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

pub struct TestManager {
    test_state: TestState,
    manager_state: ManagerState,
}

// Implementations
impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter Systems
            .add_systems(OnEnter(AppState::Game), TestManager::start_test)
            // Update Systems
            .add_systems(
                Update,
                (
                    TestManager::update_scene_position_system,
                    TestManager::update_absolute_local_chunk_position_system,
                    TestManager::update_apparent_local_chunk_position_system,
                    TestManager::update_global_chunk_position_system,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit Systems
            .add_systems(OnExit(AppState::Game), TestManager::stop_test);
    }
}

impl Manager for TestManager {
    fn initialize(&mut self) -> Result<(), ManagerInitializeError> {
        match self.manager_state {
            ManagerState::Created => {}
            ManagerState::Initialized => {
                return Err(ManagerInitializeError::ManagerAlreadyInitialized);
            }
            ManagerState::Finalized => {
                return Err(ManagerInitializeError::ManagerAlreadyFinalized);
            }
        }

        self.manager_state = ManagerState::Initialized;

        Ok(())
    }

    fn finalize(&mut self) -> Result<(), ManagerFinalizeError> {
        match self.manager_state {
            ManagerState::Created => {
                return Err(ManagerFinalizeError::ManagerNotInitialized);
            }
            ManagerState::Initialized => {}
            ManagerState::Finalized => {
                return Err(ManagerFinalizeError::ManagerAlreadyFinalized);
            }
        }

        self.manager_state = ManagerState::Finalized;

        Ok(())
    }

    fn get_manager_state(&self) -> &ManagerState {
        &self.manager_state
    }
}

impl TestManager {
    fn new() -> TestManager {
        TestManager {
            test_state: TestState::Stopped,
            manager_state: ManagerState::Created,
        }
    }

    fn start_test(mut commands: Commands, asset_server: Res<AssetServer>) {
        let test_manager = TEST_MANAGER.clone();
        let mut test_manager = match test_manager.lock() {
            Ok(test_manager) => {
                trace!("Locked test manager mutex.");
                test_manager
            },
            Err(_) => panic!("Failed to lock test manager mutex!"),
        };

        match test_manager.test_state {
            TestState::Stopped => {}
            TestState::Running => {
                error!("Test already running!");
                return;
            }
        };

        drop(test_manager);

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

        test_manager.test_state = TestState::Running;
    }

    fn stop_test(
        mut commands: Commands,
        debug_text_panel_query: Query<bevy::prelude::Entity, With<DebugTextPanel>>,
    ) {
        if let Ok(entity) = debug_text_panel_query.get_single() {
            let test_manager = TEST_MANAGER.clone();
            let mut test_manager = match test_manager.lock() {
                Ok(test_manager) => {
                    trace!("Locked test manager mutex.");
                    test_manager
                },
                Err(_) => panic!("Failed to lock test manager mutex!"),
            };

            match test_manager.test_state {
                TestState::Stopped => {
                    error!("Test already stopped!");
                    return;
                }
                TestState::Running => {}
            };

            drop(test_manager);

            commands.entity(entity).despawn();

            test_manager.test_state = TestState::Stopped;
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
