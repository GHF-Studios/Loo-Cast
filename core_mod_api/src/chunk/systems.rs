use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use core_mod_macros::{composite_workflow, composite_workflow_return};
use std::marker::PhantomData;

use crate::chunk::types::ChunkOwnerId;
use crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput;
use crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput;
use crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput;
use crate::chunk_loader::components::ChunkLoader;
use crate::chunk_loader::resources::RemovedChunkLoaders;
use crate::config::statics::CONFIG;
use crate::usf::scale::*;
use crate::utils::components::InitHook;
use crate::workflow::functions::handle_composite_workflow_return_now;

use super::components::Chunk;
use super::functions::{chunk_pos_to_world, world_pos_to_chunk};
use super::intent::ActionIntent;
use super::resources::ChunkRenderHandles;
use super::types::ChunkActionWorkflowHandles;
use super::ActionIntentCommitBuffer;

#[tracing::instrument(skip_all)]
pub(crate) fn chunk_startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    let workgroup_size_x = CONFIG().get::<u32>("gpu/texture_generator/workgroup_size_x");
    let workgroup_size_y = CONFIG().get::<u32>("gpu/texture_generator/workgroup_size_y");
    let workgroup_size_total = workgroup_size_x * workgroup_size_y;
    let chunk_size = CONFIG().get::<u32>("chunk/size");

    if chunk_size % workgroup_size_total != 0 {
        panic!("Chunk size {} is not divisible by {workgroup_size_x}x{workgroup_size_y}={workgroup_size_total}(the configured texture generator shader workgroup size)", chunk_size);
    }

    let quad = meshes.add(Mesh::from(Rectangle::new(1.0, 1.0)));
    let light_material: Handle<ColorMaterial> = materials.add(ColorMaterial::from_color(Color::srgb(0.75, 0.75, 0.75)));
    let dark_material = materials.add(ColorMaterial::from_color(Color::srgb(0.25, 0.25, 0.25)));

    commands.insert_resource(ChunkRenderHandles {
        quad,
        light_material,
        dark_material,
    });
}

#[tracing::instrument(skip_all)]
pub(crate) fn chunk_update_system<S: ConstScale>(
    mut commands: Commands,
    chunk_query: Query<(Entity, &Transform, &Chunk)>,
    removed_chunk_loaders: Res<RemovedChunkLoaders>,
) {
    for (entity, transform, chunk) in chunk_query.iter() {
        let world_pos = transform.translation.truncate();
        let chunk_pos = world_pos_to_chunk(world_pos);

        assert_eq!(chunk.coord, chunk_pos, "Attempted to move chunk entity");
        assert_eq!(chunk_pos_to_world(chunk.coord), world_pos, "Attempted to move chunk entity");

        if let Some(chunk_owner_id) = chunk.owner_id.clone() {
            if removed_chunk_loaders.0.iter().any(|rcl| rcl.id == chunk_owner_id) {
                commands.entity(entity).despawn();
            }
        } else {
            commands.entity(entity).despawn();
        }
    }
}

