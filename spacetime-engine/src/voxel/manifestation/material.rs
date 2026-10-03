//! Voxel presentation materials and coarse/fine refinement clipping.
//!
//! Voxel geometry remains ordinary StandardMaterial PBR. The extension adds one
//! presentation-only operation: fragments covered by a ready immediately-finer
//! USF presentation aperture are discarded so refinement is make-before-break.
//! Exposed parent/child frontier faces retain a narrow coarse support band while
//! true transition geometry is still being proven.
//!
//! The clip list is a GPU storage buffer because one coarse realization can be
//! refined by an arbitrary sparse set of fine materializations. Empty fine
//! materializations still contribute aperture coverage even when they have no
//! triangles: "known empty" must remove a counterfeit coarse approximation.

use std::collections::HashMap;

use bevy::{
    asset::{load_internal_asset, uuid_handle},
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    reflect::TypePath,
    render::{render_resource::AsBindGroup, storage::ShaderBuffer},
    shader::{Shader, ShaderRef},
};

use crate::{
    procedural_assets::{
        DEBUG_GRID_BASE_UV_METRES_PER_UNIT, ProceduralAssetLibrary,
    },
    ecs::{UsfAuthorityPartitionOf, UsfLogicalRealizationOf},
    spatial::{
        SpatialScale, UsfCapabilityRealization, UsfCapabilitySet,
        UsfPosition, UsfPresentationProbe, UsfPrimaryInteractionSlice, UsfScaleLayer,
        UsfScaleRoleMask, UsfSpatialSet,
        UsfViewContext, UsfViewRenderAnchor,
    },
};

use super::{
    VoxelMaterializationRuntime,
    frontier::{
        VoxelRefinementFrontierSet, VoxelRefinementFrontierSnapshot,
    },
};
use super::super::{VoxelMaterializationKey, VoxelPostUpdateSet, VoxelWorld};

/// Coarse parent support retained inward from an exposed fine frontier face.
///
/// One fine native unit is one fine sampling cell. This is deliberately narrow:
/// enough to hide the hard clip crack without turning coarse overlap back into
/// the default presentation policy.
const REFINEMENT_SUPPORT_BAND_FINE_NATIVE: f32 = 1.0;

pub(super) const REFINEMENT_CLIP_SHADER: Handle<Shader> =
    uuid_handle!("04b0bd90-e3e3-4bc1-87d6-87563111dc8d");

pub(in crate::voxel) type VoxelRenderMaterial =
    ExtendedMaterial<StandardMaterial, VoxelRefinementClipExtension>;

#[derive(Debug, Clone, Copy)]
struct RefinementClipSource {
    world: Entity,
    authority: Entity,
    fine_scale: SpatialScale,
    key: VoxelMaterializationKey,
    center: UsfPosition,
    half_extent_native: Vec3,
    exposed_faces: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct RefinementClipMergeKey {
    world: Entity,
    authority: Entity,
    fine_scale: SpatialScale,
    exposed_faces: u8,
}

#[derive(Default)]
struct RefinementClipCache {
    source_coverage_revision: Option<u64>,
    sources: Vec<RefinementClipSource>,
}

/// Exact box compaction for a lattice of equally-sized refinement cells.
///
/// Cells are only merged when authority/world/Scale/frontier-mask all match.
/// Therefore the union of clip volume remains identical while the shader sees
/// far fewer boxes. In particular, large interior regions (mask=0) collapse to
/// a handful of maximal cuboids instead of one box per materialization.
fn compact_refinement_clip_sources(
    sources: Vec<RefinementClipSource>,
) -> Vec<RefinementClipSource> {
    let mut groups =
        HashMap::<RefinementClipMergeKey, HashMap<[i64; 3], RefinementClipSource>>::new();

    for source in sources {
        groups
            .entry(RefinementClipMergeKey {
                world: source.world,
                authority: source.authority,
                fine_scale: source.fine_scale,
                exposed_faces: source.exposed_faces,
            })
            .or_default()
            .insert(source.key.components(), source);
    }

    let mut compacted = Vec::<RefinementClipSource>::new();

    for (_group, mut cells) in groups {
        while !cells.is_empty() {
            let start = *cells
                .keys()
                .min()
                .expect("non-empty refinement clip group");
            let seed = *cells
                .get(&start)
                .expect("selected refinement clip seed exists");

            let mut max_x = start[0];
            while max_x
                .checked_add(1)
                .is_some_and(|next| cells.contains_key(&[next, start[1], start[2]]))
            {
                max_x += 1;
            }

            let mut max_y = start[1];
            loop {
                let Some(next_y) = max_y.checked_add(1) else {
                    break;
                };
                if (start[0]..=max_x)
                    .all(|x| cells.contains_key(&[x, next_y, start[2]]))
                {
                    max_y = next_y;
                } else {
                    break;
                }
            }

            let mut max_z = start[2];
            'expand_z: loop {
                let Some(next_z) = max_z.checked_add(1) else {
                    break;
                };
                for y in start[1]..=max_y {
                    for x in start[0]..=max_x {
                        if !cells.contains_key(&[x, y, next_z]) {
                            break 'expand_z;
                        }
                    }
                }
                max_z = next_z;
            }

            let nx = max_x - start[0] + 1;
            let ny = max_y - start[1] + 1;
            let nz = max_z - start[2] + 1;

            let step = seed.half_extent_native * 2.0;
            let offset = Vec3::new(
                (nx - 1) as f32 * step.x * 0.5,
                (ny - 1) as f32 * step.y * 0.5,
                (nz - 1) as f32 * step.z * 0.5,
            );
            let center = seed
                .center
                .translated_at_scale(seed.fine_scale, offset)
                .expect("bounded refinement clip merge offset");
            let half_extent_native = Vec3::new(
                nx as f32 * seed.half_extent_native.x,
                ny as f32 * seed.half_extent_native.y,
                nz as f32 * seed.half_extent_native.z,
            );

            for z in start[2]..=max_z {
                for y in start[1]..=max_y {
                    for x in start[0]..=max_x {
                        cells.remove(&[x, y, z]);
                    }
                }
            }

            compacted.push(RefinementClipSource {
                center,
                half_extent_native,
                ..seed
            });
        }
    }

