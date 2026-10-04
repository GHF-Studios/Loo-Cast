// GPU-native binary terrain presentation.
// Canonical authority remains CPU/USF-owned; this shader receives only one
// bounded local semantic chart. No f64/i64 and no universe-scale coordinates.
// gpu-binary-presentation-production-v1
// gpu-terrain-wgsl-reserved-sweep-v1
// gpu-terrain-wgsl-reserved-name-repair-v1

const BLOCK: u32 = 8u;
const REGULAR_LATTICE: u32 = 9u;
const REGULAR_LATTICE_LEN: u32 = 729u;
const FACE_LATTICE: u32 = 17u;
const FACE_LATTICE_LEN: u32 = 289u;
const HASH_DIGITS: u32 = 20u;
const MAX_COARSE_BANDS: u32 = 35u;
const MAX_FINE_BANDS: u32 = 36u;

const REG_CLASS_OFFSET: u32 = 0u;
const REG_COUNTS_OFFSET: u32 = 256u;
const REG_TRI_OFFSET: u32 = 272u;
const REG_VERTEX_OFFSET: u32 = 512u;
const TRANS_CLASS_OFFSET: u32 = 3584u;
const TRANS_COUNTS_OFFSET: u32 = 4096u;
const TRANS_TRI_OFFSET: u32 = 4152u;
const TRANS_VERTEX_OFFSET: u32 = 6168u;

struct GpuCoarseBand {
    params: vec4<f32>,
    seeds: vec4<u32>,
}

struct GpuFineBand {
    params: vec4<f32>,
    base_offset: vec4<f32>,
    band_meta: vec4<i32>,
    prefix: vec4<u32>,
    digits_x: array<vec4<i32>, 5>,
    digits_y: array<vec4<i32>, 5>,
    digits_z: array<vec4<i32>, 5>,
}

struct GpuCaveChart {
    base_cell: vec4<i32>,
    fraction_and_wavelength: vec4<f32>,
    seed: vec4<u32>,
}

struct GpuTerrainDescriptor {
    chart_origin_and_spacing: vec4<f32>,
    anchor_direction_and_inverse_radius: vec4<f32>,
    extent_uv_radius: vec4<f32>,
    terrain_meta: vec4<u32>,
    semantic_meta: vec4<i32>,
    seed_meta: vec4<u32>,
    coarse: array<GpuCoarseBand, 35>,
    fine: array<GpuFineBand, 36>,
    caves: array<GpuCaveChart, 5>,
}

struct GpuTerrainDispatch {
    ranges: vec4<u32>,
    descriptor: GpuTerrainDescriptor,
}

struct SurfaceVertex {
    position: vec3<f32>,
    normal: vec3<f32>,
}

// gpu-terrain-split-pipeline-v1
@group(0) @binding(0) var<uniform> dispatch: GpuTerrainDispatch;
@group(0) @binding(1) var<storage, read> tables: array<u32>;
@group(0) @binding(2) var<storage, read> scratch: array<f32>;
@group(0) @binding(3) var<storage, read_write> vertex_data: array<f32>;
@group(0) @binding(4) var<storage, read_write> index_data: array<u32>;

var<workgroup> output_count: atomic<u32>;

fn regular_lattice_index(index: vec3<i32>) -> u32 {
    return u32(index.x)
        + REGULAR_LATTICE * u32(index.y)
        + REGULAR_LATTICE * REGULAR_LATTICE * u32(index.z);
}

fn regular_sample(index: vec3<i32>) -> f32 {
    return scratch[regular_lattice_index(index)];
}

fn regular_corner(index: u32) -> vec3<i32> {
    return vec3<i32>(
        i32(index & 1u),
        i32((index >> 1u) & 1u),
        i32((index >> 2u) & 1u),
    );
}

fn transition_enabled(side: u32) -> bool {
    return (dispatch.descriptor.terrain_meta.y & (1u << side)) != 0u;
}

