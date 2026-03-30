use crate::bevy::asset::RenderAssetUsages;
use crate::bevy::mesh::Indices;
use crate::bevy::prelude::*;
use crate::bevy::render::render_resource::PrimitiveTopology;

use crate::usf::mod_runtime::surface_field::{CHUNK_SPAN_UNITS_I64, HALF_CHUNK_SPAN_F32, PersistedChunkRecord};

pub(crate) fn build_chunk_mesh(record: &PersistedChunkRecord) -> Option<Mesh> {
    let axis_points = record.axis_samples.len();
    if axis_points < 2 {
        return None;
    }

    let total_points = axis_points * axis_points * axis_points;
    let rho_values = record.rho_field.expand(total_points)?;

    let mut points = vec![Vec3::ZERO; total_points];
    let mut signed_field = vec![0.0_f32; total_points];

    for iz in 0..axis_points {
        for iy in 0..axis_points {
            for ix in 0..axis_points {
                let idx = grid_index(ix, iy, iz, axis_points);
                let x = record.axis_samples[ix] as f32 - HALF_CHUNK_SPAN_F32;
                let y = record.axis_samples[iy] as f32 - HALF_CHUNK_SPAN_F32;
                let z = record.axis_samples[iz] as f32 - HALF_CHUNK_SPAN_F32;
                points[idx] = Vec3::new(x, y, z);
                signed_field[idx] = record.iso_level as f32 - rho_values[idx] as f32;
            }
        }
    }

    let cube_corners: [[usize; 3]; 8] = [[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0], [0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]];
    let tetrahedra: [[usize; 4]; 6] = [[0, 5, 1, 6], [0, 1, 2, 6], [0, 2, 3, 6], [0, 3, 7, 6], [0, 7, 4, 6], [0, 4, 5, 6]];

    let mut out_positions = Vec::<[f32; 3]>::new();
    let mut out_normals = Vec::<[f32; 3]>::new();
    let mut out_uvs = Vec::<[f32; 2]>::new();

    for iz in 0..(axis_points - 1) {
        for iy in 0..(axis_points - 1) {
            for ix in 0..(axis_points - 1) {
                let mut cube_points = [Vec3::ZERO; 8];
                let mut cube_values = [0.0_f32; 8];
                for (corner_i, [ox, oy, oz]) in cube_corners.iter().copied().enumerate() {
                    let sx = ix + ox;
                    let sy = iy + oy;
                    let sz = iz + oz;
                    let idx = grid_index(sx, sy, sz, axis_points);
                    cube_points[corner_i] = points[idx];
                    cube_values[corner_i] = signed_field[idx];
                }

                for tet in tetrahedra {
                    let p = [cube_points[tet[0]], cube_points[tet[1]], cube_points[tet[2]], cube_points[tet[3]]];
                    let s = [cube_values[tet[0]], cube_values[tet[1]], cube_values[tet[2]], cube_values[tet[3]]];
                    emit_tetra_surface(p, s, &mut out_positions, &mut out_normals, &mut out_uvs);
                }
            }
        }
    }

    if out_positions.is_empty() {
        return None;
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
    let triangle_indices = (0..out_positions.len() as u32).collect::<Vec<_>>();
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, out_positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, out_normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, out_uvs);
    mesh.insert_indices(Indices::U32(triangle_indices));
    Some(mesh)
}

