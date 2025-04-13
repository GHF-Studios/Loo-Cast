crate::workflow_stage_util!("FindAndDespawn");

pub mod core_types {
    crate::workflow_stage_core_types_util!();

    #[derive(SystemParam)]
    pub struct MainAccess<'w, 's> {
        commands: Commands<'w, 's>,
        chunk_query: Query<'w, 's, (Entity, &'static ChunkComponent)>,
        chunk_manager: ResMut<'w, ChunkManager>,
    }
    pub struct Input {
        chunk_coord: (i32, i32)
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
        
        let mut commands = main_access.commands;
        let chunk_query = main_access.chunk_query;
        let mut chunk_manager = main_access.chunk_manager;

        if let Some((entity, _)) = chunk_query.iter().find(|(_, chunk)| chunk.coord == chunk_coord) {
            chunk_manager.loaded_chunks.remove(&chunk_coord);
            chunk_manager.owned_chunks.remove(&chunk_coord);

            commands.entity(entity).despawn_recursive();

            Ok(())
        } else {
            Err(Error::ChunkNotLoaded { chunk_coord })
        }
    }
}