fn can_shrink(index: vec3<i32>) -> bool {
    let n = i32(BLOCK);
    let bits = dispatch.descriptor.terrain_meta.y;
    let dont_shrink =
        (index.x == 0 && (bits & 1u) == 0u)
        || (index.x == n && (bits & 2u) == 0u)
        || (index.y == 0 && (bits & 4u) == 0u)
        || (index.y == n && (bits & 8u) == 0u)
        || (index.z == 0 && (bits & 16u) == 0u)
        || (index.z == n && (bits & 32u) == 0u);
    return !dont_shrink;
}

fn regular_position(index: vec3<i32>) -> vec3<f32> {
    let spacing = dispatch.descriptor.chart_origin_and_spacing.w;
    var p = vec3<f32>(index) * spacing;
    if !can_shrink(index) {
        return p;
    }
    let shrink = spacing * 0.15;
    let n = i32(BLOCK);
    let bits = dispatch.descriptor.terrain_meta.y;

    if index.x == 0 && (bits & 1u) != 0u { p.x += shrink; }
    if index.x == n && (bits & 2u) != 0u { p.x -= shrink; }
    if index.y == 0 && (bits & 4u) != 0u { p.y += shrink; }
    if index.y == n && (bits & 8u) != 0u { p.y -= shrink; }
    if index.z == 0 && (bits & 16u) != 0u { p.z += shrink; }
    if index.z == n && (bits & 32u) != 0u { p.z -= shrink; }
    return p;
}

fn regular_gradient(index: vec3<i32>) -> vec3<f32> {
    let n = i32(BLOCK);

    var gx = 0.0;
    if index.x == 0 {
        gx = 2.0 * (
            regular_sample(index + vec3<i32>(1, 0, 0))
                - regular_sample(index)
        );
    } else if index.x == n {
        gx = 2.0 * (
            regular_sample(index)
                - regular_sample(index - vec3<i32>(1, 0, 0))
        );
    } else {
        gx = regular_sample(index + vec3<i32>(1, 0, 0))
            - regular_sample(index - vec3<i32>(1, 0, 0));
    }

    var gy = 0.0;
    if index.y == 0 {
        gy = 2.0 * (
            regular_sample(index + vec3<i32>(0, 1, 0))
                - regular_sample(index)
        );
    } else if index.y == n {
        gy = 2.0 * (
            regular_sample(index)
                - regular_sample(index - vec3<i32>(0, 1, 0))
        );
    } else {
        gy = regular_sample(index + vec3<i32>(0, 1, 0))
            - regular_sample(index - vec3<i32>(0, 1, 0));
    }

    var gz = 0.0;
    if index.z == 0 {
        gz = 2.0 * (
            regular_sample(index + vec3<i32>(0, 0, 1))
                - regular_sample(index)
        );
    } else if index.z == n {
        gz = 2.0 * (
            regular_sample(index)
                - regular_sample(index - vec3<i32>(0, 0, 1))
        );
    } else {
        gz = regular_sample(index + vec3<i32>(0, 0, 1))
            - regular_sample(index - vec3<i32>(0, 0, 1));
    }

    return vec3<f32>(gx, gy, gz);
}

fn interpolate_vertex(
    pa: vec3<f32>,
    pb: vec3<f32>,
    ga: vec3<f32>,
    gb: vec3<f32>,
    da: f32,
    db: f32,
) -> SurfaceVertex {
    var t = 0.5;
    if abs(db - da) > 1.0e-7 {
        t = -da / (db - da);
    }
    t = clamp(t, 0.0, 1.0);
    let p = mix(pa, pb, t);
    let g = mix(ga, gb, t);
    let gl = length(g);
    let normal = select(vec3<f32>(0.0, 1.0, 0.0), -g / gl, gl > 1.0e-8);
    return SurfaceVertex(p, normal);
}

fn regular_edge_vertex(cell: vec3<i32>, vd: u32) -> SurfaceVertex {
    let edge = vd & 0xFFu;
    let ai = (edge >> 4u) & 0xFu;
    let bi = edge & 0xFu;
    let a = cell + regular_corner(ai);
    let b = cell + regular_corner(bi);
    return interpolate_vertex(
        regular_position(a),
        regular_position(b),
        regular_gradient(a),
        regular_gradient(b),
        regular_sample(a),
        regular_sample(b),
    );
}

