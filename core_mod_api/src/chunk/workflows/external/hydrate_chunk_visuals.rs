use crate::bevy::prelude::*;
use crate::chunk::components::Chunk;
use crate::chunk::demo::{
    ChunkDemoHydrationArtifact,
    ChunkDemoHydrationTask,
    UsfDemoChunkStore,
    UsfDemoChunkVisual,
    UsfDemoSettings,
    apply_chunk_demo_hydration_artifact,
    prepare_chunk_demo_hydration_artifact,
};

#[derive(Clone)]
pub struct AsyncInput {
    pub settings: UsfDemoSettings,
    pub tasks: Vec<ChunkDemoHydrationTask>,
}

pub struct ArtifactsOutput {
    pub artifacts: Vec<ChunkDemoHydrationArtifact>,
}

pub struct CommitOutput {
    pub hydrated: usize,
    pub skipped: usize,
}

pub struct Input {
    pub inner: AsyncInput,
}

#[derive(Debug)]
pub enum Error {}

#[derive(crate::bevy::ecs::system::SystemParam)]
pub struct MainAccess<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub chunk_query: Query<'w, 's, Option<&'static UsfDemoChunkVisual>, With<Chunk>>,
    pub chunk_store: ResMut<'w, UsfDemoChunkStore>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
}

pub fn run_async(input: Input) -> Result<ArtifactsOutput, Error> {
    let artifacts = input
        .inner
        .tasks
        .into_iter()
        .map(|task| prepare_chunk_demo_hydration_artifact(&input.inner.settings, task))
        .collect::<Vec<_>>();

    Ok(ArtifactsOutput { artifacts })
}

pub fn run_ecs(input: ArtifactsOutput, main_access: MainAccess) -> Result<CommitOutput, Error> {
    let MainAccess {
        mut commands,
        chunk_query,
        mut chunk_store,
        mut meshes,
        mut materials,
    } = main_access;

    let mut hydrated = 0usize;
    let mut skipped = 0usize;

    for artifact in input.artifacts {
        let Ok(maybe_visual) = chunk_query.get(artifact.chunk_entity) else {
            skipped += 1;
            continue;
        };

        if maybe_visual.is_some() || commands.get_entity(artifact.chunk_entity).is_err() {
            skipped += 1;
            continue;
        }

        apply_chunk_demo_hydration_artifact(artifact, &mut commands, &mut chunk_store, &mut meshes, &mut materials);
        hydrated += 1;
    }

    Ok(CommitOutput { hydrated, skipped })
}
