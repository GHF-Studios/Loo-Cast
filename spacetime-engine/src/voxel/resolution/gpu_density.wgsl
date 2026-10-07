// Volumetric sphere baseline in one bounded surface-relative chart.
// Density is positive inside; the canonical CPU signed distance is negative inside.

#import spacetime_engine::voxel::gpu_schema::{
    BLOCK, REGULAR_LATTICE, REGULAR_LATTICE_LEN,
    FACE_LATTICE, FACE_LATTICE_LEN,
    GpuTerrainDispatch,
}

@group(0) @binding(0) var<uniform> dispatch: GpuTerrainDispatch;
@group(0) @binding(1) var<storage, read_write> scratch: array<f32>;

fn sample_density(local_in_block: vec3<f32>) -> f32 {
    let anchor = dispatch.descriptor.anchor_direction_and_inverse_radius;
    let delta = dispatch.descriptor.chart_origin_and_spacing.xyz + local_in_block;
    let inv_r = max(anchor.w, 0.0);
    let q = anchor.xyz + delta * inv_r;
    let q_len = max(length(q), 1.0e-12);
    let numerator = 2.0 * dot(anchor.xyz, delta) + dot(delta, delta) * inv_r;
    return -numerator / (q_len + 1.0);
}


fn transition_enabled(side: u32) -> bool {
    return (dispatch.descriptor.terrain_meta.y & (1u << side)) != 0u;
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


const REGULAR_SCRATCH_BASE: u32 = 0u;
const FACE_SCRATCH_BASE: u32 = 729u;
const FACE_SAMPLE_STRIDE: u32 = 4u;

fn face_scratch_index(side: u32, face_sample: u32) -> u32 {
    return FACE_SCRATCH_BASE
        + (side * FACE_LATTICE_LEN + face_sample) * FACE_SAMPLE_STRIDE;
}

fn face_density(side: u32, hu: u32, hv: u32) -> f32 {
    let sample = hu + FACE_LATTICE * hv;
    return scratch[face_scratch_index(side, sample)];
}

fn face_difference_u(side: u32, hu: u32, hv: u32) -> f32 {
    let last = FACE_LATTICE - 1u;
    if hu == 0u {
        return 2.0 * (
            face_density(side, 1u, hv)
                - face_density(side, 0u, hv)
        );
    }
    if hu == last {
        return 2.0 * (
            face_density(side, last, hv)
                - face_density(side, last - 1u, hv)
        );
    }
    return face_density(side, hu + 1u, hv)
        - face_density(side, hu - 1u, hv);
}

fn face_difference_v(side: u32, hu: u32, hv: u32) -> f32 {
    let last = FACE_LATTICE - 1u;
    if hv == 0u {
        return 2.0 * (
            face_density(side, hu, 1u)
                - face_density(side, hu, 0u)
        );
    }
    if hv == last {
        return 2.0 * (
            face_density(side, hu, last)
                - face_density(side, hu, last - 1u)
        );
    }
    return face_density(side, hu, hv + 1u)
        - face_density(side, hu, hv - 1u);
}

fn transition_gradient(
    side: u32,
    hu: u32,
    hv: u32,
    position: vec3<f32>,
) -> vec3<f32> {
    let half_spacing =
        dispatch.descriptor.chart_origin_and_spacing.w * 0.5;
    let u = vec3<f32>(rotation_u(side));
    let v = vec3<f32>(rotation_v(side));
    let w = vec3<f32>(rotation_w(side));

    let du = face_difference_u(side, hu, hv);
    let dv = face_difference_v(side, hu, hv);
    let dw = sample_density(position + w * half_spacing)
        - sample_density(position - w * half_spacing);

    return u * du + v * dv + w * dw;
}

@compute @workgroup_size(64)

fn main(@builtin(local_invocation_index) lane: u32) {
    var sample = lane;
    loop {
        if sample >= REGULAR_LATTICE_LEN {
            break;
        }
        let x = i32(sample % REGULAR_LATTICE);
        let y = i32((sample / REGULAR_LATTICE) % REGULAR_LATTICE);
        let z = i32(sample / (REGULAR_LATTICE * REGULAR_LATTICE));
        let spacing = dispatch.descriptor.chart_origin_and_spacing.w;
        scratch[REGULAR_SCRATCH_BASE + sample] =
            sample_density(vec3<f32>(f32(x), f32(y), f32(z)) * spacing);
        sample += 64u;
    }

    for (var side = 0u; side < 6u; side += 1u) {
        if !transition_enabled(side) {
            continue;
        }

        var face_sample = lane;
        loop {
            if face_sample >= FACE_LATTICE_LEN {
                break;
            }
            let hu = face_sample % FACE_LATTICE;
            let hv = face_sample / FACE_LATTICE;
            let position =
                transition_position_half(side, i32(hu), i32(hv), 0);
            let out = face_scratch_index(side, face_sample);
            scratch[out] = sample_density(position);
            face_sample += 64u;
        }

        workgroupBarrier();
        storageBarrier();

        face_sample = lane;
        loop {
            if face_sample >= FACE_LATTICE_LEN {
                break;
            }
            let hu = face_sample % FACE_LATTICE;
            let hv = face_sample / FACE_LATTICE;
            let position =
                transition_position_half(side, i32(hu), i32(hv), 0);
            let gradient = transition_gradient(
                side,
                hu,
                hv,
                position,
            );
            let out = face_scratch_index(side, face_sample);
            scratch[out + 1u] = gradient.x;
            scratch[out + 2u] = gradient.y;
            scratch[out + 3u] = gradient.z;
            face_sample += 64u;
        }

        workgroupBarrier();
        storageBarrier();
    }
}