fn rotation_base(side: u32) -> vec3<i32> {
    switch side {
        case 0u: { return vec3<i32>(0, 0, 1); }
        case 1u: { return vec3<i32>(1, 0, 0); }
        case 2u: { return vec3<i32>(0, 0, 1); }
        case 3u: { return vec3<i32>(0, 1, 0); }
        case 4u: { return vec3<i32>(0, 0, 0); }
        default: { return vec3<i32>(1, 0, 1); }
    }
}

fn rotation_u(side: u32) -> vec3<i32> {
    switch side {
        case 0u: { return vec3<i32>(0, 0, -1); }
        case 1u: { return vec3<i32>(0, 0, 1); }
        case 2u: { return vec3<i32>(1, 0, 0); }
        case 3u: { return vec3<i32>(1, 0, 0); }
        case 4u: { return vec3<i32>(1, 0, 0); }
        default: { return vec3<i32>(-1, 0, 0); }
    }
}

fn rotation_v(side: u32) -> vec3<i32> {
    switch side {
        case 0u: { return vec3<i32>(0, 1, 0); }
        case 1u: { return vec3<i32>(0, 1, 0); }
        case 2u: { return vec3<i32>(0, 0, -1); }
        case 3u: { return vec3<i32>(0, 0, 1); }
        case 4u: { return vec3<i32>(0, 1, 0); }
        default: { return vec3<i32>(0, 1, 0); }
    }
}

fn rotation_w(side: u32) -> vec3<i32> {
    switch side {
        case 0u: { return vec3<i32>(1, 0, 0); }
        case 1u: { return vec3<i32>(-1, 0, 0); }
        case 2u: { return vec3<i32>(0, 1, 0); }
        case 3u: { return vec3<i32>(0, -1, 0); }
        case 4u: { return vec3<i32>(0, 0, 1); }
        default: { return vec3<i32>(0, 0, -1); }
    }
}

fn transition_position_half(
    side: u32,
    hu: i32,
    hv: i32,
    hw: i32,
) -> vec3<f32> {
    let spacing = dispatch.descriptor.chart_origin_and_spacing.w;
    let extent = dispatch.descriptor.extent_uv_radius.x;
    return vec3<f32>(rotation_base(side)) * extent
        + vec3<f32>(rotation_u(side)) * (f32(hu) * spacing * 0.5)
        + vec3<f32>(rotation_v(side)) * (f32(hv) * spacing * 0.5)
        + vec3<f32>(rotation_w(side)) * (f32(hw) * spacing * 0.5);
}

fn transition_regular_index(
    side: u32,
    cell_u: i32,
    cell_v: i32,
    face_u: i32,
    face_v: i32,
) -> vec3<i32> {
    return rotation_base(side) * i32(BLOCK)
        + rotation_u(side) * (cell_u + face_u)
        + rotation_v(side) * (cell_v + face_v);
}

fn face_index(hu: u32, hv: u32) -> u32 {
    return hu + FACE_LATTICE * hv;
}


const FACE_SCRATCH_BASE: u32 = 729u;
const FACE_SAMPLE_STRIDE: u32 = 4u;

fn face_scratch_index(side: u32, hu: u32, hv: u32) -> u32 {
    return FACE_SCRATCH_BASE
        + (side * FACE_LATTICE_LEN + face_index(hu, hv))
            * FACE_SAMPLE_STRIDE;
}

