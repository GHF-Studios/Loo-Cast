//! Physics-backend and gravity-field observability.

use avian3d::{
    debug_render::{PhysicsDebugPlugin, PhysicsGizmos},
    prelude::Gravity,
};
use bevy::prelude::*;

use crate::observability::{
    AppObservabilityExt, DebugColorRamp, DebugContext, DebugControlSpec, DebugControls,
    DebugFrame, DebugFrameBatch, DebugId, DebugScalarRange, DebugVectorField, DebugVectorSpace,
    ObservabilitySet, CATEGORY_PHYSICS,
};

const AVIAN_TOOL: DebugId = DebugId("physics.avian");
const AVIAN_GEOMETRY: DebugId = DebugId("physics.avian.geometry");
const AVIAN_CONTACTS: DebugId = DebugId("physics.avian.contacts");
const AVIAN_QUERIES: DebugId = DebugId("physics.avian.queries");
const AVIAN_STRUCTURE: DebugId = DebugId("physics.avian.structure");
const AXES: DebugId = DebugId("physics.backend.axes");
const AABBS: DebugId = DebugId("physics.backend.aabbs");
const COLLIDERS: DebugId = DebugId("physics.backend.colliders");
const CONTACT_POINTS: DebugId = DebugId("physics.backend.contact_points");
const CONTACT_NORMALS: DebugId = DebugId("physics.backend.contact_normals");
const JOINTS: DebugId = DebugId("physics.backend.joints");
const RAYCASTS: DebugId = DebugId("physics.backend.raycasts");
const SHAPECASTS: DebugId = DebugId("physics.backend.shapecasts");
const ISLANDS: DebugId = DebugId("physics.backend.islands");
const COLLIDER_TREE: DebugId = DebugId("physics.backend.collider_tree");
const HIDE_MESHES: DebugId = DebugId("physics.backend.hide_meshes");

const GRAVITY_FIELD: DebugId = DebugId("physics.gravity_field");
const GRAVITY_SAMPLING: DebugId = DebugId("physics.gravity_field.sampling");
const GRAVITY_RENDERING: DebugId = DebugId("physics.gravity_field.rendering");
const GRAVITY_SIZE: DebugId = DebugId("physics.gravity_field.size");
const GRAVITY_RESOLUTION: DebugId = DebugId("physics.gravity_field.resolution");
const GRAVITY_SCALE: DebugId = DebugId("physics.gravity_field.arrow_scale");

const GRAVITY_OBSERVATION: DebugId = DebugId("observation.physics.gravity");

pub(crate) fn configure(app: &mut App) {
    app.register_debug_control(
        DebugControlSpec::tool(
            AVIAN_TOOL,
            Some(CATEGORY_PHYSICS),
            "Avian physics",
            0,
            false,
        )
        .described("Avian colliders, contacts, queries, joints and solver diagnostics."),
    )
    .register_debug_control(DebugControlSpec::group(
        AVIAN_GEOMETRY,
        Some(AVIAN_TOOL),
        "Bodies / geometry",
        0,
    ))
    .register_debug_control(DebugControlSpec::group(
        AVIAN_CONTACTS,
        Some(AVIAN_TOOL),
        "Contacts / constraints",
        10,
    ))
    .register_debug_control(DebugControlSpec::group(
        AVIAN_QUERIES,
        Some(AVIAN_TOOL),
        "Spatial queries",
        20,
    ))
    .register_debug_control(DebugControlSpec::group(
        AVIAN_STRUCTURE,
        Some(AVIAN_TOOL),
        "Solver / broad phase",
        30,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        AXES,
        Some(AVIAN_GEOMETRY),
        "Rigid-body axes / COM",
        0,
        false,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        AABBS,
        Some(AVIAN_GEOMETRY),
        "AABBs",
        1,
        false,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        COLLIDERS,
        Some(AVIAN_GEOMETRY),
        "Colliders",
        2,
        true,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        CONTACT_POINTS,
        Some(AVIAN_CONTACTS),
        "Contact points",
        3,
        false,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        CONTACT_NORMALS,
        Some(AVIAN_CONTACTS),
        "Contact normals",
        4,
        false,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        JOINTS,
        Some(AVIAN_CONTACTS),
        "Joints",
        5,
        false,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        RAYCASTS,
        Some(AVIAN_QUERIES),
        "Ray casters",
        6,
        false,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        SHAPECASTS,
        Some(AVIAN_QUERIES),
        "Shape casters",
        7,
        false,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        ISLANDS,
        Some(AVIAN_STRUCTURE),
        "Simulation islands",
        8,
        false,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        COLLIDER_TREE,
        Some(AVIAN_STRUCTURE),
        "Collider tree",
        9,
        false,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        HIDE_MESHES,
        Some(AVIAN_GEOMETRY),
        "Hide ordinary meshes",
        10,
        false,
    ))
    .register_debug_control(
        DebugControlSpec::tool(
            GRAVITY_FIELD,
            Some(CATEGORY_PHYSICS),
            "Gravity vector field",
            5,
            false,
        )
        .described("Observer-centered XZ slice of the current global gravity vector."),
    )
    .register_debug_control(DebugControlSpec::group(
        GRAVITY_SAMPLING,
        Some(GRAVITY_FIELD),
        "Sampling",
        0,
    ))
    .register_debug_control(DebugControlSpec::group(
        GRAVITY_RENDERING,
        Some(GRAVITY_FIELD),
        "Rendering",
        10,
    ))
    .register_debug_control(DebugControlSpec::scalar(
        GRAVITY_SIZE,
        Some(GRAVITY_SAMPLING),
        "Field size",
        0,
        16.0,
        4.0,
        80.0,
        2.0,
        "m",
    ))
    .register_debug_control(DebugControlSpec::integer(
        GRAVITY_RESOLUTION,
        Some(GRAVITY_SAMPLING),
        "Resolution",
        1,
        9,
        3,
        33,
        2,
        "",
    ))
    .register_debug_control(DebugControlSpec::scalar(
        GRAVITY_SCALE,
        Some(GRAVITY_RENDERING),
        "Arrow scale",
        2,
        0.12,
        0.02,
        0.50,
        0.02,
        "",
    ))
    .add_plugins(PhysicsDebugPlugin)
    .add_systems(
        PreUpdate,
        sync_avian_debug.after(ObservabilitySet::Control),
    )
    .add_systems(
        PostUpdate,
        collect_gravity_field.in_set(ObservabilitySet::Collect),
    );
}

