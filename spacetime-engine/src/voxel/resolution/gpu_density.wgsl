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
@group(0) @binding(1) var<storage, read_write> scratch: array<f32>;

fn hash_noise_3d(cell: vec3<i32>, seed: u32) -> f32 {
    var value = seed
        ^ (bitcast<u32>(cell.x) * 0x9E3779B9u)
        ^ (bitcast<u32>(cell.y) * 0x85EBCA6Bu)
        ^ (bitcast<u32>(cell.z) * 0xC2B2AE35u);
    value = value ^ (value >> 16u);
    value = value * 0x7FEB352Du;
    value = value ^ (value >> 15u);
    value = value * 0x846CA68Bu;
    value = value ^ (value >> 16u);
    return (f32(value) / 4294967295.0) * 2.0 - 1.0;
}

fn value_noise_3d(point: vec3<f32>, seed: u32) -> f32 {
    let floor_point = floor(point);
    let cell = vec3<i32>(floor_point);
    let fraction = point - floor_point;
    let hermite = fraction * fraction * (vec3<f32>(3.0) - fraction * 2.0);

    let c000 = hash_noise_3d(cell + vec3<i32>(0, 0, 0), seed);
    let c100 = hash_noise_3d(cell + vec3<i32>(1, 0, 0), seed);
    let c010 = hash_noise_3d(cell + vec3<i32>(0, 1, 0), seed);
    let c110 = hash_noise_3d(cell + vec3<i32>(1, 1, 0), seed);
    let c001 = hash_noise_3d(cell + vec3<i32>(0, 0, 1), seed);
    let c101 = hash_noise_3d(cell + vec3<i32>(1, 0, 1), seed);
    let c011 = hash_noise_3d(cell + vec3<i32>(0, 1, 1), seed);
    let c111 = hash_noise_3d(cell + vec3<i32>(1, 1, 1), seed);

    let x00 = mix(c000, c100, hermite.x);
    let x10 = mix(c010, c110, hermite.x);
    let x01 = mix(c001, c101, hermite.x);
    let x11 = mix(c011, c111, hermite.x);
    return mix(mix(x00, x10, hermite.y), mix(x01, x11, hermite.y), hermite.z);
}

fn mix_semantic(state_in: u32, input: u32) -> u32 {
    var state = state_in;
    state = state ^ (input * 0x85EBCA6Bu);
    state = state ^ (state >> 16u);
    state = state * 0x7FEB352Du;
    state = state ^ (state >> 15u);
    return state;
}

fn floor_div_10(value: i32) -> i32 {
    if value >= 0 {
        return value / 10;
    }
    return -((-value + 9) / 10);
}

fn packed_digit(band: GpuFineBand, axis: u32, index: u32) -> i32 {
    var pack = band.digits_x[index / 4u];
    if axis == 1u {
        pack = band.digits_y[index / 4u];
    } else if axis == 2u {
        pack = band.digits_z[index / 4u];
    }
    switch index & 3u {
        case 0u: { return pack.x; }
        case 1u: { return pack.y; }
        case 2u: { return pack.z; }
        default: { return pack.w; }
    }
}

fn canonical_f32_bits(value: f32) -> u32 {
    if value == 0.0 {
        return 0u;
    }
    return bitcast<u32>(value);
}

