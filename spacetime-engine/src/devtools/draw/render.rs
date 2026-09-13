use std::collections::{HashMap, HashSet};

use bevy::{
    asset::RenderAssetUsages,
    mesh::PrimitiveTopology,
    prelude::*,
};

use crate::observability::DebugArtifact;

use super::{
    DrawDepth, DrawId, ScalarFieldMode, VectorSpace, WorldDrawFrame, WorldPrimitive,
    WorldScalarField, WorldVectorField,
};
use crate::devtools::{DeveloperArtifact, DeveloperSet};

#[derive(Default, Reflect, GizmoConfigGroup)]
struct DeveloperWorldGizmos;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct DeveloperOverlayGizmos;

#[derive(Resource)]
struct WorldDrawAssets {
    scalar_material: Handle<StandardMaterial>,
}

struct ScalarFieldVisual {
    entity: Entity,
    mesh: Handle<Mesh>,
}

#[derive(Resource, Default)]
struct WorldDrawCache {
    scalar_fields: HashMap<DrawId, ScalarFieldVisual>,
}

pub(super) fn configure(app: &mut App) {
    app.init_gizmo_group::<DeveloperWorldGizmos>()
        .init_gizmo_group::<DeveloperOverlayGizmos>()
        .init_resource::<WorldDrawCache>()
        .add_systems(Startup, setup_world_draw_backend)
        .add_systems(
            PostUpdate,
            (render_primitives_and_vectors, sync_scalar_field_visuals)
                .in_set(DeveloperSet::RenderWorldDraw),
        );
}

fn setup_world_draw_backend(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut store: ResMut<GizmoConfigStore>,
) {
    let (world, _) = store.config_mut::<DeveloperWorldGizmos>();
    world.line.width = 2.0;

    let (overlay, _) = store.config_mut::<DeveloperOverlayGizmos>();
    overlay.line.width = 2.0;
    overlay.depth_bias = -1.0;

    commands.insert_resource(WorldDrawAssets {
        scalar_material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: None,
            ..default()
        }),
    });
}

fn render_primitives_and_vectors(
    frame: Res<WorldDrawFrame>,
    mut world: Gizmos<DeveloperWorldGizmos>,
    mut overlay: Gizmos<DeveloperOverlayGizmos>,
) {
    let frame = frame.read();

    for primitive in &frame.primitives {
        match primitive_depth(primitive) {
            DrawDepth::World => draw_world_primitive(&mut world, primitive),
            DrawDepth::Overlay => draw_overlay_primitive(&mut overlay, primitive),
        }
    }

    for field in &frame.vector_fields {
        draw_vector_field(&mut world, field);
    }
}

fn primitive_depth(primitive: &WorldPrimitive) -> DrawDepth {
    match primitive {
        WorldPrimitive::Line { depth, .. }
        | WorldPrimitive::Arrow { depth, .. }
        | WorldPrimitive::Axes { depth, .. }
        | WorldPrimitive::Rect { depth, .. }
        | WorldPrimitive::Cross { depth, .. }
        | WorldPrimitive::Sphere { depth, .. } => *depth,
    }
}

fn draw_world_primitive(gizmos: &mut Gizmos<DeveloperWorldGizmos>, primitive: &WorldPrimitive) {
    match primitive {
        WorldPrimitive::Line { start, end, color, .. } => gizmos.line(*start, *end, *color),
        WorldPrimitive::Arrow { start, end, color, .. } => {
            gizmos.arrow(*start, *end, *color);
        }
        WorldPrimitive::Axes { transform, length, .. } => gizmos.axes(*transform, *length),
        WorldPrimitive::Rect { isometry, size, color, .. } => {
            gizmos.rect(*isometry, *size, *color);
        }
        WorldPrimitive::Cross { isometry, size, color, .. } => {
            gizmos.cross(*isometry, *size, *color);
        }
        WorldPrimitive::Sphere {
            isometry,
            radius,
            color,
            resolution,
            ..
        } => {
            gizmos
                .sphere(*isometry, *radius, *color)
                .resolution(*resolution);
        }
    }
}