    compacted.sort_unstable_by_key(|source| {
        (
            source.authority.to_bits(),
            source.fine_scale.exponent(),
            source.world.to_bits(),
            source.exposed_faces,
            source.key.components(),
        )
    });
    compacted
}

// analytical-procedural-debug-grid-v1
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(in crate::voxel) struct VoxelRefinementClipExtension {
    #[storage(100, read_only)]
    clip_boxes: Handle<ShaderBuffer>,
    #[uniform(101)]
    clip_meta: UVec4,
    /// x: analytical-grid enabled (0/1)
    /// y: physical metres represented by one incoming UV unit
    /// z/w: reserved for future procedural-asset recipe parameters
    #[uniform(102)]
    debug_grid_meta: Vec4,
}

impl MaterialExtension for VoxelRefinementClipExtension {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(REFINEMENT_CLIP_SHADER.clone())
    }

    fn deferred_fragment_shader() -> ShaderRef {
        ShaderRef::Handle(REFINEMENT_CLIP_SHADER.clone())
    }
}


fn debug_grid_meta(uv_metres_per_unit: Option<f32>) -> Vec4 {
    let Some(uv_metres_per_unit) = uv_metres_per_unit
        .filter(|value| value.is_finite() && *value > 0.0)
    else {
        return Vec4::ZERO;
    };
    Vec4::new(1.0, uv_metres_per_unit, 0.0, 0.0)
}

fn strip_raster_debug_grid(base: &mut StandardMaterial, enabled: bool) {
    if enabled {
        // The development grid is analytical. A stale bitmap must never be
        // multiplied underneath it, otherwise minification aliases return.
        base.base_color_texture = None;
    }
}

/// Creates one ordinary voxel render material with no active refinement clips.
///
/// Binary presentation uses the same material/shader contract as dense voxel
/// presentation rather than maintaining a second dev-texture implementation.
pub(in crate::voxel) fn create_voxel_render_material(
    mut base: StandardMaterial,
    debug_grid_uv_metres_per_unit: Option<f32>,
    buffers: &mut Assets<ShaderBuffer>,
    materials: &mut Assets<VoxelRenderMaterial>,
) -> Handle<VoxelRenderMaterial> {
    let clip_boxes = buffers.add(ShaderBuffer::from(vec![
        [0.0_f32; 4],
        [0.0_f32; 4],
    ]));
    let grid_meta = debug_grid_meta(debug_grid_uv_metres_per_unit);
    strip_raster_debug_grid(&mut base, grid_meta.x > 0.5);

    materials.add(ExtendedMaterial {
        base,
        extension: VoxelRefinementClipExtension {
            clip_boxes,
            clip_meta: UVec4::ZERO,
            debug_grid_meta: grid_meta,
        },
    })
}

/// Authored voxel material plus its realization-local GPU presentation state.
///
/// The public constructor still accepts an ordinary [`StandardMaterial`] handle.
/// Each active Scale Slice lazily derives its own extended material because its
/// refinement aperture buffer is scale/authority specific.
#[derive(Component, Debug, Clone)]
pub struct VoxelPresentationMaterial {
    base: Handle<StandardMaterial>,
    opaque: Option<Handle<VoxelRenderMaterial>>,
    translucent: Option<Handle<VoxelRenderMaterial>>,
    clip_boxes: Option<Handle<ShaderBuffer>>,
    clip_data: Vec<[f32; 4]>,
    clip_count: u32,
}

