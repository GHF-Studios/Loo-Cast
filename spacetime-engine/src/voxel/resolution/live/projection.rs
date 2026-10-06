//! View projection and dense presentation fallback.

use super::*;

pub(super) fn binary_frontier_projection_complete(
    expected_meshes: usize,
    projected_meshes: usize,
) -> bool {
    expected_meshes > 0 && projected_meshes == expected_meshes
}

pub(super) fn project_clipmap_shell(
    block: &mut CelestialClipmapBlock,
    transform: &mut Transform,
    visibility: &mut Visibility,
    frame: ClipmapProjectedAuthorityFrame,
    projection_eye: DVec3,
    presentation_origin: Vec3,
    metre_to_view_f64: f64,
    metre_to_view: f32,
) -> bool {
    let local_origin = block.spec.key.origin_local_metres();
    let relative_metres = frame.relative_metres + frame.orientation * local_origin;
    let projected = (relative_metres - projection_eye) * metre_to_view_f64;
    if !projected.is_finite() {
        block.projection_ready = false;
        *visibility = Visibility::Hidden;
        return false;
    }

    let projected = Vec3::new(projected.x as f32, projected.y as f32, projected.z as f32);
    let translation = presentation_origin + projected;
    if !translation.is_finite() {
        block.projection_ready = false;
        *visibility = Visibility::Hidden;
        return false;
    }

    transform.translation = translation;
    transform.rotation = frame.rotation;
    transform.scale = Vec3::splat(metre_to_view);
    block.projection_ready = true;
    true
}