fn transition_grid_vertex(
    side: u32,
    cell_u: i32,
    cell_v: i32,
    point_index: u32,
) -> SurfaceVertex {
    if point_index < 9u {
        let du = i32(point_index % 3u);
        let dv = i32(point_index / 3u);
        let hu = cell_u * 2 + du;
        let hv = cell_v * 2 + dv;
        let p = transition_position_half(side, hu, hv, 0);
        let base = face_scratch_index(side, u32(hu), u32(hv));
        let g = vec3<f32>(
            scratch[base + 1u],
            scratch[base + 2u],
            scratch[base + 3u],
        );
        let gl = length(g);
        let n = select(
            vec3<f32>(0.0, 1.0, 0.0),
            -g / max(gl, 1.0e-8),
            gl > 1.0e-8,
        );
        return SurfaceVertex(p, n);
    }

    var fu = 0;
    var fv = 0;
    switch point_index {
        case 9u: { fu = 0; fv = 0; }
        case 10u: { fu = 1; fv = 0; }
        case 11u: { fu = 0; fv = 1; }
        default: { fu = 1; fv = 1; }
    }
    let index = transition_regular_index(side, cell_u, cell_v, fu, fv);
    let g = regular_gradient(index);
    let gl = length(g);
    let n = select(vec3<f32>(0.0, 1.0, 0.0), -g / gl, gl > 1.0e-8);
    return SurfaceVertex(regular_position(index), n);
}

fn transition_grid_density(
    side: u32,
    cell_u: i32,
    cell_v: i32,
    point_index: u32,
) -> f32 {
    if point_index < 9u {
        let du = u32(point_index % 3u);
        let dv = u32(point_index / 3u);
        let hu = u32(cell_u * 2) + du;
        let hv = u32(cell_v * 2) + dv;
        return scratch[face_scratch_index(side, hu, hv)];
    }
    var fu = 0;
    var fv = 0;
    switch point_index {
        case 9u: { fu = 0; fv = 0; }
        case 10u: { fu = 1; fv = 0; }
        case 11u: { fu = 0; fv = 1; }
        default: { fu = 1; fv = 1; }
    }
    return regular_sample(
        transition_regular_index(side, cell_u, cell_v, fu, fv)
    );
}

fn transition_edge_vertex(
    side: u32,
    cell_u: i32,
    cell_v: i32,
    vd: u32,
) -> SurfaceVertex {
    let edge = vd & 0xFFu;
    let ai = (edge >> 4u) & 0xFu;
    let bi = edge & 0xFu;
    let a = transition_grid_vertex(side, cell_u, cell_v, ai);
    let b = transition_grid_vertex(side, cell_u, cell_v, bi);
    let da = transition_grid_density(side, cell_u, cell_v, ai);
    let db = transition_grid_density(side, cell_u, cell_v, bi);
    return interpolate_vertex(a.position, b.position, -a.normal, -b.normal, da, db);
}

fn write_vertex(slot: u32, vertex: SurfaceVertex) {
    let start = dispatch.ranges.x + slot * 8u;
    vertex_data[start + 0u] = vertex.position.x;
    vertex_data[start + 1u] = vertex.position.y;
    vertex_data[start + 2u] = vertex.position.z;
    vertex_data[start + 3u] = vertex.normal.x;
    vertex_data[start + 4u] = vertex.normal.y;
    vertex_data[start + 5u] = vertex.normal.z;
    vertex_data[start + 6u] =
        dispatch.descriptor.extent_uv_radius.y + vertex.position.x * 0.5;
    vertex_data[start + 7u] =
        dispatch.descriptor.extent_uv_radius.z + vertex.position.z * 0.5;
    index_data[dispatch.ranges.z + slot] = slot;
}

fn emit_triangle(a: SurfaceVertex, b: SurfaceVertex, c: SurfaceVertex) {
    let capacity = min(
        (dispatch.ranges.y - dispatch.ranges.x) / 8u,
        dispatch.ranges.w - dispatch.ranges.z,
    );
    let base = atomicAdd(&output_count, 3u);
    if base + 2u >= capacity {
        return;
    }
    write_vertex(base + 0u, a);
    write_vertex(base + 1u, b);
    write_vertex(base + 2u, c);
}

fn regular_case(cell: vec3<i32>) -> u32 {
    var result = 0u;
    for (var i = 0u; i < 8u; i += 1u) {
        if regular_sample(cell + regular_corner(i)) > 0.0 {
            result |= 1u << i;
        }
    }
    return result;
}

