//! Distance-driven mesh refinement for scale-authored scenery.
//!
//! LOD selection is evaluated in the representation's own native chart. It is
//! therefore independent from radial render compression and observer display
//! scale: a Moon knows the observer is ten lunar radii away even if its visual
//! manifestation is being projected into a bounded render shell.

use bevy::prelude::*;

use crate::spatial::{
    UsfActiveScaleLayer, UsfScaleLayerFrames, UsfSceneryPresentation, UsfViewContext, UsfViewRenderAnchor,
};

#[derive(Debug, Clone)]
struct UsfMeshLodLevel {
    max_distance_radii: f64,
    mesh: Handle<Mesh>,
    material: Option<Handle<StandardMaterial>>,
}

/// Selects one of several meshes for a scale-authored scenery manifestation.
///
/// `reference_radius_native` gives distance a semantic scale. Thresholds are
/// dimensionless "object radii away", which makes the same mechanism useful for
/// moons, planets, stars, asteroids, megastructures, etc.
#[derive(Component, Debug, Clone)]
pub struct UsfDistanceMeshLod {
    reference_radius_native: f64,
    levels: Vec<UsfMeshLodLevel>,
    current: Option<usize>,
}

impl UsfDistanceMeshLod {
    pub fn new(
        reference_radius_native: f64,
        levels: impl IntoIterator<Item = (f64, Handle<Mesh>)>,
    ) -> Self {
        let mut levels = levels
            .into_iter()
            .map(|(max_distance_radii, mesh)| UsfMeshLodLevel {
                max_distance_radii,
                mesh,
                material: None,
            })
            .collect::<Vec<_>>();

        levels.sort_by(|a, b| a.max_distance_radii.total_cmp(&b.max_distance_radii));

        assert!(
            reference_radius_native.is_finite() && reference_radius_native > 0.0,
            "mesh LOD reference radius must be finite and positive"
        );
        assert!(!levels.is_empty(), "mesh LOD needs at least one level");

        Self {
            reference_radius_native,
            levels,
            current: None,
        }
    }

    /// Variant for diagnostic/presentation LODs that want the selected
    /// detail level to be visually unmistakable as well as geometrically different.
    pub fn with_materials(
        reference_radius_native: f64,
        levels: impl IntoIterator<Item = (f64, Handle<Mesh>, Handle<StandardMaterial>)>,
    ) -> Self {
        let mut levels = levels
            .into_iter()
            .map(|(max_distance_radii, mesh, material)| UsfMeshLodLevel {
                max_distance_radii,
                mesh,
                material: Some(material),
            })
            .collect::<Vec<_>>();

        levels.sort_by(|a, b| a.max_distance_radii.total_cmp(&b.max_distance_radii));

        assert!(
            reference_radius_native.is_finite() && reference_radius_native > 0.0,
            "mesh LOD reference radius must be finite and positive"
        );
        assert!(!levels.is_empty(), "mesh LOD needs at least one level");

        Self {
            reference_radius_native,
            levels,
            current: None,
        }
    }

    fn level_for_distance(&self, distance_native: f64) -> usize {
        let distance_radii = distance_native / self.reference_radius_native;
        self.levels
            .iter()
            .position(|level| distance_radii <= level.max_distance_radii)
            .unwrap_or(self.levels.len() - 1)
    }
}

pub(in crate::spatial) fn select_distance_mesh_lods(
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    active: Res<UsfActiveScaleLayer>,
    frames: Res<UsfScaleLayerFrames>,
    mut presentations: Query<(
        &UsfSceneryPresentation,
        &mut UsfDistanceMeshLod,
        &mut Mesh3d,
        Option<&mut MeshMaterial3d<StandardMaterial>>,
    )>,
) {
    let active_scale = active.scale();
    let observer_absolute = frames.absolute(active_scale, view.runtime_anchor());

    for (presentation, mut lod, mut mesh, material) in &mut presentations {
        let observer_in_scale =
            frames.convert_absolute(observer_absolute, active_scale, presentation.scale());
        let distance_native = (presentation.absolute() - observer_in_scale).length();
        if !distance_native.is_finite() {
            continue;
        }

        let target = lod.level_for_distance(distance_native);
        if lod.current == Some(target) {
            continue;
        }

        mesh.0 = lod.levels[target].mesh.clone();
        if let (Some(level_material), Some(mut material)) =
            (&lod.levels[target].material, material)
        {
            material.0 = level_material.clone();
        }
        lod.current = Some(target);
    }
}
