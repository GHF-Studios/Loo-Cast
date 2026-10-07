//! Planning ticket completion and bounded refresh admission for the live clipmap.
//!
//! Plans are replaceable presentation proposals. They do not become the
//! committed frontier until GPU publication later proves projection readiness.

use super::*;

pub(super) fn poll_plan_tasks(
    registry: &mut CelestialClipmapRealizations,
    telemetry: &mut CelestialClipmapTelemetry,
    current_inputs: &HashMap<Entity, (CelestialClipmapPlanInput, CelestialVoxelField)>,
) -> (HashSet<Entity>, bool) {
    let mut planning_authorities = HashSet::<Entity>::new();
    let mut plans_changed = false;

    let _poll_plan_tasks_span = bevy::log::info_span!("celestial_clipmap.poll_plans").entered();
    let mut pending_plan_tasks = Vec::with_capacity(registry.plan_tasks.len());
    for mut build in std::mem::take(&mut registry.plan_tasks) {
        let Some(&(current_input, current_field)) = current_inputs.get(&build.authority) else {
            continue;
        };
        planning_authorities.insert(build.authority);

        let output = match build.task.try_take() {
            Ok(None) => {
                pending_plan_tasks.push(build);
                continue;
            }
            Ok(Some(output)) => output,
            Err(failure) => {
                planning_authorities.remove(&build.authority);
                warn!(?failure, authority = ?build.authority, "celestial clipmap plan worker failed");
                continue;
            }
        };
        planning_authorities.remove(&build.authority);
        registry
            .planner_caches
            .insert(build.authority, output.surface_cache);

        let committed_anchor_local = registry
            .plans
            .get(&build.authority)
            .map(|plan| plan.observer_anchor_local);
        let relevance = plan_task_relevance(
            build.input,
            current_input,
            build.field,
            current_field,
            committed_anchor_local,
        );
        telemetry.record_plan_task_relevance(relevance);
        if relevance == ClipmapPlanTaskRelevance::Stale {
            continue;
        }

        let Some(stages) = output.stages else {
            continue;
        };

        let (committed_generation, committed_specs) = registry
            .plans
            .get(&build.authority)
            .map(|plan| (plan.committed_generation, plan.committed_specs.clone()))
            .unwrap_or((None, HashSet::new()));

        let Some(stage_index) = initial_stage_for_plan(&stages, &committed_specs, build.input)
        else {
            continue;
        };
        let desired = stages[stage_index].clone();
        if desired.is_empty() {
            continue;
        }

        telemetry.record_plan_quality(build.input, &stages);
        let actual_finest_spacing_metres = stages.last().and_then(|stage| {
            stage
                .iter()
                .map(|spec| spec.key.spacing_metres())
                .min_by(f64::total_cmp)
        });
        let refinement_pending = actual_finest_spacing_metres.is_none_or(|spacing| {
            spacing > build.input.finest.sample_spacing_metres() * 1.001
        });

        let current_target_lag_metres =
            (current_input.planning_anchor_local - build.input.planning_anchor_local).length();

        trace!(
            authority = ?build.authority,
            ?relevance,
            current_target_lag_metres,
            canonical_clearance_metres = build.input.clearance_metres,
            requested_finest_spacing_metres = build.input.finest.sample_spacing_metres(),
            actual_finest_spacing_metres = ?actual_finest_spacing_metres,
            coarsest_spacing_metres = build.input.coarsest.sample_spacing_metres(),
            refinement_stages = stages.len(),
            initial_blocks = desired.len(),
            final_blocks = stages.last().map_or(0, Vec::len),
            requested_target_reached =
                actual_finest_spacing_metres.is_some_and(|spacing| {
                    spacing
                        <= build.input.finest.sample_spacing_metres() * 1.001
                }),
            "celestial clipmap staged plan ready"
        );

        let generation = registry.next_generation();
        registry.plans.insert(
            build.authority,
            CelestialClipmapPlan {
                key: build.input.key,
                field: build.field,
                observer_anchor_local: build.input.observer_anchor_local,
                planning_anchor_local: build.input.planning_anchor_local,
                validity_radius_metres: build.input.validity_radius_metres,
                visibility: build.input.visibility,
                generation,
                stages,
                stage_index,
                desired_set: desired.iter().copied().collect(),
                desired,
                completed: HashSet::new(),
                meshful: HashSet::new(),
                committed_specs,
                committed_generation,
                refinement_pending,
            },
        );

        {
            let CelestialClipmapRealizations {
                plans,
                active_entities,
                ..
            } = &mut *registry;
            if let Some(plan) = plans.get_mut(&build.authority) {
                seed_clipmap_stage_completion(build.authority, plan, active_entities);
            }
        }
        registry.mark_frontier_changed();
        plans_changed = true;
    }

    registry.plan_tasks = pending_plan_tasks;
    drop(_poll_plan_tasks_span);
    (planning_authorities, plans_changed)
}

pub(super) fn schedule_plan_tasks(
    registry: &mut CelestialClipmapRealizations,
    telemetry: &mut CelestialClipmapTelemetry,
    current_inputs: &HashMap<Entity, (CelestialClipmapPlanInput, CelestialVoxelField)>,
    planning_authorities: &mut HashSet<Entity>,
    workers: &VoxelWorkExecutor,
    frame_budget: &mut ReconstructibleFrameBudget,
) {
    let _schedule_plan_span = bevy::log::info_span!("celestial_clipmap.schedule_plans").entered();
    let mut planning_slots = workers.available_slots(VoxelWorkLane::PresentationPlanning);
    let committed_focus_lag = current_inputs
        .iter()
        .filter_map(|(authority, (input, _))| {
            registry.plans.get(authority).map(|plan| {
                (input.observer_anchor_local - plan.observer_anchor_local,)
                    .0
                    .length()
            })
        })
        .filter(|lag| lag.is_finite())
        .max_by(f64::total_cmp);
    telemetry.record_committed_focus_lag(committed_focus_lag);

    for (&authority, &(input, field)) in current_inputs {
        let existing = registry.plans.get(&authority);
        let replace = existing.is_none_or(|plan| {
            if plan.refinement_pending {
                // Do not let fine planning replace the fallback before the
                // fallback has ever become visible.
                plan.committed_generation == Some(plan.generation)
            } else {
                should_schedule_plan_refresh(plan, input, field)
            }
        });
        if !replace || planning_authorities.contains(&authority) || planning_slots == 0 {
            continue;
        }
        let warm_replan = existing.is_some_and(|plan| !plan.committed_specs.is_empty());

        let Some(work_token) = frame_budget.begin(ReconstructibleWorkClass::Maintenance) else {
            break;
        };

        let mut surface_cache = registry
            .planner_caches
            .remove(&authority)
            .unwrap_or_default();

        let Some(task) = workers.try_submit(VoxelWorkLane::PresentationPlanning, move || {
            let stages = build_plan(field, input, &mut surface_cache, warm_replan);
            CelestialClipmapPlanBuildOutput {
                stages,
                surface_cache,
            }
        }) else {
            frame_budget.finish(work_token);
            break;
        };

        telemetry.record_plan_request(warm_replan);
        registry.plan_tasks.push(CelestialClipmapPlanBuildTask {
            authority,
            input,
            field,
            task,
        });
        planning_authorities.insert(authority);
        planning_slots -= 1;
        frame_budget.finish(work_token);
    }
    drop(_schedule_plan_span);
}