impl VoxelPresentationMaterial {
    pub fn new(base: Handle<StandardMaterial>) -> Self {
        Self {
            base,
            opaque: None,
            translucent: None,
            clip_boxes: None,
            clip_data: Vec::new(),
            clip_count: 0,
        }
    }

    pub(super) fn handles(
        &self,
    ) -> Option<(
        &Handle<VoxelRenderMaterial>,
        &Handle<VoxelRenderMaterial>,
    )> {
        Some((self.opaque.as_ref()?, self.translucent.as_ref()?))
    }

    fn initialize(
        &mut self,
        mut base: StandardMaterial,
        debug_grid_uv_metres_per_unit: Option<f32>,
        buffers: &mut Assets<ShaderBuffer>,
        materials: &mut Assets<VoxelRenderMaterial>,
    ) {
        if self.opaque.is_some() {
            return;
        }

        // Keep a real buffer bound even when no aperture is active. clip_meta.x
        // is authoritative for length, so the dummy values are never read.
        let clip_boxes = buffers.add(ShaderBuffer::from(vec![
            [0.0_f32; 4],
            [0.0_f32; 4],
        ]));

        let grid_meta = debug_grid_meta(debug_grid_uv_metres_per_unit);
        strip_raster_debug_grid(&mut base, grid_meta.x > 0.5);

        let opaque = materials.add(ExtendedMaterial {
            base,
            extension: VoxelRefinementClipExtension {
                clip_boxes: clip_boxes.clone(),
                clip_meta: UVec4::ZERO,
                debug_grid_meta: grid_meta,
            },
        });

        let translucent = materials.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::WHITE,
                alpha_mode: AlphaMode::Blend,
                perceptual_roughness: 0.18,
                metallic: 0.0,
                double_sided: true,
                ..default()
            },
            extension: VoxelRefinementClipExtension {
                clip_boxes: clip_boxes.clone(),
                clip_meta: UVec4::ZERO,
                debug_grid_meta: Vec4::ZERO,
            },
        });

        self.opaque = Some(opaque);
        self.translucent = Some(translucent);
        self.clip_boxes = Some(clip_boxes);
        self.clip_data.clear();
    }

    fn set_clip_data(
        &mut self,
        mut data: Vec<[f32; 4]>,
        buffers: &mut Assets<ShaderBuffer>,
        materials: &mut Assets<VoxelRenderMaterial>,
    ) {
        let Some(buffer_handle) = self.clip_boxes.as_ref() else {
            return;
        };
        let Some(opaque_handle) = self.opaque.as_ref() else {
            return;
        };
        let Some(translucent_handle) = self.translucent.as_ref() else {
            return;
        };

        let count = data.len() / 2;
        debug_assert_eq!(data.len() % 2, 0);

        if data.is_empty() {
            data.extend([[0.0; 4], [0.0; 4]]);
        }

        if self.clip_data == data {
            return;
        }

        let Some(mut buffer) = buffers.get_mut(buffer_handle) else {
            return;
        };
        buffer.set_data(data.clone());

        let count = u32::try_from(count).unwrap_or(u32::MAX);
        if self.clip_count != count {
            for handle in [opaque_handle, translucent_handle] {
                if let Some(mut material) = materials.get_mut(handle) {
                    material.extension.clip_meta = UVec4::new(count, 0, 0, 0);
                }
            }
            self.clip_count = count;
        }

        self.clip_data = data;
    }
}