pub(super) fn sync_celestial_clipmap_transforms(
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    mut registry: ResMut<CelestialClipmapRegistry>,
    mut material_params: CelestialClipmapMaterialParams,
    authorities: Query<(
        &UsfPosition,
        &UsfSemanticFrame,
        &CelestialVoxelField,
        &CelestialVoxelRealizationPolicy,
    )>,
    mut coverage: ResMut<CelestialClipmapCoverageSnapshot>,
    mut presentation_state: ResMut<CelestialTerrainPresentationState>,
    mut telemetry: ResMut<CelestialClipmapTelemetry>,
    mut blocks: Query<(
        &mut CelestialClipmapBlock,
        &mut Transform,
        &mut Visibility,
        &mut MeshMaterial3d<VoxelRenderMaterial>,
    )>,
    mut scratch: Local<CelestialClipmapTransformScratch>,
    mut logged_projection: Local<bool>,
) {
    let Some(metre_to_view_f64) = view.projection_factor_f64(SpatialScale::ZERO) else {
        for &entity in registry.active_entities.values() {
            if let Ok((mut block, _, mut visibility, _)) = blocks.get_mut(entity) {
                block.projection_ready = false;
                *visibility = Visibility::Hidden;
            }
        }
        scratch.live_authorities.clear();
        scratch
            .live_authorities
            .extend(registry.plans.keys().copied());
        coverage.retain_authorities(&scratch.live_authorities);
        for &authority in &scratch.live_authorities {
            coverage.replace_authority_from_slice(authority, &[]);
        }
        presentation_state.clear();
        telemetry.record_visible_frontier(0, 0, &HashSet::new(), None, None);
        scratch.initialized = false;
        return;
    };

    let metre_to_view = metre_to_view_f64 as f32;
    if !metre_to_view.is_finite() || metre_to_view <= 0.0 {
        return;
    }

    let view_stamp = ClipmapViewProjectionStamp {
        anchor: *view.anchor(),
        metre_to_view_f64,
        projection_eye: view.projection_eye_offset_metres(),
        presentation_origin: view.presentation_origin(),
    };
    let view_changed = scratch.last_view_stamp != Some(view_stamp);

    let _dirty_span = bevy::log::info_span!("celestial_clipmap.transform_dirty_check").entered();

    scratch.next_authority_stamps.clear();
    let mut authority_changed = scratch.authority_stamps.len() != registry.plans.len();
    for &authority in registry.plans.keys() {
        let Ok((body_origin, body_frame, _, policy)) = authorities.get(authority) else {
            authority_changed = true;
            continue;
        };
        let stamp = ClipmapAuthorityProjectionStamp {
            origin: *body_origin,
            orientation: body_frame.orientation(),
            presentation_material: policy.presentation_material().clone(),
        };
        if scratch.authority_stamps.get(&authority) != Some(&stamp) {
            authority_changed = true;
        }
        scratch.next_authority_stamps.insert(authority, stamp);
    }
    {
        let CelestialClipmapTransformScratch {
            authority_stamps,
            next_authority_stamps,
            ..
        } = &mut *scratch;
        std::mem::swap(authority_stamps, next_authority_stamps);
    }

    let frontier_changed =
        !scratch.initialized || scratch.last_frontier_epoch != registry.frontier_epoch;
    let projection_changed = !scratch.initialized
        || scratch.last_projection_epoch != registry.projection_epoch
        || !registry.projection_pending.is_empty();

    drop(_dirty_span);

    if scratch.initialized
        && !view_changed
        && !authority_changed
        && !frontier_changed
        && !projection_changed
    {
        return;
    }

    if scratch.initialized
        && !view_changed
        && !authority_changed
        && !frontier_changed
        && projection_changed
    {
        let _span = bevy::log::info_span!("celestial_clipmap.project_pending").entered();

        scratch.pending_entities.clear();
        std::mem::swap(
            &mut scratch.pending_entities,
            &mut registry.projection_pending,
        );

        // Move the reusable Vec out while iterating so the drain does not
        // retain a mutable borrow of `scratch` while we read `frames`.
        // Restore the now-empty allocation afterward for reuse next frame.
        let mut pending_entities = std::mem::take(&mut scratch.pending_entities);

        for entity in pending_entities.drain(..) {
            let Ok((mut block, mut transform, mut visibility, _)) = blocks.get_mut(entity) else {
                continue;
            };
            if !block.active || block.committed {
                continue;
            }
            let Some(frame) = scratch.frames.get(&block.authority).copied() else {
                continue;
            };
            project_clipmap_shell(
                &mut block,
                &mut transform,
                &mut visibility,
                frame,
                view_stamp.projection_eye,
                view_stamp.presentation_origin,
                metre_to_view_f64,
                metre_to_view,
            );
        }

        scratch.pending_entities = pending_entities;
        scratch.last_projection_epoch = registry.projection_epoch;
        return;
    }

    let _full_span = bevy::log::info_span!("celestial_clipmap.transform_full").entered();

    scratch.pending_entities.clear();
    std::mem::swap(
        &mut scratch.pending_entities,
        &mut registry.projection_pending,
    );
    scratch.pending_entities.clear();

    scratch.frames.clear();
    scratch.counts.clear();
    scratch.binary_primary.clear();
    scratch.projected_committed.clear();
    scratch.visible_levels.clear();
    scratch.live_authorities.clear();
    scratch
        .live_authorities
        .extend(registry.plans.keys().copied());
    for cells in scratch.visible_by_authority.values_mut() {
        cells.clear();
    }

    {
        let _span = bevy::log::info_span!("celestial_clipmap.transform_authority_frames").entered();

        for &authority in registry.plans.keys() {
            let Ok((body_origin, body_frame, _, _)) = authorities.get(authority) else {
                continue;
            };
            let Ok(relative_metres) = body_origin.relative_at_scale_bounded_f64(
                view.anchor(),
                SpatialScale::ZERO,
                f64::MAX,
            ) else {
                continue;
            };
            let orientation = body_frame.orientation();
            let rotation = Quat::from_xyzw(
                orientation.x as f32,
                orientation.y as f32,
                orientation.z as f32,
                orientation.w as f32,
            )
            .normalize();

            scratch.frames.insert(
                authority,
                ClipmapProjectedAuthorityFrame {
                    relative_metres,
                    orientation,
                    rotation,
                },
            );
            scratch.counts.insert(authority, (0, 0));
        }
    }

    let mut projected_any = false;

    {
        let _span = bevy::log::info_span!("celestial_clipmap.transform_active_shells").entered();

        for &entity in registry.active_entities.values() {
            let Ok((mut block, mut transform, mut visibility, mut material)) =
                blocks.get_mut(entity)
            else {
                continue;
            };
            if !block.active {
                continue;
            }

            let Some(frame) = scratch.frames.get(&block.authority).copied() else {
                block.projection_ready = false;
                *visibility = Visibility::Hidden;
                continue;
            };
            let Some(plan) = registry.plans.get(&block.authority) else {
                block.projection_ready = false;
                *visibility = Visibility::Hidden;
                continue;
            };

            let relative_level = block
                .spec
                .key
                .resolution
                .binary_exponent()
                .saturating_sub(plan.key.finest_exponent);
            if block.material_relative_level != relative_level {
                if let Ok((_, _, _, policy)) = authorities.get(block.authority) {
                    let standard_materials = &material_params.standard_materials;
                    let debug_grid = &material_params.library.debug_grid;
                    let render_materials = &mut material_params.render_materials;
                    let shader_buffers = &mut material_params.shader_buffers;
                    let band_materials = &mut material_params.band_materials;
                    if let Some(desired) = band_materials.material_for(
                        standard_materials,
                        render_materials,
                        shader_buffers,
                        debug_grid,
                        policy.presentation_material(),
                        relative_level,
                    ) {
                        material.0 = desired;
                        block.material_relative_level = relative_level;
                    }
                }
            }

            if block.committed {
                let counts = scratch.counts.entry(block.authority).or_default();
                counts.0 = counts.0.saturating_add(1);
            }

            if project_clipmap_shell(
                &mut block,
                &mut transform,
                &mut visibility,
                frame,
                view_stamp.projection_eye,
                view_stamp.presentation_origin,
                metre_to_view_f64,
                metre_to_view,
            ) {
                projected_any = true;
                if block.committed {
                    let counts = scratch.counts.entry(block.authority).or_default();
                    counts.1 = counts.1.saturating_add(1);
                    scratch.projected_committed.push(entity);
                }
            }
        }
    }

    {
        let CelestialClipmapTransformScratch {
            counts,
            binary_primary,
            ..
        } = &mut *scratch;

        for (&authority, &(expected, projected)) in counts.iter() {
            if binary_frontier_projection_complete(expected, projected) {
                binary_primary.insert(authority);
            }
        }
    }
    let binary_primary_count = scratch.binary_primary.len();

    let mut visible_blocks = 0usize;
    let mut finest_visible_spacing = None::<f64>;
    let mut coarsest_visible_spacing = None::<f64>;

    {
        let _span =
            bevy::log::info_span!("celestial_clipmap.transform_visibility_coverage").entered();

        let CelestialClipmapTransformScratch {
            projected_committed,
            binary_primary,
            visible_levels,
            visible_by_authority,
            ..
        } = &mut *scratch;

        for &entity in projected_committed.iter() {
            let Ok((block, _, mut visibility, _)) = blocks.get_mut(entity) else {
                continue;
            };
            let visible = binary_primary.contains(&block.authority);
            *visibility = if visible {
                Visibility::Inherited
            } else {
                Visibility::Hidden
            };
            if !visible {
                continue;
            }

            let spacing = block.spec.key.spacing_metres();
            visible_blocks = visible_blocks.saturating_add(1);
            visible_levels.insert(block.spec.key.resolution.binary_exponent());
            finest_visible_spacing =
                Some(finest_visible_spacing.map_or(spacing, |value| value.min(spacing)));
            coarsest_visible_spacing =
                Some(coarsest_visible_spacing.map_or(spacing, |value| value.max(spacing)));
            visible_by_authority
                .entry(block.authority)
                .or_default()
                .push(CelestialClipmapCoverageCell {
                    center_local_metres: block.spec.key.center_local_metres(),
                    half_extent_metres: block.spec.key.half_extent_metres(),
                    sample_spacing_metres: spacing,
                });
        }
    }

    {
        let _span = bevy::log::info_span!("celestial_clipmap.transform_publish_coverage").entered();

        coverage.retain_authorities(&scratch.live_authorities);
        for &authority in &scratch.live_authorities {
            let next = scratch
                .visible_by_authority
                .get(&authority)
                .map(Vec::as_slice)
                .unwrap_or(&[]);
            coverage.replace_authority_from_slice(authority, next);
        }

        presentation_state.replace_binary_primary_from(&scratch.binary_primary);
        telemetry.record_visible_frontier(
            visible_blocks,
            binary_primary_count,
            &scratch.visible_levels,
            finest_visible_spacing,
            coarsest_visible_spacing,
        );
    }

    scratch.last_view_stamp = Some(view_stamp);
    scratch.last_projection_epoch = registry.projection_epoch;
    scratch.last_frontier_epoch = registry.frontier_epoch;
    scratch.initialized = true;

    if projected_any && !*logged_projection {
        info!(
            view_scale = %view.scale(),
            view_exponent = view.continuous_exponent(),
            metre_to_view,
            visible_binary_blocks = visible_blocks,
            binary_primary_authorities = binary_primary_count,
            visible_binary_levels = scratch.visible_levels.len(),
            "celestial binary presentation owns complete authority-level frontiers"
        );
        *logged_projection = true;
    }
}

