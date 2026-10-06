//! ECS residency reconciliation and hot/warm transitions.

use super::*;

/// Reconciles active voxel materialization residency with the latest spatial
/// demand snapshot.
///
/// This stage owns demand interpretation and hot/warm residency transitions. It
/// does not spawn asynchronous generation work.
pub(in crate::voxel) fn reconcile_voxel_materialization_residency(
    config: Res<EngineConfig>,
    residency: Res<UsfContextResidency>,
    realization_demand: Res<VoxelRealizationDemandSnapshot>,
    view_demands: Res<UsfViewDemandSnapshot>,
    motions: Res<SpatialDemandMotionSnapshot>,
    workers: Res<VoxelWorkExecutor>,
    mut frame_budget: ResMut<ReconstructibleFrameBudget>,
    runtimes: Query<(&VoxelPresentationManifestation, &UsfCapabilityRealization)>,
    mut worlds: Query<(
        Entity,
        &mut VoxelScaleRealization,
        &mut VoxelMaterializationResidency,
        &UsfScaleLayer,
        Option<&CelestialVoxelScaleRealization>,
        Option<&VoxelPinnedMaterializationDemand>,
        Option<&VoxelCollisionDisabled>,
        Option<&VoxelEditingDisabled>,
    )>,
    mut voxel_demands: Local<Vec<VoxelRealizationScope>>,
    mut runtime_roles: Local<HashMap<(Entity, VoxelMaterializationKey), UsfScaleRoleMask>>,
) {
    let configured_warm_limit = config.voxel.streaming.warm_inactive_materialization_limit;
    let expected_dense_build_seconds = workers.estimated_latency_seconds(VoxelWorkLane::Generation)
        + workers.estimated_latency_seconds(VoxelWorkLane::Derivation);

    runtime_roles.clear();
    let mut runtime_roles_ready = false;

    for (
        world_entity,
        mut world,
        mut streaming,
        layer,
        celestial_realization,
        pinned,
        collision_disabled,
        editing_disabled,
    ) in &mut worlds
    {
        voxel_demands.clear();
        voxel_demands.extend(realization_demand.requests_for(world_entity));
        let pinned_shell = pinned
            .and_then(|pinned| pinned.surface_radius_native())
            .map(|radius| {
                (
                    celestial_realization
                        .map_or(world_entity, |realization| realization.authority()),
                    radius,
                )
            });

        let Some(work_token) = frame_budget.begin(ReconstructibleWorkClass::Planning) else {
            break;
        };
        let plan_result = refresh_demand_plan(
            &world,
            &voxel_demands,
            &mut streaming,
            pinned_shell,
            &residency,
            &view_demands,
            &motions,
            layer.scale(),
            expected_dense_build_seconds,
        );
        frame_budget.finish(work_token);

        let plan_changed = match plan_result {
            Ok(changed) => changed,
            Err(error) => {
                error!(
                    error = %error,
                    world = ?world_entity,
                    scale = %layer.scale(),
                    world_leaf = %world.origin().leaf_scale(),
                    "voxel spatial demand could not be represented canonically; retiring stale voxel residency and retrying next frame"
                );
                if streaming.retire_all_desired() {
                    reconcile_materialization_residency(&mut world, &mut streaming, 0);
                }
                continue;
            }
        };

        let candidate_committed = if streaming.migration_active() {
            if !runtime_roles_ready {
                let _span = bevy::log::info_span!("voxel_residency.runtime_roles").entered();
                for (runtime, realization) in &runtimes {
                    if realization.revision() == runtime.revision() {
                        runtime_roles
                            .insert((runtime.realization(), runtime.key()), realization.roles());
                    }
                }
                runtime_roles_ready = true;
            }
            if candidate_plan_ready(
                world_entity,
                &world,
                &streaming,
                collision_disabled.is_none(),
                editing_disabled.is_none(),
                &runtime_roles,
            ) {
                streaming.commit_candidate()
            } else {
                false
            }
        } else {
            false
        };

        if plan_changed || candidate_committed {
            //
            // Incremental entering chunks must be allowed to jump ahead of old
            // background backlog according to current role/focus/trajectory.
            prioritize_pending_work(
                &mut streaming,
                pinned.and_then(|pinned| pinned.surface_radius_native()),
            );
            let warm_limit =
                adaptive_warm_inactive_limit(configured_warm_limit, &world, &streaming);
            reconcile_materialization_residency(&mut world, &mut streaming, warm_limit);
        }
    }
}

