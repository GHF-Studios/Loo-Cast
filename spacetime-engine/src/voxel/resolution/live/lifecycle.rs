//! GPU work admission and make-before-break frontier publication.

use super::*;

pub(super) fn park_clipmap_entity(
    commands: &mut Commands,
    entity: Entity,
    block: &mut CelestialClipmapBlock,
    visibility: &mut Visibility,
    registry: &mut CelestialClipmapRealizations,
) {
    if !block.active {
        return;
    }

    let affected_visible_frontier = block.committed;
    registry
        .active_entities
        .remove(&(block.authority, block.spec));
    block.active = false;
    block.committed = false;
    block.projection_ready = false;
    block.material_relative_level = i16::MIN;
    *visibility = Visibility::Hidden;

    //
    // GPU-native clipmap shells own substantial persistent MeshAllocator
    // ranges. Retaining retired shells kept those ranges alive and allowed the
    // old 4096-entity pool to consume gigabytes. Despawn instead: once the
    // Mesh3d handle drops, Bevy propagates AssetEvent::Unused to RenderAssets,
    // which releases the allocator ranges.
    commands.entity(entity).despawn();

    if affected_visible_frontier {
        registry.mark_frontier_changed();
    }
}

pub(super) fn spawn_gpu_clipmap_entity(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    admission: PendingGpuAdmission,
    presentation_material: Handle<VoxelRenderMaterial>,
    build_id: u64,
) -> Entity {
    let extent = admission.spec.key.extent_metres() as f32;
    let transition_face_count = admission.spec.transition_faces.bits().count_ones();

    assert!(
        extent.is_finite() && extent > 0.0,
        "GPU clipmap extent must be finite and positive"
    );
    assert!(
        transition_face_count <= 6,
        "GPU clipmap transitions must describe at most six faces"
    );

    let bounds = Aabb::from_min_max(Vec3::ZERO, Vec3::splat(extent));
    let mesh = meshes.add(allocation_mesh(transition_face_count));

    commands
        .spawn((
            Name::new("Celestial Binary Clipmap"),
            CelestialClipmapBlock {
                authority: admission.authority,
                spec: admission.spec,
                active: true,
                committed: false,
                projection_ready: false,
                material_relative_level: admission.relative_level,
            },
            UsfPresentationProjectionOf(admission.authority),
            Mesh3d(mesh.clone()),
            MeshMaterial3d(presentation_material),
            bounds,
            NoAutoAabb,
            GpuTerrainBuild::new(mesh, build_id, admission.descriptor),
            Transform::IDENTITY,
            RenderLayers::layer(USF_PRESENTATION_LAYER),
            NotShadowCaster,
            NotShadowReceiver,
            Visibility::Hidden,
        ))
        .id()
}

/// Stable rotation prevents a limited admission budget from favoring the
/// incidental iteration order of the plan map.
pub(super) fn gpu_admission_authorities(registry: &CelestialClipmapRealizations) -> Vec<Entity> {
    // Resume after the last successful authority even when the plan set changes.
    let mut authorities = registry.plans.keys().copied().collect::<Vec<_>>();
    authorities.sort_unstable_by_key(|authority| authority.to_bits());
    if let Some(cursor) = registry.gpu_admission_cursor {
        let start =
            authorities.partition_point(|authority| authority.to_bits() <= cursor.to_bits());
        let count = authorities.len();
        if count != 0 {
            authorities.rotate_left(start % count);
        }
    }
    authorities
}

