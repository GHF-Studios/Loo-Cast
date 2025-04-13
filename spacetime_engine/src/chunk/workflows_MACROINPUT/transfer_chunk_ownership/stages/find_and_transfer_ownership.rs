crate::workflow_stage_util!("FindAndTransferOwnership");

pub mod core_types {
    crate::workflow_stage_core_types_util!();

    #[derive(SystemParam)]
    pub struct MainAccess<'w, 's> {
        chunk_query: Query<'w, 's, &'static mut ChunkComponent>,
        chunk_manager: ResMut<'w, ChunkManager>,
    }
    pub struct Input {
        chunk_coord: (i32, i32),
        new_owner: Entity
    }
    pub enum Error {
        ChunkNotLoaded { chunk_coord: (i32, i32) },
    }
}

pub mod core_functions {
    crate::workflow_stage_core_functions_util!();
    crate::workflow_stage_core_function_util!(run_ecs);

    pub fn run_ecs_inner(input: Input, main_access: MainAccess) -> Result<(), Error> {
        let chunk_coord = input.chunk_coord;
        let new_owner = input.new_owner;

        let mut chunk_query = main_access.chunk_query;
        let mut chunk_manager = main_access.chunk_manager;

        if let Some(mut chunk) = chunk_query.iter_mut().find(|chunk| chunk.coord == chunk_coord) {
            if chunk.owner.is_some() {
                chunk_manager.owned_chunks.remove(&chunk_coord);
            }

            chunk.owner = Some(new_owner);
            chunk_manager.owned_chunks.insert(chunk_coord, new_owner);

            Ok(())
        } else {
            Err(Error::ChunkNotLoaded { chunk_coord })
        }
    }
}