fn transition_case(cell_u: i32, cell_v: i32) -> u32 {
    let contributions = array<u32, 9>(
        0x01u, 0x02u, 0x04u,
        0x80u, 0x100u, 0x08u,
        0x40u, 0x20u, 0x10u,
    );
    var result = 0u;
    for (var i = 0u; i < 9u; i += 1u) {
        let du = u32(i % 3u);
        let dv = u32(i / 3u);
        let hu = u32(cell_u * 2) + du;
        let hv = u32(cell_v * 2) + dv;
        let density = scratch[face_scratch_index(side, hu, hv)];
        if density > 0.0 {
            result |= contributions[i];
        }
    }
    return result;
}

fn emit_regular_cell(cell_id: u32) {
    let x = i32(cell_id % BLOCK);
    let y = i32((cell_id / BLOCK) % BLOCK);
    let z = i32(cell_id / (BLOCK * BLOCK));
    let cell = vec3<i32>(x, y, z);
    let case_number = regular_case(cell);
    let cell_class = tables[REG_CLASS_OFFSET + case_number];
    let counts = tables[REG_COUNTS_OFFSET + cell_class];
    let triangle_count = counts & 0xFu;

    for (var t = 0u; t < triangle_count; t += 1u) {
        let tri_base = REG_TRI_OFFSET + cell_class * 15u + t * 3u;
        let ia = tables[tri_base + 0u];
        let ib = tables[tri_base + 1u];
        let ic = tables[tri_base + 2u];
        let vertices = REG_VERTEX_OFFSET + case_number * 12u;
        emit_triangle(
            regular_edge_vertex(cell, tables[vertices + ia]),
            regular_edge_vertex(cell, tables[vertices + ib]),
            regular_edge_vertex(cell, tables[vertices + ic]),
        );
    }
}

fn emit_transition_cell(side: u32, cell_id: u32) {
    let cell_u = i32(cell_id % BLOCK);
    let cell_v = i32(cell_id / BLOCK);
    let case_number = transition_case(cell_u, cell_v);
    let raw_class = tables[TRANS_CLASS_OFFSET + case_number];
    let cell_class = raw_class & 0x7Fu;
    let invert = (raw_class & 0x80u) != 0u;
    let counts = tables[TRANS_COUNTS_OFFSET + cell_class];
    let triangle_count = counts & 0xFu;
    let vertices = TRANS_VERTEX_OFFSET + case_number * 12u;

    for (var t = 0u; t < triangle_count; t += 1u) {
        let tri_base = TRANS_TRI_OFFSET + cell_class * 36u + t * 3u;
        let ia = tables[tri_base + 0u];
        let ib = tables[tri_base + 1u];
        let ic = tables[tri_base + 2u];

        let a = transition_edge_vertex(side, cell_u, cell_v, tables[vertices + ia]);
        let b = transition_edge_vertex(side, cell_u, cell_v, tables[vertices + ib]);
        let c = transition_edge_vertex(side, cell_u, cell_v, tables[vertices + ic]);

        // transvoxel_rs uses LowZ as its base orientation, so the table's
        // inversion bit is deliberately complemented in its transition path.
        if !invert {
            emit_triangle(a, b, c);
        } else {
            emit_triangle(c, b, a);
        }
    }
}



@compute @workgroup_size(64)
fn main(@builtin(local_invocation_index) lane: u32) {
    if lane == 0u {
        atomicStore(&output_count, 0u);
    }

    let index_capacity = dispatch.ranges.w - dispatch.ranges.z;
    var clear = lane;
    loop {
        if clear >= index_capacity {
            break;
        }
        index_data[dispatch.ranges.z + clear] = 0u;
        clear += 64u;
    }

    workgroupBarrier();

    var cell = lane;
    loop {
        if cell >= BLOCK * BLOCK * BLOCK {
            break;
        }
        emit_regular_cell(cell);
        cell += 64u;
    }

    workgroupBarrier();

    for (var side = 0u; side < 6u; side += 1u) {
        if !transition_enabled(side) {
            continue;
        }

        var transition_cell = lane;
        loop {
            if transition_cell >= BLOCK * BLOCK {
                break;
            }
            emit_transition_cell(side, transition_cell);
            transition_cell += 64u;
        }

        workgroupBarrier();
    }
}
