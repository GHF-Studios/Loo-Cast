use crate::{
    camera::components::MainCamera,
    chunk::{components::ChunkComponent, functions::world_pos_to_chunk, resources::ChunkManager},
    chunk_loader::components::ChunkLoaderComponent,
};

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{
    egui::{self, ScrollArea},
    EguiContexts,
};

use super::components::{DebugObjectComponent, DebugObjectMovement};

pub(super) fn debug_ui_startup(mut has_spawned: Local<bool>, mut commands: Commands) {
    use iyes_perf_ui::{
        entries::{PerfUiFramerateEntries, PerfUiSystemEntries},
        prelude::{PerfUiEntryEntityCount, PerfUiRoot},
    };

    if !*has_spawned {
        *has_spawned = true;
        commands.spawn((
            PerfUiRoot::default(),
            PerfUiFramerateEntries::default(),
            PerfUiSystemEntries::default(),
            PerfUiEntryEntityCount::default(),
        ));
    }
}

pub(super) fn debug_object_movement_system(time: Res<Time>, mut query: Query<(&mut Transform, &DebugObjectComponent)>) {
    for (mut transform, debug_object) in query.iter_mut() {
        match &debug_object.movement {
            DebugObjectMovement::Static => {}
            DebugObjectMovement::Circle { radius, speed } => {
                let time_factor = time.elapsed_seconds() * speed;
                transform.translation.x = radius * time_factor.cos();
                transform.translation.y = radius * time_factor.sin();
            }
            DebugObjectMovement::Line { distance, speed } => {
                let time_factor = time.elapsed_seconds() * speed;
                let offset = time_factor.sin() * distance;
                transform.translation.x = offset;
            }
        }
    }
}

pub(super) fn chunk_inspection_system(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    chunk_query: Query<&ChunkComponent>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        let (camera, camera_transform) = camera_query.single();
        let window = window_query.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let chunk_coord = world_pos_to_chunk(world_position);
            if let Some(chunk) = chunk_query.iter().find(|chunk| chunk.coord == chunk_coord) {
                debug!("Inspecting chunk: {:?}", chunk);
            }
        }
    }
}

pub(super) fn chunk_loader_inspection_system(chunk_loader_query: Query<Entity, With<ChunkLoaderComponent>>, keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::KeyL) {
        let mut chunk_loader_entities = vec![];
        for chunk_loader_entity in chunk_loader_query.iter() {
            chunk_loader_entities.push(chunk_loader_entity);
        }

        debug!("Inspecting chunk loaders: {:?}", chunk_loader_entities);
    }
}

pub fn chunk_manager_debug_ui(chunk_manager: Res<ChunkManager>, mut egui_ctxs: EguiContexts) {
    const GROUP_SIZE: usize = 50;

    fn display_chunk_group<T: std::fmt::Debug>(ui: &mut egui::Ui, label: &str, items: impl Iterator<Item = T>) {
        let grouped = items.collect::<Vec<_>>();
        let total = grouped.len();

        for (i, group) in grouped.chunks(GROUP_SIZE).enumerate() {
            let range = i * GROUP_SIZE..(i + 1) * GROUP_SIZE;
            let range_end = range.end.min(total);
            let header = format!("{label} [{}–{})", range.start, range_end);

            ui.collapsing(header, |ui| {
                for item in group {
                    ui.monospace(format!(" - {:?}", item));
                }
            });
        }
    }

    egui::Window::new("Chunk Manager")
        .vscroll(true) // Optional: ensures vertical scrollbar appears
        .show(egui_ctxs.ctx_mut(), |ui| {
            ScrollArea::vertical()
                .auto_shrink([false; 2]) // Prevents weird size compression
                .show(ui, |ui| {
                    ui.collapsing("Loaded Chunks", |ui| {
                        display_chunk_group(ui, "Loaded", chunk_manager.loaded_chunks.iter());
                    });

                    ui.collapsing("Owned Chunks", |ui| {
                        display_chunk_group(
                            ui,
                            "Owned",
                            chunk_manager
                                .owned_chunks
                                .iter()
                                .map(|(coord, owner_id)| format!("{:?} → {:?}", coord, owner_id)),
                        );
                    });
                });
        });
}
