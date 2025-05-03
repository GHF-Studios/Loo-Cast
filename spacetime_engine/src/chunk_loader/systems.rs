use bevy::prelude::*;
use tokio::task::JoinHandle;
use spacetime_engine_macros::{composite_workflow, define_composite_workflow_inner};
use std::collections::HashSet;

use crate::chunk::enums::ChunkAction;
use crate::chunk::functions::calculate_chunks_in_radius;
use crate::chunk_loader::components::ChunkLoaderComponent;
use crate::chunk::resources::ChunkActionBuffer;

pub(crate) fn update_chunk_loader_system(
    mut composite_workflow_handle: Local<Option<JoinHandle<()>>>,
    chunk_loader_query: Query<(Entity, &Transform, &ChunkLoaderComponent)>,
    mut chunk_action_buffer: ResMut<ChunkActionBuffer>,
) {
    for (_, transform, chunk_loader) in chunk_loader_query.iter() {
        let position = transform.translation.truncate();
        let radius = chunk_loader.radius;
        let loader_range = calculate_chunks_in_radius(position, radius)
            .into_iter()
            .collect::<HashSet<(i32, i32)>>();

        let mut invalid_actions = vec![];
        for (chunk_coord, action) in chunk_action_buffer.iter() {
            match action {
                ChunkAction::Spawn { .. } => {
                    if !loader_range.contains(chunk_coord) {
                        invalid_actions.push(*chunk_coord);
                    }
                }
                ChunkAction::Despawn { .. } => {
                    if loader_range.contains(chunk_coord) {
                        invalid_actions.push(*chunk_coord);
                    }
                }
                ChunkAction::TransferOwnership { .. } => {}
            }
        }

        let mut invalid_chunk_actions = Vec::new();
        for chunk_coord in invalid_actions {
            chunk_action_buffer.remove_action(&chunk_coord);
            invalid_chunk_actions.push((chunk_coord, chunk_loader.id));
        }
    }

    let sex1 = 17;
    let sex2 = 35;
    let sex3 = 83;
    let sex4 = 74;

    composite_workflow!(sex1: i32, sex2: i32, sex3: i32, sex4: i32, JustDoIt {
        let categorize_chunks_output = workflow!(O, ChunkLoader::CategorizeChunks);
        workflow!(I, ChunkLoader::LoadChunks, Input {
            inputs: categorize_chunks_output.load_chunk_inputs
        });
        workflow!(I, ChunkLoader::UnloadChunks, Input {
            inputs: categorize_chunks_output.unload_chunk_inputs
        });

        println!("sex1: {:?}", sex1);
        println!("sex2: {:?}", sex2);
        println!("sex3: {:?}", sex3);
        println!("sex3: {:?}", sex4);
    });


    println!("sex1: {:?}", sex1);
    println!("sex2: {:?}", sex2);
    println!("sex3: {:?}", sex3);
    println!("sex3: {:?}", sex4);

    //define_composite_workflow_inner!(JustDoIt {
    //    let categorize_chunks_output = workflow!(O, ChunkLoader::CategorizeChunks);
    //    workflow!(I, ChunkLoader::LoadChunks, Input {
    //        inputs: categorize_chunks_output.load_chunk_inputs
    //    });
    //    workflow!(I, ChunkLoader::UnloadChunks, Input {
    //        inputs: categorize_chunks_output.unload_chunk_inputs
    //    });
    //});
//
    //match *composite_workflow_handle {
    //    Some(ref handle) if handle.is_finished() => {
    //        *composite_workflow_handle = None;
    //    },
    //    Some(_) => todo!(),
    //    None => {
    //        *composite_workflow_handle = Some(crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME
    //            .lock()
    //            .unwrap()
    //            .spawn(Box::pin(just_do_it())));
    //    }
    //}
}
