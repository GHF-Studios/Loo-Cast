crate::workflow_stage_util!("ValidateAndSpawn");

pub mod core_types {
    crate::workflow_stage_core_types_util!();

    #[derive(SystemParam)]
    pub struct MainAccess<'w, 's> {
        commands: Commands<'w, 's>,
        chunk_query: Query<'w, 's, &'static ChunkComponent>,
        chunk_manager: ResMut<'w, ChunkManager>,
    }
    pub struct Input {
        chunk_coord: (i32, i32),
        chunk_owner: Option<Entity>,
        metric_texture: Handle<Image>
    }
    pub enum Error {
        ChunkAlreadyLoaded { chunk_coord: (i32, i32) },
    }
}

pub mod core_functions {
    crate::workflow_stage_core_functions_util!();
    crate::workflow_stage_core_function_util!(run_ecs);

    pub fn run_ecs_inner(input: Input, main_access: MainAccess) -> Result<(), Error> {
        let chunk_coord = input.chunk_coord;
        let chunk_owner = input.chunk_owner;
        let metric_texture = input.metric_texture.clone();

        let mut commands = main_access.commands;
        let chunk_query = main_access.chunk_query;
        let mut chunk_manager = main_access.chunk_manager;

        if chunk_query.iter().any(|chunk| chunk.coord == chunk_coord) {
            return Err(Error::ChunkAlreadyLoaded { chunk_coord });
        }

        let default_chunk_z = CONFIG.get::<f32>("chunk/default_z");
        let chunk_transform = Transform {
            translation: chunk_pos_to_world(chunk_coord).extend(default_chunk_z),
            ..Default::default()
        };

        commands.spawn((
            SpriteBundle {
                texture: metric_texture,
                transform: chunk_transform,
                ..Default::default()
            },
            ChunkComponent {
                coord: chunk_coord,
                owner: chunk_owner,
            },
        ));

        chunk_manager.loaded_chunks.insert(chunk_coord);
        if let Some(owner) = chunk_owner {
            chunk_manager.owned_chunks.insert(chunk_coord, owner);
        }

        Ok(())
    }
}