/// Select bounded GPU builds. This decides work admission only; it does not
/// publish a frontier or claim GPU completion.
pub(super) fn select_gpu_admissions(
    registry: &mut CelestialClipmapRealizations,
    frame_budget: &mut ReconstructibleFrameBudget,
    inflight: &HashSet<(Entity, CelestialClipmapBlockSpec)>,
) -> Vec<PendingGpuAdmission> {
    let mut admissions = Vec::new();
    {
        let _span = bevy::log::info_span!("celestial_clipmap.schedule_gpu_builds").entered();

        // Descriptor/publication admissions remain bounded even though there
        // are no CPU mesh jobs anymore, preventing allocator/entity bursts.
        const MAX_GPU_BUILDS_IN_FLIGHT: usize = 32;
        const MAX_GPU_ADMISSIONS_PER_FRAME: usize = 8;
        let mut admitted = 0usize;
        let mut next_admission_cursor = registry.gpu_admission_cursor;

        'authorities: for authority in gpu_admission_authorities(registry) {
            let plan = &registry.plans[&authority];
            for &spec in &plan.desired {
                let key = (authority, spec);
                if plan.completed.contains(&spec)
                    || registry.active_entities.contains_key(&key)
                    || inflight.contains(&key)
                {
                    continue;
                }

                if inflight.len().saturating_add(admissions.len()) >= MAX_GPU_BUILDS_IN_FLIGHT
                    || admitted >= MAX_GPU_ADMISSIONS_PER_FRAME
                {
                    break 'authorities;
                }

                let Some(work_token) = frame_budget.begin(ReconstructibleWorkClass::Maintenance)
                else {
                    break 'authorities;
                };

                let Some(descriptor) = descriptor_for_block(
                    plan.field,
                    spec.key.origin_local_metres(),
                    spec.key.extent_metres(),
                    spec.key.spacing_metres(),
                    spec.transition_faces.bits(),
                ) else {
                    frame_budget.finish(work_token);
                    continue;
                };

                admissions.push(PendingGpuAdmission {
                    authority,
                    generation: plan.generation,
                    spec,
                    field: plan.field,
                    relative_level: spec
                        .key
                        .resolution
                        .binary_exponent()
                        .saturating_sub(plan.key.finest_exponent),
                    descriptor,
                });
                admitted += 1;
                next_admission_cursor = Some(authority);
                frame_budget.finish(work_token);
            }
        }
        registry.gpu_admission_cursor = next_admission_cursor;
    }
    admissions
}

/// Retire invalid build records and promote acknowledged GPU submissions to
/// projection-pending blocks. An acknowledgment is not a visibility certificate.
pub(super) fn poll_gpu_builds(
    commands: &mut Commands,
    blocks: &mut Query<(Entity, &mut CelestialClipmapBlock, &mut Visibility)>,
    registry: &mut CelestialClipmapRealizations,
    completed_gpu_builds: &HashSet<u64>,
) -> HashSet<(Entity, CelestialClipmapBlockSpec)> {
    let mut inflight = HashSet::new();
    {
        let _span = bevy::log::info_span!("celestial_clipmap.poll_gpu_builds").entered();
        let mut pending_builds = Vec::with_capacity(registry.build_tasks.len());

        for build in std::mem::take(&mut registry.build_tasks) {
            let key = (build.authority, build.spec);

            let valid = registry.plans.get(&build.authority).is_some_and(|plan| {
                build.generation == plan.generation
                    && plan.desired_set.contains(&build.spec)
                    && build.field == plan.field
            });

            if !valid {
                if let Ok((_, mut block, mut visibility)) = blocks.get_mut(build.entity) {
                    commands.entity(build.entity).remove::<GpuTerrainBuild>();
                    park_clipmap_entity(
                        commands,
                        build.entity,
                        &mut block,
                        &mut visibility,
                        registry,
                    );
                }
                continue;
            }

            if !completed_gpu_builds.contains(&build.build_id) {
                inflight.insert(key);
                pending_builds.push(build);
                continue;
            }

            if blocks.get_mut(build.entity).is_err() {
                continue;
            }

            commands.entity(build.entity).remove::<GpuTerrainBuild>();
            registry.active_entities.insert(key, build.entity);
            registry.mark_projection_pending(build.entity);

            if let Some(plan) = registry.plans.get_mut(&build.authority) {
                plan.completed.insert(build.spec);
                plan.meshful.insert(build.spec);
            }
        }

        registry.build_tasks = pending_builds;
    }
    inflight
}

