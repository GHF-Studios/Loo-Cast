//! Scale-local GPU material state and shader buffer publication.

use bevy::{
    asset::uuid_handle,
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    reflect::TypePath,
    render::{render_resource::AsBindGroup, storage::ShaderBuffer},
    shader::{Shader, ShaderRef},
};

pub(super) const REFINEMENT_CLIP_SHADER: Handle<Shader> =
    uuid_handle!("04b0bd90-e3e3-4bc1-87d6-87563111dc8d");

pub(in crate::voxel) type VoxelRenderMaterial =
    ExtendedMaterial<StandardMaterial, VoxelRefinementClipExtension>;

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
    let Some(uv_metres_per_unit) =
        uv_metres_per_unit.filter(|value| value.is_finite() && *value > 0.0)
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
    pub(super) fn base(&self) -> &Handle<StandardMaterial> {
        &self.base
    }

    pub(super) fn is_initialized(&self) -> bool {
        self.opaque.is_some()
    }

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

    pub(in crate::voxel::manifestation) fn handles(
        &self,
    ) -> Option<(&Handle<VoxelRenderMaterial>, &Handle<VoxelRenderMaterial>)> {
        Some((self.opaque.as_ref()?, self.translucent.as_ref()?))
    }

    pub(super) fn initialize(
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
        let clip_boxes = buffers.add(ShaderBuffer::from(vec![[0.0_f32; 4], [0.0_f32; 4]]));

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

    pub(super) fn set_clip_data(
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