fn sync_avian_debug(
    controls: Res<DebugControls>,
    mut store: ResMut<GizmoConfigStore>,
) {
    let (gizmo_config, physics) = store.config_mut::<PhysicsGizmos>();
    gizmo_config.enabled = controls.active(AVIAN_TOOL);

    if !gizmo_config.enabled {
        return;
    }

    let mut desired = PhysicsGizmos::all();

    if !controls.active(AXES) {
        desired.axis_lengths = None;
    }
    if !controls.active(AABBS) {
        desired.aabb_color = None;
    }
    if !controls.active(COLLIDERS) {
        desired.collider_color = None;
    }
    if !controls.active(CONTACT_POINTS) {
        desired.contact_point_color = None;
    }
    if !controls.active(CONTACT_NORMALS) {
        desired.contact_normal_color = None;
    }
    if !controls.active(JOINTS) {
        desired.joint_anchor_color = None;
        desired.joint_separation_color = None;
    }
    if !controls.active(RAYCASTS) {
        desired.raycast_color = None;
        desired.raycast_point_color = None;
        desired.raycast_normal_color = None;
    }
    if !controls.active(SHAPECASTS) {
        desired.shapecast_color = None;
        desired.shapecast_shape_color = None;
        desired.shapecast_point_color = None;
        desired.shapecast_normal_color = None;
    }
    if !controls.active(ISLANDS) {
        desired.island_color = None;
    }
    if !controls.active(COLLIDER_TREE) {
        desired.collider_tree_color = None;
    }
    desired.hide_meshes = controls.active(HIDE_MESHES);

    *physics = desired;
}

fn collect_gravity_field(
    controls: Res<DebugControls>,
    context: Res<DebugContext>,
    gravity: Res<Gravity>,
    transforms: Query<&GlobalTransform>,
    frame: Res<DebugFrame>,
) {
    if !controls.active(GRAVITY_FIELD) {
        return;
    }

    let Some(observer) = context
        .observer
        .and_then(|entity| transforms.get(entity).ok())
        .map(|global| global.translation())
    else {
        return;
    };

    let size = controls.scalar_value(GRAVITY_SIZE).unwrap_or(16.0);
    let resolution = controls.integer_value(GRAVITY_RESOLUTION).unwrap_or(9);
    let resolution = UVec2::splat(resolution.max(2));
    let gravity_vector = gravity.0;

    let mut batch = DebugFrameBatch::default();
    batch.vector_field(DebugVectorField {
        id: GRAVITY_OBSERVATION,
        transform: Transform {
            translation: Vec3::new(observer.x, 0.06, observer.z),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
            ..default()
        },
        size: Vec2::splat(size),
        resolution,
        vectors: vec![gravity_vector; (resolution.x * resolution.y) as usize],
        vector_space: DebugVectorSpace::World,
        vector_scale: controls.scalar_value(GRAVITY_SCALE).unwrap_or(0.12),
        maximum_arrow_length: 1.5,
        color: Color::WHITE,
        magnitude_range: Some(DebugScalarRange::new(
            0.0,
            gravity_vector.length().max(1.0),
        )),
        magnitude_ramp: DebugColorRamp::METRIC,
        title: "Gravity".to_owned(),
        unit: "m/s^2",
        legend: true,
    });
    frame.submit(batch);
}