fn semantic_corner(
    band: GpuFineBand,
    chunk_adjust: vec3<i32>,
    local_offset: vec3<f32>,
    broad: bool,
) -> f32 {
    var dx: array<i32, 20>;
    var dy: array<i32, 20>;
    var dz: array<i32, 20>;
    let digit_len = u32(max(band.band_meta.y, 0));

    var i = 0u;
    loop {
        if i >= digit_len || i >= HASH_DIGITS {
            break;
        }
        dx[i] = packed_digit(band, 0u, i);
        dy[i] = packed_digit(band, 1u, i);
        dz[i] = packed_digit(band, 2u, i);
        i += 1u;
    }

    var carry_x = chunk_adjust.x;
    var carry_y = chunk_adjust.y;
    var carry_z = chunk_adjust.z;
    i = 0u;
    loop {
        if i >= digit_len || i >= HASH_DIGITS {
            break;
        }

        var tx = dx[i] + carry_x;
        var ty = dy[i] + carry_y;
        var tz = dz[i] + carry_z;

        carry_x = floor_div_10(tx + 5);
        carry_y = floor_div_10(ty + 5);
        carry_z = floor_div_10(tz + 5);

        dx[i] = tx - carry_x * 10;
        dy[i] = ty - carry_y * 10;
        dz[i] = tz - carry_z * 10;
        i += 1u;
    }

    var state = select(band.prefix.y, band.prefix.x, broad);
    var reverse = i32(digit_len) - 1;
    loop {
        if reverse < 0 {
            break;
        }
        let r = u32(reverse);
        state = mix_semantic(state, bitcast<u32>(dx[r]));
        state = mix_semantic(state, bitcast<u32>(dy[r]));
        state = mix_semantic(state, bitcast<u32>(dz[r]));
        reverse -= 1;
    }

    state = mix_semantic(state, canonical_f32_bits(local_offset.x));
    state = mix_semantic(state, canonical_f32_bits(local_offset.y));
    state = mix_semantic(state, canonical_f32_bits(local_offset.z));
    return (f32(state) / 4294967295.0) * 2.0 - 1.0;
}

fn semantic_value_noise(
    band: GpuFineBand,
    delta_native: vec3<f32>,
    cell_size: f32,
    broad: bool,
) -> f32 {
    let native_offset = band.base_offset.xyz + delta_native;
    let lower = floor(native_offset / cell_size) * cell_size;
    let fraction = (native_offset - lower) / cell_size;
    let hermite = fraction * fraction * (vec3<f32>(3.0) - fraction * 2.0);

    var corners: array<f32, 8>;
    var corner = 0u;
    loop {
        if corner >= 8u {
            break;
        }
        let bits = vec3<i32>(
            i32(corner & 1u),
            i32((corner >> 1u) & 1u),
            i32((corner >> 2u) & 1u),
        );
        let raw = lower + vec3<f32>(bits) * cell_size;
        let chunk_adjust = vec3<i32>(floor((raw + 500.0) / 1000.0));
        let local_offset = raw - vec3<f32>(chunk_adjust) * 1000.0;
        corners[corner] = semantic_corner(
            band,
            chunk_adjust,
            local_offset,
            broad,
        );
        corner += 1u;
    }

    let x00 = mix(corners[0], corners[1], hermite.x);
    let x10 = mix(corners[2], corners[3], hermite.x);
    let x01 = mix(corners[4], corners[5], hermite.x);
    let x11 = mix(corners[6], corners[7], hermite.x);
    return mix(mix(x00, x10, hermite.y), mix(x01, x11, hermite.y), hermite.z);
}

fn fine_noise(band: GpuFineBand, delta_ref_metres: vec3<f32>) -> f32 {
    let delta_native = delta_ref_metres / max(band.params.x, 1.0e-30);
    let broad = semantic_value_noise(band, delta_native, 20.0, true);
    let fine = semantic_value_noise(band, delta_native, 5.0, false);
    return broad * 0.72 + fine * 0.28;
}

fn scale_mix(state_in: u32, input: u32) -> u32 {
    var state = state_in;
    state = state ^ (input * 0x85EBCA6Bu);
    state = state ^ (state >> 16u);
    state = state * 0x7FEB352Du;
    state = state ^ (state >> 15u);
    return state;
}

fn scale_layer_seed(seed: u32, exponent: i32) -> u32 {
    return scale_mix(seed ^ 0xA17E5CA1u, u32(exponent + 35));
}

