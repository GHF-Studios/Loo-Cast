//! Generic sampled scalar/vector field visualization.
//!
//! Domains own the meaning and sampling of a field. This module only turns a
//! regularly sampled 2D plane into a heatmap mesh or vector arrows.

use std::collections::HashSet;

use bevy::{
    asset::RenderAssetUsages,
    mesh::PrimitiveTopology,
    prelude::*,
};

use super::{
    AppDebugExt, DebugColorRamp, DebugGizmos, DebugScalarRange, DebugView, DebugViews,
};

struct ScalarFieldDebugView;

impl DebugView for ScalarFieldDebugView {
    const NAME: &'static str = "Fields / Scalar Heatmaps";
    const DESCRIPTION: &'static str = "Render sampled scalar planes as colored heatmaps.";
}

struct VectorFieldDebugView;

impl DebugView for VectorFieldDebugView {
    const NAME: &'static str = "Fields / Vector Arrows";
    const DESCRIPTION: &'static str = "Render sampled vector planes as world-space arrows.";
}

/// Regular scalar cells on a local XY plane centered on the owning entity.
///
/// `resolution` is the number of *cells* along each axis and `values` is row
/// major (`x + y * resolution.x`). The entity's `Transform` places/orients the
/// plane in world space, so this works for ground slices, walls, arbitrary
/// metric-map cuts, portal-local fields, etc.
#[derive(Component, Debug, Clone)]
#[require(Transform, Visibility)]
pub struct DebugScalarField2d {
    pub size: Vec2,
    pub resolution: UVec2,
    pub values: Vec<f32>,
    pub range: DebugScalarRange,
    pub ramp: DebugColorRamp,
    pub opacity: f32,
}

impl DebugScalarField2d {
    pub fn new(
        size: Vec2,
        resolution: UVec2,
        values: Vec<f32>,
        range: DebugScalarRange,
        ramp: DebugColorRamp,
    ) -> Self {
        Self {
            size,
            resolution,
            values,
            range,
            ramp,
            opacity: 0.65,
        }
    }

    pub fn with_opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DebugVectorSpace {
    World,
    Local,
}

/// Regular vector samples on a local XY plane centered on the owning entity.
#[derive(Component, Debug, Clone)]
#[require(Transform, Visibility)]
pub struct DebugVectorField2d {
    pub size: Vec2,
    pub resolution: UVec2,
    pub vectors: Vec<Vec3>,
    pub vector_space: DebugVectorSpace,
    pub vector_scale: f32,
    pub maximum_arrow_length: f32,
    pub color: Color,
    pub magnitude_range: Option<DebugScalarRange>,
    pub magnitude_ramp: DebugColorRamp,
}

impl DebugVectorField2d {
    pub fn new(size: Vec2, resolution: UVec2, vectors: Vec<Vec3>) -> Self {
        Self {
            size,
            resolution,
            vectors,
            vector_space: DebugVectorSpace::World,
            vector_scale: 1.0,
            maximum_arrow_length: f32::INFINITY,
            color: Color::WHITE,
            magnitude_range: None,
            magnitude_ramp: DebugColorRamp::METRIC,
        }
    }

    pub fn with_vector_space(mut self, space: DebugVectorSpace) -> Self {
        self.vector_space = space;
        self
    }

    pub fn with_vector_scale(mut self, scale: f32) -> Self {
        self.vector_scale = scale.max(0.0);
        self
    }

