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
    ecs::{UsfAuthorityPartitionOf, UsfLogicalRealizationOf},
    spatial::{
        SpatialScale, UsfCapabilityRealization, UsfCapabilitySet,
        UsfPresentationProbe, UsfPrimaryInteractionSlice, UsfScaleLayer,
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
use super::super::{VoxelPostUpdateSet, VoxelWorld};

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

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(super) struct VoxelRefinementClipExtension {
    #[storage(100, read_only)]
    clip_boxes: Handle<ShaderBuffer>,
    #[uniform(101)]
    clip_meta: UVec4,
}

impl MaterialExtension for VoxelRefinementClipExtension {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(REFINEMENT_CLIP_SHADER.clone())
    }

    fn deferred_fragment_shader() -> ShaderRef {
        ShaderRef::Handle(REFINEMENT_CLIP_SHADER.clone())
    }
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
}

impl VoxelPresentationMaterial {
    pub fn new(base: Handle<StandardMaterial>) -> Self {
        Self {
            base,
            opaque: None,
            translucent: None,
            clip_boxes: None,
            clip_data: Vec::new(),
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
        base: StandardMaterial,
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

        let extension = || VoxelRefinementClipExtension {
            clip_boxes: clip_boxes.clone(),
            clip_meta: UVec4::ZERO,
        };

        let opaque = materials.add(ExtendedMaterial {
            base,
            extension: extension(),
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
            extension: extension(),
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
        for handle in [opaque_handle, translucent_handle] {
            if let Some(mut material) = materials.get_mut(handle) {
                material.extension.clip_meta = UVec4::new(count, 0, 0, 0);
            }
        }

        self.clip_data = data;
    }
}

fn initialize_voxel_presentation_materials(
    mut worlds: Query<(&VoxelWorld, &mut VoxelPresentationMaterial)>,
    standard_materials: Res<Assets<StandardMaterial>>,
    mut buffers: ResMut<Assets<ShaderBuffer>>,
    mut materials: ResMut<Assets<VoxelRenderMaterial>>,
) {
    for (world, mut material) in &mut worlds {
        if world.is_empty() || material.opaque.is_some() {
            continue;
        }
        let Some(base) = standard_materials.get(&material.base).cloned() else {
            continue;
        };
        material.initialize(base, &mut buffers, &mut materials);
    }
}

/// Builds immediate-parent clip volumes from *realized* fine presentation
/// coverage and uploads them once per owning Scale Slice.
///
/// A clip box is expressed in the final USF projection render space. This keeps
/// the shader entirely presentation-local: it never needs canonical large-range
/// arithmetic or semantic authority.
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
) {
    let view = view.into_inner();

    for value in boxes.values_mut() {
        value.clear();
    }

    // Query iteration order is not an ownership contract. Sort by runtime entity
    // so unchanged aperture sets upload identical buffers across frames.
    let mut realized = realizations.iter().collect::<Vec<_>>();
    realized.sort_by_key(|(entity, _, _)| entity.to_bits());

    for (_, runtime, realization) in realized {
        if !realization
            .roles()
            .contains(UsfScaleRoleMask::PRESENTATION)
        {
            continue;
        }

        let fine_scale = realization.scale();
        let Some(coarse_raw) = fine_scale.exponent().checked_add(1) else {
            continue;
        };
        let Some(coarse_scale) = SpatialScale::new(coarse_raw) else {
            continue;
        };

        let fine_is_visible = if fine_scale == interaction.scale() {
            probe.physical_enabled()
        } else {
            probe.context_enabled() && view.context_scale_eligible(fine_scale)
        };
        if !fine_is_visible {
            continue;
        }

        let factor = view.projection_factor(fine_scale);
        if !factor.is_finite() || factor <= f32::EPSILON {
            continue;
        }

        let bound = 1_000_000.0_f32;
        let Ok(relative) = realization.center().relative_at_scale_bounded(
            view.anchor(),
            fine_scale,
            bound,
        ) else {
            continue;
        };

        let center = view.presentation_origin() + relative * factor;
        let half = realization.half_extent_native() * factor;
        if !center.is_finite() || !half.is_finite() {
            continue;
        }

        let exposed_faces = frontier
            .exposed_faces(
                runtime.world(),
                realization.authority(),
                fine_scale,
                runtime.key(),
            )
            .bits();
        let support_band = if exposed_faces == 0 {
            0.0
        } else {
            REFINEMENT_SUPPORT_BAND_FINE_NATIVE * factor
        };

        let entry = boxes
            .entry((realization.authority(), coarse_scale))
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

    for (layer, logical, mut material) in &mut worlds {
        let Some(_handles) = material.handles() else {
            continue;
        };

        // Physical interaction geometry belongs to the ordinary local camera and
        // must never be clipped in USF compressed-projection coordinates.
        if layer.scale() == interaction.scale() {
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
