//! Debug adapters for the underlying physics backend and engine-owned fields.

use avian3d::{
    debug_render::{PhysicsDebugPlugin, PhysicsGizmos},
    prelude::Gravity,
};
use bevy::prelude::*;

use crate::debug::{
    AppDebugExt, DebugCamera, DebugColorRamp, DebugScalarRange, DebugVectorField2d,
    DebugView, DebugViews,
};

struct PhysicsBackendDebugView;

impl DebugView for PhysicsBackendDebugView {
    const NAME: &'static str = "Physics / Avian";
    const DESCRIPTION: &'static str = "Avian colliders, contacts, bodies and backend geometry.";
}

struct GravityFieldDebugView;

impl DebugView for GravityFieldDebugView {
    const NAME: &'static str = "Physics / Gravity Field";
    const DESCRIPTION: &'static str =
        "Sample the current global gravity vector across a plane around the debug camera.";
    const ENABLED_BY_DEFAULT: bool = false;
}

#[derive(Component)]
struct GravityFieldEntity;

pub(crate) fn configure(app: &mut App) {
    app.register_debug_view::<PhysicsBackendDebugView>()
        .register_debug_view::<GravityFieldDebugView>()
        .add_plugins(PhysicsDebugPlugin)
        .add_systems(PreUpdate, sync_avian_debug_visibility)
        .add_systems(PostUpdate, update_gravity_field);
}

fn sync_avian_debug_visibility(
    views: Res<DebugViews>,
    mut store: ResMut<GizmoConfigStore>,
) {
    let (config, _) = store.config_mut::<PhysicsGizmos>();
    config.enabled = views.enabled::<PhysicsBackendDebugView>();
}

fn update_gravity_field(
    mut commands: Commands,
    views: Res<DebugViews>,
    gravity: Res<Gravity>,
    camera: Query<
        &Transform,
        (With<DebugCamera>, Without<GravityFieldEntity>),
    >,
    mut existing: Query<(Entity, &mut DebugVectorField2d, &mut Transform), With<GravityFieldEntity>>,
) {
    if !views.enabled::<GravityFieldDebugView>() {
        for (entity, _, _) in &mut existing {
            commands.entity(entity).despawn();
        }
        return;
    }

    let Some(camera) = camera.iter().next() else {
        return;
    };

    const SIZE: f32 = 16.0;
    const RESOLUTION: UVec2 = UVec2::new(9, 9);
    let gravity_vector = gravity.0;
    let field = DebugVectorField2d::new(
        Vec2::splat(SIZE),
        RESOLUTION,
        vec![gravity_vector; (RESOLUTION.x * RESOLUTION.y) as usize],
    )
    .with_vector_scale(0.12)
    .with_maximum_arrow_length(1.25)
    .with_magnitude_colors(
        DebugScalarRange::new(0.0, gravity_vector.length().max(1.0)),
        DebugColorRamp::METRIC,
    );
    let transform = Transform {
        translation: Vec3::new(camera.translation.x, 0.06, camera.translation.z),
        rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
        ..default()
    };

    if let Some((_, mut current_field, mut current_transform)) = existing.iter_mut().next() {
        *current_field = field;
        *current_transform = transform;
    } else {
        commands.spawn((
            Name::new("Debug Gravity Field"),
            GravityFieldEntity,
            field,
            transform,
        ));
    }
}
