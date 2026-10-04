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
const REGULAR_EDGE_VERTEX_COUNT: u32 = 1944u;
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
// gpu-terrain-transition-case-side-repair-v1
// gpu-terrain-frontier-stability-v1
// gpu-terrain-duplicate-entry-attribute-repair-v2
@group(0) @binding(0) var<uniform> dispatch: GpuTerrainDispatch;
@group(0) @binding(1) var<storage, read> tables: array<u32>;
@group(0) @binding(2) var<storage, read> scratch: array<f32>;
@group(0) @binding(3) var<storage, read_write> vertex_data: array<f32>;
@group(0) @binding(4) var<storage, read_write> index_data: array<u32>;

var<workgroup> index_count: atomic<u32>;
var<workgroup> transition_vertex_count: atomic<u32>;

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


fn regular_edge_slot(cell: vec3<i32>, vd: u32) -> u32 {
    let edge = vd & 0xFFu;
    let ai = (edge >> 4u) & 0xFu;
    let bi = edge & 0xFu;
    let a = cell + regular_corner(ai);
    let b = cell + regular_corner(bi);
    let p = vec3<i32>(
        min(a.x, b.x),
        min(a.y, b.y),
        min(a.z, b.z),
    );
    let d = abs(b - a);

    // 8*9*9 edges for each of the three lattice axes.
    if d.x != 0 {
        return u32(p.x + 8 * p.y + 8 * 9 * p.z);
    }
    if d.y != 0 {
        return 648u + u32(p.y + 8 * p.x + 8 * 9 * p.z);
    }
    return 1296u + u32(p.z + 8 * p.x + 8 * 9 * p.y);
}

fn regular_edge_vertex_from_slot(slot: u32) -> SurfaceVertex {
    var a = vec3<i32>(0);
    var b = vec3<i32>(0);

    if slot < 648u {
        let x = i32(slot % 8u);
        let q = slot / 8u;
        let y = i32(q % 9u);
        let z = i32(q / 9u);
        a = vec3<i32>(x, y, z);
        b = a + vec3<i32>(1, 0, 0);
    } else if slot < 1296u {
        let local = slot - 648u;
        let y = i32(local % 8u);
        let q = local / 8u;
        let x = i32(q % 9u);
        let z = i32(q / 9u);
        a = vec3<i32>(x, y, z);
        b = a + vec3<i32>(0, 1, 0);
    } else {
        let local = slot - 1296u;
        let z = i32(local % 8u);
        let q = local / 8u;
        let x = i32(q % 9u);
        let y = i32(q / 9u);
        a = vec3<i32>(x, y, z);
        b = a + vec3<i32>(0, 0, 1);
    }

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


fn write_surface_vertex(slot: u32, vertex: SurfaceVertex) {
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
}



fn write_triangle_indices(
    index_base: u32,
    a: u32,
    b: u32,
    c: u32,
) {
    let capacity = dispatch.ranges.w - dispatch.ranges.z;
    if index_base + 2u >= capacity {
        return;
    }
    index_data[dispatch.ranges.z + index_base + 0u] = a;
    index_data[dispatch.ranges.z + index_base + 1u] = b;
    index_data[dispatch.ranges.z + index_base + 2u] = c;
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

fn transition_case(side: u32, cell_u: i32, cell_v: i32) -> u32 {
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

        let index_base = atomicAdd(&index_count, 3u);
        write_triangle_indices(
            index_base,
            regular_edge_slot(cell, tables[vertices + ia]),
            regular_edge_slot(cell, tables[vertices + ib]),
            regular_edge_slot(cell, tables[vertices + ic]),
        );
    }
}



fn emit_transition_cell(side: u32, cell_id: u32) {
    let cell_u = i32(cell_id % BLOCK);
    let cell_v = i32(cell_id / BLOCK);
    let case_number = transition_case(side, cell_u, cell_v);
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

        let a = transition_edge_vertex(
            side,
            cell_u,
            cell_v,
            tables[vertices + ia],
        );
        let b = transition_edge_vertex(
            side,
            cell_u,
            cell_v,
            tables[vertices + ib],
        );
        let c = transition_edge_vertex(
            side,
            cell_u,
            cell_v,
            tables[vertices + ic],
        );

        let index_base = atomicAdd(&index_count, 3u);
        let transition_base =
            atomicAdd(&transition_vertex_count, 3u);
        let vertex_base =
            REGULAR_EDGE_VERTEX_COUNT + transition_base;

        let vertex_capacity =
            (dispatch.ranges.y - dispatch.ranges.x) / 8u;
        let index_capacity =
            dispatch.ranges.w - dispatch.ranges.z;
        if vertex_base + 2u >= vertex_capacity
            || index_base + 2u >= index_capacity
        {
            continue;
        }

        if !invert {
            write_surface_vertex(vertex_base + 0u, a);
            write_surface_vertex(vertex_base + 1u, b);
            write_surface_vertex(vertex_base + 2u, c);
        } else {
            write_surface_vertex(vertex_base + 0u, c);
            write_surface_vertex(vertex_base + 1u, b);
            write_surface_vertex(vertex_base + 2u, a);
        }

        write_triangle_indices(
            index_base,
            vertex_base + 0u,
            vertex_base + 1u,
            vertex_base + 2u,
        );
    }
}




@compute @workgroup_size(64)
fn main(@builtin(local_invocation_index) lane: u32) {
    if lane == 0u {
        atomicStore(&index_count, 0u);
        atomicStore(&transition_vertex_count, 0u);
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

    // Every regular Transvoxel vertex lies on one of the 1,944 unique edges
    // of the 9^3 block lattice. Populate each edge slot once, then topology
    // emits only indices into these shared vertices.
    var edge_slot = lane;
    loop {
        if edge_slot >= REGULAR_EDGE_VERTEX_COUNT {
            break;
        }
        write_surface_vertex(
            edge_slot,
            regular_edge_vertex_from_slot(edge_slot),
        );
        edge_slot += 64u;
    }

    workgroupBarrier();
    storageBarrier();

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