#[tracing::instrument(skip_all)]
pub(crate) fn process_chunk_actions_system(
    mut chunk_loader_init_hook_query: Query<'w, 's, &'static mut InitHook<ChunkLoader>>,
    mut action_intent_commit_buffer: ResMut<'w, ActionIntentCommitBuffer>,
    mut workflow_handles: Local<Option<ChunkActionWorkflowHandles>>,
) {
    // Step 1: If workflows are running, wait for all to complete
    if let Some(handles) = &mut *workflow_handles {
        let spawn_done = handles.spawn.as_ref().is_none_or(|h| h.is_finished());
        let despawn_done = handles.despawn.as_ref().is_none_or(|h| h.is_finished());
        let transfer_done = handles.transfer.as_ref().is_none_or(|h| h.is_finished());

        if !spawn_done || !despawn_done || !transfer_done {
            //warn!(
            //    "Waiting for chunk action workflows to finish... spawn_done: {}, despawn_done: {}, transfer_done: {}",
            //    spawn_done, despawn_done, transfer_done
            //);
            return;
        }

        // Cleanup finished handles
        if let Some(handle) = handles.spawn.take() {
            handle_composite_workflow_return_now(handle, |ctx| {
                composite_workflow_return!(
                    new_chunk_loaders_scale_quecto_meter_000001: Vec<Entity>,
                    new_chunk_loaders_scale_quecto_meter_00001: Vec<Entity>,
                    new_chunk_loaders_scale_quecto_meter_0001: Vec<Entity>,
                    new_chunk_loaders_scale_quecto_meter_001: Vec<Entity>,
                    new_chunk_loaders_scale_quecto_meter_01: Vec<Entity>,
                    new_chunk_loaders_scale_quecto_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_quecto_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_quecto_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_ronto_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_ronto_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_ronto_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_yocto_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_yocto_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_yocto_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_zepto_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_zepto_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_zepto_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_atto_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_atto_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_atto_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_femto_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_femto_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_femto_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_pico_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_pico_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_pico_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_nano_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_nano_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_nano_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_micro_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_micro_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_micro_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_milli_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_milli_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_milli_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_kilo_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_kilo_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_kilo_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_mega_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_mega_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_mega_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_giga_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_giga_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_giga_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_tera_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_tera_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_tera_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_peta_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_peta_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_peta_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_exa_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_exa_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_exa_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_zetta_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_zetta_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_zetta_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_yotta_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_yotta_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_yotta_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_ronna_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_ronna_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_ronna_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_quetta_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_quetta_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_quetta_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_quetta_meter_1000: Vec<Entity>,
                    new_chunk_loaders_scale_quetta_meter_10000: Vec<Entity>,
                    new_chunk_loaders_scale_quetta_meter_100000: Vec<Entity>,
                );

                for entity in new_chunk_loaders_scale_quecto_meter_000001 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_000001.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quecto_meter_00001 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_00001.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quecto_meter_0001 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_0001.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quecto_meter_001 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_001.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quecto_meter_01 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_01.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quecto_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quecto_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quecto_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_ronto_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronto_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_ronto_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronto_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_ronto_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronto_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_yocto_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yocto_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_yocto_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yocto_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_yocto_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yocto_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_zepto_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zepto_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_zepto_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zepto_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_zepto_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zepto_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_atto_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_atto_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_atto_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_atto_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_atto_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_atto_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_femto_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_femto_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_femto_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_femto_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_femto_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_femto_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_pico_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_pico_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_pico_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_pico_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_pico_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_pico_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_nano_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_nano_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_nano_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_nano_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_nano_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_nano_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_micro_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_micro_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_micro_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_micro_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_micro_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_micro_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_milli_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_milli_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_milli_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_milli_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_milli_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_milli_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_kilo_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_kilo_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_kilo_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_kilo_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_kilo_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_kilo_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_mega_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_mega_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_mega_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_mega_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_mega_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_mega_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_giga_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_giga_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_giga_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_giga_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_giga_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_giga_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_tera_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_tera_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_tera_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_tera_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_tera_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_tera_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_peta_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_peta_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_peta_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_peta_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_peta_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_peta_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_exa_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_exa_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_exa_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_exa_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_exa_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_exa_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_zetta_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zetta_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_zetta_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zetta_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_zetta_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zetta_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_yotta_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yotta_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_yotta_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yotta_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_yotta_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yotta_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_ronna_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronna_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_ronna_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronna_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_ronna_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronna_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quetta_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quetta_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quetta_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quetta_meter_1000 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_1000.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quetta_meter_10000 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_10000.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quetta_meter_100000 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_100000.get_mut(entity) {
                        init_hook.fire();
                    }
                }

                warn!("Finished composite workflow 'SpawnChunks'");
            });
        }
        if let Some(handle) = handles.despawn.take() {
            handle_composite_workflow_return_now(handle, |_ctx| {
                composite_workflow_return!();

                warn!("Finished composite workflow 'DespawnChunks'");
            });
        }
        if let Some(handle) = handles.transfer.take() {
            handle_composite_workflow_return_now(handle, |ctx| {
                composite_workflow_return!(
                    new_chunk_loaders_scale_quecto_meter_000001: Vec<Entity>,
                    new_chunk_loaders_scale_quecto_meter_00001: Vec<Entity>,
                    new_chunk_loaders_scale_quecto_meter_0001: Vec<Entity>,
                    new_chunk_loaders_scale_quecto_meter_001: Vec<Entity>,
                    new_chunk_loaders_scale_quecto_meter_01: Vec<Entity>,
                    new_chunk_loaders_scale_quecto_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_quecto_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_quecto_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_ronto_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_ronto_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_ronto_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_yocto_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_yocto_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_yocto_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_zepto_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_zepto_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_zepto_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_atto_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_atto_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_atto_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_femto_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_femto_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_femto_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_pico_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_pico_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_pico_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_nano_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_nano_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_nano_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_micro_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_micro_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_micro_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_milli_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_milli_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_milli_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_kilo_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_kilo_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_kilo_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_mega_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_mega_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_mega_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_giga_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_giga_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_giga_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_tera_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_tera_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_tera_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_peta_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_peta_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_peta_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_exa_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_exa_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_exa_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_zetta_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_zetta_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_zetta_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_yotta_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_yotta_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_yotta_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_ronna_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_ronna_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_ronna_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_quetta_meter_1: Vec<Entity>,
                    new_chunk_loaders_scale_quetta_meter_10: Vec<Entity>,
                    new_chunk_loaders_scale_quetta_meter_100: Vec<Entity>,
                    new_chunk_loaders_scale_quetta_meter_1000: Vec<Entity>,
                    new_chunk_loaders_scale_quetta_meter_10000: Vec<Entity>,
                    new_chunk_loaders_scale_quetta_meter_100000: Vec<Entity>,
                );

                for entity in new_chunk_loaders_scale_quecto_meter_000001 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_000001.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quecto_meter_00001 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_00001.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quecto_meter_0001 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_0001.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quecto_meter_001 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_001.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quecto_meter_01 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_01.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quecto_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quecto_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quecto_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_ronto_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronto_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_ronto_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronto_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_ronto_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronto_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_yocto_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yocto_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_yocto_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yocto_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_yocto_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yocto_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_zepto_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zepto_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_zepto_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zepto_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_zepto_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zepto_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_atto_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_atto_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_atto_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_atto_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_atto_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_atto_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_femto_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_femto_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_femto_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_femto_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_femto_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_femto_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_pico_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_pico_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_pico_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_pico_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_pico_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_pico_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_nano_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_nano_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_nano_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_nano_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_nano_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_nano_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_micro_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_micro_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_micro_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_micro_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_micro_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_micro_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_milli_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_milli_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_milli_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_milli_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_milli_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_milli_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_kilo_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_kilo_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_kilo_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_kilo_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_kilo_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_kilo_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_mega_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_mega_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_mega_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_mega_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_mega_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_mega_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_giga_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_giga_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_giga_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_giga_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_giga_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_giga_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_tera_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_tera_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_tera_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_tera_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_tera_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_tera_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_peta_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_peta_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_peta_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_peta_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_peta_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_peta_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_exa_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_exa_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_exa_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_exa_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_exa_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_exa_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_zetta_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zetta_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_zetta_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zetta_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_zetta_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zetta_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_yotta_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yotta_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_yotta_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yotta_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_yotta_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yotta_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_ronna_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronna_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_ronna_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronna_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_ronna_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronna_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quetta_meter_1 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_1.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quetta_meter_10 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_10.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quetta_meter_100 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_100.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quetta_meter_1000 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_1000.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quetta_meter_10000 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_10000.get_mut(entity) {
                        init_hook.fire();
                    }
                }
                for entity in new_chunk_loaders_scale_quetta_meter_100000 {
                    if let Ok(mut init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_100000.get_mut(entity) {
                        init_hook.fire();
                    }
                }

                warn!("Finished composite workflow 'TransferChunkOwnerships'");
            });
        }

        *workflow_handles = None;
    }

    // Step 2: Drain the buffer
    let mut processed_coords_scale_quecto_meter_000001 = vec![];
    let mut spawn_inputs_scale_quecto_meter_000001 = vec![];
    let mut spawn_coords_scale_quecto_meter_000001 = vec![];
    let mut despawn_inputs_scale_quecto_meter_000001 = vec![];
    let mut transfer_inputs_scale_quecto_meter_000001 = vec![];

    let mut processed_coords_scale_quecto_meter_00001 = vec![];
    let mut spawn_inputs_scale_quecto_meter_00001 = vec![];
    let mut spawn_coords_scale_quecto_meter_00001 = vec![];
    let mut despawn_inputs_scale_quecto_meter_00001 = vec![];
    let mut transfer_inputs_scale_quecto_meter_00001 = vec![];

    let mut processed_coords_scale_quecto_meter_0001 = vec![];
    let mut spawn_inputs_scale_quecto_meter_0001 = vec![];
    let mut spawn_coords_scale_quecto_meter_0001 = vec![];
    let mut despawn_inputs_scale_quecto_meter_0001 = vec![];
    let mut transfer_inputs_scale_quecto_meter_0001 = vec![];

    let mut processed_coords_scale_quecto_meter_001 = vec![];
    let mut spawn_inputs_scale_quecto_meter_001 = vec![];
    let mut spawn_coords_scale_quecto_meter_001 = vec![];
    let mut despawn_inputs_scale_quecto_meter_001 = vec![];
    let mut transfer_inputs_scale_quecto_meter_001 = vec![];

    let mut processed_coords_scale_quecto_meter_01 = vec![];
    let mut spawn_inputs_scale_quecto_meter_01 = vec![];
    let mut spawn_coords_scale_quecto_meter_01 = vec![];
    let mut despawn_inputs_scale_quecto_meter_01 = vec![];
    let mut transfer_inputs_scale_quecto_meter_01 = vec![];

    let mut processed_coords_scale_quecto_meter_1 = vec![];
    let mut spawn_inputs_scale_quecto_meter_1 = vec![];
    let mut spawn_coords_scale_quecto_meter_1 = vec![];
    let mut despawn_inputs_scale_quecto_meter_1 = vec![];
    let mut transfer_inputs_scale_quecto_meter_1 = vec![];

    let mut processed_coords_scale_quecto_meter_10 = vec![];
    let mut spawn_inputs_scale_quecto_meter_10 = vec![];
    let mut spawn_coords_scale_quecto_meter_10 = vec![];
    let mut despawn_inputs_scale_quecto_meter_10 = vec![];
    let mut transfer_inputs_scale_quecto_meter_10 = vec![];

    let mut processed_coords_scale_quecto_meter_100 = vec![];
    let mut spawn_inputs_scale_quecto_meter_100 = vec![];
    let mut spawn_coords_scale_quecto_meter_100 = vec![];
    let mut despawn_inputs_scale_quecto_meter_100 = vec![];
    let mut transfer_inputs_scale_quecto_meter_100 = vec![];

    let mut processed_coords_scale_ronto_meter_1 = vec![];
    let mut spawn_inputs_scale_ronto_meter_1 = vec![];
    let mut spawn_coords_scale_ronto_meter_1 = vec![];
    let mut despawn_inputs_scale_ronto_meter_1 = vec![];
    let mut transfer_inputs_scale_ronto_meter_1 = vec![];

    let mut processed_coords_scale_ronto_meter_10 = vec![];
    let mut spawn_inputs_scale_ronto_meter_10 = vec![];
    let mut spawn_coords_scale_ronto_meter_10 = vec![];
    let mut despawn_inputs_scale_ronto_meter_10 = vec![];
    let mut transfer_inputs_scale_ronto_meter_10 = vec![];

    let mut processed_coords_scale_ronto_meter_100 = vec![];
    let mut spawn_inputs_scale_ronto_meter_100 = vec![];
    let mut spawn_coords_scale_ronto_meter_100 = vec![];
    let mut despawn_inputs_scale_ronto_meter_100 = vec![];
    let mut transfer_inputs_scale_ronto_meter_100 = vec![];

    let mut processed_coords_scale_yocto_meter_1 = vec![];
    let mut spawn_inputs_scale_yocto_meter_1 = vec![];
    let mut spawn_coords_scale_yocto_meter_1 = vec![];
    let mut despawn_inputs_scale_yocto_meter_1 = vec![];
    let mut transfer_inputs_scale_yocto_meter_1 = vec![];

    let mut processed_coords_scale_yocto_meter_10 = vec![];
    let mut spawn_inputs_scale_yocto_meter_10 = vec![];
    let mut spawn_coords_scale_yocto_meter_10 = vec![];
    let mut despawn_inputs_scale_yocto_meter_10 = vec![];
    let mut transfer_inputs_scale_yocto_meter_10 = vec![];

    let mut processed_coords_scale_yocto_meter_100 = vec![];
    let mut spawn_inputs_scale_yocto_meter_100 = vec![];
    let mut spawn_coords_scale_yocto_meter_100 = vec![];
    let mut despawn_inputs_scale_yocto_meter_100 = vec![];
    let mut transfer_inputs_scale_yocto_meter_100 = vec![];

    let mut processed_coords_scale_zepto_meter_1 = vec![];
    let mut spawn_inputs_scale_zepto_meter_1 = vec![];
    let mut spawn_coords_scale_zepto_meter_1 = vec![];
    let mut despawn_inputs_scale_zepto_meter_1 = vec![];
    let mut transfer_inputs_scale_zepto_meter_1 = vec![];

    let mut processed_coords_scale_zepto_meter_10 = vec![];
    let mut spawn_inputs_scale_zepto_meter_10 = vec![];
    let mut spawn_coords_scale_zepto_meter_10 = vec![];
    let mut despawn_inputs_scale_zepto_meter_10 = vec![];
    let mut transfer_inputs_scale_zepto_meter_10 = vec![];

    let mut processed_coords_scale_zepto_meter_100 = vec![];
    let mut spawn_inputs_scale_zepto_meter_100 = vec![];
    let mut spawn_coords_scale_zepto_meter_100 = vec![];
    let mut despawn_inputs_scale_zepto_meter_100 = vec![];
    let mut transfer_inputs_scale_zepto_meter_100 = vec![];

    let mut processed_coords_scale_atto_meter_1 = vec![];
    let mut spawn_inputs_scale_atto_meter_1 = vec![];
    let mut spawn_coords_scale_atto_meter_1 = vec![];
    let mut despawn_inputs_scale_atto_meter_1 = vec![];
    let mut transfer_inputs_scale_atto_meter_1 = vec![];

    let mut processed_coords_scale_atto_meter_10 = vec![];
    let mut spawn_inputs_scale_atto_meter_10 = vec![];
    let mut spawn_coords_scale_atto_meter_10 = vec![];
    let mut despawn_inputs_scale_atto_meter_10 = vec![];
    let mut transfer_inputs_scale_atto_meter_10 = vec![];

    let mut processed_coords_scale_atto_meter_100 = vec![];
    let mut spawn_inputs_scale_atto_meter_100 = vec![];
    let mut spawn_coords_scale_atto_meter_100 = vec![];
    let mut despawn_inputs_scale_atto_meter_100 = vec![];
    let mut transfer_inputs_scale_atto_meter_100 = vec![];

    let mut processed_coords_scale_femto_meter_1 = vec![];
    let mut spawn_inputs_scale_femto_meter_1 = vec![];
    let mut spawn_coords_scale_femto_meter_1 = vec![];
    let mut despawn_inputs_scale_femto_meter_1 = vec![];
    let mut transfer_inputs_scale_femto_meter_1 = vec![];

    let mut processed_coords_scale_femto_meter_10 = vec![];
    let mut spawn_inputs_scale_femto_meter_10 = vec![];
    let mut spawn_coords_scale_femto_meter_10 = vec![];
    let mut despawn_inputs_scale_femto_meter_10 = vec![];
    let mut transfer_inputs_scale_femto_meter_10 = vec![];

    let mut processed_coords_scale_femto_meter_100 = vec![];
    let mut spawn_inputs_scale_femto_meter_100 = vec![];
    let mut spawn_coords_scale_femto_meter_100 = vec![];
    let mut despawn_inputs_scale_femto_meter_100 = vec![];
    let mut transfer_inputs_scale_femto_meter_100 = vec![];

    let mut processed_coords_scale_pico_meter_1 = vec![];
    let mut spawn_inputs_scale_pico_meter_1 = vec![];
    let mut spawn_coords_scale_pico_meter_1 = vec![];
    let mut despawn_inputs_scale_pico_meter_1 = vec![];
    let mut transfer_inputs_scale_pico_meter_1 = vec![];

    let mut processed_coords_scale_pico_meter_10 = vec![];
    let mut spawn_inputs_scale_pico_meter_10 = vec![];
    let mut spawn_coords_scale_pico_meter_10 = vec![];
    let mut despawn_inputs_scale_pico_meter_10 = vec![];
    let mut transfer_inputs_scale_pico_meter_10 = vec![];

    let mut processed_coords_scale_pico_meter_100 = vec![];
    let mut spawn_inputs_scale_pico_meter_100 = vec![];
    let mut spawn_coords_scale_pico_meter_100 = vec![];
    let mut despawn_inputs_scale_pico_meter_100 = vec![];
    let mut transfer_inputs_scale_pico_meter_100 = vec![];

    let mut processed_coords_scale_nano_meter_1 = vec![];
    let mut spawn_inputs_scale_nano_meter_1 = vec![];
    let mut spawn_coords_scale_nano_meter_1 = vec![];
    let mut despawn_inputs_scale_nano_meter_1 = vec![];
    let mut transfer_inputs_scale_nano_meter_1 = vec![];

    let mut processed_coords_scale_nano_meter_10 = vec![];
    let mut spawn_inputs_scale_nano_meter_10 = vec![];
    let mut spawn_coords_scale_nano_meter_10 = vec![];
    let mut despawn_inputs_scale_nano_meter_10 = vec![];
    let mut transfer_inputs_scale_nano_meter_10 = vec![];

    let mut processed_coords_scale_nano_meter_100 = vec![];
    let mut spawn_inputs_scale_nano_meter_100 = vec![];
    let mut spawn_coords_scale_nano_meter_100 = vec![];
    let mut despawn_inputs_scale_nano_meter_100 = vec![];
    let mut transfer_inputs_scale_nano_meter_100 = vec![];

    let mut processed_coords_scale_micro_meter_1 = vec![];
    let mut spawn_inputs_scale_micro_meter_1 = vec![];
    let mut spawn_coords_scale_micro_meter_1 = vec![];
    let mut despawn_inputs_scale_micro_meter_1 = vec![];
    let mut transfer_inputs_scale_micro_meter_1 = vec![];

    let mut processed_coords_scale_micro_meter_10 = vec![];
    let mut spawn_inputs_scale_micro_meter_10 = vec![];
    let mut spawn_coords_scale_micro_meter_10 = vec![];
    let mut despawn_inputs_scale_micro_meter_10 = vec![];
    let mut transfer_inputs_scale_micro_meter_10 = vec![];

    let mut processed_coords_scale_micro_meter_100 = vec![];
    let mut spawn_inputs_scale_micro_meter_100 = vec![];
    let mut spawn_coords_scale_micro_meter_100 = vec![];
    let mut despawn_inputs_scale_micro_meter_100 = vec![];
    let mut transfer_inputs_scale_micro_meter_100 = vec![];

    let mut processed_coords_scale_milli_meter_1 = vec![];
    let mut spawn_inputs_scale_milli_meter_1 = vec![];
    let mut spawn_coords_scale_milli_meter_1 = vec![];
    let mut despawn_inputs_scale_milli_meter_1 = vec![];
    let mut transfer_inputs_scale_milli_meter_1 = vec![];

    let mut processed_coords_scale_milli_meter_10 = vec![];
    let mut spawn_inputs_scale_milli_meter_10 = vec![];
    let mut spawn_coords_scale_milli_meter_10 = vec![];
    let mut despawn_inputs_scale_milli_meter_10 = vec![];
    let mut transfer_inputs_scale_milli_meter_10 = vec![];

    let mut processed_coords_scale_milli_meter_100 = vec![];
    let mut spawn_inputs_scale_milli_meter_100 = vec![];
    let mut spawn_coords_scale_milli_meter_100 = vec![];
    let mut despawn_inputs_scale_milli_meter_100 = vec![];
    let mut transfer_inputs_scale_milli_meter_100 = vec![];

    let mut processed_coords_scale_meter_1 = vec![];
    let mut spawn_inputs_scale_meter_1 = vec![];
    let mut spawn_coords_scale_meter_1 = vec![];
    let mut despawn_inputs_scale_meter_1 = vec![];
    let mut transfer_inputs_scale_meter_1 = vec![];

    let mut processed_coords_scale_meter_10 = vec![];
    let mut spawn_inputs_scale_meter_10 = vec![];
    let mut spawn_coords_scale_meter_10 = vec![];
    let mut despawn_inputs_scale_meter_10 = vec![];
    let mut transfer_inputs_scale_meter_10 = vec![];

    let mut processed_coords_scale_meter_100 = vec![];
    let mut spawn_inputs_scale_meter_100 = vec![];
    let mut spawn_coords_scale_meter_100 = vec![];
    let mut despawn_inputs_scale_meter_100 = vec![];
    let mut transfer_inputs_scale_meter_100 = vec![];

    let mut processed_coords_scale_kilo_meter_1 = vec![];
    let mut spawn_inputs_scale_kilo_meter_1 = vec![];
    let mut spawn_coords_scale_kilo_meter_1 = vec![];
    let mut despawn_inputs_scale_kilo_meter_1 = vec![];
    let mut transfer_inputs_scale_kilo_meter_1 = vec![];

    let mut processed_coords_scale_kilo_meter_10 = vec![];
    let mut spawn_inputs_scale_kilo_meter_10 = vec![];
    let mut spawn_coords_scale_kilo_meter_10 = vec![];
    let mut despawn_inputs_scale_kilo_meter_10 = vec![];
    let mut transfer_inputs_scale_kilo_meter_10 = vec![];

    let mut processed_coords_scale_kilo_meter_100 = vec![];
    let mut spawn_inputs_scale_kilo_meter_100 = vec![];
    let mut spawn_coords_scale_kilo_meter_100 = vec![];
    let mut despawn_inputs_scale_kilo_meter_100 = vec![];
    let mut transfer_inputs_scale_kilo_meter_100 = vec![];

    let mut processed_coords_scale_mega_meter_1 = vec![];
    let mut spawn_inputs_scale_mega_meter_1 = vec![];
    let mut spawn_coords_scale_mega_meter_1 = vec![];
    let mut despawn_inputs_scale_mega_meter_1 = vec![];
    let mut transfer_inputs_scale_mega_meter_1 = vec![];

    let mut processed_coords_scale_mega_meter_10 = vec![];
    let mut spawn_inputs_scale_mega_meter_10 = vec![];
    let mut spawn_coords_scale_mega_meter_10 = vec![];
    let mut despawn_inputs_scale_mega_meter_10 = vec![];
    let mut transfer_inputs_scale_mega_meter_10 = vec![];

    let mut processed_coords_scale_mega_meter_100 = vec![];
    let mut spawn_inputs_scale_mega_meter_100 = vec![];
    let mut spawn_coords_scale_mega_meter_100 = vec![];
    let mut despawn_inputs_scale_mega_meter_100 = vec![];
    let mut transfer_inputs_scale_mega_meter_100 = vec![];

    let mut processed_coords_scale_giga_meter_1 = vec![];
    let mut spawn_inputs_scale_giga_meter_1 = vec![];
    let mut spawn_coords_scale_giga_meter_1 = vec![];
    let mut despawn_inputs_scale_giga_meter_1 = vec![];
    let mut transfer_inputs_scale_giga_meter_1 = vec![];

    let mut processed_coords_scale_giga_meter_10 = vec![];
    let mut spawn_inputs_scale_giga_meter_10 = vec![];
    let mut spawn_coords_scale_giga_meter_10 = vec![];
    let mut despawn_inputs_scale_giga_meter_10 = vec![];
    let mut transfer_inputs_scale_giga_meter_10 = vec![];

    let mut processed_coords_scale_giga_meter_100 = vec![];
    let mut spawn_inputs_scale_giga_meter_100 = vec![];
    let mut spawn_coords_scale_giga_meter_100 = vec![];
    let mut despawn_inputs_scale_giga_meter_100 = vec![];
    let mut transfer_inputs_scale_giga_meter_100 = vec![];

    let mut processed_coords_scale_tera_meter_1 = vec![];
    let mut spawn_inputs_scale_tera_meter_1 = vec![];
    let mut spawn_coords_scale_tera_meter_1 = vec![];
    let mut despawn_inputs_scale_tera_meter_1 = vec![];
    let mut transfer_inputs_scale_tera_meter_1 = vec![];

    let mut processed_coords_scale_tera_meter_10 = vec![];
    let mut spawn_inputs_scale_tera_meter_10 = vec![];
    let mut spawn_coords_scale_tera_meter_10 = vec![];
    let mut despawn_inputs_scale_tera_meter_10 = vec![];
    let mut transfer_inputs_scale_tera_meter_10 = vec![];

    let mut processed_coords_scale_tera_meter_100 = vec![];
    let mut spawn_inputs_scale_tera_meter_100 = vec![];
    let mut spawn_coords_scale_tera_meter_100 = vec![];
    let mut despawn_inputs_scale_tera_meter_100 = vec![];
    let mut transfer_inputs_scale_tera_meter_100 = vec![];

    let mut processed_coords_scale_peta_meter_1 = vec![];
    let mut spawn_inputs_scale_peta_meter_1 = vec![];
    let mut spawn_coords_scale_peta_meter_1 = vec![];
    let mut despawn_inputs_scale_peta_meter_1 = vec![];
    let mut transfer_inputs_scale_peta_meter_1 = vec![];

    let mut processed_coords_scale_peta_meter_10 = vec![];
    let mut spawn_inputs_scale_peta_meter_10 = vec![];
    let mut spawn_coords_scale_peta_meter_10 = vec![];
    let mut despawn_inputs_scale_peta_meter_10 = vec![];
    let mut transfer_inputs_scale_peta_meter_10 = vec![];

    let mut processed_coords_scale_peta_meter_100 = vec![];
    let mut spawn_inputs_scale_peta_meter_100 = vec![];
    let mut spawn_coords_scale_peta_meter_100 = vec![];
    let mut despawn_inputs_scale_peta_meter_100 = vec![];
    let mut transfer_inputs_scale_peta_meter_100 = vec![];

    let mut processed_coords_scale_exa_meter_1 = vec![];
    let mut spawn_inputs_scale_exa_meter_1 = vec![];
    let mut spawn_coords_scale_exa_meter_1 = vec![];
    let mut despawn_inputs_scale_exa_meter_1 = vec![];
    let mut transfer_inputs_scale_exa_meter_1 = vec![];

    let mut processed_coords_scale_exa_meter_10 = vec![];
    let mut spawn_inputs_scale_exa_meter_10 = vec![];
    let mut spawn_coords_scale_exa_meter_10 = vec![];
    let mut despawn_inputs_scale_exa_meter_10 = vec![];
    let mut transfer_inputs_scale_exa_meter_10 = vec![];

    let mut processed_coords_scale_exa_meter_100 = vec![];
    let mut spawn_inputs_scale_exa_meter_100 = vec![];
    let mut spawn_coords_scale_exa_meter_100 = vec![];
    let mut despawn_inputs_scale_exa_meter_100 = vec![];
    let mut transfer_inputs_scale_exa_meter_100 = vec![];

    let mut processed_coords_scale_zetta_meter_1 = vec![];
    let mut spawn_inputs_scale_zetta_meter_1 = vec![];
    let mut spawn_coords_scale_zetta_meter_1 = vec![];
    let mut despawn_inputs_scale_zetta_meter_1 = vec![];
    let mut transfer_inputs_scale_zetta_meter_1 = vec![];

    let mut processed_coords_scale_zetta_meter_10 = vec![];
    let mut spawn_inputs_scale_zetta_meter_10 = vec![];
    let mut spawn_coords_scale_zetta_meter_10 = vec![];
    let mut despawn_inputs_scale_zetta_meter_10 = vec![];
    let mut transfer_inputs_scale_zetta_meter_10 = vec![];

    let mut processed_coords_scale_zetta_meter_100 = vec![];
    let mut spawn_inputs_scale_zetta_meter_100 = vec![];
    let mut spawn_coords_scale_zetta_meter_100 = vec![];
    let mut despawn_inputs_scale_zetta_meter_100 = vec![];
    let mut transfer_inputs_scale_zetta_meter_100 = vec![];

    let mut processed_coords_scale_yotta_meter_1 = vec![];
    let mut spawn_inputs_scale_yotta_meter_1 = vec![];
    let mut spawn_coords_scale_yotta_meter_1 = vec![];
    let mut despawn_inputs_scale_yotta_meter_1 = vec![];
    let mut transfer_inputs_scale_yotta_meter_1 = vec![];

    let mut processed_coords_scale_yotta_meter_10 = vec![];
    let mut spawn_inputs_scale_yotta_meter_10 = vec![];
    let mut spawn_coords_scale_yotta_meter_10 = vec![];
    let mut despawn_inputs_scale_yotta_meter_10 = vec![];
    let mut transfer_inputs_scale_yotta_meter_10 = vec![];

    let mut processed_coords_scale_yotta_meter_100 = vec![];
    let mut spawn_inputs_scale_yotta_meter_100 = vec![];
    let mut spawn_coords_scale_yotta_meter_100 = vec![];
    let mut despawn_inputs_scale_yotta_meter_100 = vec![];
    let mut transfer_inputs_scale_yotta_meter_100 = vec![];

    let mut processed_coords_scale_ronna_meter_1 = vec![];
    let mut spawn_inputs_scale_ronna_meter_1 = vec![];
    let mut spawn_coords_scale_ronna_meter_1 = vec![];
    let mut despawn_inputs_scale_ronna_meter_1 = vec![];
    let mut transfer_inputs_scale_ronna_meter_1 = vec![];

    let mut processed_coords_scale_ronna_meter_10 = vec![];
    let mut spawn_inputs_scale_ronna_meter_10 = vec![];
    let mut spawn_coords_scale_ronna_meter_10 = vec![];
    let mut despawn_inputs_scale_ronna_meter_10 = vec![];
    let mut transfer_inputs_scale_ronna_meter_10 = vec![];

    let mut processed_coords_scale_ronna_meter_100 = vec![];
    let mut spawn_inputs_scale_ronna_meter_100 = vec![];
    let mut spawn_coords_scale_ronna_meter_100 = vec![];
    let mut despawn_inputs_scale_ronna_meter_100 = vec![];
    let mut transfer_inputs_scale_ronna_meter_100 = vec![];

    let mut processed_coords_scale_quetta_meter_1 = vec![];
    let mut spawn_inputs_scale_quetta_meter_1 = vec![];
    let mut spawn_coords_scale_quetta_meter_1 = vec![];
    let mut despawn_inputs_scale_quetta_meter_1 = vec![];
    let mut transfer_inputs_scale_quetta_meter_1 = vec![];

    let mut processed_coords_scale_quetta_meter_10 = vec![];
    let mut spawn_inputs_scale_quetta_meter_10 = vec![];
    let mut spawn_coords_scale_quetta_meter_10 = vec![];
    let mut despawn_inputs_scale_quetta_meter_10 = vec![];
    let mut transfer_inputs_scale_quetta_meter_10 = vec![];

    let mut processed_coords_scale_quetta_meter_100 = vec![];
    let mut spawn_inputs_scale_quetta_meter_100 = vec![];
    let mut spawn_coords_scale_quetta_meter_100 = vec![];
    let mut despawn_inputs_scale_quetta_meter_100 = vec![];
    let mut transfer_inputs_scale_quetta_meter_100 = vec![];

    let mut processed_coords_scale_quetta_meter_1000 = vec![];
    let mut spawn_inputs_scale_quetta_meter_1000 = vec![];
    let mut spawn_coords_scale_quetta_meter_1000 = vec![];
    let mut despawn_inputs_scale_quetta_meter_1000 = vec![];
    let mut transfer_inputs_scale_quetta_meter_1000 = vec![];

    let mut processed_coords_scale_quetta_meter_10000 = vec![];
    let mut spawn_inputs_scale_quetta_meter_10000 = vec![];
    let mut spawn_coords_scale_quetta_meter_10000 = vec![];
    let mut despawn_inputs_scale_quetta_meter_10000 = vec![];
    let mut transfer_inputs_scale_quetta_meter_10000 = vec![];

    let mut processed_coords_scale_quetta_meter_100000 = vec![];
    let mut spawn_inputs_scale_quetta_meter_100000 = vec![];
    let mut spawn_coords_scale_quetta_meter_100000 = vec![];
    let mut despawn_inputs_scale_quetta_meter_100000 = vec![];
    let mut transfer_inputs_scale_quetta_meter_100000 = vec![];

    let mut chunk_loaders_performing_chunk_loads_scale_quecto_meter_000001: Vec<ChunkOwnerId<ScaleQuectoMeter000001>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_quecto_meter_00001: Vec<ChunkOwnerId<ScaleQuectoMeter00001>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_quecto_meter_0001: Vec<ChunkOwnerId<ScaleQuectoMeter0001>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_quecto_meter_001: Vec<ChunkOwnerId<ScaleQuectoMeter001>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_quecto_meter_01: Vec<ChunkOwnerId<ScaleQuectoMeter01>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_quecto_meter_1: Vec<ChunkOwnerId<ScaleQuectoMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_quecto_meter_10: Vec<ChunkOwnerId<ScaleQuectoMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_quecto_meter_100: Vec<ChunkOwnerId<ScaleQuectoMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_ronto_meter_1: Vec<ChunkOwnerId<ScaleRontoMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_ronto_meter_10: Vec<ChunkOwnerId<ScaleRontoMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_ronto_meter_100: Vec<ChunkOwnerId<ScaleRontoMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_yocto_meter_1: Vec<ChunkOwnerId<ScaleYoctoMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_yocto_meter_10: Vec<ChunkOwnerId<ScaleYoctoMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_yocto_meter_100: Vec<ChunkOwnerId<ScaleYoctoMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_zepto_meter_1: Vec<ChunkOwnerId<ScaleZeptoMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_zepto_meter_10: Vec<ChunkOwnerId<ScaleZeptoMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_zepto_meter_100: Vec<ChunkOwnerId<ScaleZeptoMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_atto_meter_1: Vec<ChunkOwnerId<ScaleAttoMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_atto_meter_10: Vec<ChunkOwnerId<ScaleAttoMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_atto_meter_100: Vec<ChunkOwnerId<ScaleAttoMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_femto_meter_1: Vec<ChunkOwnerId<ScaleFemtoMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_femto_meter_10: Vec<ChunkOwnerId<ScaleFemtoMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_femto_meter_100: Vec<ChunkOwnerId<ScaleFemtoMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_pico_meter_1: Vec<ChunkOwnerId<ScalePicoMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_pico_meter_10: Vec<ChunkOwnerId<ScalePicoMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_pico_meter_100: Vec<ChunkOwnerId<ScalePicoMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_nano_meter_1: Vec<ChunkOwnerId<ScaleNanoMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_nano_meter_10: Vec<ChunkOwnerId<ScaleNanoMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_nano_meter_100: Vec<ChunkOwnerId<ScaleNanoMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_micro_meter_1: Vec<ChunkOwnerId<ScaleMicroMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_micro_meter_10: Vec<ChunkOwnerId<ScaleMicroMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_micro_meter_100: Vec<ChunkOwnerId<ScaleMicroMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_milli_meter_1: Vec<ChunkOwnerId<ScaleMilliMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_milli_meter_10: Vec<ChunkOwnerId<ScaleMilliMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_milli_meter_100: Vec<ChunkOwnerId<ScaleMilliMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_meter_1: Vec<ChunkOwnerId<ScaleMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_meter_10: Vec<ChunkOwnerId<ScaleMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_meter_100: Vec<ChunkOwnerId<ScaleMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_kilo_meter_1: Vec<ChunkOwnerId<ScaleKiloMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_kilo_meter_10: Vec<ChunkOwnerId<ScaleKiloMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_kilo_meter_100: Vec<ChunkOwnerId<ScaleKiloMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_mega_meter_1: Vec<ChunkOwnerId<ScaleMegaMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_mega_meter_10: Vec<ChunkOwnerId<ScaleMegaMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_mega_meter_100: Vec<ChunkOwnerId<ScaleMegaMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_giga_meter_1: Vec<ChunkOwnerId<ScaleGigaMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_giga_meter_10: Vec<ChunkOwnerId<ScaleGigaMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_giga_meter_100: Vec<ChunkOwnerId<ScaleGigaMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_tera_meter_1: Vec<ChunkOwnerId<ScaleTeraMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_tera_meter_10: Vec<ChunkOwnerId<ScaleTeraMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_tera_meter_100: Vec<ChunkOwnerId<ScaleTeraMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_peta_meter_1: Vec<ChunkOwnerId<ScalePetaMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_peta_meter_10: Vec<ChunkOwnerId<ScalePetaMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_peta_meter_100: Vec<ChunkOwnerId<ScalePetaMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_exa_meter_1: Vec<ChunkOwnerId<ScaleExaMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_exa_meter_10: Vec<ChunkOwnerId<ScaleExaMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_exa_meter_100: Vec<ChunkOwnerId<ScaleExaMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_zetta_meter_1: Vec<ChunkOwnerId<ScaleZettaMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_zetta_meter_10: Vec<ChunkOwnerId<ScaleZettaMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_zetta_meter_100: Vec<ChunkOwnerId<ScaleZettaMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_yotta_meter_1: Vec<ChunkOwnerId<ScaleYottaMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_yotta_meter_10: Vec<ChunkOwnerId<ScaleYottaMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_yotta_meter_100: Vec<ChunkOwnerId<ScaleYottaMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_ronna_meter_1: Vec<ChunkOwnerId<ScaleRonnaMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_ronna_meter_10: Vec<ChunkOwnerId<ScaleRonnaMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_ronna_meter_100: Vec<ChunkOwnerId<ScaleRonnaMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_quetta_meter_1: Vec<ChunkOwnerId<ScaleQuettaMeter1>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_quetta_meter_10: Vec<ChunkOwnerId<ScaleQuettaMeter10>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_quetta_meter_100: Vec<ChunkOwnerId<ScaleQuettaMeter100>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_quetta_meter_1000: Vec<ChunkOwnerId<ScaleQuettaMeter1000>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_quetta_meter_10000: Vec<ChunkOwnerId<ScaleQuettaMeter10000>> = Vec::new();
    let mut chunk_loaders_performing_chunk_loads_scale_quetta_meter_100000: Vec<ChunkOwnerId<ScaleQuettaMeter100000>> = Vec::new();

    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_000001.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_000001.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_000001)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_quecto_meter_000001.push(coord);
                    spawn_inputs_scale_quecto_meter_000001.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_quecto_meter_000001.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_000001.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_quecto_meter_000001.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_quecto_meter_000001.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_quecto_meter_000001.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_quecto_meter_000001.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_000001.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_00001.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_00001.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_00001)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_quecto_meter_00001.push(coord);
                    spawn_inputs_scale_quecto_meter_00001.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_quecto_meter_00001.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_00001.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_quecto_meter_00001.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_quecto_meter_00001.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_quecto_meter_00001.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_quecto_meter_00001.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_00001.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_0001.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_0001.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_0001)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_quecto_meter_0001.push(coord);
                    spawn_inputs_scale_quecto_meter_0001.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_quecto_meter_0001.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_0001.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_quecto_meter_0001.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_quecto_meter_0001.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_quecto_meter_0001.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_quecto_meter_0001.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_0001.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_001.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_001.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_001)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_quecto_meter_001.push(coord);
                    spawn_inputs_scale_quecto_meter_001.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_quecto_meter_001.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_001.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_quecto_meter_001.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_quecto_meter_001.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_quecto_meter_001.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_quecto_meter_001.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_001.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_01.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_01.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_01)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_quecto_meter_01.push(coord);
                    spawn_inputs_scale_quecto_meter_01.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_quecto_meter_01.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_01.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_quecto_meter_01.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_quecto_meter_01.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_quecto_meter_01.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_quecto_meter_01.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_01.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_quecto_meter_1.push(coord);
                    spawn_inputs_scale_quecto_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_quecto_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_quecto_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_quecto_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_quecto_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_quecto_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_quecto_meter_10.push(coord);
                    spawn_inputs_scale_quecto_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_quecto_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_quecto_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_quecto_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_quecto_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_quecto_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_quecto_meter_100.push(coord);
                    spawn_inputs_scale_quecto_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_quecto_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_quecto_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_quecto_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_quecto_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_quecto_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quecto_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_ronto_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_ronto_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_ronto_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_ronto_meter_1.push(coord);
                    spawn_inputs_scale_ronto_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_ronto_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_ronto_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_ronto_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_ronto_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_ronto_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_ronto_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_ronto_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_ronto_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_ronto_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_ronto_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_ronto_meter_10.push(coord);
                    spawn_inputs_scale_ronto_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_ronto_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_ronto_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_ronto_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_ronto_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_ronto_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_ronto_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_ronto_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_ronto_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_ronto_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_ronto_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_ronto_meter_100.push(coord);
                    spawn_inputs_scale_ronto_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_ronto_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_ronto_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_ronto_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_ronto_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_ronto_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_ronto_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_ronto_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_yocto_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_yocto_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_yocto_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_yocto_meter_1.push(coord);
                    spawn_inputs_scale_yocto_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_yocto_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_yocto_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_yocto_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_yocto_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_yocto_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_yocto_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_yocto_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_yocto_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_yocto_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_yocto_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_yocto_meter_10.push(coord);
                    spawn_inputs_scale_yocto_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_yocto_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_yocto_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_yocto_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_yocto_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_yocto_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_yocto_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_yocto_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_yocto_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_yocto_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_yocto_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_yocto_meter_100.push(coord);
                    spawn_inputs_scale_yocto_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_yocto_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_yocto_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_yocto_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_yocto_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_yocto_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_yocto_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_yocto_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_zepto_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_zepto_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_zepto_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_zepto_meter_1.push(coord);
                    spawn_inputs_scale_zepto_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_zepto_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_zepto_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_zepto_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_zepto_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_zepto_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_zepto_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_zepto_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_zepto_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_zepto_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_zepto_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_zepto_meter_10.push(coord);
                    spawn_inputs_scale_zepto_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_zepto_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_zepto_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_zepto_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_zepto_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_zepto_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_zepto_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_zepto_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_zepto_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_zepto_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_zepto_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_zepto_meter_100.push(coord);
                    spawn_inputs_scale_zepto_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_zepto_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_zepto_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_zepto_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_zepto_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_zepto_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_zepto_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_zepto_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_atto_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_atto_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_atto_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_atto_meter_1.push(coord);
                    spawn_inputs_scale_atto_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_atto_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_atto_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_atto_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_atto_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_atto_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_atto_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_atto_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_atto_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_atto_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_atto_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_atto_meter_10.push(coord);
                    spawn_inputs_scale_atto_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_atto_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_atto_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_atto_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_atto_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_atto_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_atto_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_atto_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_atto_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_atto_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_atto_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_atto_meter_100.push(coord);
                    spawn_inputs_scale_atto_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_atto_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_atto_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_atto_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_atto_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_atto_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_atto_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_atto_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_femto_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_femto_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_femto_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_femto_meter_1.push(coord);
                    spawn_inputs_scale_femto_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_femto_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_femto_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_femto_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_femto_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_femto_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_femto_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_femto_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_femto_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_femto_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_femto_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_femto_meter_10.push(coord);
                    spawn_inputs_scale_femto_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_femto_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_femto_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_femto_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_femto_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_femto_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_femto_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_femto_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_femto_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_femto_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_femto_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_femto_meter_100.push(coord);
                    spawn_inputs_scale_femto_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_femto_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_femto_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_femto_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_femto_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_femto_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_femto_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_femto_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_pico_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_pico_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_pico_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_pico_meter_1.push(coord);
                    spawn_inputs_scale_pico_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_pico_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_pico_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_pico_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_pico_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_pico_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_pico_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_pico_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_pico_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_pico_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_pico_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_pico_meter_10.push(coord);
                    spawn_inputs_scale_pico_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_pico_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_pico_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_pico_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_pico_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_pico_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_pico_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_pico_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_pico_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_pico_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_pico_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_pico_meter_100.push(coord);
                    spawn_inputs_scale_pico_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_pico_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_pico_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_pico_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_pico_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_pico_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_pico_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_pico_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_nano_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_nano_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_nano_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_nano_meter_1.push(coord);
                    spawn_inputs_scale_nano_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_nano_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_nano_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_nano_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_nano_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_nano_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_nano_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_nano_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_nano_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_nano_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_nano_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_nano_meter_10.push(coord);
                    spawn_inputs_scale_nano_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_nano_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_nano_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_nano_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_nano_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_nano_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_nano_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_nano_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_nano_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_nano_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_nano_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_nano_meter_100.push(coord);
                    spawn_inputs_scale_nano_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_nano_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_nano_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_nano_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_nano_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_nano_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_nano_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_nano_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_micro_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_micro_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_micro_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_micro_meter_1.push(coord);
                    spawn_inputs_scale_micro_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_micro_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_micro_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_micro_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_micro_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_micro_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_micro_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_micro_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_micro_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_micro_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_micro_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_micro_meter_10.push(coord);
                    spawn_inputs_scale_micro_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_micro_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_micro_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_micro_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_micro_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_micro_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_micro_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_micro_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_micro_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_micro_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_micro_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_micro_meter_100.push(coord);
                    spawn_inputs_scale_micro_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_micro_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_micro_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_micro_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_micro_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_micro_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_micro_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_micro_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_milli_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_milli_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_milli_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_milli_meter_1.push(coord);
                    spawn_inputs_scale_milli_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_milli_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_milli_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_milli_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_milli_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_milli_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_milli_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_milli_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_milli_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_milli_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_milli_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_milli_meter_10.push(coord);
                    spawn_inputs_scale_milli_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_milli_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_milli_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_milli_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_milli_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_milli_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_milli_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_milli_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_milli_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_milli_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_milli_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_milli_meter_100.push(coord);
                    spawn_inputs_scale_milli_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_milli_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_milli_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_milli_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_milli_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_milli_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_milli_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_milli_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_meter_1.push(coord);
                    spawn_inputs_scale_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_meter_10.push(coord);
                    spawn_inputs_scale_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_meter_100.push(coord);
                    spawn_inputs_scale_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_kilo_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_kilo_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_kilo_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_kilo_meter_1.push(coord);
                    spawn_inputs_scale_kilo_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_kilo_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_kilo_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_kilo_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_kilo_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_kilo_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_kilo_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_kilo_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_kilo_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_kilo_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_kilo_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_kilo_meter_10.push(coord);
                    spawn_inputs_scale_kilo_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_kilo_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_kilo_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_kilo_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_kilo_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_kilo_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_kilo_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_kilo_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_kilo_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_kilo_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_kilo_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_kilo_meter_100.push(coord);
                    spawn_inputs_scale_kilo_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_kilo_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_kilo_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_kilo_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_kilo_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_kilo_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_kilo_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_kilo_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_mega_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_mega_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_mega_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_mega_meter_1.push(coord);
                    spawn_inputs_scale_mega_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_mega_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_mega_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_mega_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_mega_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_mega_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_mega_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_mega_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_mega_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_mega_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_mega_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_mega_meter_10.push(coord);
                    spawn_inputs_scale_mega_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_mega_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_mega_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_mega_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_mega_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_mega_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_mega_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_mega_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_mega_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_mega_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_mega_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_mega_meter_100.push(coord);
                    spawn_inputs_scale_mega_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_mega_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_mega_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_mega_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_mega_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_mega_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_mega_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_mega_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_giga_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_giga_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_giga_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_giga_meter_1.push(coord);
                    spawn_inputs_scale_giga_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_giga_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_giga_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_giga_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_giga_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_giga_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_giga_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_giga_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_giga_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_giga_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_giga_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_giga_meter_10.push(coord);
                    spawn_inputs_scale_giga_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_giga_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_giga_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_giga_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_giga_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_giga_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_giga_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_giga_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_giga_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_giga_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_giga_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_giga_meter_100.push(coord);
                    spawn_inputs_scale_giga_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_giga_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_giga_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_giga_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_giga_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_giga_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_giga_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_giga_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_tera_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_tera_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_tera_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_tera_meter_1.push(coord);
                    spawn_inputs_scale_tera_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_tera_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_tera_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_tera_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_tera_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_tera_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_tera_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_tera_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_tera_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_tera_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_tera_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_tera_meter_10.push(coord);
                    spawn_inputs_scale_tera_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_tera_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_tera_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_tera_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_tera_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_tera_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_tera_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_tera_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_tera_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_tera_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_tera_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_tera_meter_100.push(coord);
                    spawn_inputs_scale_tera_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_tera_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_tera_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_tera_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_tera_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_tera_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_tera_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_tera_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_peta_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_peta_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_peta_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_peta_meter_1.push(coord);
                    spawn_inputs_scale_peta_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_peta_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_peta_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_peta_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_peta_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_peta_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_peta_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_peta_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_peta_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_peta_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_peta_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_peta_meter_10.push(coord);
                    spawn_inputs_scale_peta_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_peta_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_peta_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_peta_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_peta_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_peta_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_peta_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_peta_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_peta_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_peta_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_peta_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_peta_meter_100.push(coord);
                    spawn_inputs_scale_peta_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_peta_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_peta_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_peta_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_peta_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_peta_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_peta_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_peta_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_exa_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_exa_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_exa_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_exa_meter_1.push(coord);
                    spawn_inputs_scale_exa_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_exa_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_exa_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_exa_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_exa_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_exa_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_exa_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_exa_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_exa_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_exa_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_exa_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_exa_meter_10.push(coord);
                    spawn_inputs_scale_exa_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_exa_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_exa_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_exa_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_exa_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_exa_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_exa_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_exa_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_exa_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_exa_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_exa_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_exa_meter_100.push(coord);
                    spawn_inputs_scale_exa_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_exa_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_exa_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_exa_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_exa_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_exa_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_exa_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_exa_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_zetta_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_zetta_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_zetta_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_zetta_meter_1.push(coord);
                    spawn_inputs_scale_zetta_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_zetta_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_zetta_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_zetta_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_zetta_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_zetta_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_zetta_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_zetta_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_zetta_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_zetta_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_zetta_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_zetta_meter_10.push(coord);
                    spawn_inputs_scale_zetta_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_zetta_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_zetta_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_zetta_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_zetta_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_zetta_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_zetta_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_zetta_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_zetta_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_zetta_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_zetta_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_zetta_meter_100.push(coord);
                    spawn_inputs_scale_zetta_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_zetta_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_zetta_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_zetta_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_zetta_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_zetta_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_zetta_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_zetta_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_yotta_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_yotta_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_yotta_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_yotta_meter_1.push(coord);
                    spawn_inputs_scale_yotta_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_yotta_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_yotta_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_yotta_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_yotta_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_yotta_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_yotta_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_yotta_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_yotta_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_yotta_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_yotta_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_yotta_meter_10.push(coord);
                    spawn_inputs_scale_yotta_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_yotta_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_yotta_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_yotta_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_yotta_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_yotta_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_yotta_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_yotta_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_yotta_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_yotta_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_yotta_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_yotta_meter_100.push(coord);
                    spawn_inputs_scale_yotta_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_yotta_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_yotta_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_yotta_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_yotta_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_yotta_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_yotta_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_yotta_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_ronna_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_ronna_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_ronna_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_ronna_meter_1.push(coord);
                    spawn_inputs_scale_ronna_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_ronna_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_ronna_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_ronna_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_ronna_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_ronna_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_ronna_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_ronna_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_ronna_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_ronna_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_ronna_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_ronna_meter_10.push(coord);
                    spawn_inputs_scale_ronna_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_ronna_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_ronna_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_ronna_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_ronna_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_ronna_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_ronna_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_ronna_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_ronna_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_ronna_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_ronna_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_ronna_meter_100.push(coord);
                    spawn_inputs_scale_ronna_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_ronna_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_ronna_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_ronna_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_ronna_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_ronna_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_ronna_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_ronna_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_1.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_1.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_1)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_quetta_meter_1.push(coord);
                    spawn_inputs_scale_quetta_meter_1.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_quetta_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quetta_meter_1.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_quetta_meter_1.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_quetta_meter_1.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_quetta_meter_1.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_quetta_meter_1.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quetta_meter_1.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_10.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_10.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_10)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_quetta_meter_10.push(coord);
                    spawn_inputs_scale_quetta_meter_10.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_quetta_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quetta_meter_10.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_quetta_meter_10.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_quetta_meter_10.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_quetta_meter_10.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_quetta_meter_10.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quetta_meter_10.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_100.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_100.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_100)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_quetta_meter_100.push(coord);
                    spawn_inputs_scale_quetta_meter_100.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_quetta_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quetta_meter_100.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_quetta_meter_100.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_quetta_meter_100.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_quetta_meter_100.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_quetta_meter_100.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quetta_meter_100.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_1000.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_1000.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_1000)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_quetta_meter_1000.push(coord);
                    spawn_inputs_scale_quetta_meter_1000.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_quetta_meter_1000.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quetta_meter_1000.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_quetta_meter_1000.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_quetta_meter_1000.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_quetta_meter_1000.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_quetta_meter_1000.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quetta_meter_1000.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_10000.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_10000.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_10000)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_quetta_meter_10000.push(coord);
                    spawn_inputs_scale_quetta_meter_10000.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_quetta_meter_10000.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quetta_meter_10000.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_quetta_meter_10000.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_quetta_meter_10000.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_quetta_meter_10000.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_quetta_meter_10000.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quetta_meter_10000.push(new_owner_id);
                }
            }
        }
    }
    for (_, coords) in action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_100000.priority_buckets.iter() {
        for coord in coords {
            let action_intent = action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_100000.action_intent.get(coord)
                .unwrap_or_else(|| panic!("Failed to get ActionIntent for chunk at {:?}! Full commit-buffer printout: {:?}", coord, action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_100000)).clone();
            // warn!("Processing chunk action intent: {:?}", action_intent);

            match action_intent {
                ActionIntent::Spawn { owner_id, coord, .. } => {
                    spawn_coords_scale_quetta_meter_100000.push(coord);
                    spawn_inputs_scale_quetta_meter_100000.push(crate::chunk::workflows::external::spawn_chunks::SpawnChunkInput {
                        chunk_coord: coord,
                        chunk_owner_id: owner_id.clone(),
                        metric_texture: Handle::default(),
                    });
                    processed_coords_scale_quetta_meter_100000.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quetta_meter_100000.push(owner_id);
                }
                ActionIntent::Despawn { coord, .. } => {
                    despawn_inputs_scale_quetta_meter_100000.push(crate::chunk::workflows::external::despawn_chunks::DespawnChunkInput { chunk_coord: coord });
                    processed_coords_scale_quetta_meter_100000.push(coord);
                }
                ActionIntent::TransferOwnership { new_owner_id, coord, .. } => {
                    transfer_inputs_scale_quetta_meter_100000.push(
                        crate::chunk::workflows::external::transfer_chunk_ownerships::TransferChunkOwnershipInput {
                            new_chunk_owner_id: new_owner_id.clone(),
                            chunk_coord: coord,
                        },
                    );
                    processed_coords_scale_quetta_meter_100000.push(coord);
                    chunk_loaders_performing_chunk_loads_scale_quetta_meter_100000.push(new_owner_id);
                }
            }
        }
    }

    if spawn_inputs_scale_quecto_meter_000001.is_empty() && despawn_inputs_scale_quecto_meter_000001.is_empty() && transfer_inputs_scale_quecto_meter_000001.is_empty()
        && spawn_inputs_scale_quecto_meter_00001.is_empty() && despawn_inputs_scale_quecto_meter_00001.is_empty() && transfer_inputs_scale_quecto_meter_00001.is_empty()
        && spawn_inputs_scale_quecto_meter_0001.is_empty() && despawn_inputs_scale_quecto_meter_0001.is_empty() && transfer_inputs_scale_quecto_meter_0001.is_empty()
        && spawn_inputs_scale_quecto_meter_001.is_empty() && despawn_inputs_scale_quecto_meter_001.is_empty() && transfer_inputs_scale_quecto_meter_001.is_empty()
        && spawn_inputs_scale_quecto_meter_01.is_empty() && despawn_inputs_scale_quecto_meter_01.is_empty() && transfer_inputs_scale_quecto_meter_01.is_empty()
        && spawn_inputs_scale_quecto_meter_1.is_empty() && despawn_inputs_scale_quecto_meter_1.is_empty() && transfer_inputs_scale_quecto_meter_1.is_empty()
        && spawn_inputs_scale_quecto_meter_10.is_empty() && despawn_inputs_scale_quecto_meter_10.is_empty() && transfer_inputs_scale_quecto_meter_10.is_empty()
        && spawn_inputs_scale_quecto_meter_100.is_empty() && despawn_inputs_scale_quecto_meter_100.is_empty() && transfer_inputs_scale_quecto_meter_100.is_empty()
        && spawn_inputs_scale_ronto_meter_1.is_empty() && despawn_inputs_scale_ronto_meter_1.is_empty() && transfer_inputs_scale_ronto_meter_1.is_empty()
        && spawn_inputs_scale_ronto_meter_10.is_empty() && despawn_inputs_scale_ronto_meter_10.is_empty() && transfer_inputs_scale_ronto_meter_10.is_empty()
        && spawn_inputs_scale_ronto_meter_100.is_empty() && despawn_inputs_scale_ronto_meter_100.is_empty() && transfer_inputs_scale_ronto_meter_100.is_empty()
        && spawn_inputs_scale_yocto_meter_1.is_empty() && despawn_inputs_scale_yocto_meter_1.is_empty() && transfer_inputs_scale_yocto_meter_1.is_empty()
        && spawn_inputs_scale_yocto_meter_10.is_empty() && despawn_inputs_scale_yocto_meter_10.is_empty() && transfer_inputs_scale_yocto_meter_10.is_empty()
        && spawn_inputs_scale_yocto_meter_100.is_empty() && despawn_inputs_scale_yocto_meter_100.is_empty() && transfer_inputs_scale_yocto_meter_100.is_empty()
        && spawn_inputs_scale_zepto_meter_1.is_empty() && despawn_inputs_scale_zepto_meter_1.is_empty() && transfer_inputs_scale_zepto_meter_1.is_empty()
        && spawn_inputs_scale_zepto_meter_10.is_empty() && despawn_inputs_scale_zepto_meter_10.is_empty() && transfer_inputs_scale_zepto_meter_10.is_empty()
        && spawn_inputs_scale_zepto_meter_100.is_empty() && despawn_inputs_scale_zepto_meter_100.is_empty() && transfer_inputs_scale_zepto_meter_100.is_empty()
        && spawn_inputs_scale_atto_meter_1.is_empty() && despawn_inputs_scale_atto_meter_1.is_empty() && transfer_inputs_scale_atto_meter_1.is_empty()
        && spawn_inputs_scale_atto_meter_10.is_empty() && despawn_inputs_scale_atto_meter_10.is_empty() && transfer_inputs_scale_atto_meter_10.is_empty()
        && spawn_inputs_scale_atto_meter_100.is_empty() && despawn_inputs_scale_atto_meter_100.is_empty() && transfer_inputs_scale_atto_meter_100.is_empty()
        && spawn_inputs_scale_femto_meter_1.is_empty() && despawn_inputs_scale_femto_meter_1.is_empty() && transfer_inputs_scale_femto_meter_1.is_empty()
        && spawn_inputs_scale_femto_meter_10.is_empty() && despawn_inputs_scale_femto_meter_10.is_empty() && transfer_inputs_scale_femto_meter_10.is_empty()
        && spawn_inputs_scale_femto_meter_100.is_empty() && despawn_inputs_scale_femto_meter_100.is_empty() && transfer_inputs_scale_femto_meter_100.is_empty()
        && spawn_inputs_scale_pico_meter_1.is_empty() && despawn_inputs_scale_pico_meter_1.is_empty() && transfer_inputs_scale_pico_meter_1.is_empty()
        && spawn_inputs_scale_pico_meter_10.is_empty() && despawn_inputs_scale_pico_meter_10.is_empty() && transfer_inputs_scale_pico_meter_10.is_empty()
        && spawn_inputs_scale_pico_meter_100.is_empty() && despawn_inputs_scale_pico_meter_100.is_empty() && transfer_inputs_scale_pico_meter_100.is_empty()
        && spawn_inputs_scale_nano_meter_1.is_empty() && despawn_inputs_scale_nano_meter_1.is_empty() && transfer_inputs_scale_nano_meter_1.is_empty()
        && spawn_inputs_scale_nano_meter_10.is_empty() && despawn_inputs_scale_nano_meter_10.is_empty() && transfer_inputs_scale_nano_meter_10.is_empty()
        && spawn_inputs_scale_nano_meter_100.is_empty() && despawn_inputs_scale_nano_meter_100.is_empty() && transfer_inputs_scale_nano_meter_100.is_empty()
        && spawn_inputs_scale_micro_meter_1.is_empty() && despawn_inputs_scale_micro_meter_1.is_empty() && transfer_inputs_scale_micro_meter_1.is_empty()
        && spawn_inputs_scale_micro_meter_10.is_empty() && despawn_inputs_scale_micro_meter_10.is_empty() && transfer_inputs_scale_micro_meter_10.is_empty()
        && spawn_inputs_scale_micro_meter_100.is_empty() && despawn_inputs_scale_micro_meter_100.is_empty() && transfer_inputs_scale_micro_meter_100.is_empty()
        && spawn_inputs_scale_milli_meter_1.is_empty() && despawn_inputs_scale_milli_meter_1.is_empty() && transfer_inputs_scale_milli_meter_1.is_empty()
        && spawn_inputs_scale_milli_meter_10.is_empty() && despawn_inputs_scale_milli_meter_10.is_empty() && transfer_inputs_scale_milli_meter_10.is_empty()
        && spawn_inputs_scale_milli_meter_100.is_empty() && despawn_inputs_scale_milli_meter_100.is_empty() && transfer_inputs_scale_milli_meter_100.is_empty()
        && spawn_inputs_scale_meter_1.is_empty() && despawn_inputs_scale_meter_1.is_empty() && transfer_inputs_scale_meter_1.is_empty()
        && spawn_inputs_scale_meter_10.is_empty() && despawn_inputs_scale_meter_10.is_empty() && transfer_inputs_scale_meter_10.is_empty()
        && spawn_inputs_scale_meter_100.is_empty() && despawn_inputs_scale_meter_100.is_empty() && transfer_inputs_scale_meter_100.is_empty()
        && spawn_inputs_scale_kilo_meter_1.is_empty() && despawn_inputs_scale_kilo_meter_1.is_empty() && transfer_inputs_scale_kilo_meter_1.is_empty()
        && spawn_inputs_scale_kilo_meter_10.is_empty() && despawn_inputs_scale_kilo_meter_10.is_empty() && transfer_inputs_scale_kilo_meter_10.is_empty()
        && spawn_inputs_scale_kilo_meter_100.is_empty() && despawn_inputs_scale_kilo_meter_100.is_empty() && transfer_inputs_scale_kilo_meter_100.is_empty()
        && spawn_inputs_scale_mega_meter_1.is_empty() && despawn_inputs_scale_mega_meter_1.is_empty() && transfer_inputs_scale_mega_meter_1.is_empty()
        && spawn_inputs_scale_mega_meter_10.is_empty() && despawn_inputs_scale_mega_meter_10.is_empty() && transfer_inputs_scale_mega_meter_10.is_empty()
        && spawn_inputs_scale_mega_meter_100.is_empty() && despawn_inputs_scale_mega_meter_100.is_empty() && transfer_inputs_scale_mega_meter_100.is_empty()
        && spawn_inputs_scale_giga_meter_1.is_empty() && despawn_inputs_scale_giga_meter_1.is_empty() && transfer_inputs_scale_giga_meter_1.is_empty()
        && spawn_inputs_scale_giga_meter_10.is_empty() && despawn_inputs_scale_giga_meter_10.is_empty() && transfer_inputs_scale_giga_meter_10.is_empty()
        && spawn_inputs_scale_giga_meter_100.is_empty() && despawn_inputs_scale_giga_meter_100.is_empty() && transfer_inputs_scale_giga_meter_100.is_empty()
        && spawn_inputs_scale_tera_meter_1.is_empty() && despawn_inputs_scale_tera_meter_1.is_empty() && transfer_inputs_scale_tera_meter_1.is_empty()
        && spawn_inputs_scale_tera_meter_10.is_empty() && despawn_inputs_scale_tera_meter_10.is_empty() && transfer_inputs_scale_tera_meter_10.is_empty()
        && spawn_inputs_scale_tera_meter_100.is_empty() && despawn_inputs_scale_tera_meter_100.is_empty() && transfer_inputs_scale_tera_meter_100.is_empty()
        && spawn_inputs_scale_peta_meter_1.is_empty() && despawn_inputs_scale_peta_meter_1.is_empty() && transfer_inputs_scale_peta_meter_1.is_empty()
        && spawn_inputs_scale_peta_meter_10.is_empty() && despawn_inputs_scale_peta_meter_10.is_empty() && transfer_inputs_scale_peta_meter_10.is_empty()
        && spawn_inputs_scale_peta_meter_100.is_empty() && despawn_inputs_scale_peta_meter_100.is_empty() && transfer_inputs_scale_peta_meter_100.is_empty()
        && spawn_inputs_scale_exa_meter_1.is_empty() && despawn_inputs_scale_exa_meter_1.is_empty() && transfer_inputs_scale_exa_meter_1.is_empty()
        && spawn_inputs_scale_exa_meter_10.is_empty() && despawn_inputs_scale_exa_meter_10.is_empty() && transfer_inputs_scale_exa_meter_10.is_empty()
        && spawn_inputs_scale_exa_meter_100.is_empty() && despawn_inputs_scale_exa_meter_100.is_empty() && transfer_inputs_scale_exa_meter_100.is_empty()
        && spawn_inputs_scale_zetta_meter_1.is_empty() && despawn_inputs_scale_zetta_meter_1.is_empty() && transfer_inputs_scale_zetta_meter_1.is_empty()
        && spawn_inputs_scale_zetta_meter_10.is_empty() && despawn_inputs_scale_zetta_meter_10.is_empty() && transfer_inputs_scale_zetta_meter_10.is_empty()
        && spawn_inputs_scale_zetta_meter_100.is_empty() && despawn_inputs_scale_zetta_meter_100.is_empty() && transfer_inputs_scale_zetta_meter_100.is_empty()
        && spawn_inputs_scale_yotta_meter_1.is_empty() && despawn_inputs_scale_yotta_meter_1.is_empty() && transfer_inputs_scale_yotta_meter_1.is_empty()
        && spawn_inputs_scale_yotta_meter_10.is_empty() && despawn_inputs_scale_yotta_meter_10.is_empty() && transfer_inputs_scale_yotta_meter_10.is_empty()
        && spawn_inputs_scale_yotta_meter_100.is_empty() && despawn_inputs_scale_yotta_meter_100.is_empty() && transfer_inputs_scale_yotta_meter_100.is_empty()
        && spawn_inputs_scale_ronna_meter_1.is_empty() && despawn_inputs_scale_ronna_meter_1.is_empty() && transfer_inputs_scale_ronna_meter_1.is_empty()
        && spawn_inputs_scale_ronna_meter_10.is_empty() && despawn_inputs_scale_ronna_meter_10.is_empty() && transfer_inputs_scale_ronna_meter_10.is_empty()
        && spawn_inputs_scale_ronna_meter_100.is_empty() && despawn_inputs_scale_ronna_meter_100.is_empty() && transfer_inputs_scale_ronna_meter_100.is_empty()
        && spawn_inputs_scale_quetta_meter_1.is_empty() && despawn_inputs_scale_quetta_meter_1.is_empty() && transfer_inputs_scale_quetta_meter_1.is_empty()
        && spawn_inputs_scale_quetta_meter_10.is_empty() && despawn_inputs_scale_quetta_meter_10.is_empty() && transfer_inputs_scale_quetta_meter_10.is_empty()
        && spawn_inputs_scale_quetta_meter_100.is_empty() && despawn_inputs_scale_quetta_meter_100.is_empty() && transfer_inputs_scale_quetta_meter_100.is_empty()
        && spawn_inputs_scale_quetta_meter_1000.is_empty() && despawn_inputs_scale_quetta_meter_1000.is_empty() && transfer_inputs_scale_quetta_meter_1000.is_empty()
        && spawn_inputs_scale_quetta_meter_10000.is_empty() && despawn_inputs_scale_quetta_meter_10000.is_empty() && transfer_inputs_scale_quetta_meter_10000.is_empty()
        && spawn_inputs_scale_quetta_meter_100000.is_empty() && despawn_inputs_scale_quetta_meter_100000.is_empty() && transfer_inputs_scale_quetta_meter_100000.is_empty()
    {
        // warn!("No chunk actions to process");
        return;
    }

    let mut new_chunk_loaders_scale_quecto_meter_000001 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_quecto_meter_000001 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_000001.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_quecto_meter_000001.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_quecto_meter_00001 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_quecto_meter_00001 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_00001.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_quecto_meter_00001.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_quecto_meter_0001 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_quecto_meter_0001 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_0001.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_quecto_meter_0001.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_quecto_meter_001 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_quecto_meter_001 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_001.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_quecto_meter_001.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_quecto_meter_01 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_quecto_meter_01 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_01.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_quecto_meter_01.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_quecto_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_quecto_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_quecto_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_quecto_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_quecto_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_quecto_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_quecto_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_quecto_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quecto_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_quecto_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_ronto_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_ronto_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronto_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_ronto_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_ronto_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_ronto_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronto_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_ronto_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_ronto_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_ronto_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronto_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_ronto_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_yocto_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_yocto_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yocto_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_yocto_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_yocto_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_yocto_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yocto_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_yocto_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_yocto_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_yocto_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yocto_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_yocto_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_zepto_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_zepto_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zepto_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_zepto_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_zepto_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_zepto_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zepto_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_zepto_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_zepto_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_zepto_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zepto_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_zepto_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_atto_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_atto_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_atto_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_atto_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_atto_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_atto_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_atto_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_atto_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_atto_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_atto_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_atto_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_atto_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_femto_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_femto_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_femto_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_femto_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_femto_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_femto_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_femto_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_femto_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_femto_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_femto_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_femto_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_femto_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_pico_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_pico_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_pico_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_pico_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_pico_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_pico_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_pico_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_pico_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_pico_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_pico_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_pico_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_pico_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_nano_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_nano_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_nano_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_nano_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_nano_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_nano_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_nano_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_nano_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_nano_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_nano_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_nano_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_nano_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_micro_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_micro_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_micro_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_micro_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_micro_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_micro_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_micro_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_micro_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_micro_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_micro_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_micro_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_micro_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_milli_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_milli_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_milli_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_milli_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_milli_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_milli_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_milli_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_milli_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_milli_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_milli_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_milli_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_milli_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_kilo_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_kilo_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_kilo_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_kilo_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_kilo_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_kilo_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_kilo_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_kilo_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_kilo_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_kilo_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_kilo_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_kilo_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_mega_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_mega_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_mega_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_mega_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_mega_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_mega_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_mega_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_mega_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_mega_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_mega_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_mega_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_mega_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_giga_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_giga_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_giga_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_giga_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_giga_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_giga_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_giga_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_giga_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_giga_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_giga_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_giga_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_giga_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_tera_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_tera_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_tera_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_tera_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_tera_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_tera_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_tera_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_tera_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_tera_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_tera_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_tera_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_tera_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_peta_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_peta_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_peta_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_peta_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_peta_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_peta_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_peta_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_peta_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_peta_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_peta_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_peta_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_peta_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_exa_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_exa_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_exa_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_exa_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_exa_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_exa_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_exa_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_exa_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_exa_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_exa_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_exa_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_exa_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_zetta_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_zetta_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zetta_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_zetta_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_zetta_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_zetta_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zetta_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_zetta_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_zetta_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_zetta_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_zetta_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_zetta_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_yotta_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_yotta_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yotta_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_yotta_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_yotta_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_yotta_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yotta_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_yotta_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_yotta_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_yotta_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_yotta_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_yotta_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_ronna_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_ronna_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronna_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_ronna_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_ronna_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_ronna_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronna_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_ronna_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_ronna_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_ronna_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_ronna_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_ronna_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_quetta_meter_1 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_quetta_meter_1 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_1.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_quetta_meter_1.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_quetta_meter_10 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_quetta_meter_10 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_10.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_quetta_meter_10.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_quetta_meter_100 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_quetta_meter_100 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_100.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_quetta_meter_100.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_quetta_meter_1000 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_quetta_meter_1000 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_1000.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_quetta_meter_1000.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_quetta_meter_10000 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_quetta_meter_10000 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_10000.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_quetta_meter_10000.push(loader_entity);
            }
        }
    }
    let mut new_chunk_loaders_scale_quetta_meter_100000 = Vec::new();
    for chunk_loader_performing_chunk_load in chunk_loaders_performing_chunk_loads_scale_quetta_meter_100000 {
        let loader_entity = chunk_loader_performing_chunk_load.entity();
        if let Ok(init_hook) = chunk_loader_init_hook_queries.chunk_loader_init_hook_query_scale_quetta_meter_100000.get(loader_entity) {
            if !init_hook.has_fired() {
                new_chunk_loaders_scale_quetta_meter_100000.push(loader_entity);
            }
        }
    }

    // Step 3: Build & launch composite workflows
    let spawn_handle = if !spawn_inputs_scale_quetta_meter_100000.is_empty() 
        || !spawn_inputs_scale_quetta_meter_10000.is_empty()
        || !spawn_inputs_scale_quetta_meter_1000.is_empty()
        || !spawn_inputs_scale_quetta_meter_100.is_empty()
        || !spawn_inputs_scale_quetta_meter_10.is_empty()
        || !spawn_inputs_scale_quetta_meter_1.is_empty()
        || !spawn_inputs_scale_ronna_meter_100.is_empty()
        || !spawn_inputs_scale_ronna_meter_10.is_empty()
        || !spawn_inputs_scale_ronna_meter_1.is_empty()
        || !spawn_inputs_scale_yotta_meter_100.is_empty()
        || !spawn_inputs_scale_yotta_meter_10.is_empty()
        || !spawn_inputs_scale_yotta_meter_1.is_empty()
        || !spawn_inputs_scale_zetta_meter_100.is_empty()
        || !spawn_inputs_scale_zetta_meter_10.is_empty()
        || !spawn_inputs_scale_zetta_meter_1.is_empty()
        || !spawn_inputs_scale_exa_meter_100.is_empty()
        || !spawn_inputs_scale_exa_meter_10.is_empty()
        || !spawn_inputs_scale_exa_meter_1.is_empty()
        || !spawn_inputs_scale_peta_meter_100.is_empty()
        || !spawn_inputs_scale_peta_meter_10.is_empty()
        || !spawn_inputs_scale_peta_meter_1.is_empty()
        || !spawn_inputs_scale_tera_meter_100.is_empty()
        || !spawn_inputs_scale_tera_meter_10.is_empty()
        || !spawn_inputs_scale_tera_meter_1.is_empty()
        || !spawn_inputs_scale_giga_meter_100.is_empty()
        || !spawn_inputs_scale_giga_meter_10.is_empty()
        || !spawn_inputs_scale_giga_meter_1.is_empty()
        || !spawn_inputs_scale_mega_meter_100.is_empty()
        || !spawn_inputs_scale_mega_meter_10.is_empty()
        || !spawn_inputs_scale_mega_meter_1.is_empty()
        || !spawn_inputs_scale_kilo_meter_100.is_empty()
        || !spawn_inputs_scale_kilo_meter_10.is_empty()
        || !spawn_inputs_scale_kilo_meter_1.is_empty()
        || !spawn_inputs_scale_meter_100.is_empty()
        || !spawn_inputs_scale_meter_10.is_empty()
        || !spawn_inputs_scale_meter_1.is_empty()
        || !spawn_inputs_scale_milli_meter_100.is_empty()
        || !spawn_inputs_scale_milli_meter_10.is_empty()
        || !spawn_inputs_scale_milli_meter_1.is_empty()
        || !spawn_inputs_scale_micro_meter_100.is_empty()
        || !spawn_inputs_scale_micro_meter_10.is_empty()
        || !spawn_inputs_scale_micro_meter_1.is_empty()
        || !spawn_inputs_scale_nano_meter_100.is_empty()
        || !spawn_inputs_scale_nano_meter_10.is_empty()
        || !spawn_inputs_scale_nano_meter_1.is_empty()
        || !spawn_inputs_scale_pico_meter_100.is_empty()
        || !spawn_inputs_scale_pico_meter_10.is_empty()
        || !spawn_inputs_scale_pico_meter_1.is_empty()
        || !spawn_inputs_scale_femto_meter_100.is_empty()
        || !spawn_inputs_scale_femto_meter_10.is_empty()
        || !spawn_inputs_scale_femto_meter_1.is_empty()
        || !spawn_inputs_scale_atto_meter_100.is_empty()
        || !spawn_inputs_scale_atto_meter_10.is_empty()
        || !spawn_inputs_scale_atto_meter_1.is_empty()
        || !spawn_inputs_scale_zepto_meter_100.is_empty()
        || !spawn_inputs_scale_zepto_meter_10.is_empty()
        || !spawn_inputs_scale_zepto_meter_1.is_empty()
        || !spawn_inputs_scale_yocto_meter_100.is_empty()
        || !spawn_inputs_scale_yocto_meter_10.is_empty()
        || !spawn_inputs_scale_yocto_meter_1.is_empty()
        || !spawn_inputs_scale_ronto_meter_100.is_empty()
        || !spawn_inputs_scale_ronto_meter_10.is_empty()
        || !spawn_inputs_scale_ronto_meter_1.is_empty()
        || !spawn_inputs_scale_quecto_meter_100.is_empty()
        || !spawn_inputs_scale_quecto_meter_10.is_empty()
        || !spawn_inputs_scale_quecto_meter_1.is_empty()
        || !spawn_inputs_scale_quecto_meter_01.is_empty()
        || !spawn_inputs_scale_quecto_meter_001.is_empty()
        || !spawn_inputs_scale_quecto_meter_0001.is_empty()
        || !spawn_inputs_scale_quecto_meter_00001.is_empty()
        || !spawn_inputs_scale_quecto_meter_000001.is_empty()
    {
        let texture_size = CONFIG().get::<u32>("chunk/size");
        let chunk_size = CONFIG().get::<u32>("chunk/size");
        let current_view_scale = CONFIG().get::<i32>("chunk_loader/current_view_scale");

        let param_data_scale_quecto_meter_000001 = spawn_coords_scale_quecto_meter_000001
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -35,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_quecto_meter_00001 = spawn_coords_scale_quecto_meter_00001
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -34,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_quecto_meter_0001 = spawn_coords_scale_quecto_meter_0001
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -33,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_quecto_meter_001 = spawn_coords_scale_quecto_meter_001
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -32,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_quecto_meter_01 = spawn_coords_scale_quecto_meter_01
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -31,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_quecto_meter_1 = spawn_coords_scale_quecto_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -30,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_quecto_meter_10 = spawn_coords_scale_quecto_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -29,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_quecto_meter_100 = spawn_coords_scale_quecto_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -28,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_ronto_meter_1 = spawn_coords_scale_ronto_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -27,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_ronto_meter_10 = spawn_coords_scale_ronto_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -26,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_ronto_meter_100 = spawn_coords_scale_ronto_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -25,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_yocto_meter_1 = spawn_coords_scale_yocto_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -24,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_yocto_meter_10 = spawn_coords_scale_yocto_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -23,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_yocto_meter_100 = spawn_coords_scale_yocto_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -22,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_zepto_meter_1 = spawn_coords_scale_zepto_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -21,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_zepto_meter_10 = spawn_coords_scale_zepto_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -20,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_zepto_meter_100 = spawn_coords_scale_zepto_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -19,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_atto_meter_1 = spawn_coords_scale_atto_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -18,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_atto_meter_10 = spawn_coords_scale_atto_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -17,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_atto_meter_100 = spawn_coords_scale_atto_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -16,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_femto_meter_1 = spawn_coords_scale_femto_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -15,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_femto_meter_10 = spawn_coords_scale_femto_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -14,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_femto_meter_100 = spawn_coords_scale_femto_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -13,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_pico_meter_1 = spawn_coords_scale_pico_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -12,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_pico_meter_10 = spawn_coords_scale_pico_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -11,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_pico_meter_100 = spawn_coords_scale_pico_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -10,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_nano_meter_1 = spawn_coords_scale_nano_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -9,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_nano_meter_10 = spawn_coords_scale_nano_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -8,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_nano_meter_100 = spawn_coords_scale_nano_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -7,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_micro_meter_1 = spawn_coords_scale_micro_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -6,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_micro_meter_10 = spawn_coords_scale_micro_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -5,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_micro_meter_100 = spawn_coords_scale_micro_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -4,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_milli_meter_1 = spawn_coords_scale_milli_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -3,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_milli_meter_10 = spawn_coords_scale_milli_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -2,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_milli_meter_100 = spawn_coords_scale_milli_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: -1,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_meter_1 = spawn_coords_scale_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 0,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_meter_10 = spawn_coords_scale_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 1,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_meter_100 = spawn_coords_scale_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 2,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_kilo_meter_1 = spawn_coords_scale_kilo_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 3,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_kilo_meter_10 = spawn_coords_scale_kilo_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 4,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_kilo_meter_100 = spawn_coords_scale_kilo_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 5,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_mega_meter_1 = spawn_coords_scale_mega_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 6,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_mega_meter_10 = spawn_coords_scale_mega_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 7,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_mega_meter_100 = spawn_coords_scale_mega_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 8,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_giga_meter_1 = spawn_coords_scale_giga_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 9,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_giga_meter_10 = spawn_coords_scale_giga_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 10,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_giga_meter_100 = spawn_coords_scale_giga_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 11,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_tera_meter_1 = spawn_coords_scale_tera_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 12,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_tera_meter_10 = spawn_coords_scale_tera_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 13,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_tera_meter_100 = spawn_coords_scale_tera_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 14,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_peta_meter_1 = spawn_coords_scale_peta_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 15,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_peta_meter_10 = spawn_coords_scale_peta_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 16,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_peta_meter_100 = spawn_coords_scale_peta_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 17,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_exa_meter_1 = spawn_coords_scale_exa_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 18,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_exa_meter_10 = spawn_coords_scale_exa_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 19,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_exa_meter_100 = spawn_coords_scale_exa_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 20,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_zetta_meter_1 = spawn_coords_scale_zetta_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 21,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_zetta_meter_10 = spawn_coords_scale_zetta_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 22,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_zetta_meter_100 = spawn_coords_scale_zetta_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 23,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_yotta_meter_1 = spawn_coords_scale_yotta_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 24,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_yotta_meter_10 = spawn_coords_scale_yotta_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 25,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_yotta_meter_100 = spawn_coords_scale_yotta_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 26,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_ronna_meter_1 = spawn_coords_scale_ronna_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 27,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_ronna_meter_10 = spawn_coords_scale_ronna_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 28,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_ronna_meter_100 = spawn_coords_scale_ronna_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 29,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_quetta_meter_1 = spawn_coords_scale_quetta_meter_1
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 30,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_quetta_meter_10 = spawn_coords_scale_quetta_meter_10
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 31,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_quetta_meter_100 = spawn_coords_scale_quetta_meter_100
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 32,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_quetta_meter_1000 = spawn_coords_scale_quetta_meter_1000
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 33,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_quetta_meter_10000 = spawn_coords_scale_quetta_meter_10000
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 34,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();
        let param_data_scale_quetta_meter_100000 = spawn_coords_scale_quetta_meter_100000
            .iter()
            .map(|&(x, y)| crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams {
                chunk_pos: [x, y],
                chunk_size,
                chunk_scale: 35,
                current_view_scale,
                _padding0: 0,
                _padding1: [0, 0, 0, 0],
            })
            .collect::<Vec<_>>();

        let new_chunk_loaders_scale_quecto_meter_000001 = new_chunk_loaders_scale_quecto_meter_000001.clone();
        let new_chunk_loaders_scale_quecto_meter_00001 = new_chunk_loaders_scale_quecto_meter_00001.clone();
        let new_chunk_loaders_scale_quecto_meter_0001 = new_chunk_loaders_scale_quecto_meter_0001.clone();
        let new_chunk_loaders_scale_quecto_meter_001 = new_chunk_loaders_scale_quecto_meter_001.clone();
        let new_chunk_loaders_scale_quecto_meter_01 = new_chunk_loaders_scale_quecto_meter_01.clone();
        let new_chunk_loaders_scale_quecto_meter_1 = new_chunk_loaders_scale_quecto_meter_1.clone();
        let new_chunk_loaders_scale_quecto_meter_10 = new_chunk_loaders_scale_quecto_meter_10.clone();
        let new_chunk_loaders_scale_quecto_meter_100 = new_chunk_loaders_scale_quecto_meter_100.clone();
        let new_chunk_loaders_scale_ronto_meter_1 = new_chunk_loaders_scale_ronto_meter_1.clone();
        let new_chunk_loaders_scale_ronto_meter_10 = new_chunk_loaders_scale_ronto_meter_10.clone();
        let new_chunk_loaders_scale_ronto_meter_100 = new_chunk_loaders_scale_ronto_meter_100.clone();
        let new_chunk_loaders_scale_yocto_meter_1 = new_chunk_loaders_scale_yocto_meter_1.clone();
        let new_chunk_loaders_scale_yocto_meter_10 = new_chunk_loaders_scale_yocto_meter_10.clone();
        let new_chunk_loaders_scale_yocto_meter_100 = new_chunk_loaders_scale_yocto_meter_100.clone();
        let new_chunk_loaders_scale_zepto_meter_1 = new_chunk_loaders_scale_zepto_meter_1.clone();
        let new_chunk_loaders_scale_zepto_meter_10 = new_chunk_loaders_scale_zepto_meter_10.clone();
        let new_chunk_loaders_scale_zepto_meter_100 = new_chunk_loaders_scale_zepto_meter_100.clone();
        let new_chunk_loaders_scale_atto_meter_1 = new_chunk_loaders_scale_atto_meter_1.clone();
        let new_chunk_loaders_scale_atto_meter_10 = new_chunk_loaders_scale_atto_meter_10.clone();
        let new_chunk_loaders_scale_atto_meter_100 = new_chunk_loaders_scale_atto_meter_100.clone();
        let new_chunk_loaders_scale_femto_meter_1 = new_chunk_loaders_scale_femto_meter_1.clone();
        let new_chunk_loaders_scale_femto_meter_10 = new_chunk_loaders_scale_femto_meter_10.clone();
        let new_chunk_loaders_scale_femto_meter_100 = new_chunk_loaders_scale_femto_meter_100.clone();
        let new_chunk_loaders_scale_pico_meter_1 = new_chunk_loaders_scale_pico_meter_1.clone();
        let new_chunk_loaders_scale_pico_meter_10 = new_chunk_loaders_scale_pico_meter_10.clone();
        let new_chunk_loaders_scale_pico_meter_100 = new_chunk_loaders_scale_pico_meter_100.clone();
        let new_chunk_loaders_scale_nano_meter_1 = new_chunk_loaders_scale_nano_meter_1.clone();
        let new_chunk_loaders_scale_nano_meter_10 = new_chunk_loaders_scale_nano_meter_10.clone();
        let new_chunk_loaders_scale_nano_meter_100 = new_chunk_loaders_scale_nano_meter_100.clone();
        let new_chunk_loaders_scale_micro_meter_1 = new_chunk_loaders_scale_micro_meter_1.clone();
        let new_chunk_loaders_scale_micro_meter_10 = new_chunk_loaders_scale_micro_meter_10.clone();
        let new_chunk_loaders_scale_micro_meter_100 = new_chunk_loaders_scale_micro_meter_100.clone();
        let new_chunk_loaders_scale_milli_meter_1 = new_chunk_loaders_scale_milli_meter_1.clone();
        let new_chunk_loaders_scale_milli_meter_10 = new_chunk_loaders_scale_milli_meter_10.clone();
        let new_chunk_loaders_scale_milli_meter_100 = new_chunk_loaders_scale_milli_meter_100.clone();
        let new_chunk_loaders_scale_meter_1 = new_chunk_loaders_scale_meter_1.clone();
        let new_chunk_loaders_scale_meter_10 = new_chunk_loaders_scale_meter_10.clone();
        let new_chunk_loaders_scale_meter_100 = new_chunk_loaders_scale_meter_100.clone();
        let new_chunk_loaders_scale_kilo_meter_1 = new_chunk_loaders_scale_kilo_meter_1.clone();
        let new_chunk_loaders_scale_kilo_meter_10 = new_chunk_loaders_scale_kilo_meter_10.clone();
        let new_chunk_loaders_scale_kilo_meter_100 = new_chunk_loaders_scale_kilo_meter_100.clone();
        let new_chunk_loaders_scale_mega_meter_1 = new_chunk_loaders_scale_mega_meter_1.clone();
        let new_chunk_loaders_scale_mega_meter_10 = new_chunk_loaders_scale_mega_meter_10.clone();
        let new_chunk_loaders_scale_mega_meter_100 = new_chunk_loaders_scale_mega_meter_100.clone();
        let new_chunk_loaders_scale_giga_meter_1 = new_chunk_loaders_scale_giga_meter_1.clone();
        let new_chunk_loaders_scale_giga_meter_10 = new_chunk_loaders_scale_giga_meter_10.clone();
        let new_chunk_loaders_scale_giga_meter_100 = new_chunk_loaders_scale_giga_meter_100.clone();
        let new_chunk_loaders_scale_tera_meter_1 = new_chunk_loaders_scale_tera_meter_1.clone();
        let new_chunk_loaders_scale_tera_meter_10 = new_chunk_loaders_scale_tera_meter_10.clone();
        let new_chunk_loaders_scale_tera_meter_100 = new_chunk_loaders_scale_tera_meter_100.clone();
        let new_chunk_loaders_scale_peta_meter_1 = new_chunk_loaders_scale_peta_meter_1.clone();
        let new_chunk_loaders_scale_peta_meter_10 = new_chunk_loaders_scale_peta_meter_10.clone();
        let new_chunk_loaders_scale_peta_meter_100 = new_chunk_loaders_scale_peta_meter_100.clone();
        let new_chunk_loaders_scale_exa_meter_1 = new_chunk_loaders_scale_exa_meter_1.clone();
        let new_chunk_loaders_scale_exa_meter_10 = new_chunk_loaders_scale_exa_meter_10.clone();
        let new_chunk_loaders_scale_exa_meter_100 = new_chunk_loaders_scale_exa_meter_100.clone();
        let new_chunk_loaders_scale_zetta_meter_1 = new_chunk_loaders_scale_zetta_meter_1.clone();
        let new_chunk_loaders_scale_zetta_meter_10 = new_chunk_loaders_scale_zetta_meter_10.clone();
        let new_chunk_loaders_scale_zetta_meter_100 = new_chunk_loaders_scale_zetta_meter_100.clone();
        let new_chunk_loaders_scale_yotta_meter_1 = new_chunk_loaders_scale_yotta_meter_1.clone();
        let new_chunk_loaders_scale_yotta_meter_10 = new_chunk_loaders_scale_yotta_meter_10.clone();
        let new_chunk_loaders_scale_yotta_meter_100 = new_chunk_loaders_scale_yotta_meter_100.clone();
        let new_chunk_loaders_scale_ronna_meter_1 = new_chunk_loaders_scale_ronna_meter_1.clone();
        let new_chunk_loaders_scale_ronna_meter_10 = new_chunk_loaders_scale_ronna_meter_10.clone();
        let new_chunk_loaders_scale_ronna_meter_100 = new_chunk_loaders_scale_ronna_meter_100.clone();
        let new_chunk_loaders_scale_quetta_meter_1 = new_chunk_loaders_scale_quetta_meter_1.clone();
        let new_chunk_loaders_scale_quetta_meter_10 = new_chunk_loaders_scale_quetta_meter_10.clone();
        let new_chunk_loaders_scale_quetta_meter_100 = new_chunk_loaders_scale_quetta_meter_100.clone();
        let new_chunk_loaders_scale_quetta_meter_1000 = new_chunk_loaders_scale_quetta_meter_1000.clone();
        let new_chunk_loaders_scale_quetta_meter_10000 = new_chunk_loaders_scale_quetta_meter_10000.clone();
        let new_chunk_loaders_scale_quetta_meter_100000 = new_chunk_loaders_scale_quetta_meter_100000.clone();

        Some(composite_workflow!(
            SpawnChunks,
            move in texture_size: u32,
            move in spawn_inputs_scale_quecto_meter_000001: Vec<SpawnChunkInput<ScaleQuectoMeter000001>>,
            move in spawn_inputs_scale_quecto_meter_00001: Vec<SpawnChunkInput<ScaleQuectoMeter00001>>,
            move in spawn_inputs_scale_quecto_meter_0001: Vec<SpawnChunkInput<ScaleQuectoMeter0001>>,
            move in spawn_inputs_scale_quecto_meter_001: Vec<SpawnChunkInput<ScaleQuectoMeter001>>,
            move in spawn_inputs_scale_quecto_meter_01: Vec<SpawnChunkInput<ScaleQuectoMeter01>>,
            move in spawn_inputs_scale_quecto_meter_1: Vec<SpawnChunkInput<ScaleQuectoMeter1>>,
            move in spawn_inputs_scale_quecto_meter_10: Vec<SpawnChunkInput<ScaleQuectoMeter10>>,
            move in spawn_inputs_scale_quecto_meter_100: Vec<SpawnChunkInput<ScaleQuectoMeter100>>,
            move in spawn_inputs_scale_ronto_meter_1: Vec<SpawnChunkInput<ScaleRontoMeter1>>,
            move in spawn_inputs_scale_ronto_meter_10: Vec<SpawnChunkInput<ScaleRontoMeter10>>,
            move in spawn_inputs_scale_ronto_meter_100: Vec<SpawnChunkInput<ScaleRontoMeter100>>,
            move in spawn_inputs_scale_yocto_meter_1: Vec<SpawnChunkInput<ScaleYoctoMeter1>>,
            move in spawn_inputs_scale_yocto_meter_10: Vec<SpawnChunkInput<ScaleYoctoMeter10>>,
            move in spawn_inputs_scale_yocto_meter_100: Vec<SpawnChunkInput<ScaleYoctoMeter100>>,
            move in spawn_inputs_scale_zepto_meter_1: Vec<SpawnChunkInput<ScaleZeptoMeter1>>,
            move in spawn_inputs_scale_zepto_meter_10: Vec<SpawnChunkInput<ScaleZeptoMeter10>>,
            move in spawn_inputs_scale_zepto_meter_100: Vec<SpawnChunkInput<ScaleZeptoMeter100>>,
            move in spawn_inputs_scale_atto_meter_1: Vec<SpawnChunkInput<ScaleAttoMeter1>>,
            move in spawn_inputs_scale_atto_meter_10: Vec<SpawnChunkInput<ScaleAttoMeter10>>,
            move in spawn_inputs_scale_atto_meter_100: Vec<SpawnChunkInput<ScaleAttoMeter100>>,
            move in spawn_inputs_scale_femto_meter_1: Vec<SpawnChunkInput<ScaleFemtoMeter1>>,
            move in spawn_inputs_scale_femto_meter_10: Vec<SpawnChunkInput<ScaleFemtoMeter10>>,
            move in spawn_inputs_scale_femto_meter_100: Vec<SpawnChunkInput<ScaleFemtoMeter100>>,
            move in spawn_inputs_scale_pico_meter_1: Vec<SpawnChunkInput<ScalePicoMeter1>>,
            move in spawn_inputs_scale_pico_meter_10: Vec<SpawnChunkInput<ScalePicoMeter10>>,
            move in spawn_inputs_scale_pico_meter_100: Vec<SpawnChunkInput<ScalePicoMeter100>>,
            move in spawn_inputs_scale_nano_meter_1: Vec<SpawnChunkInput<ScaleNanoMeter1>>,
            move in spawn_inputs_scale_nano_meter_10: Vec<SpawnChunkInput<ScaleNanoMeter10>>,
            move in spawn_inputs_scale_nano_meter_100: Vec<SpawnChunkInput<ScaleNanoMeter100>>,
            move in spawn_inputs_scale_micro_meter_1: Vec<SpawnChunkInput<ScaleMicroMeter1>>,
            move in spawn_inputs_scale_micro_meter_10: Vec<SpawnChunkInput<ScaleMicroMeter10>>,
            move in spawn_inputs_scale_micro_meter_100: Vec<SpawnChunkInput<ScaleMicroMeter100>>,
            move in spawn_inputs_scale_milli_meter_1: Vec<SpawnChunkInput<ScaleMilliMeter1>>,
            move in spawn_inputs_scale_milli_meter_10: Vec<SpawnChunkInput<ScaleMilliMeter10>>,
            move in spawn_inputs_scale_milli_meter_100: Vec<SpawnChunkInput<ScaleMilliMeter100>>,
            move in spawn_inputs_scale_meter_1: Vec<SpawnChunkInput<ScaleMeter1>>,
            move in spawn_inputs_scale_meter_10: Vec<SpawnChunkInput<ScaleMeter10>>,
            move in spawn_inputs_scale_meter_100: Vec<SpawnChunkInput<ScaleMeter100>>,
            move in spawn_inputs_scale_kilo_meter_1: Vec<SpawnChunkInput<ScaleKiloMeter1>>,
            move in spawn_inputs_scale_kilo_meter_10: Vec<SpawnChunkInput<ScaleKiloMeter10>>,
            move in spawn_inputs_scale_kilo_meter_100: Vec<SpawnChunkInput<ScaleKiloMeter100>>,
            move in spawn_inputs_scale_mega_meter_1: Vec<SpawnChunkInput<ScaleMegaMeter1>>,
            move in spawn_inputs_scale_mega_meter_10: Vec<SpawnChunkInput<ScaleMegaMeter10>>,
            move in spawn_inputs_scale_mega_meter_100: Vec<SpawnChunkInput<ScaleMegaMeter100>>,
            move in spawn_inputs_scale_giga_meter_1: Vec<SpawnChunkInput<ScaleGigaMeter1>>,
            move in spawn_inputs_scale_giga_meter_10: Vec<SpawnChunkInput<ScaleGigaMeter10>>,
            move in spawn_inputs_scale_giga_meter_100: Vec<SpawnChunkInput<ScaleGigaMeter100>>,
            move in spawn_inputs_scale_tera_meter_1: Vec<SpawnChunkInput<ScaleTeraMeter1>>,
            move in spawn_inputs_scale_tera_meter_10: Vec<SpawnChunkInput<ScaleTeraMeter10>>,
            move in spawn_inputs_scale_tera_meter_100: Vec<SpawnChunkInput<ScaleTeraMeter100>>,
            move in spawn_inputs_scale_peta_meter_1: Vec<SpawnChunkInput<ScalePetaMeter1>>,
            move in spawn_inputs_scale_peta_meter_10: Vec<SpawnChunkInput<ScalePetaMeter10>>,
            move in spawn_inputs_scale_peta_meter_100: Vec<SpawnChunkInput<ScalePetaMeter100>>,
            move in spawn_inputs_scale_exa_meter_1: Vec<SpawnChunkInput<ScaleExaMeter1>>,
            move in spawn_inputs_scale_exa_meter_10: Vec<SpawnChunkInput<ScaleExaMeter10>>,
            move in spawn_inputs_scale_exa_meter_100: Vec<SpawnChunkInput<ScaleExaMeter100>>,
            move in spawn_inputs_scale_zetta_meter_1: Vec<SpawnChunkInput<ScaleZettaMeter1>>,
            move in spawn_inputs_scale_zetta_meter_10: Vec<SpawnChunkInput<ScaleZettaMeter10>>,
            move in spawn_inputs_scale_zetta_meter_100: Vec<SpawnChunkInput<ScaleZettaMeter100>>,
            move in spawn_inputs_scale_yotta_meter_1: Vec<SpawnChunkInput<ScaleYottaMeter1>>,
            move in spawn_inputs_scale_yotta_meter_10: Vec<SpawnChunkInput<ScaleYottaMeter10>>,
            move in spawn_inputs_scale_yotta_meter_100: Vec<SpawnChunkInput<ScaleYottaMeter100>>,
            move in spawn_inputs_scale_ronna_meter_1: Vec<SpawnChunkInput<ScaleRonnaMeter1>>,
            move in spawn_inputs_scale_ronna_meter_10: Vec<SpawnChunkInput<ScaleRonnaMeter10>>,
            move in spawn_inputs_scale_ronna_meter_100: Vec<SpawnChunkInput<ScaleRonnaMeter100>>,
            move in spawn_inputs_scale_quetta_meter_1: Vec<SpawnChunkInput<ScaleQuettaMeter1>>,
            move in spawn_inputs_scale_quetta_meter_10: Vec<SpawnChunkInput<ScaleQuettaMeter10>>,
            move in spawn_inputs_scale_quetta_meter_100: Vec<SpawnChunkInput<ScaleQuettaMeter100>>,
            move in spawn_inputs_scale_quetta_meter_1000: Vec<SpawnChunkInput<ScaleQuettaMeter1000>>,
            move in spawn_inputs_scale_quetta_meter_10000: Vec<SpawnChunkInput<ScaleQuettaMeter10000>>,
            move in spawn_inputs_scale_quetta_meter_100000: Vec<SpawnChunkInput<ScaleQuettaMeter100000>>,
            move in param_data_scale_quecto_meter_000001: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_quecto_meter_00001: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_quecto_meter_0001: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_quecto_meter_001: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_quecto_meter_01: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_quecto_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_quecto_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_quecto_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_ronto_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_ronto_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_ronto_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_yocto_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_yocto_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_yocto_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_zepto_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_zepto_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_zepto_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_atto_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_atto_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_atto_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_femto_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_femto_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_femto_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_pico_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_pico_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_pico_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_nano_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_nano_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_nano_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_micro_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_micro_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_micro_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_milli_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_milli_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_milli_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_kilo_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_kilo_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_kilo_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_mega_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_mega_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_mega_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_giga_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_giga_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_giga_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_tera_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_tera_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_tera_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_peta_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_peta_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_peta_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_exa_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_exa_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_exa_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_zetta_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_zetta_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_zetta_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_yotta_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_yotta_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_yotta_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_ronna_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_ronna_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_ronna_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_quetta_meter_1: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_quetta_meter_10: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_quetta_meter_100: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_quetta_meter_1000: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_quetta_meter_10000: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            move in param_data_scale_quetta_meter_100000: Vec<crate::gpu::workflows::gpu::generate_textures::user_items::ShaderParams>,
            new_chunk_loaders_scale_quecto_meter_000001: Vec<Entity>,
            new_chunk_loaders_scale_quecto_meter_00001: Vec<Entity>,
            new_chunk_loaders_scale_quecto_meter_0001: Vec<Entity>,
            new_chunk_loaders_scale_quecto_meter_001: Vec<Entity>,
            new_chunk_loaders_scale_quecto_meter_01: Vec<Entity>,
            new_chunk_loaders_scale_quecto_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_quecto_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_quecto_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_ronto_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_ronto_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_ronto_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_yocto_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_yocto_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_yocto_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_zepto_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_zepto_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_zepto_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_atto_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_atto_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_atto_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_femto_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_femto_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_femto_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_pico_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_pico_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_pico_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_nano_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_nano_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_nano_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_micro_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_micro_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_micro_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_milli_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_milli_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_milli_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_kilo_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_kilo_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_kilo_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_mega_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_mega_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_mega_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_giga_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_giga_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_giga_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_tera_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_tera_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_tera_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_peta_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_peta_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_peta_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_exa_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_exa_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_exa_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_zetta_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_zetta_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_zetta_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_yotta_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_yotta_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_yotta_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_ronna_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_ronna_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_ronna_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_quetta_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_quetta_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_quetta_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_quetta_meter_1000: Vec<Entity>,
            new_chunk_loaders_scale_quetta_meter_10000: Vec<Entity>,
            new_chunk_loaders_scale_quetta_meter_100000: Vec<Entity>,
        {
            warn!("Running composite workflow 'SpawnChunks'");

            let shader_name = CONFIG().get::<&'static str>("chunk/texture_generator_shader");

            let generate_output = workflow!(IO, Gpu::GenerateChunkTextures, Input {
                shader_name,
                texture_size,
                param_data_scale_quecto_meter_000001,
                param_data_scale_quecto_meter_00001,
                param_data_scale_quecto_meter_0001,
                param_data_scale_quecto_meter_001,
                param_data_scale_quecto_meter_01,
                param_data_scale_quecto_meter_1,
                param_data_scale_quecto_meter_10,
                param_data_scale_quecto_meter_100,
                param_data_scale_ronto_meter_1,
                param_data_scale_ronto_meter_10,
                param_data_scale_ronto_meter_100,
                param_data_scale_yocto_meter_1,
                param_data_scale_yocto_meter_10,
                param_data_scale_yocto_meter_100,
                param_data_scale_zepto_meter_1,
                param_data_scale_zepto_meter_10,
                param_data_scale_zepto_meter_100,
                param_data_scale_atto_meter_1,
                param_data_scale_atto_meter_10,
                param_data_scale_atto_meter_100,
                param_data_scale_femto_meter_1,
                param_data_scale_femto_meter_10,
                param_data_scale_femto_meter_100,
                param_data_scale_pico_meter_1,
                param_data_scale_pico_meter_10,
                param_data_scale_pico_meter_100,
                param_data_scale_nano_meter_1,
                param_data_scale_nano_meter_10,
                param_data_scale_nano_meter_100,
                param_data_scale_micro_meter_1,
                param_data_scale_micro_meter_10,
                param_data_scale_micro_meter_100,
                param_data_scale_milli_meter_1,
                param_data_scale_milli_meter_10,
                param_data_scale_milli_meter_100,
                param_data_scale_meter_1,
                param_data_scale_meter_10,
                param_data_scale_meter_100,
                param_data_scale_kilo_meter_1,
                param_data_scale_kilo_meter_10,
                param_data_scale_kilo_meter_100,
                param_data_scale_mega_meter_1,
                param_data_scale_mega_meter_10,
                param_data_scale_mega_meter_100,
                param_data_scale_giga_meter_1,
                param_data_scale_giga_meter_10,
                param_data_scale_giga_meter_100,
                param_data_scale_tera_meter_1,
                param_data_scale_tera_meter_10,
                param_data_scale_tera_meter_100,
                param_data_scale_peta_meter_1,
                param_data_scale_peta_meter_10,
                param_data_scale_peta_meter_100,
                param_data_scale_exa_meter_1,
                param_data_scale_exa_meter_10,
                param_data_scale_exa_meter_100,
                param_data_scale_zetta_meter_1,
                param_data_scale_zetta_meter_10,
                param_data_scale_zetta_meter_100,
                param_data_scale_yotta_meter_1,
                param_data_scale_yotta_meter_10,
                param_data_scale_yotta_meter_100,
                param_data_scale_ronna_meter_1,
                param_data_scale_ronna_meter_10,
                param_data_scale_ronna_meter_100,
                param_data_scale_quetta_meter_1,
                param_data_scale_quetta_meter_10,
                param_data_scale_quetta_meter_100,
                param_data_scale_quetta_meter_1000,
                param_data_scale_quetta_meter_10000,
                param_data_scale_quetta_meter_100000,
            });

            let spawn_inputs_with_textures_scale_quecto_meter_000001 = spawn_inputs_scale_quecto_meter_000001
                .into_iter()
                .zip(generate_output.render_executor_scale_quecto_meter_000001.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_quecto_meter_00001 = spawn_inputs_scale_quecto_meter_00001
                .into_iter()
                .zip(generate_output.render_executor_scale_quecto_meter_00001.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_quecto_meter_0001 = spawn_inputs_scale_quecto_meter_0001
                .into_iter()
                .zip(generate_output.render_executor_scale_quecto_meter_0001.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_quecto_meter_001 = spawn_inputs_scale_quecto_meter_001
                .into_iter()
                .zip(generate_output.render_executor_scale_quecto_meter_001.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_quecto_meter_01 = spawn_inputs_scale_quecto_meter_01
                .into_iter()
                .zip(generate_output.render_executor_scale_quecto_meter_01.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_quecto_meter_1 = spawn_inputs_scale_quecto_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_quecto_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_quecto_meter_10 = spawn_inputs_scale_quecto_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_quecto_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_quecto_meter_100 = spawn_inputs_scale_quecto_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_quecto_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_ronto_meter_1 = spawn_inputs_scale_ronto_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_ronto_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_ronto_meter_10 = spawn_inputs_scale_ronto_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_ronto_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_ronto_meter_100 = spawn_inputs_scale_ronto_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_ronto_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_yocto_meter_1 = spawn_inputs_scale_yocto_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_yocto_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_yocto_meter_10 = spawn_inputs_scale_yocto_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_yocto_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_yocto_meter_100 = spawn_inputs_scale_yocto_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_yocto_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_zepto_meter_1 = spawn_inputs_scale_zepto_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_zepto_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_zepto_meter_10 = spawn_inputs_scale_zepto_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_zepto_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_zepto_meter_100 = spawn_inputs_scale_zepto_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_zepto_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_atto_meter_1 = spawn_inputs_scale_atto_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_atto_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_atto_meter_10 = spawn_inputs_scale_atto_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_atto_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_atto_meter_100 = spawn_inputs_scale_atto_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_atto_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_femto_meter_1 = spawn_inputs_scale_femto_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_femto_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_femto_meter_10 = spawn_inputs_scale_femto_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_femto_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_femto_meter_100 = spawn_inputs_scale_femto_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_femto_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_pico_meter_1 = spawn_inputs_scale_pico_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_pico_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_pico_meter_10 = spawn_inputs_scale_pico_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_pico_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_pico_meter_100 = spawn_inputs_scale_pico_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_pico_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_nano_meter_1 = spawn_inputs_scale_nano_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_nano_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_nano_meter_10 = spawn_inputs_scale_nano_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_nano_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_nano_meter_100 = spawn_inputs_scale_nano_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_nano_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_micro_meter_1 = spawn_inputs_scale_micro_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_micro_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_micro_meter_10 = spawn_inputs_scale_micro_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_micro_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_micro_meter_100 = spawn_inputs_scale_micro_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_micro_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_milli_meter_1 = spawn_inputs_scale_milli_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_milli_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_milli_meter_10 = spawn_inputs_scale_milli_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_milli_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_milli_meter_100 = spawn_inputs_scale_milli_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_milli_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_meter_1 = spawn_inputs_scale_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_meter_10 = spawn_inputs_scale_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_meter_100 = spawn_inputs_scale_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_kilo_meter_1 = spawn_inputs_scale_kilo_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_kilo_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_kilo_meter_10 = spawn_inputs_scale_kilo_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_kilo_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_kilo_meter_100 = spawn_inputs_scale_kilo_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_kilo_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_mega_meter_1 = spawn_inputs_scale_mega_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_mega_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_mega_meter_10 = spawn_inputs_scale_mega_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_mega_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_mega_meter_100 = spawn_inputs_scale_mega_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_mega_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_giga_meter_1 = spawn_inputs_scale_giga_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_giga_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_giga_meter_10 = spawn_inputs_scale_giga_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_giga_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_giga_meter_100 = spawn_inputs_scale_giga_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_giga_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_tera_meter_1 = spawn_inputs_scale_tera_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_tera_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_tera_meter_10 = spawn_inputs_scale_tera_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_tera_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_tera_meter_100 = spawn_inputs_scale_tera_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_tera_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_peta_meter_1 = spawn_inputs_scale_peta_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_peta_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_peta_meter_10 = spawn_inputs_scale_peta_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_peta_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_peta_meter_100 = spawn_inputs_scale_peta_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_peta_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_exa_meter_1 = spawn_inputs_scale_exa_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_exa_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_exa_meter_10 = spawn_inputs_scale_exa_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_exa_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_exa_meter_100 = spawn_inputs_scale_exa_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_exa_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_zetta_meter_1 = spawn_inputs_scale_zetta_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_zetta_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_zetta_meter_10 = spawn_inputs_scale_zetta_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_zetta_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_zetta_meter_100 = spawn_inputs_scale_zetta_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_zetta_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_yotta_meter_1 = spawn_inputs_scale_yotta_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_yotta_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_yotta_meter_10 = spawn_inputs_scale_yotta_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_yotta_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_yotta_meter_100 = spawn_inputs_scale_yotta_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_yotta_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_ronna_meter_1 = spawn_inputs_scale_ronna_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_ronna_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_ronna_meter_10 = spawn_inputs_scale_ronna_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_ronna_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_ronna_meter_100 = spawn_inputs_scale_ronna_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_ronna_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_quetta_meter_1 = spawn_inputs_scale_quetta_meter_1
                .into_iter()
                .zip(generate_output.render_executor_scale_quetta_meter_1.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_quetta_meter_10 = spawn_inputs_scale_quetta_meter_10
                .into_iter()
                .zip(generate_output.render_executor_scale_quetta_meter_10.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_quetta_meter_100 = spawn_inputs_scale_quetta_meter_100
                .into_iter()
                .zip(generate_output.render_executor_scale_quetta_meter_100.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_quetta_meter_1000 = spawn_inputs_scale_quetta_meter_1000
                .into_iter()
                .zip(generate_output.render_executor_scale_quetta_meter_1000.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_quetta_meter_10000 = spawn_inputs_scale_quetta_meter_10000
                .into_iter()
                .zip(generate_output.render_executor_scale_quetta_meter_10000.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();
            let spawn_inputs_with_textures_scale_quetta_meter_100000 = spawn_inputs_scale_quetta_meter_100000
                .into_iter()
                .zip(generate_output.render_executor_scale_quetta_meter_100000.texture_handles.clone().into_iter())
                .map(|(mut input, tex)| {
                    input.metric_texture = tex; // Placeholder handle replaced
                    input
                })
                .collect::<Vec<_>>();

            let _ = workflow!(IOE, Chunk::SpawnChunks, Input {
                inner_scale_quecto_meter_000001: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_quecto_meter_000001 },
                inner_scale_quecto_meter_00001: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_quecto_meter_00001 },
                inner_scale_quecto_meter_0001: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_quecto_meter_0001 },
                inner_scale_quecto_meter_001: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_quecto_meter_001 },
                inner_scale_quecto_meter_01: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_quecto_meter_01 },
                inner_scale_quecto_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_quecto_meter_1 },
                inner_scale_quecto_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_quecto_meter_10 },
                inner_scale_quecto_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_quecto_meter_100 },
                inner_scale_ronto_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_ronto_meter_1 },
                inner_scale_ronto_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_ronto_meter_10 },
                inner_scale_ronto_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_ronto_meter_100 },
                inner_scale_yocto_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_yocto_meter_1 },
                inner_scale_yocto_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_yocto_meter_10 },
                inner_scale_yocto_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_yocto_meter_100 },
                inner_scale_zepto_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_zepto_meter_1 },
                inner_scale_zepto_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_zepto_meter_10 },
                inner_scale_zepto_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_zepto_meter_100 },
                inner_scale_atto_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_atto_meter_1 },
                inner_scale_atto_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_atto_meter_10 },
                inner_scale_atto_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_atto_meter_100 },
                inner_scale_femto_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_femto_meter_1 },
                inner_scale_femto_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_femto_meter_10 },
                inner_scale_femto_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_femto_meter_100 },
                inner_scale_pico_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_pico_meter_1 },
                inner_scale_pico_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_pico_meter_10 },
                inner_scale_pico_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_pico_meter_100 },
                inner_scale_nano_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_nano_meter_1 },
                inner_scale_nano_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_nano_meter_10 },
                inner_scale_nano_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_nano_meter_100 },
                inner_scale_micro_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_micro_meter_1 },
                inner_scale_micro_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_micro_meter_10 },
                inner_scale_micro_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_micro_meter_100 },
                inner_scale_milli_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_milli_meter_1 },
                inner_scale_milli_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_milli_meter_10 },
                inner_scale_milli_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_milli_meter_100 },
                inner_scale_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_meter_1 },
                inner_scale_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_meter_10 },
                inner_scale_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_meter_100 },
                inner_scale_kilo_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_kilo_meter_1 },
                inner_scale_kilo_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_kilo_meter_10 },
                inner_scale_kilo_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_kilo_meter_100 },
                inner_scale_mega_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_mega_meter_1 },
                inner_scale_mega_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_mega_meter_10 },
                inner_scale_mega_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_mega_meter_100 },
                inner_scale_giga_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_giga_meter_1 },
                inner_scale_giga_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_giga_meter_10 },
                inner_scale_giga_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_giga_meter_100 },
                inner_scale_tera_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_tera_meter_1 },
                inner_scale_tera_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_tera_meter_10 },
                inner_scale_tera_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_tera_meter_100 },
                inner_scale_peta_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_peta_meter_1 },
                inner_scale_peta_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_peta_meter_10 },
                inner_scale_peta_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_peta_meter_100 },
                inner_scale_exa_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_exa_meter_1 },
                inner_scale_exa_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_exa_meter_10 },
                inner_scale_exa_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_exa_meter_100 },
                inner_scale_zetta_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_zetta_meter_1 },
                inner_scale_zetta_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_zetta_meter_10 },
                inner_scale_zetta_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_zetta_meter_100 },
                inner_scale_yotta_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_yotta_meter_1 },
                inner_scale_yotta_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_yotta_meter_10 },
                inner_scale_yotta_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_yotta_meter_100 },
                inner_scale_ronna_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_ronna_meter_1 },
                inner_scale_ronna_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_ronna_meter_10 },
                inner_scale_ronna_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_ronna_meter_100 },
                inner_scale_quetta_meter_1: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_quetta_meter_1 },
                inner_scale_quetta_meter_10: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_quetta_meter_10 },
                inner_scale_quetta_meter_100: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_quetta_meter_100 },
                inner_scale_quetta_meter_1000: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_quetta_meter_1000 },
                inner_scale_quetta_meter_10000: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_quetta_meter_10000 },
                inner_scale_quetta_meter_100000: crate::chunk::workflows::external::spawn_chunks::Input { inputs: spawn_inputs_with_textures_scale_quetta_meter_100000 },
            });
        }))
    } else {
        None
    };

    let despawn_handle = if !despawn_inputs_scale_quetta_meter_100000.is_empty()
        || !despawn_inputs_scale_quetta_meter_10000.is_empty()
        || !despawn_inputs_scale_quetta_meter_1000.is_empty()
        || !despawn_inputs_scale_quetta_meter_100.is_empty()
        || !despawn_inputs_scale_quetta_meter_10.is_empty()
        || !despawn_inputs_scale_quetta_meter_1.is_empty()
        || !despawn_inputs_scale_ronna_meter_100.is_empty()
        || !despawn_inputs_scale_ronna_meter_10.is_empty()
        || !despawn_inputs_scale_ronna_meter_1.is_empty()
        || !despawn_inputs_scale_yotta_meter_100.is_empty()
        || !despawn_inputs_scale_yotta_meter_10.is_empty()
        || !despawn_inputs_scale_yotta_meter_1.is_empty()
        || !despawn_inputs_scale_zetta_meter_100.is_empty()
        || !despawn_inputs_scale_zetta_meter_10.is_empty()
        || !despawn_inputs_scale_zetta_meter_1.is_empty()
        || !despawn_inputs_scale_exa_meter_100.is_empty()
        || !despawn_inputs_scale_exa_meter_10.is_empty()
        || !despawn_inputs_scale_exa_meter_1.is_empty()
        || !despawn_inputs_scale_peta_meter_100.is_empty()
        || !despawn_inputs_scale_peta_meter_10.is_empty()
        || !despawn_inputs_scale_peta_meter_1.is_empty()
        || !despawn_inputs_scale_tera_meter_100.is_empty()
        || !despawn_inputs_scale_tera_meter_10.is_empty()
        || !despawn_inputs_scale_tera_meter_1.is_empty()
        || !despawn_inputs_scale_giga_meter_100.is_empty()
        || !despawn_inputs_scale_giga_meter_10.is_empty()
        || !despawn_inputs_scale_giga_meter_1.is_empty()
        || !despawn_inputs_scale_mega_meter_100.is_empty()
        || !despawn_inputs_scale_mega_meter_10.is_empty()
        || !despawn_inputs_scale_mega_meter_1.is_empty()
        || !despawn_inputs_scale_kilo_meter_100.is_empty()
        || !despawn_inputs_scale_kilo_meter_10.is_empty()
        || !despawn_inputs_scale_kilo_meter_1.is_empty()
        || !despawn_inputs_scale_meter_100.is_empty()
        || !despawn_inputs_scale_meter_10.is_empty()
        || !despawn_inputs_scale_meter_1.is_empty()
        || !despawn_inputs_scale_milli_meter_100.is_empty()
        || !despawn_inputs_scale_milli_meter_10.is_empty()
        || !despawn_inputs_scale_milli_meter_1.is_empty()
        || !despawn_inputs_scale_micro_meter_100.is_empty()
        || !despawn_inputs_scale_micro_meter_10.is_empty()
        || !despawn_inputs_scale_micro_meter_1.is_empty()
        || !despawn_inputs_scale_nano_meter_100.is_empty()
        || !despawn_inputs_scale_nano_meter_10.is_empty()
        || !despawn_inputs_scale_nano_meter_1.is_empty()
        || !despawn_inputs_scale_pico_meter_100.is_empty()
        || !despawn_inputs_scale_pico_meter_10.is_empty()
        || !despawn_inputs_scale_pico_meter_1.is_empty()
        || !despawn_inputs_scale_femto_meter_100.is_empty()
        || !despawn_inputs_scale_femto_meter_10.is_empty()
        || !despawn_inputs_scale_femto_meter_1.is_empty()
        || !despawn_inputs_scale_atto_meter_100.is_empty()
        || !despawn_inputs_scale_atto_meter_10.is_empty()
        || !despawn_inputs_scale_atto_meter_1.is_empty()
        || !despawn_inputs_scale_zepto_meter_100.is_empty()
        || !despawn_inputs_scale_zepto_meter_10.is_empty()
        || !despawn_inputs_scale_zepto_meter_1.is_empty()
        || !despawn_inputs_scale_yocto_meter_100.is_empty()
        || !despawn_inputs_scale_yocto_meter_10.is_empty()
        || !despawn_inputs_scale_yocto_meter_1.is_empty()
        || !despawn_inputs_scale_ronto_meter_100.is_empty()
        || !despawn_inputs_scale_ronto_meter_10.is_empty()
        || !despawn_inputs_scale_ronto_meter_1.is_empty()
        || !despawn_inputs_scale_quecto_meter_100.is_empty()
        || !despawn_inputs_scale_quecto_meter_10.is_empty()
        || !despawn_inputs_scale_quecto_meter_1.is_empty()
        || !despawn_inputs_scale_quecto_meter_01.is_empty()
        || !despawn_inputs_scale_quecto_meter_001.is_empty()
        || !despawn_inputs_scale_quecto_meter_0001.is_empty()
        || !despawn_inputs_scale_quecto_meter_00001.is_empty()
        || !despawn_inputs_scale_quecto_meter_000001.is_empty() 
    {
        Some(composite_workflow!(
            DespawnChunks,
            move in despawn_inputs_scale_quecto_meter_000001: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_quecto_meter_00001: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_quecto_meter_0001: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_quecto_meter_001: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_quecto_meter_01: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_quecto_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_quecto_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_quecto_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_ronto_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_ronto_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_ronto_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_yocto_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_yocto_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_yocto_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_zepto_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_zepto_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_zepto_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_atto_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_atto_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_atto_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_femto_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_femto_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_femto_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_pico_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_pico_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_pico_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_nano_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_nano_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_nano_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_micro_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_micro_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_micro_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_milli_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_milli_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_milli_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_kilo_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_kilo_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_kilo_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_mega_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_mega_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_mega_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_giga_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_giga_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_giga_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_tera_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_tera_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_tera_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_peta_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_peta_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_peta_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_exa_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_exa_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_exa_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_zetta_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_zetta_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_zetta_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_yotta_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_yotta_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_yotta_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_ronna_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_ronna_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_ronna_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_quetta_meter_1: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_quetta_meter_10: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_quetta_meter_100: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_quetta_meter_1000: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_quetta_meter_10000: Vec<DespawnChunkInput>,
            move in despawn_inputs_scale_quetta_meter_100000: Vec<DespawnChunkInput>,
        {
            warn!("Running composite workflow 'DespawnChunks'");

            let _ = workflow!(IOE, Chunk::DespawnChunks, Input {
                inner_scale_quecto_meter_000001: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_quecto_meter_000001 },
                inner_scale_quecto_meter_00001: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_quecto_meter_00001 },
                inner_scale_quecto_meter_0001: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_quecto_meter_0001 },
                inner_scale_quecto_meter_001: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_quecto_meter_001 },
                inner_scale_quecto_meter_01: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_quecto_meter_01 },
                inner_scale_quecto_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_quecto_meter_1 },
                inner_scale_quecto_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_quecto_meter_10 },
                inner_scale_quecto_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_quecto_meter_100 },
                inner_scale_ronto_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_ronto_meter_1 },
                inner_scale_ronto_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_ronto_meter_10 },
                inner_scale_ronto_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_ronto_meter_100 },
                inner_scale_yocto_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_yocto_meter_1 },
                inner_scale_yocto_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_yocto_meter_10 },
                inner_scale_yocto_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_yocto_meter_100 },
                inner_scale_zepto_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_zepto_meter_1 },
                inner_scale_zepto_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_zepto_meter_10 },
                inner_scale_zepto_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_zepto_meter_100 },
                inner_scale_atto_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_atto_meter_1 },
                inner_scale_atto_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_atto_meter_10 },
                inner_scale_atto_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_atto_meter_100 },
                inner_scale_femto_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_femto_meter_1 },
                inner_scale_femto_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_femto_meter_10 },
                inner_scale_femto_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_femto_meter_100 },
                inner_scale_pico_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_pico_meter_1 },
                inner_scale_pico_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_pico_meter_10 },
                inner_scale_pico_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_pico_meter_100 },
                inner_scale_nano_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_nano_meter_1 },
                inner_scale_nano_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_nano_meter_10 },
                inner_scale_nano_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_nano_meter_100 },
                inner_scale_micro_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_micro_meter_1 },
                inner_scale_micro_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_micro_meter_10 },
                inner_scale_micro_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_micro_meter_100 },
                inner_scale_milli_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_milli_meter_1 },
                inner_scale_milli_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_milli_meter_10 },
                inner_scale_milli_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_milli_meter_100 },
                inner_scale_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_meter_1 },
                inner_scale_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_meter_10 },
                inner_scale_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_meter_100 },
                inner_scale_kilo_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_kilo_meter_1 },
                inner_scale_kilo_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_kilo_meter_10 },
                inner_scale_kilo_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_kilo_meter_100 },
                inner_scale_mega_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_mega_meter_1 },
                inner_scale_mega_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_mega_meter_10 },
                inner_scale_mega_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_mega_meter_100 },
                inner_scale_giga_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_giga_meter_1 },
                inner_scale_giga_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_giga_meter_10 },
                inner_scale_giga_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_giga_meter_100 },
                inner_scale_tera_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_tera_meter_1 },
                inner_scale_tera_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_tera_meter_10 },
                inner_scale_tera_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_tera_meter_100 },
                inner_scale_peta_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_peta_meter_1 },
                inner_scale_peta_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_peta_meter_10 },
                inner_scale_peta_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_peta_meter_100 },
                inner_scale_exa_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_exa_meter_1 },
                inner_scale_exa_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_exa_meter_10 },
                inner_scale_exa_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_exa_meter_100 },
                inner_scale_zetta_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_zetta_meter_1 },
                inner_scale_zetta_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_zetta_meter_10 },
                inner_scale_zetta_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_zetta_meter_100 },
                inner_scale_yotta_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_yotta_meter_1 },
                inner_scale_yotta_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_yotta_meter_10 },
                inner_scale_yotta_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_yotta_meter_100 },
                inner_scale_ronna_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_ronna_meter_1 },
                inner_scale_ronna_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_ronna_meter_10 },
                inner_scale_ronna_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_ronna_meter_100 },
                inner_scale_quetta_meter_1: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_quetta_meter_1 },
                inner_scale_quetta_meter_10: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_quetta_meter_10 },
                inner_scale_quetta_meter_100: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_quetta_meter_100 },
                inner_scale_quetta_meter_1000: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_quetta_meter_1000 },
                inner_scale_quetta_meter_10000: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_quetta_meter_10000 },
                inner_scale_quetta_meter_100000: crate::chunk::workflows::external::despawn_chunks::Input { inputs: despawn_inputs_scale_quetta_meter_100000 },
            });
        }))
    } else {
        None
    };

    let transfer_handle = if !transfer_inputs_scale_quetta_meter_10000.is_empty()
        || !transfer_inputs_scale_quetta_meter_10000.is_empty()
        || !transfer_inputs_scale_quetta_meter_1000.is_empty()
        || !transfer_inputs_scale_quetta_meter_100.is_empty()
        || !transfer_inputs_scale_quetta_meter_10.is_empty()
        || !transfer_inputs_scale_quetta_meter_1.is_empty()
        || !transfer_inputs_scale_ronna_meter_100.is_empty()
        || !transfer_inputs_scale_ronna_meter_10.is_empty()
        || !transfer_inputs_scale_ronna_meter_1.is_empty()
        || !transfer_inputs_scale_yotta_meter_100.is_empty()
        || !transfer_inputs_scale_yotta_meter_10.is_empty()
        || !transfer_inputs_scale_yotta_meter_1.is_empty()
        || !transfer_inputs_scale_zetta_meter_100.is_empty()
        || !transfer_inputs_scale_zetta_meter_10.is_empty()
        || !transfer_inputs_scale_zetta_meter_1.is_empty()
        || !transfer_inputs_scale_exa_meter_100.is_empty()
        || !transfer_inputs_scale_exa_meter_10.is_empty()
        || !transfer_inputs_scale_exa_meter_1.is_empty()
        || !transfer_inputs_scale_peta_meter_100.is_empty()
        || !transfer_inputs_scale_peta_meter_10.is_empty()
        || !transfer_inputs_scale_peta_meter_1.is_empty()
        || !transfer_inputs_scale_tera_meter_100.is_empty()
        || !transfer_inputs_scale_tera_meter_10.is_empty()
        || !transfer_inputs_scale_tera_meter_1.is_empty()
        || !transfer_inputs_scale_giga_meter_100.is_empty()
        || !transfer_inputs_scale_giga_meter_10.is_empty()
        || !transfer_inputs_scale_giga_meter_1.is_empty()
        || !transfer_inputs_scale_mega_meter_100.is_empty()
        || !transfer_inputs_scale_mega_meter_10.is_empty()
        || !transfer_inputs_scale_mega_meter_1.is_empty()
        || !transfer_inputs_scale_kilo_meter_100.is_empty()
        || !transfer_inputs_scale_kilo_meter_10.is_empty()
        || !transfer_inputs_scale_kilo_meter_1.is_empty()
        || !transfer_inputs_scale_meter_100.is_empty()
        || !transfer_inputs_scale_meter_10.is_empty()
        || !transfer_inputs_scale_meter_1.is_empty()
        || !transfer_inputs_scale_milli_meter_100.is_empty()
        || !transfer_inputs_scale_milli_meter_10.is_empty()
        || !transfer_inputs_scale_milli_meter_1.is_empty()
        || !transfer_inputs_scale_micro_meter_100.is_empty()
        || !transfer_inputs_scale_micro_meter_10.is_empty()
        || !transfer_inputs_scale_micro_meter_1.is_empty()
        || !transfer_inputs_scale_nano_meter_100.is_empty()
        || !transfer_inputs_scale_nano_meter_10.is_empty()
        || !transfer_inputs_scale_nano_meter_1.is_empty()
        || !transfer_inputs_scale_pico_meter_100.is_empty()
        || !transfer_inputs_scale_pico_meter_10.is_empty()
        || !transfer_inputs_scale_pico_meter_1.is_empty()
        || !transfer_inputs_scale_femto_meter_100.is_empty()
        || !transfer_inputs_scale_femto_meter_10.is_empty()
        || !transfer_inputs_scale_femto_meter_1.is_empty()
        || !transfer_inputs_scale_atto_meter_100.is_empty()
        || !transfer_inputs_scale_atto_meter_10.is_empty()
        || !transfer_inputs_scale_atto_meter_1.is_empty()
        || !transfer_inputs_scale_zepto_meter_100.is_empty()
        || !transfer_inputs_scale_zepto_meter_10.is_empty()
        || !transfer_inputs_scale_zepto_meter_1.is_empty()
        || !transfer_inputs_scale_yocto_meter_100.is_empty()
        || !transfer_inputs_scale_yocto_meter_10.is_empty()
        || !transfer_inputs_scale_yocto_meter_1.is_empty()
        || !transfer_inputs_scale_ronto_meter_100.is_empty()
        || !transfer_inputs_scale_ronto_meter_10.is_empty()
        || !transfer_inputs_scale_ronto_meter_1.is_empty()
        || !transfer_inputs_scale_quecto_meter_100.is_empty()
        || !transfer_inputs_scale_quecto_meter_10.is_empty()
        || !transfer_inputs_scale_quecto_meter_1.is_empty()
        || !transfer_inputs_scale_quecto_meter_01.is_empty()
        || !transfer_inputs_scale_quecto_meter_001.is_empty()
        || !transfer_inputs_scale_quecto_meter_0001.is_empty()
        || !transfer_inputs_scale_quecto_meter_00001.is_empty()
        || !transfer_inputs_scale_quecto_meter_000001.is_empty() 
    {
        Some(composite_workflow!(
            TransferChunkOwnerships,
            move in transfer_inputs_scale_quecto_meter_000001: Vec<TransferChunkOwnershipInput<ScaleQuectoMeter000001>>,
            move in transfer_inputs_scale_quecto_meter_00001: Vec<TransferChunkOwnershipInput<ScaleQuectoMeter00001>>,
            move in transfer_inputs_scale_quecto_meter_0001: Vec<TransferChunkOwnershipInput<ScaleQuectoMeter0001>>,
            move in transfer_inputs_scale_quecto_meter_001: Vec<TransferChunkOwnershipInput<ScaleQuectoMeter001>>,
            move in transfer_inputs_scale_quecto_meter_01: Vec<TransferChunkOwnershipInput<ScaleQuectoMeter01>>,
            move in transfer_inputs_scale_quecto_meter_1: Vec<TransferChunkOwnershipInput<ScaleQuectoMeter1>>,
            move in transfer_inputs_scale_quecto_meter_10: Vec<TransferChunkOwnershipInput<ScaleQuectoMeter10>>,
            move in transfer_inputs_scale_quecto_meter_100: Vec<TransferChunkOwnershipInput<ScaleQuectoMeter100>>,
            move in transfer_inputs_scale_ronto_meter_1: Vec<TransferChunkOwnershipInput<ScaleRontoMeter1>>,
            move in transfer_inputs_scale_ronto_meter_10: Vec<TransferChunkOwnershipInput<ScaleRontoMeter10>>,
            move in transfer_inputs_scale_ronto_meter_100: Vec<TransferChunkOwnershipInput<ScaleRontoMeter100>>,
            move in transfer_inputs_scale_yocto_meter_1: Vec<TransferChunkOwnershipInput<ScaleYoctoMeter1>>,
            move in transfer_inputs_scale_yocto_meter_10: Vec<TransferChunkOwnershipInput<ScaleYoctoMeter10>>,
            move in transfer_inputs_scale_yocto_meter_100: Vec<TransferChunkOwnershipInput<ScaleYoctoMeter100>>,
            move in transfer_inputs_scale_zepto_meter_1: Vec<TransferChunkOwnershipInput<ScaleZeptoMeter1>>,
            move in transfer_inputs_scale_zepto_meter_10: Vec<TransferChunkOwnershipInput<ScaleZeptoMeter10>>,
            move in transfer_inputs_scale_zepto_meter_100: Vec<TransferChunkOwnershipInput<ScaleZeptoMeter100>>,
            move in transfer_inputs_scale_atto_meter_1: Vec<TransferChunkOwnershipInput<ScaleAttoMeter1>>,
            move in transfer_inputs_scale_atto_meter_10: Vec<TransferChunkOwnershipInput<ScaleAttoMeter10>>,
            move in transfer_inputs_scale_atto_meter_100: Vec<TransferChunkOwnershipInput<ScaleAttoMeter100>>,
            move in transfer_inputs_scale_femto_meter_1: Vec<TransferChunkOwnershipInput<ScaleFemtoMeter1>>,
            move in transfer_inputs_scale_femto_meter_10: Vec<TransferChunkOwnershipInput<ScaleFemtoMeter10>>,
            move in transfer_inputs_scale_femto_meter_100: Vec<TransferChunkOwnershipInput<ScaleFemtoMeter100>>,
            move in transfer_inputs_scale_pico_meter_1: Vec<TransferChunkOwnershipInput<ScalePicoMeter1>>,
            move in transfer_inputs_scale_pico_meter_10: Vec<TransferChunkOwnershipInput<ScalePicoMeter10>>,
            move in transfer_inputs_scale_pico_meter_100: Vec<TransferChunkOwnershipInput<ScalePicoMeter100>>,
            move in transfer_inputs_scale_nano_meter_1: Vec<TransferChunkOwnershipInput<ScaleNanoMeter1>>,
            move in transfer_inputs_scale_nano_meter_10: Vec<TransferChunkOwnershipInput<ScaleNanoMeter10>>,
            move in transfer_inputs_scale_nano_meter_100: Vec<TransferChunkOwnershipInput<ScaleNanoMeter100>>,
            move in transfer_inputs_scale_micro_meter_1: Vec<TransferChunkOwnershipInput<ScaleMicroMeter1>>,
            move in transfer_inputs_scale_micro_meter_10: Vec<TransferChunkOwnershipInput<ScaleMicroMeter10>>,
            move in transfer_inputs_scale_micro_meter_100: Vec<TransferChunkOwnershipInput<ScaleMicroMeter100>>,
            move in transfer_inputs_scale_milli_meter_1: Vec<TransferChunkOwnershipInput<ScaleMilliMeter1>>,
            move in transfer_inputs_scale_milli_meter_10: Vec<TransferChunkOwnershipInput<ScaleMilliMeter10>>,
            move in transfer_inputs_scale_milli_meter_100: Vec<TransferChunkOwnershipInput<ScaleMilliMeter100>>,
            move in transfer_inputs_scale_meter_1: Vec<TransferChunkOwnershipInput<ScaleMeter1>>,
            move in transfer_inputs_scale_meter_10: Vec<TransferChunkOwnershipInput<ScaleMeter10>>,
            move in transfer_inputs_scale_meter_100: Vec<TransferChunkOwnershipInput<ScaleMeter100>>,
            move in transfer_inputs_scale_kilo_meter_1: Vec<TransferChunkOwnershipInput<ScaleKiloMeter1>>,
            move in transfer_inputs_scale_kilo_meter_10: Vec<TransferChunkOwnershipInput<ScaleKiloMeter10>>,
            move in transfer_inputs_scale_kilo_meter_100: Vec<TransferChunkOwnershipInput<ScaleKiloMeter100>>,
            move in transfer_inputs_scale_mega_meter_1: Vec<TransferChunkOwnershipInput<ScaleMegaMeter1>>,
            move in transfer_inputs_scale_mega_meter_10: Vec<TransferChunkOwnershipInput<ScaleMegaMeter10>>,
            move in transfer_inputs_scale_mega_meter_100: Vec<TransferChunkOwnershipInput<ScaleMegaMeter100>>,
            move in transfer_inputs_scale_giga_meter_1: Vec<TransferChunkOwnershipInput<ScaleGigaMeter1>>,
            move in transfer_inputs_scale_giga_meter_10: Vec<TransferChunkOwnershipInput<ScaleGigaMeter10>>,
            move in transfer_inputs_scale_giga_meter_100: Vec<TransferChunkOwnershipInput<ScaleGigaMeter100>>,
            move in transfer_inputs_scale_tera_meter_1: Vec<TransferChunkOwnershipInput<ScaleTeraMeter1>>,
            move in transfer_inputs_scale_tera_meter_10: Vec<TransferChunkOwnershipInput<ScaleTeraMeter10>>,
            move in transfer_inputs_scale_tera_meter_100: Vec<TransferChunkOwnershipInput<ScaleTeraMeter100>>,
            move in transfer_inputs_scale_peta_meter_1: Vec<TransferChunkOwnershipInput<ScalePetaMeter1>>,
            move in transfer_inputs_scale_peta_meter_10: Vec<TransferChunkOwnershipInput<ScalePetaMeter10>>,
            move in transfer_inputs_scale_peta_meter_100: Vec<TransferChunkOwnershipInput<ScalePetaMeter100>>,
            move in transfer_inputs_scale_exa_meter_1: Vec<TransferChunkOwnershipInput<ScaleExaMeter1>>,
            move in transfer_inputs_scale_exa_meter_10: Vec<TransferChunkOwnershipInput<ScaleExaMeter10>>,
            move in transfer_inputs_scale_exa_meter_100: Vec<TransferChunkOwnershipInput<ScaleExaMeter100>>,
            move in transfer_inputs_scale_zetta_meter_1: Vec<TransferChunkOwnershipInput<ScaleZettaMeter1>>,
            move in transfer_inputs_scale_zetta_meter_10: Vec<TransferChunkOwnershipInput<ScaleZettaMeter10>>,
            move in transfer_inputs_scale_zetta_meter_100: Vec<TransferChunkOwnershipInput<ScaleZettaMeter100>>,
            move in transfer_inputs_scale_yotta_meter_1: Vec<TransferChunkOwnershipInput<ScaleYottaMeter1>>,
            move in transfer_inputs_scale_yotta_meter_10: Vec<TransferChunkOwnershipInput<ScaleYottaMeter10>>,
            move in transfer_inputs_scale_yotta_meter_100: Vec<TransferChunkOwnershipInput<ScaleYottaMeter100>>,
            move in transfer_inputs_scale_ronna_meter_1: Vec<TransferChunkOwnershipInput<ScaleRonnaMeter1>>,
            move in transfer_inputs_scale_ronna_meter_10: Vec<TransferChunkOwnershipInput<ScaleRonnaMeter10>>,
            move in transfer_inputs_scale_ronna_meter_100: Vec<TransferChunkOwnershipInput<ScaleRonnaMeter100>>,
            move in transfer_inputs_scale_quetta_meter_1: Vec<TransferChunkOwnershipInput<ScaleQuettaMeter1>>,
            move in transfer_inputs_scale_quetta_meter_10: Vec<TransferChunkOwnershipInput<ScaleQuettaMeter10>>,
            move in transfer_inputs_scale_quetta_meter_100: Vec<TransferChunkOwnershipInput<ScaleQuettaMeter100>>,
            move in transfer_inputs_scale_quetta_meter_1000: Vec<TransferChunkOwnershipInput<ScaleQuettaMeter1000>>,
            move in transfer_inputs_scale_quetta_meter_10000: Vec<TransferChunkOwnershipInput<ScaleQuettaMeter10000>>,
            move in transfer_inputs_scale_quetta_meter_100000: Vec<TransferChunkOwnershipInput<ScaleQuettaMeter100000>>,
            new_chunk_loaders_scale_quecto_meter_000001: Vec<Entity>,
            new_chunk_loaders_scale_quecto_meter_00001: Vec<Entity>,
            new_chunk_loaders_scale_quecto_meter_0001: Vec<Entity>,
            new_chunk_loaders_scale_quecto_meter_001: Vec<Entity>,
            new_chunk_loaders_scale_quecto_meter_01: Vec<Entity>,
            new_chunk_loaders_scale_quecto_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_quecto_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_quecto_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_ronto_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_ronto_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_ronto_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_yocto_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_yocto_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_yocto_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_zepto_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_zepto_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_zepto_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_atto_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_atto_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_atto_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_femto_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_femto_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_femto_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_pico_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_pico_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_pico_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_nano_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_nano_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_nano_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_micro_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_micro_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_micro_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_milli_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_milli_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_milli_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_kilo_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_kilo_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_kilo_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_mega_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_mega_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_mega_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_giga_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_giga_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_giga_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_tera_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_tera_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_tera_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_peta_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_peta_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_peta_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_exa_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_exa_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_exa_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_zetta_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_zetta_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_zetta_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_yotta_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_yotta_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_yotta_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_ronna_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_ronna_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_ronna_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_quetta_meter_1: Vec<Entity>,
            new_chunk_loaders_scale_quetta_meter_10: Vec<Entity>,
            new_chunk_loaders_scale_quetta_meter_100: Vec<Entity>,
            new_chunk_loaders_scale_quetta_meter_1000: Vec<Entity>,
            new_chunk_loaders_scale_quetta_meter_10000: Vec<Entity>,
            new_chunk_loaders_scale_quetta_meter_100000: Vec<Entity>,
        {
            warn!("Running composite workflow 'TransferChunkOwnerships'");

            let _ = workflow!(IOE, Chunk::TransferChunkOwnerships, Input {
                inner_scale_quecto_meter_000001: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_quecto_meter_000001 },
                inner_scale_quecto_meter_00001: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_quecto_meter_00001 },
                inner_scale_quecto_meter_0001: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_quecto_meter_0001 },
                inner_scale_quecto_meter_001: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_quecto_meter_001 },
                inner_scale_quecto_meter_01: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_quecto_meter_01 },
                inner_scale_quecto_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_quecto_meter_1 },
                inner_scale_quecto_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_quecto_meter_10 },
                inner_scale_quecto_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_quecto_meter_100 },
                inner_scale_ronto_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_ronto_meter_1 },
                inner_scale_ronto_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_ronto_meter_10 },
                inner_scale_ronto_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_ronto_meter_100 },
                inner_scale_yocto_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_yocto_meter_1 },
                inner_scale_yocto_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_yocto_meter_10 },
                inner_scale_yocto_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_yocto_meter_100 },
                inner_scale_zepto_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_zepto_meter_1 },
                inner_scale_zepto_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_zepto_meter_10 },
                inner_scale_zepto_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_zepto_meter_100 },
                inner_scale_atto_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_atto_meter_1 },
                inner_scale_atto_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_atto_meter_10 },
                inner_scale_atto_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_atto_meter_100 },
                inner_scale_femto_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_femto_meter_1 },
                inner_scale_femto_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_femto_meter_10 },
                inner_scale_femto_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_femto_meter_100 },
                inner_scale_pico_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_pico_meter_1 },
                inner_scale_pico_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_pico_meter_10 },
                inner_scale_pico_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_pico_meter_100 },
                inner_scale_nano_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_nano_meter_1 },
                inner_scale_nano_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_nano_meter_10 },
                inner_scale_nano_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_nano_meter_100 },
                inner_scale_micro_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_micro_meter_1 },
                inner_scale_micro_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_micro_meter_10 },
                inner_scale_micro_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_micro_meter_100 },
                inner_scale_milli_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_milli_meter_1 },
                inner_scale_milli_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_milli_meter_10 },
                inner_scale_milli_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_milli_meter_100 },
                inner_scale_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_meter_1 },
                inner_scale_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_meter_10 },
                inner_scale_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_meter_100 },
                inner_scale_kilo_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_kilo_meter_1 },
                inner_scale_kilo_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_kilo_meter_10 },
                inner_scale_kilo_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_kilo_meter_100 },
                inner_scale_mega_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_mega_meter_1 },
                inner_scale_mega_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_mega_meter_10 },
                inner_scale_mega_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_mega_meter_100 },
                inner_scale_giga_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_giga_meter_1 },
                inner_scale_giga_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_giga_meter_10 },
                inner_scale_giga_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_giga_meter_100 },
                inner_scale_tera_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_tera_meter_1 },
                inner_scale_tera_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_tera_meter_10 },
                inner_scale_tera_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_tera_meter_100 },
                inner_scale_peta_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_peta_meter_1 },
                inner_scale_peta_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_peta_meter_10 },
                inner_scale_peta_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_peta_meter_100 },
                inner_scale_exa_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_exa_meter_1 },
                inner_scale_exa_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_exa_meter_10 },
                inner_scale_exa_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_exa_meter_100 },
                inner_scale_zetta_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_zetta_meter_1 },
                inner_scale_zetta_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_zetta_meter_10 },
                inner_scale_zetta_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_zetta_meter_100 },
                inner_scale_yotta_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_yotta_meter_1 },
                inner_scale_yotta_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_yotta_meter_10 },
                inner_scale_yotta_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_yotta_meter_100 },
                inner_scale_ronna_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_ronna_meter_1 },
                inner_scale_ronna_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_ronna_meter_10 },
                inner_scale_ronna_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_ronna_meter_100 },
                inner_scale_quetta_meter_1: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_quetta_meter_1 },
                inner_scale_quetta_meter_10: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_quetta_meter_10 },
                inner_scale_quetta_meter_100: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_quetta_meter_100 },
                inner_scale_quetta_meter_1000: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_quetta_meter_1000 },
                inner_scale_quetta_meter_10000: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_quetta_meter_10000 },
                inner_scale_quetta_meter_100000: crate::chunk::workflows::external::transfer_chunk_ownerships::Input { inputs: transfer_inputs_scale_quetta_meter_100000 },
            });
        }))
    } else {
        None
    };

    *workflow_handles = Some(ChunkActionWorkflowHandles {
        spawn: spawn_handle,
        despawn: despawn_handle,
        transfer: transfer_handle,
    });

    // Step 4: Mark all these actions as in-progress (remove them from the commit buffer)
    action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_000001.remove_intents(processed_coords_scale_quecto_meter_000001);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_00001.remove_intents(processed_coords_scale_quecto_meter_00001);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_0001.remove_intents(processed_coords_scale_quecto_meter_0001);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_001.remove_intents(processed_coords_scale_quecto_meter_001);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_01.remove_intents(processed_coords_scale_quecto_meter_01);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_1.remove_intents(processed_coords_scale_quecto_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_10.remove_intents(processed_coords_scale_quecto_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_quecto_meter_100.remove_intents(processed_coords_scale_quecto_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_ronto_meter_1.remove_intents(processed_coords_scale_ronto_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_ronto_meter_10.remove_intents(processed_coords_scale_ronto_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_ronto_meter_100.remove_intents(processed_coords_scale_ronto_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_yocto_meter_1.remove_intents(processed_coords_scale_yocto_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_yocto_meter_10.remove_intents(processed_coords_scale_yocto_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_yocto_meter_100.remove_intents(processed_coords_scale_yocto_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_zepto_meter_1.remove_intents(processed_coords_scale_zepto_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_zepto_meter_10.remove_intents(processed_coords_scale_zepto_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_zepto_meter_100.remove_intents(processed_coords_scale_zepto_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_atto_meter_1.remove_intents(processed_coords_scale_atto_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_atto_meter_10.remove_intents(processed_coords_scale_atto_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_atto_meter_100.remove_intents(processed_coords_scale_atto_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_femto_meter_1.remove_intents(processed_coords_scale_femto_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_femto_meter_10.remove_intents(processed_coords_scale_femto_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_femto_meter_100.remove_intents(processed_coords_scale_femto_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_pico_meter_1.remove_intents(processed_coords_scale_pico_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_pico_meter_10.remove_intents(processed_coords_scale_pico_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_pico_meter_100.remove_intents(processed_coords_scale_pico_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_nano_meter_1.remove_intents(processed_coords_scale_nano_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_nano_meter_10.remove_intents(processed_coords_scale_nano_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_nano_meter_100.remove_intents(processed_coords_scale_nano_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_micro_meter_1.remove_intents(processed_coords_scale_micro_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_micro_meter_10.remove_intents(processed_coords_scale_micro_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_micro_meter_100.remove_intents(processed_coords_scale_micro_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_milli_meter_1.remove_intents(processed_coords_scale_milli_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_milli_meter_10.remove_intents(processed_coords_scale_milli_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_milli_meter_100.remove_intents(processed_coords_scale_milli_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_meter_1.remove_intents(processed_coords_scale_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_meter_10.remove_intents(processed_coords_scale_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_meter_100.remove_intents(processed_coords_scale_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_kilo_meter_1.remove_intents(processed_coords_scale_kilo_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_kilo_meter_10.remove_intents(processed_coords_scale_kilo_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_kilo_meter_100.remove_intents(processed_coords_scale_kilo_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_mega_meter_1.remove_intents(processed_coords_scale_mega_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_mega_meter_10.remove_intents(processed_coords_scale_mega_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_mega_meter_100.remove_intents(processed_coords_scale_mega_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_giga_meter_1.remove_intents(processed_coords_scale_giga_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_giga_meter_10.remove_intents(processed_coords_scale_giga_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_giga_meter_100.remove_intents(processed_coords_scale_giga_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_tera_meter_1.remove_intents(processed_coords_scale_tera_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_tera_meter_10.remove_intents(processed_coords_scale_tera_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_tera_meter_100.remove_intents(processed_coords_scale_tera_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_peta_meter_1.remove_intents(processed_coords_scale_peta_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_peta_meter_10.remove_intents(processed_coords_scale_peta_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_peta_meter_100.remove_intents(processed_coords_scale_peta_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_exa_meter_1.remove_intents(processed_coords_scale_exa_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_exa_meter_10.remove_intents(processed_coords_scale_exa_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_exa_meter_100.remove_intents(processed_coords_scale_exa_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_zetta_meter_1.remove_intents(processed_coords_scale_zetta_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_zetta_meter_10.remove_intents(processed_coords_scale_zetta_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_zetta_meter_100.remove_intents(processed_coords_scale_zetta_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_yotta_meter_1.remove_intents(processed_coords_scale_yotta_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_yotta_meter_10.remove_intents(processed_coords_scale_yotta_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_yotta_meter_100.remove_intents(processed_coords_scale_yotta_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_ronna_meter_1.remove_intents(processed_coords_scale_ronna_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_ronna_meter_10.remove_intents(processed_coords_scale_ronna_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_ronna_meter_100.remove_intents(processed_coords_scale_ronna_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_1.remove_intents(processed_coords_scale_quetta_meter_1);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_10.remove_intents(processed_coords_scale_quetta_meter_10);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_100.remove_intents(processed_coords_scale_quetta_meter_100);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_1000.remove_intents(processed_coords_scale_quetta_meter_1000);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_10000.remove_intents(processed_coords_scale_quetta_meter_10000);
    action_intent_commit_buffers.action_intent_commit_buffer_scale_quetta_meter_100000.remove_intents(processed_coords_scale_quetta_meter_100000);
}
