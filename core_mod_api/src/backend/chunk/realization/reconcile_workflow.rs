use crate::bevy::prelude::*;
use crate::usf::chunk::realization::output_channels::{
    ChunkRealizationChannelAppliedEvent, OutputChannelRegistry, apply_chunk_output_channels,
};
use crate::usf::chunk::components::Chunk;
use crate::usf::chunk::realization::field::{canonical_grid_coord, density_field_signature};
use crate::usf::chunk::realization::runtime::{
    ChunkRealizationCache, ChunkRealizationInstance, ChunkRealizationIntent, ChunkRealizationIntentSnapshot, ChunkRealizationResolvedArtifact,
    UsfChunkRealizationRuntimeSettings, resolve_chunk_realization_artifact,
};
use crate::usf::chunk::resources::ChunkManager;
use crate::workflow::types::Outcome;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AsyncInput {
    pub settings: UsfChunkRealizationRuntimeSettings,
    pub tasks: Vec<ChunkRealizationIntentSnapshot>,
    pub build_workers: usize,
    pub commit_budget: usize,
}

pub struct ResolvedIntentsOutput {
    pub artifacts: VecDeque<ChunkRealizationResolvedArtifact>,
    pub commit_budget: usize,
}

pub struct State {
    pub artifacts: VecDeque<ChunkRealizationResolvedArtifact>,
    pub commit_budget: usize,
    pub applied: usize,
    pub skipped: usize,
}

pub struct CommitOutput {
    pub applied: usize,
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
            Option<&'static ChunkRealizationInstance>,
            Option<&'static ChunkRealizationIntent>,
        ),
    >,
    pub chunk_manager: Res<'w, ChunkManager>,
    pub chunk_store: ResMut<'w, ChunkRealizationCache>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials: ResMut<'w, Assets<StandardMaterial>>,
    pub channel_registry: Res<'w, OutputChannelRegistry>,
    pub channel_events: MessageWriter<'w, ChunkRealizationChannelAppliedEvent>,
}

pub fn run_async(input: Input) -> Result<ResolvedIntentsOutput, Error> {
    let Input { inner } = input;
    let commit_budget = inner.commit_budget.max(1);
    let settings = inner.settings;
    let artifacts = build_artifacts_parallel(settings, inner.tasks, inner.build_workers.max(1));

    Ok(ResolvedIntentsOutput {
        artifacts: artifacts.into_iter().collect(),
        commit_budget,
    })
}

pub fn setup_ecs_while(input: ResolvedIntentsOutput, _main_access: MainAccess) -> Result<State, Error> {
    Ok(State {
        artifacts: input.artifacts,
        commit_budget: input.commit_budget.max(1),
        applied: 0,
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
        channel_registry,
        mut channel_events,
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

        if !chunk_manager.chunks.contains(&chunk.coord) || !reconcile_artifact_matches_intent(&artifact, chunk, maybe_binding) {
            state.skipped += 1;
            committed += 1;
            continue;
        }

        apply_chunk_output_channels(
            artifact,
            &channel_registry,
            &mut channel_events,
            &mut commands,
            &mut chunk_store,
            &mut meshes,
            &mut materials,
        );
        state.applied += 1;
        committed += 1;
    }

    if state.artifacts.is_empty() {
        return Ok(Outcome::Done(CommitOutput {
            applied: state.applied,
            skipped: state.skipped,
        }));
    }

    Ok(Outcome::Wait(state))
}

fn reconcile_artifact_matches_intent(artifact: &ChunkRealizationResolvedArtifact, chunk: &Chunk, maybe_intent: Option<&ChunkRealizationIntent>) -> bool {
    let Some(intent) = maybe_intent else {
        return false;
    };

    if chunk.coord != artifact.chunk_coord {
        return false;
    }

    let canonical_coord = canonical_grid_coord(&chunk.coord);
    if canonical_coord != artifact.canonical_coord {
        return false;
    }

    artifact.record.zone_type.eq_ignore_ascii_case(&intent.zone_type.0)
        && artifact.record.zone_density_signature == intent.zone_density_signature
        && artifact.record.phenomenon_script_id.eq_ignore_ascii_case(intent.phenomenon_script_id.as_str())
        && artifact.record.selected_model_id.eq_ignore_ascii_case(intent.selected_model_id.as_str())
        && artifact.record.density_field_signature == density_field_signature(intent.output_field_spec)
        && artifact.channel_payloads == intent.channel_payloads
}

fn build_artifacts_parallel(
    settings: UsfChunkRealizationRuntimeSettings,
    tasks: Vec<ChunkRealizationIntentSnapshot>,
    requested_workers: usize,
) -> Vec<ChunkRealizationResolvedArtifact> {
    if tasks.is_empty() {
        return Vec::new();
    }

    let task_count = tasks.len();
    let available_workers = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(1).max(1);
    let worker_count = requested_workers.max(1).min(available_workers).min(task_count);

    if worker_count <= 1 {
        return tasks.into_iter().map(|task| resolve_chunk_realization_artifact(&settings, task)).collect();
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
                        let mut queued_tasks = queued_tasks.lock().expect("Queued reconcile task mutex poisoned");
                        queued_tasks.pop_front()
                    };
                    let Some((index, task)) = next else {
                        break;
                    };

                    let artifact = resolve_chunk_realization_artifact(&settings, task);
                    let mut artifacts = artifacts.lock().expect("Reconcile artifact mutex poisoned");
                    artifacts[index] = Some(artifact);
                }
            });
        }
    });

    Arc::into_inner(artifacts)
        .expect("Reconcile artifact Arc should be uniquely owned after workers join")
        .into_inner()
        .expect("Reconcile artifact mutex poisoned")
        .into_iter()
        .flatten()
        .collect()
}
