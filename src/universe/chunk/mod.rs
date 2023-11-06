// Modules

// Local imports

// Internal imports
use crate::game::SimulationState;
use crate::AppState;

// External imports
use bevy::prelude::*;
use std::collections::HashMap;

// Static variables

// Constant variables
pub const VIEW_RADIUS: u16 = 4;
pub const CHUNK_SIZE: u16 = 64;

// Types

// Enums

// Structs
pub struct ChunkPlugin;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ChunkPos {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Chunk {
    pub local_chunk_pos: ChunkPos,
}

#[derive(Component)]
pub struct ChunkViewer {
    previously_viewed_chunk_positions: Vec<ChunkPos>,
    currently_viewed_chunk_positions: Vec<ChunkPos>,
    newly_viewed_chunk_positions: Vec<ChunkPos>,
}

#[derive(Resource)]
pub struct ChunkManager {
    loaded_chunks: HashMap<ChunkPos, Entity>,
    load_requests: Vec<ChunkPos>,
    unload_requests: Vec<ChunkPos>,
}

// Implementations
impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter Systems
            .add_systems(OnEnter(AppState::Game), ChunkManager::initialize)
            // Update Systems
            .add_systems(
                Update,
                (
                    ChunkManager::handle_load_requests,
                    ChunkManager::handle_unload_requests,
                    Chunk::render_system,
                    ChunkViewer::detect_chunks_system,
                )
                    .run_if(in_state(AppState::Game))
                    .run_if(in_state(SimulationState::Running)),
            )
            // Exit Systems
            .add_systems(OnExit(AppState::Game), ChunkManager::terminate);
    }
}

impl ChunkManager {
    fn initialize(mut commands: Commands) {
        let chunk_manager = Self {
            loaded_chunks: HashMap::new(),
            load_requests: Vec::new(),
            unload_requests: Vec::new(),
        };

        commands.insert_resource(chunk_manager);
    }

    fn terminate(mut commands: Commands) {
        commands.remove_resource::<Self>();
    }

    pub fn request_load(&mut self, chunk_pos: ChunkPos) {
        self.load_requests.push(chunk_pos);
    }

    pub fn request_unload(&mut self, chunk_pos: ChunkPos) {
        self.unload_requests.push(chunk_pos);
    }

    fn handle_load_requests(mut commands: Commands, mut chunk_manager: ResMut<ChunkManager>) {
        let load_requests = chunk_manager.load_requests.clone();
        for chunk_pos in load_requests {
            match chunk_manager.loaded_chunks.get(&chunk_pos) {
                Some(_) => panic!("Chunk already loaded"),
                None => {
                    let chunk_entity = commands
                        .spawn(Chunk {
                            local_chunk_pos: chunk_pos,
                        })
                        .id();
                    chunk_manager.loaded_chunks.insert(chunk_pos, chunk_entity);
                }
            }
        }
        chunk_manager.load_requests.clear();
    }

    fn handle_unload_requests(mut commands: Commands, mut chunk_manager: ResMut<ChunkManager>) {
        let unload_requests = chunk_manager.unload_requests.clone();
        for chunk_pos in unload_requests {
            match chunk_manager.loaded_chunks.get(&chunk_pos) {
                Some(chunk_entity) => {
                    commands.entity(*chunk_entity).despawn();
                    chunk_manager.loaded_chunks.remove(&chunk_pos);
                }
                None => panic!("Chunk not loaded"),
            }
        }
        chunk_manager.unload_requests.clear();
    }
}

impl ChunkPos {
    pub fn new(x: i32, y: i32) -> ChunkPos {
        Self { x, y }
    }
}

impl Chunk {
    fn render_system(mut gizmos: Gizmos, chunk_query: Query<&Chunk>) {
        for chunk in chunk_query.iter() {
            let chunk_pos = chunk.local_chunk_pos;
            let gizmo_position = crate::iteration_test::entity_pos_from_local_chunk_pos(
                (chunk_pos.x, chunk_pos.y),
                CHUNK_SIZE,
            );
            let gizmo_position = Vec2::new(gizmo_position.0, gizmo_position.1);
            gizmos.rect_2d(
                gizmo_position,
                0.,
                Vec2::splat(CHUNK_SIZE as f32),
                Color::RED,
            );
        }
    }
}

