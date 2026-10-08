//! Derive and publish view-local refinement clips from realized coverage.

use super::super::{VoxelPresentationManifestation, frontier::VoxelRefinementFrontierSnapshot};
use super::{
    asset::{VoxelPresentationMaterial, VoxelRenderMaterial},
    clip::{RefinementClipCache, RefinementClipSource, compact_refinement_clip_sources},
};
use crate::voxel::VoxelScaleRealization;
use crate::{
    ecs::{UsfAuthorityPartitionOf, UsfLogicalRealizationOf},
    procedural_assets::{DEBUG_GRID_BASE_UV_METRES_PER_UNIT, ProceduralPresentationAssets},
    spatial::{
        SpatialScale, UsfCapabilityRealization, UsfPresentationDomainProbe,
        UsfPrimaryInteractionSlice, UsfScaleLayer, UsfScaleRoleMask, UsfViewContext,
        UsfViewRenderAnchor,
    },
};
use bevy::{prelude::*, render::storage::ShaderBuffer};
use std::collections::HashMap;

const REFINEMENT_SUPPORT_BAND_FINE_NATIVE: f32 = 1.0;

pub(super) fn initialize_voxel_presentation_materials(
    mut worlds: Query<(
        &VoxelScaleRealization,
        &UsfScaleLayer,
        &mut VoxelPresentationMaterial,
    )>,
    library: Res<ProceduralPresentationAssets>,
    standard_materials: Res<Assets<StandardMaterial>>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
    mut materials: ResMut<Assets<VoxelRenderMaterial>>,
) {
    for (world, layer, mut material) in &mut worlds {
        if world.is_empty() || material.is_initialized() {
            continue;
        }
        let Some(base) = standard_materials.get(material.base()).cloned() else {
            continue;
        };

        // Dense Surface Nets UVs are 0.5 UV/native-unit. Convert that stable
        // historical coordinate to physical metres in the shader so the same
        // analytical asset means the same thing at every decimal Scale Slice.
        let debug_grid_uv_metres_per_unit = (material.base() == &library.debug_grid).then_some(
            DEBUG_GRID_BASE_UV_METRES_PER_UNIT * layer.scale().metres_per_native() as f32,
        );

        material.initialize(
            base,
            debug_grid_uv_metres_per_unit,
            &mut buffers,
            &mut materials,
        );
    }
}

/// Builds immediate-parent clip volumes from *realized* fine presentation
/// coverage and uploads them once per owning Scale Slice.
///
/// A clip box is expressed in the final USF projection render space. This keeps
/// the shader entirely presentation-local: it never needs canonical large-range
/// arithmetic or semantic authority.
fn refinement_source_is_presented(
    fine_scale: SpatialScale,
    interaction_scale: SpatialScale,
    view_scale: SpatialScale,
    physical_enabled: bool,
    context_enabled: bool,
) -> bool {
    let physical = fine_scale == interaction_scale && view_scale == interaction_scale;

    if physical {
        return physical_enabled;
    }

    // Presentation refinement is observer-owned. A fine realization may be
    // visible contextually before physics interaction reaches that Scale.
    context_enabled
}