fn rocky_band_sample(
    direction: vec3<f32>,
    introduced: i32,
    amplitude: f32,
    frequency: f32,
    salt: u32,
    shape: u32,
    body_seed: u32,
) -> f32 {
    if dispatch.descriptor.semantic_meta.x > introduced {
        return 0.0;
    }
    let keyed = scale_layer_seed(body_seed ^ salt, introduced);
    var value = 0.0;

    switch shape {
        case 0u: {
            value = value_noise_3d(
                direction * frequency + vec3<f32>(17.1, -9.4, 4.7),
                keyed ^ 0x9E3779B9u,
            );
        }
        case 1u: {
            let n = value_noise_3d(
                direction * frequency + vec3<f32>(-12.7, 21.3, 8.6),
                keyed ^ 0xC2B2AE35u,
            );
            value = n * (0.35 + 0.65 * abs(n));
        }
        case 2u: {
            let carrier = value_noise_3d(
                direction * frequency + vec3<f32>(5.9, 13.4, -18.1),
                keyed ^ 0x27D4EB2Du,
            );
            let ridge = pow(max(1.0 - abs(carrier), 0.0), 3.0);
            let phase = f32(keyed) / 4294967295.0 * 6.28318530718;
            let polarity = sin(
                dot(direction, normalize(vec3<f32>(0.73, -0.31, 0.61))) * 5.3
                    + phase
            );
            value = clamp(ridge * polarity * 1.35, -1.0, 1.0);
        }
        case 3u: {
            value = value_noise_3d(
                direction * frequency + vec3<f32>(-7.8, -3.1, 11.6),
                keyed ^ 0xD3A2646Cu,
            );
        }
        case 4u: {
            let carrier = value_noise_3d(
                direction * frequency + vec3<f32>(19.7, -4.3, 12.8),
                keyed ^ 0xA24BAED4u,
            );
            let ridge = pow(max(1.0 - abs(carrier), 0.0), 6.0);
            let envelope = value_noise_3d(
                direction * (frequency * 0.37) + vec3<f32>(-8.1, 15.6, 2.9),
                keyed ^ 0x9FB21C65u,
            );
            let envelope01 = clamp(envelope * 0.5 + 0.5, 0.0, 1.0);
            value = clamp(ridge * (0.20 + envelope01 * 0.80), 0.0, 1.0);
        }
        case 5u: {
            let carrier = value_noise_3d(
                direction * frequency + vec3<f32>(-14.2, 6.7, 22.1),
                keyed ^ 0x165667B1u,
            );
            let line = pow(max(1.0 - abs(carrier), 0.0), 7.0);
            let envelope = value_noise_3d(
                direction * (frequency * 0.29) + vec3<f32>(3.4, -18.9, 7.2),
                keyed ^ 0xD4EB2F6Au,
            );
            let envelope01 = clamp(envelope * 0.5 + 0.5, 0.0, 1.0);
            value = -clamp(line * (0.25 + envelope01 * 0.75), 0.0, 1.0);
        }
        default: {
            let carrier = value_noise_3d(
                direction * frequency + vec3<f32>(4.8, 9.2, -16.7),
                keyed ^ 0x85EBCA77u,
            );
            let ridge = pow(max(1.0 - abs(carrier), 0.0), 4.0);
            let polarity = value_noise_3d(
                direction * (frequency * 0.41) + vec3<f32>(-11.5, 1.8, 5.6),
                keyed ^ 0xC2B2AE3Du,
            );
            value = clamp(ridge * polarity, -1.0, 1.0);
        }
    }
    return clamp(value, -1.0, 1.0) * amplitude;
}

