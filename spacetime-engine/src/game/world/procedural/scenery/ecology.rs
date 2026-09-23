//! Surface decoration derived from the actual celestial-body field.
//!
//! Decorations never invent a parallel "terrain height". Their anchors are
//! sampled from the same [`CelestialVoxelField`] that produces render/collision
//! terrain, then stored canonically.

use bevy::prelude::*;

use crate::{
    ecs::UsfPresentationProjectionOf,
    spatial::{SpatialScale, UsfPosition, UsfSceneryPresentation},
    voxel::CelestialVoxelField,
    worldgen::EcologyState,
};

pub(super) fn north_pole_surface(
    center: UsfPosition,
    field: CelestialVoxelField,
) -> UsfPosition {
    let scale = SpatialScale::ZERO;
    let body = field.realization(scale);
    let radius = body.surface_radius_native(Vec3::Y);
    center
        .translated_at_scale(scale, Vec3::Y * radius)
        .expect("celestial north-pole surface must remain canonically addressable")
}

pub(super) fn spawn_surface_ecology(
    commands: &mut Commands,
    parent: Entity,
    surface_manifestation: Entity,
    center: UsfPosition,
    field: CelestialVoxelField,
    ecology: EcologyState,
    seed: u32,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let trunk_mesh = meshes.add(Cylinder::new(0.14, 3.0));
    let canopy_mesh = meshes.add(Sphere::new(1.15));
    let plant_mesh = meshes.add(Cylinder::new(0.035, 0.55));

    let trunk_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.22, 0.095, 0.035),
        perceptual_roughness: 0.96,
        ..default()
    });
    let canopy_material = materials.add(StandardMaterial {
        base_color: Color::srgb(
            0.055,
            0.24 + ecology.forest_affinity * 0.20,
            0.055 + ecology.productivity * 0.06,
        ),
        perceptual_roughness: 0.95,
        ..default()
    });
    let plant_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.30 + ecology.grass_affinity * 0.28, 0.075),
        perceptual_roughness: 0.98,
        ..default()
    });

    let body = field.realization(SpatialScale::ZERO);

    let tree_count = (8.0 + ecology.forest_affinity * 18.0).round() as u32;
    for index in 0..tree_count {
        let mut x = signed_unit(hash(seed ^ 0x71EE_0001 ^ index.wrapping_mul(17))) * 54.0;
        let z = signed_unit(hash(seed ^ 0x71EE_0002 ^ index.wrapping_mul(31))) * 54.0;
        if (x * x + z * z).sqrt() < 9.0 {
            x += 14.0;
        }
        let Some((surface, up, rotation)) = surface_anchor(center, body, x, z) else {
            continue;
        };

        spawn_projection(
            commands,
            parent,
            surface_manifestation,
            format!("Earth Surface Tree {index} Trunk"),
            surface
                .translated_at_scale(SpatialScale::ZERO, up * 1.5)
                .expect("tree trunk must remain canonical"),
            trunk_mesh.clone(),
            trunk_material.clone(),
            rotation,
        );
        spawn_projection(
            commands,
            parent,
            surface_manifestation,
            format!("Earth Surface Tree {index} Canopy"),
            surface
                .translated_at_scale(SpatialScale::ZERO, up * 3.25)
                .expect("tree canopy must remain canonical"),
            canopy_mesh.clone(),
            canopy_material.clone(),
            rotation,
        );
    }

    let plant_count = (12.0 + ecology.grass_affinity * 30.0).round() as u32;
    for index in 0..plant_count {
        let x = signed_unit(hash(seed ^ 0x61A5_0001 ^ index.wrapping_mul(43))) * 44.0;
        let z = signed_unit(hash(seed ^ 0x61A5_0002 ^ index.wrapping_mul(59))) * 44.0;
        let Some((surface, up, rotation)) = surface_anchor(center, body, x, z) else {
            continue;
        };

        spawn_projection(
            commands,
            parent,
            surface_manifestation,
            format!("Earth Surface Plant {index}"),
            surface
                .translated_at_scale(SpatialScale::ZERO, up * 0.275)
                .expect("plant must remain canonical"),
            plant_mesh.clone(),
            plant_material.clone(),
            rotation * Quat::from_rotation_y(signed_unit(hash(seed ^ index)) * 0.22),
        );
    }
}

fn surface_anchor(
    center: UsfPosition,
    body: crate::voxel::ProceduralCelestialBody,
    tangent_x_metres: f32,
    tangent_z_metres: f32,
) -> Option<(UsfPosition, Vec3, Quat)> {
    let radius = body.radius_native();
    let up = Vec3::new(tangent_x_metres, radius, tangent_z_metres).normalize_or_zero();
    if up == Vec3::ZERO {
        return None;
    }

    let surface_radius = body.surface_radius_native(up);
    let anchor = center
        .translated_at_scale(SpatialScale::ZERO, up * surface_radius)
        .ok()?;
    let rotation = Quat::from_rotation_arc(Vec3::Y, up);
    Some((anchor, up, rotation))
}

fn spawn_projection(
    commands: &mut Commands,
    parent: Entity,
    manifestation: Entity,
    name: String,
    anchor: UsfPosition,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    rotation: Quat,
) {
    commands.spawn((
        Name::new(name),
        ChildOf(parent),
        UsfPresentationProjectionOf(manifestation),
        UsfSceneryPresentation::from_anchor(anchor, SpatialScale::ZERO),
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_rotation(rotation),
        Visibility::Inherited,
    ));
}

fn hash(mut value: u32) -> u32 {
    value ^= value >> 16;
    value = value.wrapping_mul(0x7FEB_352D);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846C_A68B);
    value ^ (value >> 16)
}

fn signed_unit(value: u32) -> f32 {
    (value as f64 / u32::MAX as f64) as f32 * 2.0 - 1.0
}
