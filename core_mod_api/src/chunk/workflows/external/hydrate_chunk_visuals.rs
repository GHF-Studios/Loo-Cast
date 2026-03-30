use crate::bevy::prelude::*;
use crate::chunk::components::Chunk;
use crate::usf::mod_runtime::chunk_surface::{
    ChunkDemoHydrationArtifact, ChunkDemoHydrationTask, UsfDemoChunkStore, UsfDemoChunkVisual, UsfDemoSettings, apply_chunk_demo_hydration_artifact,
    prepare_chunk_demo_hydration_artifact,
};
use crate::workflow::types::Outcome;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AsyncInput {
    pub settings: UsfDemoSettings,
    pub tasks: Vec<ChunkDemoHydrationTask>,
    pub build_workers: usize,
    pub commit_budget: usize,
}

pub struct ArtifactsOutput {
    pub artifacts: VecDeque<ChunkDemoHydrationArtifact>,
    pub commit_budget: usize,
}

pub struct State {
    pub artifacts: VecDeque<ChunkDemoHydrationArtifact>,
    pub commit_budget: usize,
    pub hydrated: usize,
    pub skipped: usize,
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
    let Input { inner } = input;
    let commit_budget = inner.commit_budget.max(1);
    let artifacts = build_artifacts_parallel(inner.settings, inner.tasks, inner.build_workers.max(1));

    Ok(ArtifactsOutput {
        artifacts: artifacts.into_iter().collect(),
        commit_budget,
    })
}

pub fn setup_ecs_while(input: ArtifactsOutput, _main_access: MainAccess) -> Result<State, Error> {
    Ok(State {
        artifacts: input.artifacts,
        commit_budget: input.commit_budget.max(1),
        hydrated: 0,
        skipped: 0,
    })
}

pub fn run_ecs_while(state: State, main_access: MainAccess) -> Result<Outcome<State, CommitOutput>, Error> {
    let MainAccess {
        mut commands,
        chunk_query,
        mut chunk_store,
        mut meshes,
        mut materials,
    } = main_access;
    let mut state = state;
    let commit_budget = state.commit_budget.max(1);

    let mut committed = 0usize;
    while committed < commit_budget {
        let Some(artifact) = state.artifacts.pop_front() else {
            break;
        };

        let Ok(maybe_visual) = chunk_query.get(artifact.chunk_entity) else {
            state.skipped += 1;
            committed += 1;
            continue;
        };

        if maybe_visual.is_some() || commands.get_entity(artifact.chunk_entity).is_err() {
            state.skipped += 1;
            committed += 1;
            continue;
        }

        apply_chunk_demo_hydration_artifact(artifact, &mut commands, &mut chunk_store, &mut meshes, &mut materials);
        state.hydrated += 1;
        committed += 1;
    }

    if state.artifacts.is_empty() {
        return Ok(Outcome::Done(CommitOutput {
            hydrated: state.hydrated,
            skipped: state.skipped,
        }));
    }

    Ok(Outcome::Wait(state))
}

fn build_artifacts_parallel(settings: UsfDemoSettings, tasks: Vec<ChunkDemoHydrationTask>, requested_workers: usize) -> Vec<ChunkDemoHydrationArtifact> {
    if tasks.is_empty() {
        return Vec::new();
    }

    let task_count = tasks.len();
    let available_workers = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).max(1);
    let worker_count = requested_workers.max(1).min(available_workers).min(task_count);

    if worker_count <= 1 {
        return tasks.into_iter().map(|task| prepare_chunk_demo_hydration_artifact(&settings, task)).collect();
    }

    let queued_tasks = Arc::new(Mutex::new(tasks.into_iter().enumerate().collect::<VecDeque<_>>()));
    let artifacts = Arc::new(Mutex::new((0..task_count).map(|_| None).collect::<Vec<_>>()));

    std::thread::scope(|scope| {
        for _ in 0..worker_count {
            let queued_tasks = Arc::clone(&queued_tasks);
            let artifacts = Arc::clone(&artifacts);
            let settings = settings.clone();

            scope.spawn(move || {
                loop {
                    let next = {
                        let mut queued_tasks = queued_tasks.lock().expect("Queued hydration task mutex poisoned");
                        queued_tasks.pop_front()
                    };
                    let Some((index, task)) = next else {
                        break;
                    };

                    let artifact = prepare_chunk_demo_hydration_artifact(&settings, task);
                    let mut artifacts = artifacts.lock().expect("Hydration artifact mutex poisoned");
                    artifacts[index] = Some(artifact);
                }
            });
        }
    });

    Arc::into_inner(artifacts)
        .expect("Hydration artifact Arc should be uniquely owned after workers join")
        .into_inner()
        .expect("Hydration artifact mutex poisoned")
        .into_iter()
        .flatten()
        .collect()
}