fn rocky_macro(direction: vec3<f32>, seed: u32) -> f32 {
    var relief = 0.0;
    relief += rocky_band_sample(direction, 6, 4200.0, 1.35, 0x50524F56u, 0u, seed);
    relief += rocky_band_sample(direction, 5, 3200.0, 2.60, 0x504C4154u, 1u, seed);
    relief += rocky_band_sample(direction, 5, 6200.0, 4.20, 0x4F524F47u, 2u, seed);
    relief += rocky_band_sample(direction, 4, 1400.0, 8.00, 0x5245474Eu, 3u, seed);
    relief += rocky_band_sample(direction, 3, 11000.0, 15.0, 0x414C504Eu, 4u, seed);
    relief += rocky_band_sample(direction, 3, 8000.0, 21.0, 0x43414E59u, 5u, seed);
    relief += rocky_band_sample(direction, 2, 2600.0, 42.0, 0x52494447u, 6u, seed);

    let province = value_noise_3d(
        direction * 2.4 + vec3<f32>(7.3, -11.8, 4.1),
        seed ^ 0x50524F56u,
    );
    var exaggerated = province * 8000.0;

    if dispatch.descriptor.semantic_meta.x <= 3 {
        let alpine_carrier = value_noise_3d(
            direction * 10.0 + vec3<f32>(-17.2, 6.9, 12.4),
            seed ^ 0x414C504Eu,
        );
        let alpine_ridge = pow(max(1.0 - abs(alpine_carrier), 0.0), 9.0);
        let alpine_envelope = clamp(
            value_noise_3d(
                direction * 3.7 + vec3<f32>(3.1, 19.6, -8.8),
                seed ^ 0xA1F14E55u,
            ) * 0.5 + 0.5,
            0.0,
            1.0,
        );

        let canyon_carrier = value_noise_3d(
            direction * 15.0 + vec3<f32>(14.2, -4.7, -16.5),
            seed ^ 0x43414E59u,
        );
        let canyon_line = pow(max(1.0 - abs(canyon_carrier), 0.0), 9.0);
        let canyon_envelope = clamp(
            value_noise_3d(
                direction * 4.3 + vec3<f32>(-9.9, 5.4, 21.1),
                seed ^ 0x52494654u,
            ) * 0.5 + 0.5,
            0.0,
            1.0,
        );

        let massif_carrier = value_noise_3d(
            direction * 14.5 + vec3<f32>(22.4, 7.7, -3.6),
            seed ^ 0x4D415353u,
        );
        let massif_cross = pow(max(1.0 - abs(massif_carrier), 0.0), 8.0);
        let massif = alpine_ridge * massif_cross;

        exaggerated += alpine_ridge * alpine_envelope * 30000.0
            + massif * alpine_envelope * 16000.0
            - canyon_line * canyon_envelope * 24000.0;
    }

    if dispatch.descriptor.semantic_meta.x <= 2 {
        let serration = value_noise_3d(
            direction * 32.0 + vec3<f32>(1.7, -13.3, 9.2),
            seed ^ 0x53455252u,
        );
        exaggerated += sign(serration) * serration * serration * 5000.0;
    }
    return relief + clamp(exaggerated, -38000.0, 48000.0);
}

fn lunar_relative(direction: vec3<f32>) -> f32 {
    var height =
        sin(dot(direction, vec3<f32>(1.7, -2.3, 0.9)) * 5.0) * 0.0014
        + sin(dot(direction, vec3<f32>(-3.1, 0.7, 2.4)) * 8.0) * 0.0008;

    let centers = array<vec3<f32>, 8>(
        vec3<f32>(0.82, 0.21, 0.53),
        vec3<f32>(-0.51, 0.70, 0.49),
        vec3<f32>(0.18, -0.88, 0.44),
        vec3<f32>(-0.77, -0.23, -0.59),
        vec3<f32>(0.39, 0.48, -0.79),
        vec3<f32>(-0.08, -0.35, 0.93),
        vec3<f32>(0.63, -0.66, -0.40),
        vec3<f32>(-0.33, 0.14, -0.93),
    );
    let radii = array<f32, 8>(0.36, 0.27, 0.23, 0.19, 0.16, 0.13, 0.11, 0.095);
    let depths = array<f32, 8>(0.0100, 0.0070, 0.0060, 0.0048, 0.0040, 0.0034, 0.0028, 0.0024);

    for (var i = 0u; i < 8u; i += 1u) {
        let center = normalize(centers[i]);
        let q = distance(direction, center) / radii[i];
        if q < 1.0 {
            let bowl = 1.0 - q * q;
            height -= depths[i] * bowl * bowl;
        }
        let rim_distance = abs((q - 1.0) / 0.22);
        if rim_distance < 1.0 {
            let rim = 1.0 - rim_distance;
            height += depths[i] * 0.28 * rim * rim;
        }
    }
    return height;
}