    pub fn with_maximum_arrow_length(mut self, length: f32) -> Self {
        self.maximum_arrow_length = length.max(0.0);
        self
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_magnitude_colors(
        mut self,
        range: DebugScalarRange,
        ramp: DebugColorRamp,
    ) -> Self {
        self.magnitude_range = Some(range);
        self.magnitude_ramp = ramp;
        self
    }
}

#[derive(Resource)]
struct DebugFieldAssets {
    scalar_material: Handle<StandardMaterial>,
}

#[derive(Component, Debug, Clone, Copy)]
struct DebugScalarFieldVisual {
    source: Entity,
}

pub(super) fn configure(app: &mut App) {
    app.register_debug_view::<ScalarFieldDebugView>()
        .register_debug_view::<VectorFieldDebugView>()
        .add_systems(Startup, setup_field_assets)
        .add_systems(PostUpdate, (sync_scalar_field_visuals, draw_vector_fields));
}

fn setup_field_assets(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(DebugFieldAssets {
        scalar_material: materials.add(StandardMaterial {
            base_color: Color::WHITE,
            alpha_mode: AlphaMode::Blend,
            unlit: true,
            cull_mode: None,
            ..default()
        }),
    });
}

fn sync_scalar_field_visuals(
    mut commands: Commands,
    views: Res<DebugViews>,
    assets: Res<DebugFieldAssets>,
    fields: Query<(Entity, Ref<DebugScalarField2d>)>,
    mut visuals: Query<(
        Entity,
        &DebugScalarFieldVisual,
        &Mesh3d,
        &mut Visibility,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let show = views.enabled::<ScalarFieldDebugView>();
    let mut existing_sources = HashSet::new();

    for (visual_entity, visual, mesh, mut visibility) in &mut visuals {
        let Ok((_, field)) = fields.get(visual.source) else {
            commands.entity(visual_entity).despawn();
            continue;
        };

        existing_sources.insert(visual.source);
        *visibility = if show {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };

        if show && field.is_changed() {
            if let Some(mut mesh_asset) = meshes.get_mut(&mesh.0) {
                *mesh_asset = scalar_field_mesh(&field);
            }
        }
    }

    if !show {
        return;
    }

    for (source, field) in &fields {
        if existing_sources.contains(&source) {
            continue;
        }

        let mesh = meshes.add(scalar_field_mesh(&field));
        commands.entity(source).with_children(|parent| {
            parent.spawn((
                Name::new("Debug Scalar Field Heatmap"),
                DebugScalarFieldVisual { source },
                Mesh3d(mesh),
                MeshMaterial3d(assets.scalar_material.clone()),
                Transform::from_translation(Vec3::Z * 0.015),
            ));
        });
    }
}

fn draw_vector_fields(
    views: Res<DebugViews>,
    fields: Query<(&DebugVectorField2d, &GlobalTransform)>,
    mut gizmos: Gizmos<DebugGizmos>,
) {
    if !views.enabled::<VectorFieldDebugView>() {
        return;
    }

    for (field, global) in &fields {
        let resolution = safe_resolution(field.resolution);
        let cell = field.size / resolution.as_vec2();
        let minimum = -field.size * 0.5;
        let transform = global.compute_transform();

        for y in 0..resolution.y {
            for x in 0..resolution.x {
                let index = (x + y * resolution.x) as usize;
                let Some(mut vector) = field.vectors.get(index).copied() else {
                    continue;
                };
                if !vector.is_finite() {
                    continue;
                }

                let local = Vec3::new(
                    minimum.x + (x as f32 + 0.5) * cell.x,
                    minimum.y + (y as f32 + 0.5) * cell.y,
                    0.03,
                );
                let origin = transform.translation
                    + transform.rotation * (local * transform.scale);

                if field.vector_space == DebugVectorSpace::Local {
                    vector = transform.rotation * vector;
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
}

fn scalar_field_mesh(field: &DebugScalarField2d) -> Mesh {
    let resolution = safe_resolution(field.resolution);
    let cell = field.size / resolution.as_vec2();
    let minimum = -field.size * 0.5;
    let mut positions = Vec::<[f32; 3]>::new();
    let mut normals = Vec::<[f32; 3]>::new();
    let mut colors = Vec::<[f32; 4]>::new();
    let mut uvs = Vec::<[f32; 2]>::new();

    for y in 0..resolution.y {
        for x in 0..resolution.x {
            let index = (x + y * resolution.x) as usize;
            let value = field.values.get(index).copied().unwrap_or(field.range.minimum);
            let linear = field.ramp.sample_scalar(field.range, value).to_linear();
            let rgba = [linear.red, linear.green, linear.blue, linear.alpha * field.opacity];

            let x0 = minimum.x + x as f32 * cell.x;
            let x1 = x0 + cell.x;
            let y0 = minimum.y + y as f32 * cell.y;
            let y1 = y0 + cell.y;

            positions.extend_from_slice(&[
                [x0, y0, 0.0],
                [x1, y0, 0.0],
                [x1, y1, 0.0],
                [x0, y0, 0.0],
                [x1, y1, 0.0],
                [x0, y1, 0.0],
            ]);
            normals.extend_from_slice(&[[0.0, 0.0, 1.0]; 6]);
            colors.extend_from_slice(&[rgba; 6]);
            uvs.extend_from_slice(&[
                [0.0, 1.0],
                [1.0, 1.0],
                [1.0, 0.0],
                [0.0, 1.0],
                [1.0, 0.0],
                [0.0, 0.0],
            ]);
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

fn safe_resolution(resolution: UVec2) -> UVec2 {
    UVec2::new(resolution.x.max(1), resolution.y.max(1))
}