/// Dense physical/current-interaction presentation is a bootstrap/emergency
/// fallback only. The interaction Scale chooses which physical dense cache is
/// available; it does not choose visual LOD. Once a complete binary frontier
/// owns the authority, all dense visual chunks yield together.
pub(super) fn enforce_dense_interaction_presentation(
    mut commands: Commands,
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    view_demands: Res<UsfViewDemandSnapshot>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    presentation_state: Res<CelestialTerrainPresentationState>,
    runtimes: Query<&VoxelMaterializationRuntime>,
    worlds: Query<(
        &CelestialVoxelRealization,
        &UsfScaleLayer,
        &VoxelWorld,
        Option<&VoxelStreaming>,
    )>,
    mut presentations: Query<(&ChildOf, &mut Visibility), With<VoxelMaterializationPresentation>>,
    mut telemetry: ResMut<CelestialClipmapTelemetry>,
) {
    let primary_view_demand = view_demands.iter().next();
    let mut fallback_held = 0usize;
    let mut fallback_retire_ready = 0usize;
    let mut fallback_forced_retire = 0usize;

    for (parent, mut visibility) in &mut presentations {
        let Ok(runtime) = runtimes.get(parent.0) else {
            continue;
        };
        let Ok((realization, layer, world, streaming)) = worlds.get(runtime.world()) else {
            continue;
        };

        let target_scale = interaction.target_scale();

        let center = world
            .materialization_address(runtime.key())
            .ok()
            .and_then(|address| address.center().ok());

        let presentation_requested = streaming.is_none_or(|streaming| {
            streaming
                .effective_roles(runtime.key())
                .contains(UsfScaleRoleMask::PRESENTATION)
        });

        if layer.scale() != target_scale || !runtime.active() {
            *visibility = Visibility::Hidden;
            commands
                .entity(parent.0)
                .insert(VoxelPresentationFallbackRetireReady);
            fallback_retire_ready = fallback_retire_ready.saturating_add(1);
            continue;
        }

        // Whole-authority handoff: dense never clips or fills individual binary
        // blocks. It is either the fallback renderer for this body or not.
        if presentation_state.is_binary_primary(realization.authority()) {
            *visibility = Visibility::Hidden;
            if presentation_requested {
                commands
                    .entity(parent.0)
                    .remove::<VoxelPresentationFallbackRetireReady>();
            } else {
                commands
                    .entity(parent.0)
                    .insert(VoxelPresentationFallbackRetireReady);
                fallback_retire_ready = fallback_retire_ready.saturating_add(1);
            }
            continue;
        }

        let view_relevant = center.is_some_and(|center| {
            primary_view_demand.is_none_or(|demand| {
                demand.intersects_presentation_native_aabb(
                    layer.scale(),
                    &center,
                    Vec3::splat(MATERIALIZATION_CHUNK_SIZE as f32 * 0.5),
                )
            })
        });

        let fallback_frontier_local = center.is_some_and(|center| {
            let retention_native =
                MATERIALIZATION_CHUNK_SIZE as f32 * DENSE_FALLBACK_RETENTION_CHUNKS;
            center
                .relative_at_scale_bounded(
                    &view.anchor(),
                    layer.scale(),
                    retention_native + MATERIALIZATION_CHUNK_SIZE as f32 * 2.0,
                )
                .ok()
                .is_some_and(|relative| relative.length() <= retention_native)
        });

        if presentation_requested {
            // Active physical/view demand keeps the cache reusable. Until the
            // binary authority-level transaction succeeds, dense is the visible
            // bootstrap representation rather than one member of a mixed LOD.
            commands
                .entity(parent.0)
                .remove::<VoxelPresentationFallbackRetireReady>();
            *visibility = Visibility::Inherited;
            continue;
        }

        // Historical dense presentation is not a cache of old visual LODs.
        // Retain only a short local bridge while binary authority is absent.
        let forced_by_frontier = view_relevant && !fallback_frontier_local;

        if !view_relevant || !fallback_frontier_local {
            *visibility = Visibility::Hidden;
            commands
                .entity(parent.0)
                .insert(VoxelPresentationFallbackRetireReady);
            fallback_retire_ready = fallback_retire_ready.saturating_add(1);
            if forced_by_frontier {
                fallback_forced_retire = fallback_forced_retire.saturating_add(1);
            }
        } else {
            *visibility = Visibility::Inherited;
            commands
                .entity(parent.0)
                .remove::<VoxelPresentationFallbackRetireReady>();
            fallback_held = fallback_held.saturating_add(1);
        }
    }

    telemetry.record_dense_fallbacks(fallback_held, fallback_retire_ready, fallback_forced_retire);
}