fn macro_relief(direction: vec3<f32>) -> f32 {
    let profile = dispatch.descriptor.terrain_meta.x;
    let seed = dispatch.descriptor.seed_meta.x;
    if profile == 1u {
        return rocky_macro(direction, seed);
    }
    if profile == 0u {
        return dispatch.descriptor.extent_uv_radius.w * lunar_relative(direction);
    }
    let phase = f32(seed) / 4294967295.0 * 6.28318530718;
    return dispatch.descriptor.extent_uv_radius.w
        * sin(dot(direction, vec3<f32>(0.7, 1.3, -1.1)) * 5.0 + phase)
        * 0.00035;
}

fn coarse_relief(direction: vec3<f32>) -> f32 {
    var result = 0.0;
    let count = min(dispatch.descriptor.terrain_meta.z, MAX_COARSE_BANDS);
    var i = 0u;
    loop {
        if i >= count {
            break;
        }
        let band = dispatch.descriptor.coarse[i];
        let p = direction * band.params.y;
        let broad = value_noise_3d(
            p + vec3<f32>(13.7, -7.1, 3.9),
            band.seeds.x ^ 0xA341316Cu,
        );
        let fine = value_noise_3d(
            p * 2.31 + vec3<f32>(-5.3, 11.9, 17.2),
            band.seeds.x ^ 0xC8013EA4u,
        );
        result += (broad * 0.72 + fine * 0.28) * band.params.x;
        i += 1u;
    }
    return result;
}

fn cave_value(chart: GpuCaveChart, delta_metres: vec3<f32>) -> f32 {
    let wavelength = max(chart.fraction_and_wavelength.w, 1.0e-6);
    let q = chart.fraction_and_wavelength.xyz + delta_metres / wavelength;
    let carry_f = floor(q);
    let cell = chart.base_cell.xyz + vec3<i32>(carry_f);
    let fraction = q - carry_f;
    let hermite = fraction * fraction * (vec3<f32>(3.0) - fraction * 2.0);

    let seed = chart.seed.x;
    let c000 = hash_noise_3d(cell + vec3<i32>(0, 0, 0), seed);
    let c100 = hash_noise_3d(cell + vec3<i32>(1, 0, 0), seed);
    let c010 = hash_noise_3d(cell + vec3<i32>(0, 1, 0), seed);
    let c110 = hash_noise_3d(cell + vec3<i32>(1, 1, 0), seed);
    let c001 = hash_noise_3d(cell + vec3<i32>(0, 0, 1), seed);
    let c101 = hash_noise_3d(cell + vec3<i32>(1, 0, 1), seed);
    let c011 = hash_noise_3d(cell + vec3<i32>(0, 1, 1), seed);
    let c111 = hash_noise_3d(cell + vec3<i32>(1, 1, 1), seed);

    let x00 = mix(c000, c100, hermite.x);
    let x10 = mix(c010, c110, hermite.x);
    let x01 = mix(c001, c101, hermite.x);
    let x11 = mix(c011, c111, hermite.x);
    return mix(mix(x00, x10, hermite.y), mix(x01, x11, hermite.y), hermite.z);
}