fn initialize_voxel_presentation_materials(
    mut worlds: Query<(
        &VoxelWorld,
        &UsfScaleLayer,
        &mut VoxelPresentationMaterial,
    )>,
    library: Res<ProceduralAssetLibrary>,
    standard_materials: Res<Assets<StandardMaterial>>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
    mut materials: ResMut<Assets<VoxelRenderMaterial>>,
) {
    for (world, layer, mut material) in &mut worlds {
        if world.is_empty() || material.opaque.is_some() {
            continue;
        }
        let Some(base) = standard_materials.get(&material.base).cloned() else {
            continue;
        };

        // Dense Surface Nets UVs are 0.5 UV/native-unit. Convert that stable
        // historical coordinate to physical metres in the shader so the same
        // analytical asset means the same thing at every decimal Scale Slice.
        let debug_grid_uv_metres_per_unit =
            (material.base == library.debug_grid).then_some(
                DEBUG_GRID_BASE_UV_METRES_PER_UNIT
                    * layer.scale().metres_per_native() as f32,
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
    context_eligible: bool,
) -> bool {
    let physical = fine_scale == interaction_scale
        && view_scale == interaction_scale;

    if physical {
        return physical_enabled;
    }

    // Presentation refinement is observer-owned. A fine realization may be
    // visible contextually before physics interaction reaches that Scale.
    context_enabled && context_eligible
}

fn sync_refinement_clip_materials(
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    probe: Res<UsfPresentationProbe>,
    frontier: Res<VoxelRefinementFrontierSnapshot>,
    realizations: Query<(
        Entity,
        &VoxelMaterializationRuntime,
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
        let _span =
            bevy::log::info_span!("voxel.refinement_clip.topology").entered();

        let mut raw = Vec::<RefinementClipSource>::with_capacity(realizations.iter().len());
        for (_, runtime, realization) in &realizations {
            if !realization
                .roles()
                .contains(UsfScaleRoleMask::PRESENTATION)
            {
                continue;
            }

            raw.push(RefinementClipSource {
                world: runtime.world(),
                authority: realization.authority(),
                fine_scale: realization.scale(),
                key: runtime.key(),
                center: realization.center(),
                half_extent_native: realization.half_extent_native(),
                exposed_faces: frontier
                    .exposed_faces(
                        runtime.world(),
                        realization.authority(),
                        realization.scale(),
                        runtime.key(),
                    )
                    .bits(),
            });
        }

        let _compact_span =
            bevy::log::info_span!("voxel.refinement_clip.compact").entered();
        cache.sources = compact_refinement_clip_sources(raw);
        cache.source_coverage_revision = Some(source_revision);
    }

    for value in boxes.values_mut() {
        value.clear();
    }

    let projection_span =
        bevy::log::info_span!("voxel.refinement_clip.project").entered();

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
            view.context_scale_eligible(fine_scale),
        );
        if !fine_is_visible {
            continue;
        }

        let factor = view.projection_factor(fine_scale);
        if !factor.is_finite() || factor <= f32::EPSILON {
            continue;
        }

        let bound = 1_000_000.0_f32;
        let Ok(relative) = source.center.relative_at_scale_bounded(
            view.anchor(),
            fine_scale,
            bound,
        ) else {
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

        let entry = boxes
            .entry((source.authority, coarse_scale))
            .or_default();

        // w carries presentation-only transition metadata:
        // center.w = exposed-face bit mask, half.w = support-band width.
        entry.push([
            center.x,
            center.y,
            center.z,
            f32::from(exposed_faces),
        ]);
        entry.push([half.x, half.y, half.z, support_band]);
    }

    drop(projection_span);
    let _publish_span =
        bevy::log::info_span!("voxel.refinement_clip.publish").entered();

    for (layer, logical, mut material) in &mut worlds {
        let Some(_handles) = material.handles() else {
            continue;
        };

        // Only terrain that is actually rendered through the physical/local
        // projection path must avoid contextual clip coordinates. If the view
        // has refined past interaction, the interaction Scale is now a
        // contextual ancestor and must receive child aperture clipping too.
        if layer.scale() == interaction.scale()
            && view.scale() == interaction.scale()
        {
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

#[cfg(test)]
mod refinement_presentation_domain_tests {
    use super::*;

    #[test]
    fn view_can_present_refinement_finer_than_interaction() {
        let interaction = SpatialScale::new(5).unwrap();
        let view = SpatialScale::new(3).unwrap();
        let fine = SpatialScale::new(3).unwrap();

        assert!(refinement_source_is_presented(
            fine,
            interaction,
            view,
            true,
            true,
            true,
        ));
    }

    #[test]
    fn interaction_scale_is_physical_only_when_view_matches_it() {
        let interaction = SpatialScale::new(5).unwrap();

        assert!(refinement_source_is_presented(
            interaction,
            interaction,
            interaction,
            true,
            true,
            true,
        ));

        let finer_view = SpatialScale::new(3).unwrap();
        assert!(refinement_source_is_presented(
            interaction,
            interaction,
            finer_view,
            false,
            true,
            true,
        ));
    }
}

pub(super) fn configure(app: &mut App) {
    load_internal_asset!(
        app,
        REFINEMENT_CLIP_SHADER,
        "../shader/refinement_clip.wgsl",
        Shader::from_wgsl
    );

    app.add_plugins(MaterialPlugin::<VoxelRenderMaterial>::default())
        .add_systems(
            PostUpdate,
            initialize_voxel_presentation_materials
                .before(VoxelPostUpdateSet::Rebuild),
        )
        .add_systems(
            PostUpdate,
            sync_refinement_clip_materials
                .after(UsfCapabilitySet::ReconcileCoverage)
                .after(VoxelRefinementFrontierSet)
                .after(UsfSpatialSet::ViewAnchor)
                .before(UsfSpatialSet::ViewProjection),
        );
}