fn candidate_plan_ready(
    world_entity: Entity,
    world: &VoxelScaleRealization,
    streaming: &VoxelMaterializationResidency,
    collision_enabled: bool,
    editing_enabled: bool,
    runtime_roles: &HashMap<(Entity, VoxelMaterializationKey), UsfScaleRoleMask>,
) -> bool {
    let _span = bevy::log::info_span!("voxel_residency.candidate_ready").entered();
    streaming
        .migration_candidate_addresses()
        .all(|(key, requested_roles)| {
            let store = world.materializations();
            // REALIZATION-only context is ready when dense truth is current. A
            // derived surface is required only for roles that actually consume one.
            if super::super::roles_require_surface(requested_roles) {
                if !store.is_derived_current(key) {
                    return false;
                }
            } else if store.active_dense_revision(key).is_none() {
                return false;
            }

            let Some(cache) = store.surface(key) else {
                return true;
            };
            if !cache.surface.has_triangles() {
                return true;
            }

            let mut required = UsfScaleRoleMask::REALIZATION;
            if requested_roles.contains(UsfScaleRoleMask::PRESENTATION) {
                required = required.union(UsfScaleRoleMask::PRESENTATION);
            }
            if collision_enabled
                && requested_roles.contains(UsfScaleRoleMask::COLLISION)
                && cache.surface.has_rigid_triangles()
            {
                required = required.union(UsfScaleRoleMask::COLLISION);
            }
            if editing_enabled && requested_roles.contains(UsfScaleRoleMask::EDITING) {
                required = required.union(UsfScaleRoleMask::EDITING);
            }

            runtime_roles
                .get(&(world_entity, key))
                .is_some_and(|roles| roles.contains(required))
        })
}

fn prioritize_pending_work(
    streaming: &mut VoxelMaterializationResidency,
    surface_radius_native: Option<f32>,
) {
    let _span = bevy::log::info_span!("voxel_residency.priority_sort").entered();
    let mut pending = streaming.pending_desired.drain(..).collect::<Vec<_>>();
    pending.sort_by(|a, b| {
        let shell_order = surface_radius_native.map_or(std::cmp::Ordering::Equal, |radius| {
            let a_error = (a.distance_squared.sqrt() - radius).abs();
            let b_error = (b.distance_squared.sqrt() - radius).abs();
            a_error.total_cmp(&b_error)
        });

        compare_work_ranks(a.work_rank(), b.work_rank()).then(shell_order)
    });
    streaming.pending_desired = pending.into();
}

fn adaptive_warm_inactive_limit(
    configured_limit: usize,
    world: &VoxelScaleRealization,
    streaming: &VoxelMaterializationResidency,
) -> usize {
    if configured_limit == 0 {
        return 0;
    }

    let throughput = streaming.load_budget_per_frame().max(1).min(512);
    let pending = streaming.pending_desired_len();

    // Missing desired work always outranks disposable inactive cache history.
    if pending > throughput.saturating_mul(2) {
        return 0;
    }

    let active = world.materializations().active_count();
    let turnover_window = throughput.saturating_mul(4).max(32);
    let locality_window = active.saturating_div(4).max(16);

    configured_limit
        .min(turnover_window.max(locality_window))
        .min(512)
}

fn reconcile_materialization_residency(
    world: &mut VoxelScaleRealization,
    streaming: &mut VoxelMaterializationResidency,
    warm_inactive_materialization_limit: usize,
) {
    let (activate, deactivate) = {
        let _span = bevy::log::info_span!("voxel_residency.delta_collect").entered();
        streaming.take_residency_delta()
    };
    let activated = activate.clone();
    let role_refresh = streaming.take_role_refresh();

    {
        let _span = bevy::log::info_span!("voxel_residency.store_delta").entered();
        world.materializations_mut().apply_residency_delta(
            activate,
            deactivate,
            warm_inactive_materialization_limit,
        );
    }

    // Role changes are capability-pipeline changes, not regeneration events.
    // Reuse resident dense truth and wake only the products now requested.
    for key in activated.into_iter().chain(role_refresh) {
        if streaming.surface_required(key) {
            world.materializations_mut().ensure_derived_dirty(key);
        }
        world.materializations_mut().refresh_render_membership(key);
    }

    {
        let _span = bevy::log::info_span!("voxel_residency.pending_retain").entered();
        streaming
            .pending_desired
            .retain(|demanded| !world.materializations().is_active(demanded.key));
    }
}