fn cave_void_sdf(delta_metres: vec3<f32>, outer_density: f32) -> f32 {
    if outer_density < -12.0 {
        return -12.0 - outer_density;
    }
    if outer_density > 2400.0 {
        return outer_density - 2400.0;
    }

    let a = abs(cave_value(dispatch.descriptor.caves[0], delta_metres));
    let b = abs(cave_value(dispatch.descriptor.caves[1], delta_metres));
    let major = max(a - 0.23, b - 0.21) * 145.0;

    let c = abs(cave_value(dispatch.descriptor.caves[2], delta_metres));
    let d = abs(cave_value(dispatch.descriptor.caves[3], delta_metres));
    let branching = max(c - 0.20, d - 0.18) * 90.0;

    let chamber = (cave_value(dispatch.descriptor.caves[4], delta_metres) + 0.58) * 110.0;
    var raw = min(min(major, branching), chamber);
    raw = max(raw, -12.0 - outer_density);
    raw = max(raw, outer_density - 2400.0);
    return raw;
}

fn sample_density(local_in_block: vec3<f32>) -> f32 {
    let anchor = dispatch.descriptor.anchor_direction_and_inverse_radius;
    let n0 = anchor.xyz;
    let inv_r = max(anchor.w, 0.0);
    let delta = dispatch.descriptor.chart_origin_and_spacing.xyz + local_in_block;

    let a = dot(n0, delta);
    let delta2 = dot(delta, delta);
    let q = n0 + delta * inv_r;
    let q_len = max(length(q), 1.0e-12);
    let direction = q / q_len;

    // Exact local-chart forms that avoid multiplying or adding an absolute
    // planet radius:
    //   radial_delta = R * (|q| - 1)
    //   sphere_delta = R * (direction - n0)
    let numerator = 2.0 * a + delta2 * inv_r;
    let radial_delta = numerator / (q_len + 1.0);
    let sphere_scalar = -numerator / (q_len * (q_len + 1.0));
    let sphere_delta = delta / q_len + n0 * sphere_scalar;

    let pre_fine_delta =
        (macro_relief(direction) - macro_relief(n0))
        + (coarse_relief(direction) - coarse_relief(n0));

    let delta_ref = sphere_delta + direction * pre_fine_delta;
    var fine_delta = 0.0;
    let fine_count = min(dispatch.descriptor.terrain_meta.w, MAX_FINE_BANDS);
    var i = 0u;
    loop {
        if i >= fine_count {
            break;
        }
        let band = dispatch.descriptor.fine[i];
        fine_delta += fine_noise(band, delta_ref) * band.params.y;
        i += 1u;
    }

    let outer_density = pre_fine_delta + fine_delta - radial_delta;
    if dispatch.descriptor.semantic_meta.y != 0
        && dispatch.descriptor.terrain_meta.x == 1u
    {
        return min(outer_density, cave_void_sdf(delta, outer_density));
    }
    return outer_density;
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

            let hu = i32(face_sample % FACE_LATTICE);
            let hv = i32(face_sample / FACE_LATTICE);
            let position = transition_position_half(side, hu, hv, 0);
            let half_spacing =
                dispatch.descriptor.chart_origin_and_spacing.w * 0.5;

            let density = sample_density(position);
            let gradient = vec3<f32>(
                sample_density(position + vec3<f32>(half_spacing, 0.0, 0.0))
                    - sample_density(position - vec3<f32>(half_spacing, 0.0, 0.0)),
                sample_density(position + vec3<f32>(0.0, half_spacing, 0.0))
                    - sample_density(position - vec3<f32>(0.0, half_spacing, 0.0)),
                sample_density(position + vec3<f32>(0.0, 0.0, half_spacing))
                    - sample_density(position - vec3<f32>(0.0, 0.0, half_spacing)),
            );

            let out = face_scratch_index(side, face_sample);
            scratch[out + 0u] = density;
            scratch[out + 1u] = gradient.x;
            scratch[out + 2u] = gradient.y;
            scratch[out + 3u] = gradient.z;

            face_sample += 64u;
        }
    }
}