fn emit_tetra_surface(points: [Vec3; 4], values: [f32; 4], out_positions: &mut Vec<[f32; 3]>, out_normals: &mut Vec<[f32; 3]>, out_uvs: &mut Vec<[f32; 2]>) {
    let mut inside = [0usize; 4];
    let mut outside = [0usize; 4];
    let mut inside_count = 0usize;
    let mut outside_count = 0usize;

    for i in 0..4 {
        if values[i] <= 0.0 {
            inside[inside_count] = i;
            inside_count += 1;
        } else {
            outside[outside_count] = i;
            outside_count += 1;
        }
    }

    if inside_count == 0 || inside_count == 4 {
        return;
    }

    let edge_point = |a_i: usize, b_i: usize| interpolate_iso(points[a_i], values[a_i], points[b_i], values[b_i]);

    match inside_count {
        1 => {
            let i = inside[0];
            let a = outside[0];
            let b = outside[1];
            let c = outside[2];
            let inside_ref = points[i];
            let outside_ref = (points[a] + points[b] + points[c]) / 3.0;
            push_oriented_triangle(
                edge_point(i, a),
                edge_point(i, b),
                edge_point(i, c),
                inside_ref,
                outside_ref,
                out_positions,
                out_normals,
                out_uvs,
            );
        }
        3 => {
            let o = outside[0];
            let a = inside[0];
            let b = inside[1];
            let c = inside[2];
            let inside_ref = (points[a] + points[b] + points[c]) / 3.0;
            let outside_ref = points[o];
            push_oriented_triangle(
                edge_point(o, a),
                edge_point(o, c),
                edge_point(o, b),
                inside_ref,
                outside_ref,
                out_positions,
                out_normals,
                out_uvs,
            );
        }
        2 => {
            let a = inside[0];
            let b = inside[1];
            let c = outside[0];
            let d = outside[1];
            let inside_ref = (points[a] + points[b]) * 0.5;
            let outside_ref = (points[c] + points[d]) * 0.5;

            let p0 = edge_point(a, c);
            let p1 = edge_point(b, c);
            let p2 = edge_point(b, d);
            let p3 = edge_point(a, d);
            push_oriented_triangle(p0, p1, p2, inside_ref, outside_ref, out_positions, out_normals, out_uvs);
            push_oriented_triangle(p0, p2, p3, inside_ref, outside_ref, out_positions, out_normals, out_uvs);
        }
        _ => {}
    }
}

fn push_oriented_triangle(
    a: Vec3,
    mut b: Vec3,
    mut c: Vec3,
    inside_ref: Vec3,
    outside_ref: Vec3,
    out_positions: &mut Vec<[f32; 3]>,
    out_normals: &mut Vec<[f32; 3]>,
    out_uvs: &mut Vec<[f32; 2]>,
) {
    let mut normal = (b - a).cross(c - a);
    let mut len_sq = normal.length_squared();
    if len_sq <= 1e-10 {
        return;
    }

    let expected_outward = outside_ref - inside_ref;
    if expected_outward.length_squared() > 1e-10 && normal.dot(expected_outward) < 0.0 {
        std::mem::swap(&mut b, &mut c);
        normal = (b - a).cross(c - a);
        len_sq = normal.length_squared();
        if len_sq <= 1e-10 {
            return;
        }
    }

    let n = normal / len_sq.sqrt();
    for p in [a, b, c] {
        out_positions.push([p.x, p.y, p.z]);
        out_normals.push([n.x, n.y, n.z]);
        out_uvs.push([
            ((p.x + HALF_CHUNK_SPAN_F32) / CHUNK_SPAN_UNITS_I64 as f32).clamp(0.0, 1.0),
            ((p.y + HALF_CHUNK_SPAN_F32) / CHUNK_SPAN_UNITS_I64 as f32).clamp(0.0, 1.0),
        ]);
    }
}

#[inline]
fn interpolate_iso(a_pos: Vec3, a_val: f32, b_pos: Vec3, b_val: f32) -> Vec3 {
    let denom = a_val - b_val;
    let t = if denom.abs() <= 1e-6 { 0.5 } else { (a_val / denom).clamp(0.0, 1.0) };
    a_pos + (b_pos - a_pos) * t
}

#[inline]
fn grid_index(ix: usize, iy: usize, iz: usize, axis_points: usize) -> usize {
    ix + axis_points * (iy + axis_points * iz)
}
