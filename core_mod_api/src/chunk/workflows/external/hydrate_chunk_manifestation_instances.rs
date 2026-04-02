use crate::bevy::prelude::*;
use crate::chunk::components::Chunk;
use crate::chunk::resources::ChunkManager;
use crate::usf::runtime::chunk_manifestation::{
    ChunkManifestationBinding, ChunkManifestationHydrationArtifact, ChunkManifestationHydrationTask, UsfChunkManifestationInstance,
    UsfChunkManifestationRuntimeSettings, UsfChunkManifestationStore, prepare_chunk_manifestation_hydration_artifact,
};
use crate::usf::runtime::manifestation_capability::{ChunkManifestationCapabilityPolicy, apply_chunk_manifestation_capabilities};
use crate::usf::runtime::manifestation_field::{canonical_grid_coord, density_field_signature};
use crate::workflow::types::Outcome;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AsyncInput {
    pub settings: UsfChunkManifestationRuntimeSettings,
    pub tasks: Vec<ChunkManifestationHydrationTask>,
    pub build_workers: usize,
    pub commit_budget: usize,
}

pub struct ArtifactsOutput {
    pub artifacts: VecDeque<ChunkManifestationHydrationArtifact>,
    pub commit_budget: usize,
    pub attach_meshes: bool,
}

pub struct State {
    pub artifacts: VecDeque<ChunkManifestationHydrationArtifact>,
    pub commit_budget: usize,
    pub attach_meshes: bool,
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
    pub chunk_query: Query<
        'w,
        's,
        (
            &'static Chunk,
            Option<&'static UsfChunkManifestationInstance>,
            Option<&'static ChunkManifestationBinding>,
        ),
    >,
    pub chunk_manager: Res<'w, ChunkManager>,
    pub chunk_store: ResMut<'w, UsfChunkManifestationStore>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
}

pub fn run_async(input: Input) -> Result<ArtifactsOutput, Error> {
    let Input { inner } = input;
    let commit_budget = inner.commit_budget.max(1);
    let settings = inner.settings;
    let attach_meshes = settings.attach_meshes;
    let artifacts = build_artifacts_parallel(settings, inner.tasks, inner.build_workers.max(1));

    Ok(ArtifactsOutput {
        artifacts: artifacts.into_iter().collect(),
        commit_budget,
        attach_meshes,
    })
}

pub fn setup_ecs_while(input: ArtifactsOutput, _main_access: MainAccess) -> Result<State, Error> {
    Ok(State {
        artifacts: input.artifacts,
        commit_budget: input.commit_budget.max(1),
        attach_meshes: input.attach_meshes,
        hydrated: 0,
        skipped: 0,
    })
}

pub fn run_ecs_while(state: State, main_access: MainAccess) -> Result<Outcome<State, CommitOutput>, Error> {
    let MainAccess {
        mut commands,
        chunk_query,
        chunk_manager,
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

        let Ok((chunk, maybe_instance, maybe_binding)) = chunk_query.get(artifact.chunk_entity) else {
            state.skipped += 1;
            committed += 1;
            continue;
        };

        if maybe_instance.is_some() || commands.get_entity(artifact.chunk_entity).is_err() {
            state.skipped += 1;
            committed += 1;
            continue;
        }

        if !chunk_manager.chunks.contains(&chunk.coord) || !hydration_artifact_matches_binding(&artifact, chunk, maybe_binding) {
            state.skipped += 1;
            committed += 1;
            continue;
        }

        apply_chunk_manifestation_capabilities(
            artifact,
            ChunkManifestationCapabilityPolicy {
                attach_meshes: state.attach_meshes,
            },
            &mut commands,
            &mut chunk_store,
            &mut meshes,
            &mut materials,
        );
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

fn hydration_artifact_matches_binding(
    artifact: &ChunkManifestationHydrationArtifact,
    chunk: &Chunk,
    maybe_binding: Option<&ChunkManifestationBinding>,
) -> bool {
    let Some(binding) = maybe_binding else {
        return false;
    };

    if chunk.coord != artifact.chunk_coord {
        return false;
    }

    let canonical_coord = canonical_grid_coord(&chunk.coord);
    if canonical_coord != artifact.canonical_coord {
        return false;
    }

    artifact.record.zone_type.eq_ignore_ascii_case(&binding.zone_type.0)
        && artifact.record.zone_density_signature == binding.zone_density_signature
        && artifact.record.phenomenon_script_id.eq_ignore_ascii_case(binding.phenomenon_script_id.as_str())
        && artifact.record.density_field_signature == density_field_signature(binding.manifestation_field_contract)
        && artifact.manifestation_material_profile == binding.manifestation_material_profile
        && artifact.manifestation_collider_enabled == binding.manifestation_collider_enabled
}

fn build_artifacts_parallel(
    settings: UsfChunkManifestationRuntimeSettings,
    tasks: Vec<ChunkManifestationHydrationTask>,
    requested_workers: usize,
) -> Vec<ChunkManifestationHydrationArtifact> {
    if tasks.is_empty() {
        return Vec::new();
    }

    let task_count = tasks.len();
    let available_workers = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).max(1);
    let worker_count = requested_workers.max(1).min(available_workers).min(task_count);

    if worker_count <= 1 {
        return tasks
            .into_iter()
            .map(|task| prepare_chunk_manifestation_hydration_artifact(&settings, task))
            .collect();
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

                    let artifact = prepare_chunk_manifestation_hydration_artifact(&settings, task);
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
