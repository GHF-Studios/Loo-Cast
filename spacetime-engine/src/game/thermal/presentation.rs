//! Derived presentation of combustion.

use std::collections::HashSet;

use bevy::{color::LinearRgba, prelude::*};

use crate::{
    ecs::UsfManifestationOf,
    game::GameSet,
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive},
};

use super::{Combustion, ThermalSpatialSample};

#[derive(Resource)]
struct FirePresentationAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

#[derive(Component, Debug, Clone, Copy)]
struct FlameVisual {
    semantic: Entity,
    manifestation: Entity,
    phase: f32,
}

pub(super) fn configure(app: &mut App) {
    app.add_systems(Startup, setup_fire_presentation).add_systems(
        Update,
        (sync_flame_visuals, animate_flame_visuals)
            .chain()
            .in_set(GameSet::Presentation),
    );
}

fn setup_fire_presentation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(FirePresentationAssets {
        mesh: meshes.add(Sphere::new(0.35)),
        material: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.18, 0.015),
            emissive: LinearRgba::rgb(18.0, 2.4, 0.12),
            emissive_exposure_weight: 0.0,
            perceptual_roughness: 1.0,
            unlit: true,
            ..default()
        }),
    });
}

fn sync_flame_visuals(
    mut commands: Commands,
    assets: Res<FirePresentationAssets>,
    combustions: Query<(), With<Combustion>>,
    manifestations: Query<
        (Entity, &UsfManifestationOf, &Transform),
        (
            With<ThermalSpatialSample>,
            Without<FlameVisual>,
            Or<(Without<SpatialSplitPeer>, With<SpatialSplitPeerActive>)>,
        ),
    >,
    visuals: Query<(Entity, &FlameVisual)>,
) {
    let existing: HashSet<Entity> = visuals
        .iter()
        .map(|(_, visual)| visual.manifestation)
        .collect();

    for (manifestation, relation, transform) in &manifestations {
        if existing.contains(&manifestation) || !combustions.contains(relation.0) {
            continue;
        }

        let phase = (manifestation.to_bits() as f32 * 0.000_001).fract()
            * std::f32::consts::TAU;

        commands.spawn((
            Name::new("Combustion Flame"),
            FlameVisual {
                semantic: relation.0,
                manifestation,
                phase,
            },
            Mesh3d(assets.mesh.clone()),
            MeshMaterial3d(assets.material.clone()),
            PointLight {
                color: Color::srgb(1.0, 0.32, 0.05),
                intensity: 900.0,
                range: 4.0,
                shadow_maps_enabled: false,
                ..default()
            },
            Transform {
                translation: transform.translation + Vec3::Y * 0.75,
                scale: Vec3::new(0.55, 1.15, 0.55),
                ..default()
            },
        ));
    }

    for (entity, visual) in &visuals {
        let valid = combustions.contains(visual.semantic)
            && manifestations.get(visual.manifestation).is_ok();
        if !valid {
            commands.entity(entity).despawn();
        }
    }
}

fn animate_flame_visuals(
    time: Res<Time>,
    manifestations: Query<&Transform, Without<FlameVisual>>,
    mut visuals: Query<(&FlameVisual, &mut Transform, &mut PointLight)>,
) {
    let t = time.elapsed_secs();

    for (visual, mut transform, mut light) in &mut visuals {
        let Ok(source) = manifestations.get(visual.manifestation) else {
            continue;
        };

        let fast = (t * 10.7 + visual.phase).sin();
        let slow = (t * 5.3 + visual.phase * 1.7).sin();
        let flicker = (0.88 + fast * 0.08 + slow * 0.04).max(0.65);

        transform.translation = source.translation
            + Vec3::new(slow * 0.035, 0.72 + fast * 0.06, fast * 0.025);
        transform.scale = Vec3::new(0.52, 1.12, 0.52) * flicker;
        light.intensity = 850.0 * (0.9 + 0.15 * fast);
    }
}