fn draw_overlay_primitive(
    gizmos: &mut Gizmos<DeveloperOverlayGizmos>,
    primitive: &WorldPrimitive,
) {
    match primitive {
        WorldPrimitive::Line { start, end, color, .. } => gizmos.line(*start, *end, *color),
        WorldPrimitive::Arrow { start, end, color, .. } => {
            gizmos.arrow(*start, *end, *color);
        }
        WorldPrimitive::Axes { transform, length, .. } => gizmos.axes(*transform, *length),
        WorldPrimitive::Rect { isometry, size, color, .. } => {
            gizmos.rect(*isometry, *size, *color);
        }
        WorldPrimitive::Cross { isometry, size, color, .. } => {
            gizmos.cross(*isometry, *size, *color);
        }
        WorldPrimitive::Sphere {
            isometry,
            radius,
            color,
            resolution,
            ..
        } => {
            gizmos
                .sphere(*isometry, *radius, *color)
                .resolution(*resolution);
        }
    }
}

fn draw_vector_field(gizmos: &mut Gizmos<DeveloperWorldGizmos>, field: &WorldVectorField) {
    let resolution = field.resolution;
    let expected = (resolution.x * resolution.y) as usize;
    if resolution.x < 2 || resolution.y < 2 || field.vectors.len() != expected {
        gizmos.rect(
            Isometry3d::new(field.transform.translation, field.transform.rotation),
            field.size,
            Color::srgb(1.0, 0.0, 1.0),
        );
        return;
    }

    let minimum = -field.size * 0.5;
    let denominator = (resolution - UVec2::ONE).as_vec2();

    for y in 0..resolution.y {
        for x in 0..resolution.x {
            let index = (x + y * resolution.x) as usize;
            let mut vector = field.vectors[index];
            if !vector.is_finite() {
                continue;
            }

            let uv = Vec2::new(x as f32, y as f32) / denominator;
            let local_xy = minimum + field.size * uv;
            let local = Vec3::new(local_xy.x, local_xy.y, 0.03);
            let origin = field.transform.transform_point(local);

            if field.vector_space == VectorSpace::Local {
                vector = field.transform.rotation * vector;
            }

            let magnitude = vector.length();
            if magnitude <= f32::EPSILON {
                continue;
            }

            let mut rendered = vector * field.vector_scale;
            if field.maximum_arrow_length.is_finite() {
                rendered = rendered.clamp_length_max(field.maximum_arrow_length);
            }
            if rendered.length_squared() <= f32::EPSILON {
                continue;
            }

            let color = field
                .magnitude_range
                .map(|range| field.magnitude_ramp.sample_scalar(range, magnitude))
                .unwrap_or(field.color);
            gizmos.arrow(origin, origin + rendered, color);
        }
    }
}

fn sync_scalar_field_visuals(
    mut commands: Commands,
    frame: Res<WorldDrawFrame>,
    assets: Res<WorldDrawAssets>,
    mut cache: ResMut<WorldDrawCache>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut visuals: Query<
        (&mut Transform, &mut GlobalTransform, &mut Visibility),
        With<WorldScalarFieldVisual>,
    >,
) {
    let frame = frame.read();
    let mut seen = HashSet::new();

    for field in &frame.scalar_fields {
        seen.insert(field.id);

        if !cache.scalar_fields.contains_key(&field.id) {
            let mesh = meshes.add(scalar_field_mesh(field));
            let entity = commands
                .spawn((
                    Name::new(format!("Developer scalar field {}", field.id)),
                    DeveloperArtifact,
                    // Temporary Stage-5 telemetry compatibility marker.
                    DebugArtifact,
                    WorldScalarFieldVisual,
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(assets.scalar_material.clone()),
                    field.transform,
                    GlobalTransform::from(field.transform),
                ))
                .id();
            cache
                .scalar_fields
                .insert(field.id, ScalarFieldVisual { entity, mesh });
            continue;
        }

        let visual = &cache.scalar_fields[&field.id];
        if let Some(mut mesh) = meshes.get_mut(&visual.mesh) {
            *mesh = scalar_field_mesh(field);
        }
        if let Ok((mut transform, mut global, mut visibility)) = visuals.get_mut(visual.entity) {
            *transform = field.transform;
            *global = GlobalTransform::from(field.transform);
            *visibility = Visibility::Inherited;
        }
    }

    for (id, visual) in &cache.scalar_fields {
        if seen.contains(id) {
            continue;
        }
        if let Ok((_, _, mut visibility)) = visuals.get_mut(visual.entity) {
            *visibility = Visibility::Hidden;
        }
    }
}