pub(super) fn sync_refinement_clip_materials(
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    probe: Res<UsfPresentationDomainProbe>,
    frontier: Res<VoxelRefinementFrontierSnapshot>,
    realizations: Query<(
        Entity,
        &VoxelPresentationManifestation,
        &UsfCapabilityRealization,
    )>,
    authority_partitions: Query<&UsfAuthorityPartitionOf>,
    mut worlds: Query<(
        &UsfScaleLayer,
        &UsfLogicalRealizationOf,
        &mut VoxelPresentationMaterial,
    )>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
    mut materials: ResMut<Assets<VoxelRenderMaterial>>,
    mut boxes: Local<HashMap<(Entity, SpatialScale), Vec<[f32; 4]>>>,
    mut cache: Local<RefinementClipCache>,
) {
    let view = view.into_inner();

    let source_revision = frontier.source_coverage_revision();
    if cache.source_coverage_revision != Some(source_revision) {
        let _span = bevy::log::info_span!("voxel.refinement_clip.topology").entered();

        let mut raw = Vec::<RefinementClipSource>::with_capacity(realizations.iter().len());
        for (_, runtime, realization) in &realizations {
            if !realization.roles().contains(UsfScaleRoleMask::PRESENTATION) {
                continue;
            }

            raw.push(RefinementClipSource {
                world: runtime.realization(),
                authority: realization.authority(),
                fine_scale: realization.scale(),
                key: runtime.key(),
                center: realization.center(),
                half_extent_native: realization.half_extent_native(),
                exposed_faces: frontier
                    .exposed_faces(
                        runtime.realization(),
                        realization.authority(),
                        realization.scale(),
                        runtime.key(),
                    )
                    .bits(),
            });
        }

        let _compact_span = bevy::log::info_span!("voxel.refinement_clip.compact").entered();
        cache.sources = compact_refinement_clip_sources(raw);
        cache.source_coverage_revision = Some(source_revision);
    }

    for value in boxes.values_mut() {
        value.clear();
    }

    let projection_span = bevy::log::info_span!("voxel.refinement_clip.project").entered();

    for source in cache.sources.iter().copied() {
        let fine_scale = source.fine_scale;
        let Some(coarse_raw) = fine_scale.exponent().checked_add(1) else {
            continue;
        };
        let Some(coarse_scale) = SpatialScale::new(coarse_raw) else {
            continue;
        };

        let fine_is_visible = refinement_source_is_presented(
            fine_scale,
            interaction.scale(),
            view.scale(),
            probe.physical_enabled(),
            probe.context_enabled(),
        );
        if !fine_is_visible {
            continue;
        }

        let factor = view.projection_factor(fine_scale);
        if !factor.is_finite() || factor <= f32::EPSILON {
            continue;
        }

        let bound = 1_000_000.0_f32;
        let Ok(relative) =
            source
                .center
                .relative_at_scale_bounded(view.anchor(), fine_scale, bound)
        else {
            continue;
        };

        let center = view.presentation_origin() + relative * factor;
        let half = source.half_extent_native * factor;
        if !center.is_finite() || !half.is_finite() {
            continue;
        }

        let exposed_faces = source.exposed_faces;
        let support_band = if exposed_faces == 0 {
            0.0
        } else {
            REFINEMENT_SUPPORT_BAND_FINE_NATIVE * factor
        };

        let entry = boxes.entry((source.authority, coarse_scale)).or_default();

        // w carries presentation-only transition metadata:
        // center.w = exposed-face bit mask, half.w = support-band width.
        entry.push([center.x, center.y, center.z, f32::from(exposed_faces)]);
        entry.push([half.x, half.y, half.z, support_band]);
    }

    drop(projection_span);
    let _publish_span = bevy::log::info_span!("voxel.refinement_clip.publish").entered();

    for (layer, logical, mut material) in &mut worlds {
        let Some(_handles) = material.handles() else {
            continue;
        };

        // Only terrain that is actually rendered through the physical/local
        // projection path must avoid contextual clip coordinates. If the view
        // has refined past interaction, the interaction Scale is now a
        // contextual ancestor and must receive child aperture clipping too.
        if layer.scale() == interaction.scale() && view.scale() == interaction.scale() {
            material.set_clip_data(Vec::new(), &mut buffers, &mut materials);
            continue;
        }

        let Ok(partition) = authority_partitions.get(logical.0) else {
            material.set_clip_data(Vec::new(), &mut buffers, &mut materials);
            continue;
        };

        let data = boxes
            .get(&(partition.0, layer.scale()))
            .cloned()
            .unwrap_or_default();
        material.set_clip_data(data, &mut buffers, &mut materials);
    }
}