impl ChunkViewer {
    pub fn new() -> ChunkViewer {
        Self {
            previously_viewed_chunk_positions: Vec::new(),
            currently_viewed_chunk_positions: Vec::new(),
            newly_viewed_chunk_positions: Vec::new(),
        }
    }

    fn detect_chunks_system(
        mut chunk_viewer_query: Query<(&mut ChunkViewer, &Transform)>,
        chunk_manager: ResMut<ChunkManager>,
    ) {
        Self::gather_chunk_positions(&mut chunk_viewer_query);
        Self::process_chunk_positions(&mut chunk_viewer_query, chunk_manager);
    }

    fn gather_chunk_positions(
        chunk_viewer_query: &mut Query<(&mut ChunkViewer, &Transform)>,
    ) {
        for (mut chunk_viewer, chunk_viewer_transform) in chunk_viewer_query.iter_mut() {
            if chunk_viewer.previously_viewed_chunk_positions.len() > 0 {
                panic!("Chunk viewer's previously viewed chunk positions are not empty");
            }
            if chunk_viewer.newly_viewed_chunk_positions.len() > 0 {
                panic!("Chunk viewer's newly viewed chunk positions are not empty");
            }

            let chunk_viewer_scene_position = (chunk_viewer_transform.translation.x, chunk_viewer_transform.translation.y);
            let chunk_viewer_local_chunk_position = crate::iteration_test::local_chunk_pos_from_entity_pos(chunk_viewer_scene_position, CHUNK_SIZE);
            let chunk_viewer_local_chunk_position = ChunkPos::new(chunk_viewer_local_chunk_position.0, chunk_viewer_local_chunk_position.1);
            let detected_chunk_positions = Self::get_chunks_in_range(&chunk_viewer_local_chunk_position);
            let currently_viewed_chunk_positions = chunk_viewer.currently_viewed_chunk_positions.clone();

            for currently_viewed_chunk_position in currently_viewed_chunk_positions {
                if !detected_chunk_positions.contains(&currently_viewed_chunk_position) {
                    chunk_viewer.previously_viewed_chunk_positions.push(currently_viewed_chunk_position);
                }
            }

            for detected_chunk_position in &detected_chunk_positions {
                if !chunk_viewer.currently_viewed_chunk_positions.contains(detected_chunk_position) {
                    chunk_viewer.newly_viewed_chunk_positions.push(*detected_chunk_position);
                }
            }
        }
    }

    fn process_chunk_positions(
        chunk_viewer_query: &mut Query<(&mut ChunkViewer, &Transform)>,
        mut chunk_manager: ResMut<ChunkManager>,
    ) {
        for (mut chunk_viewer, _) in chunk_viewer_query.iter_mut() {
            // Unload chunks that have exited the view
            let old_chunk_positions = chunk_viewer.previously_viewed_chunk_positions.clone();

            for chunk_pos in &old_chunk_positions {
                chunk_manager.request_unload(*chunk_pos);
            }

            chunk_viewer
                .currently_viewed_chunk_positions
                .retain(|chunk_pos| !old_chunk_positions.contains(chunk_pos));

            chunk_viewer.previously_viewed_chunk_positions.clear();

            // Load chunks that have entered the view
            let mut new_chunks_positions = chunk_viewer.newly_viewed_chunk_positions.clone();

            for chunk_pos in &new_chunks_positions {
                chunk_manager.request_load(*chunk_pos);
            }

            chunk_viewer
                .currently_viewed_chunk_positions
                .append(&mut new_chunks_positions);

            chunk_viewer.newly_viewed_chunk_positions.clear();
        }
    }

    fn get_chunks_in_range(center: &ChunkPos) -> Vec<ChunkPos> {
        let mut chunks = Vec::new();
        let r = VIEW_RADIUS as i32;
        for x in (center.x - r)..=(center.x + r) {
            for y in (center.y - r)..=(center.y + r) {
                chunks.push(ChunkPos { x, y });
            }
        }
        chunks
    }
}

// Module Functions
