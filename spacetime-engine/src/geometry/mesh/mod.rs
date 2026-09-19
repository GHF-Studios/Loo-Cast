use bevy::{asset::RenderAssetUsages, prelude::*, render::render_resource::PrimitiveTopology};

/// Builds a flat-shaded convex prism by extruding an XY cross-section along local Z.
///
/// The cross-section is expected to be convex and listed in winding order. We duplicate
/// vertices per triangle so each face gets an unambiguous flat normal; this keeps the
/// authoring/runtime code simple and predictable for generated geometry.
pub(super) fn convex_prism_mesh(cross_section: &[Vec2], depth: f32) -> Mesh {
    let mut cross_section = cross_section.to_vec();
    if signed_area(&cross_section) < 0.0 {
        cross_section.reverse();
    }

    let front_z = depth * 0.5;
    let back_z = -front_z;

    let mut positions: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();

    let mut push_triangle = |a: Vec3, b: Vec3, c: Vec3| {
        let normal = (b - a).cross(c - a).normalize_or_zero();
        positions.extend([a.to_array(), b.to_array(), c.to_array()]);
        normals.extend([normal.to_array(); 3]);
    };

    // Front and back caps, triangulated as fans.
    let first = cross_section[0];
    for index in 1..cross_section.len() - 1 {
        let second = cross_section[index];
        let third = cross_section[index + 1];

        push_triangle(
            Vec3::new(first.x, first.y, front_z),
            Vec3::new(second.x, second.y, front_z),
            Vec3::new(third.x, third.y, front_z),
        );

        push_triangle(
            Vec3::new(first.x, first.y, back_z),
            Vec3::new(third.x, third.y, back_z),
            Vec3::new(second.x, second.y, back_z),
        );
    }

    // Side quads, split into two triangles each.
    for index in 0..cross_section.len() {
        let next = (index + 1) % cross_section.len();
        let a = cross_section[index];
        let b = cross_section[next];

        let a_front = Vec3::new(a.x, a.y, front_z);
        let b_front = Vec3::new(b.x, b.y, front_z);
        let a_back = Vec3::new(a.x, a.y, back_z);
        let b_back = Vec3::new(b.x, b.y, back_z);

        push_triangle(a_front, b_back, a_back);
        push_triangle(a_front, b_front, b_back);
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
}

fn signed_area(points: &[Vec2]) -> f32 {
    let mut area = 0.0;
    for index in 0..points.len() {
        let a = points[index];
        let b = points[(index + 1) % points.len()];
        area += a.x * b.y - b.x * a.y;
    }
    area * 0.5
}

pub(super) fn convex_prism_points(cross_section: &[Vec2], depth: f32) -> Vec<Vec3> {
    let half_depth = depth * 0.5;
    let mut points = Vec::with_capacity(cross_section.len() * 2);

    for point in cross_section {
        points.push(Vec3::new(point.x, point.y, -half_depth));
        points.push(Vec3::new(point.x, point.y, half_depth));
    }

    points
}