/// A planned frontier becomes authoritative presentation only after every
/// desired block is complete and each new meshful block has projected. Retire
/// old coverage after that commit, never during admission or GPU acknowledgment.
pub(super) fn frontier_ready_for_commit(
    authority: Entity,
    plan: &CelestialClipmapPlan,
    active_entities: &HashMap<(Entity, CelestialClipmapBlockSpec), Entity>,
    blocks: &Query<(Entity, &mut CelestialClipmapBlock, &mut Visibility)>,
) -> bool {
    if plan.committed_generation == Some(plan.generation)
        || !plan
            .desired
            .iter()
            .all(|spec| plan.completed.contains(spec))
    {
        return false;
    }
    // Unchanged committed specs need no new proof. Replacement/new meshful
    // blocks must project before make-before-break changes the frontier.
    plan.meshful
        .iter()
        .filter(|spec| !plan.committed_specs.contains(*spec))
        .all(|spec| {
            active_entities
                .get(&(authority, *spec))
                .and_then(|entity| blocks.get(*entity).ok())
                .is_some_and(|(_, block, _)| block.active && block.projection_ready)
        })
}

/// Advance staged refinement only after its current frontier was committed.
pub(super) fn advance_clipmap_plan(
    authority: Entity,
    plan: &mut CelestialClipmapPlan,
    active_entities: &HashMap<(Entity, CelestialClipmapBlockSpec), Entity>,
) {
    let added_blocks = plan.desired_set.difference(&plan.committed_specs).count();
    let retired_blocks = plan.committed_specs.difference(&plan.desired_set).count();

    std::mem::swap(&mut plan.committed_specs, &mut plan.desired_set);

    let committed_generation = plan.generation;
    if plan.stage_index + 1 < plan.stages.len() {
        plan.committed_generation = Some(committed_generation);
        plan.stage_index += 1;
        plan.generation = plan.generation.wrapping_add(1).max(1);
        plan.desired.clone_from(&plan.stages[plan.stage_index]);
        plan.desired_set.clear();
        plan.desired_set.extend(plan.desired.iter().copied());
        seed_clipmap_stage_completion(authority, plan, active_entities);

        trace!(
            authority = ?authority,
            committed_stage = plan.stage_index,
            total_stages = plan.stages.len(),
            committed_blocks = plan.committed_specs.len(),
            transaction_added_blocks = added_blocks,
            transaction_retired_blocks = retired_blocks,
            next_blocks = plan.desired.len(),
            "clipmap refinement frontier committed; arming next stage"
        );
    } else {
        plan.committed_generation = Some(committed_generation);
        plan.desired_set.clear();
        trace!(
            authority = ?authority,
            committed_stage = plan.stage_index + 1,
            total_stages = plan.stages.len(),
            committed_blocks = plan.committed_specs.len(),
            transaction_added_blocks = added_blocks,
            transaction_retired_blocks = retired_blocks,
            "clipmap final refinement frontier committed"
        );
    }
}

pub(super) fn commit_ready_frontiers(
    commands: &mut Commands,
    blocks: &mut Query<(Entity, &mut CelestialClipmapBlock, &mut Visibility)>,
    registry: &mut CelestialClipmapRealizations,
) {
    {
        let _span = bevy::log::info_span!("celestial_clipmap.commit").entered();
        let mut retired_entities = Vec::<Entity>::new();

        let CelestialClipmapRealizations {
            plans,
            active_entities,
            ..
        } = &mut *registry;
        let mut committed_any_frontier = false;
        for (&authority, plan) in plans {
            if !frontier_ready_for_commit(authority, plan, active_entities, blocks) {
                continue;
            }

            for ((block_authority, _), &entity) in active_entities.iter() {
                if *block_authority != authority {
                    continue;
                }
                let Ok((_, mut block, _)) = blocks.get_mut(entity) else {
                    continue;
                };
                if !block.active {
                    continue;
                }
                if plan.desired_set.contains(&block.spec) {
                    block.committed = true;
                } else {
                    retired_entities.push(entity);
                }
            }

            advance_clipmap_plan(authority, plan, active_entities);
            committed_any_frontier = true;
        }

        // End the disjoint field borrows before touching the pool itself.
        let _ = plans;
        let _ = active_entities;
        if committed_any_frontier {
            registry.mark_frontier_changed();
        }
        for entity in retired_entities {
            if let Ok((_, mut block, mut visibility)) = blocks.get_mut(entity) {
                park_clipmap_entity(commands, entity, &mut block, &mut visibility, registry);
            }
        }
    }
}