#[derive(Component)]
struct WorldScalarFieldVisual;

fn scalar_field_mesh(field: &WorldScalarField) -> Mesh {
    let resolution = field.resolution;
    if resolution.x < 2 || resolution.y < 2 {
        return invalid_field_mesh(field);
    }

    let expected = (resolution.x * resolution.y) as usize;
    if field.values.len() != expected {
        return invalid_field_mesh(field);
    }

    let minimum = -field.size * 0.5;
    let denominator = (resolution - UVec2::ONE).as_vec2();
    let cell_count = ((resolution.x - 1) * (resolution.y - 1)) as usize;
    let mut positions = Vec::<[f32; 3]>::with_capacity(cell_count * 6);
    let mut normals = Vec::<[f32; 3]>::with_capacity(cell_count * 6);
    let mut colors = Vec::<[f32; 4]>::with_capacity(cell_count * 6);
    let mut uvs = Vec::<[f32; 2]>::with_capacity(cell_count * 6);

    let vertex = |x: u32, y: u32| {
        let index = (x + y * resolution.x) as usize;
        let value = field.values[index];
        let normalized = field.range.normalize(value);
        let color = normalized
            .map(|value| field.ramp.sample(value))
            .unwrap_or(Color::srgb(1.0, 0.0, 1.0));
        let linear = color.to_linear();
        let uv = Vec2::new(x as f32, y as f32) / denominator;
        let xy = minimum + field.size * uv;
        let height = if field.mode == ScalarFieldMode::HeightField {
            normalized.unwrap_or(0.0) * field.height_scale.max(0.0)
        } else {
            0.0
        };
        (
            [xy.x, xy.y, height],
            [
                linear.red,
                linear.green,
                linear.blue,
                linear.alpha * field.opacity.clamp(0.0, 1.0),
            ],
            [uv.x, 1.0 - uv.y],
        )
    };

    for y in 0..(resolution.y - 1) {
        for x in 0..(resolution.x - 1) {
            let a = vertex(x, y);
            let b = vertex(x + 1, y);
            let c = vertex(x + 1, y + 1);
            let d = vertex(x, y + 1);

            for point in [a, b, c, a, c, d] {
                positions.push(point.0);
                colors.push(point.1);
                uvs.push(point.2);
            }
            normals.extend_from_slice(&[[0.0, 0.0, 1.0]; 6]);
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
}

fn invalid_field_mesh(field: &WorldScalarField) -> Mesh {
    let half = field.size * 0.5;
    let color = Color::srgb(1.0, 0.0, 1.0).to_linear();
    let rgba = [color.red, color.green, color.blue, 0.85];

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-half.x, -half.y, 0.0],
            [half.x, -half.y, 0.0],
            [half.x, half.y, 0.0],
            [-half.x, -half.y, 0.0],
            [half.x, half.y, 0.0],
            [-half.x, half.y, 0.0],
        ],
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 6])
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, vec![rgba; 6])
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [0.0, 1.0],
            [1.0, 1.0],
            [1.0, 0.0],
            [0.0, 1.0],
            [1.0, 0.0],
            [0.0, 0.0],
        ],
    )
}
