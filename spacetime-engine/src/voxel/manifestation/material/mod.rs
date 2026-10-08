//! Voxel presentation material and fine-aperture clipping.
//!
//! Realized fine coverage is compacted exactly, projected through the current
//! view, then uploaded as disposable GPU material state. It has no collision
//! or semantic authority.

use super::frontier::VoxelRefinementFrontierSet;
use crate::spatial::{UsfCapabilitySet, UsfSpatialSet};
use crate::voxel::VoxelPostUpdateSet;
use bevy::{asset::load_internal_asset, pbr::MaterialPlugin, prelude::*, shader::Shader};

mod asset;
mod clip;
mod systems;

use asset::REFINEMENT_CLIP_SHADER;
pub use asset::VoxelPresentationMaterial;
pub(in crate::voxel) use asset::VoxelRenderMaterial;

pub(super) fn configure(app: &mut App) {
    load_internal_asset!(
        app,
        REFINEMENT_CLIP_SHADER,
        "../../shader/refinement_clip.wgsl",
        Shader::from_wgsl
    );
    app.add_plugins(MaterialPlugin::<VoxelRenderMaterial>::default())
        .add_systems(
            PostUpdate,
            systems::initialize_voxel_presentation_materials.before(VoxelPostUpdateSet::Rebuild),
        )
        .add_systems(
            PostUpdate,
            systems::sync_refinement_clip_materials
                .after(UsfCapabilitySet::ReconcileCoverage)
                .after(VoxelRefinementFrontierSet)
                .after(UsfSpatialSet::ViewAnchor)
                .before(UsfSpatialSet::ViewProjection),
        );
}