/// Derive current presentation planning evidence from semantic authorities.
/// Edited fields remain on the dense path; prediction adds planning interest
/// while visibility retains the actual observer position.
pub(super) fn collect_clipmap_plan_inputs(
    authorities: &Query<(
        Entity,
        Option<&Name>,
        &UsfPosition,
        &UsfSemanticFrame,
        &CelestialVoxelField,
        &VoxelSemanticAuthority,
        &CelestialVoxelRealizationPolicy,
    )>,
    view: &UsfViewDemand,
    expected_build_seconds: f64,
) -> (
    HashSet<Entity>,
    HashMap<Entity, (CelestialClipmapPlanInput, CelestialVoxelField)>,
) {
    let observer_speed = view.velocity_metres_per_second().length();
    let mut live_authorities = HashSet::<Entity>::new();
    let mut current_inputs =
        HashMap::<Entity, (CelestialClipmapPlanInput, CelestialVoxelField)>::new();

    {
        let _span = bevy::log::info_span!("celestial_clipmap.plan_identity").entered();
        for (authority, _name, body_origin, body_frame, field, voxel_authority, _policy) in
            authorities.iter()
        {
            if !voxel_authority.is_empty() {
                continue;
            }
            let Ok(observer_local) = body_frame.world_to_local_metres(
                body_origin,
                &view.anchor(),
                SpatialScale::ZERO,
                f64::MAX,
            ) else {
                continue;
            };

            //
            // Aim reconstructible presentation work where the observer is
            // expected to be when planning+mesh work drains, not at the point
            // already being left behind.
            let prediction_seconds = (expected_build_seconds * CLIPMAP_LATENCY_MULTIPLIER)
                .clamp(0.0, CLIPMAP_MAX_VALIDITY_SECONDS);
            let local_velocity_metres_per_second =
                body_frame.orientation().conjugate() * view.velocity_metres_per_second();
            let predicted_observer_local =
                observer_local + local_velocity_metres_per_second * prediction_seconds;

            let visibility = ClipmapVisibilityDemand::new(
                *field,
                observer_local,
                predicted_observer_local,
                body_frame,
                view,
            );
            let Some(input) = derive_plan_input(
                *field,
                predicted_observer_local,
                view.pixels_per_radian_for_presentation_resolution(),
                observer_speed,
                expected_build_seconds,
                visibility,
            ) else {
                continue;
            };
            live_authorities.insert(authority);
            current_inputs.insert(authority, (input, *field));
        }
    }
    (live_authorities, current_inputs)
}

/// Remove plans for authorities no longer eligible for binary presentation,
/// retiring their active shells only when the plan set actually changes.
pub(super) fn retire_dead_clipmap_plans(
    commands: &mut Commands,
    blocks: &mut Query<(Entity, &mut CelestialClipmapBlock, &mut Visibility)>,
    registry: &mut CelestialClipmapRealizations,
    live_authorities: &HashSet<Entity>,
) -> bool {
    let plan_count_before_retain = registry.plans.len();
    registry
        .plans
        .retain(|authority, _| live_authorities.contains(authority));
    let plans_removed = registry.plans.len() != plan_count_before_retain;
    registry
        .planner_caches
        .retain(|authority, _| live_authorities.contains(authority));
    if plans_removed {
        let dead_entities = registry
            .active_entities
            .iter()
            .filter_map(|((authority, _), &entity)| {
                (!live_authorities.contains(authority)).then_some(entity)
            })
            .collect::<Vec<_>>();
        for entity in dead_entities {
            if let Ok((_, mut block, mut visibility)) = blocks.get_mut(entity) {
                park_clipmap_entity(commands, entity, &mut block, &mut visibility, registry);
            }
        }
    }
    plans_removed
}

pub(super) fn reconcile_celestial_clipmap_realizations(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material_params: CelestialClipmapMaterialParams,
    views: Res<UsfViewDemandSnapshot>,
    workers: Res<VoxelWorkExecutor>,
    authorities: Query<(
        Entity,
        Option<&Name>,
        &UsfPosition,
        &UsfSemanticFrame,
        &CelestialVoxelField,
        &VoxelSemanticAuthority,
        &CelestialVoxelRealizationPolicy,
    )>,
    mut blocks: Query<(Entity, &mut CelestialClipmapBlock, &mut Visibility)>,
    mut registry: ResMut<CelestialClipmapRealizations>,
    mut telemetry: ResMut<CelestialClipmapTelemetry>,
    mut frame_budget: ResMut<ReconstructibleFrameBudget>,
    mut gpu_runtime: ResMut<GpuTerrainBuilds>,
) {
    let Some(view) = views.iter().next() else {
        return;
    };

    let expected_build_seconds =
        workers.estimated_latency_seconds(VoxelWorkLane::PresentationPlanning);
    let (live_authorities, current_inputs) =
        collect_clipmap_plan_inputs(&authorities, view, expected_build_seconds);

    let (mut planning_authorities, plans_changed) =
        planning::poll_plan_tasks(&mut registry, &mut telemetry, &current_inputs);

    planning::schedule_plan_tasks(
        &mut registry,
        &mut telemetry,
        &current_inputs,
        &mut planning_authorities,
        &workers,
        &mut frame_budget,
    );

    let plans_removed =
        retire_dead_clipmap_plans(&mut commands, &mut blocks, &mut registry, &live_authorities);

    let no_plan_tasks = registry.plan_tasks.is_empty();
    let no_build_tasks = registry.build_tasks.is_empty();
    let plans_settled = registry
        .plans
        .values()
        .all(|plan| plan.committed_generation == Some(plan.generation));
    if !plans_changed && !plans_removed && no_plan_tasks && no_build_tasks && plans_settled {
        return;
    }

    // Current-stage completion is seeded exactly once when a stage
    // becomes active and incrementally updated by worker results.

    //
    // CPU planning decides which semantic blocks are required. Binary density,
    // Transvoxel extraction and terrain-buffer writes are GPU-owned.
    //
    // RenderWorld returns only a tiny dispatch acknowledgement. This is not
    // density/geometry readback; it preserves the make-before-break projection
    // barrier before the previous committed frontier may retire.
    let completed_gpu_builds = gpu_runtime
        .drain_completed()
        .into_iter()
        .collect::<HashSet<_>>();
    let mut inflight = poll_gpu_builds(
        &mut commands,
        &mut blocks,
        &mut registry,
        &completed_gpu_builds,
    );

    let admissions = select_gpu_admissions(&mut registry, &mut frame_budget, &inflight);

    let mut scheduled_builds = Vec::<CelestialClipmapBuildTask>::new();

    for admission in admissions {
        let still_valid = registry
            .plans
            .get(&admission.authority)
            .is_some_and(|plan| {
                admission.generation == plan.generation
                    && plan.desired_set.contains(&admission.spec)
                    && admission.field == plan.field
            });
        if !still_valid {
            continue;
        }

        let Ok((_, _name, _, _, _, _, policy)) = authorities.get(admission.authority) else {
            continue;
        };

        let standard_materials = &material_params.standard_materials;
        let debug_grid = &material_params.library.debug_grid;
        let render_materials = &mut material_params.render_materials;
        let shader_buffers = &mut material_params.shader_buffers;
        let band_materials = &mut material_params.band_materials;

        let Some(presentation_material) = band_materials.material_for(
            standard_materials,
            render_materials,
            shader_buffers,
            debug_grid,
            policy.presentation_material(),
            admission.relative_level,
        ) else {
            continue;
        };

        let build_id = gpu_runtime.next_build_id();
        let entity = spawn_gpu_clipmap_entity(
            &mut commands,
            &mut meshes,
            admission,
            presentation_material,
            build_id,
        );

        let key = (admission.authority, admission.spec);
        inflight.insert(key);
        scheduled_builds.push(CelestialClipmapBuildTask {
            authority: admission.authority,
            generation: admission.generation,
            spec: admission.spec,
            field: admission.field,
            entity,
            build_id,
        });
    }

    registry.build_tasks.extend(scheduled_builds);

    commit_ready_frontiers(&mut commands, &mut blocks, &mut registry);
}
