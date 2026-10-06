//! CPU projection of canonical field parameters into one bounded GPU block descriptor.

use super::*;

fn profile_id(profile: CelestialBodyProfile) -> u32 {
    match profile {
        CelestialBodyProfile::Lunar => 0,
        CelestialBodyProfile::Rocky => 1,
        CelestialBodyProfile::Stellar => 2,
    }
}

fn detail_parameters(profile: CelestialBodyProfile) -> (f64, f64, f64, u32) {
    match profile {
        CelestialBodyProfile::Lunar => (0.82, 0.040, 1.70, 0x4C55_4E41),
        CelestialBodyProfile::Rocky => (0.72, 0.0025, 1.65, 0x524F_434B),
        CelestialBodyProfile::Stellar => (0.48, 0.008, 1.30, 0x5354_4152),
    }
}

#[inline]
fn mix_u32(mut state: u32, input: u32) -> u32 {
    state ^= input.wrapping_mul(0x85EB_CA6B);
    state ^= state >> 16;
    state = state.wrapping_mul(0x7FEB_352D);
    state ^= state >> 15;
    state
}

fn scale_layer_seed(seed: u32, exponent: i8) -> u32 {
    mix_u32(
        seed ^ 0xA17E_5CA1,
        (i32::from(exponent) - i32::from(SPATIAL_SCALE_MIN)) as u32,
    )
}

fn zero_prefix_state(seed: u32, zero_scale_count: usize) -> u32 {
    let mut state = seed ^ 0x517C_C1B7;
    for _ in 0..zero_scale_count {
        state = mix_u32(state, 0);
        state = mix_u32(state, 0);
        state = mix_u32(state, 0);
    }
    state
}

fn canonical_axis(value: f64) -> Option<(i64, f32)> {
    const CHUNK: f64 = 1_000.0;
    const HALF: f64 = 500.0;

    if !value.is_finite() {
        return None;
    }
    let carry_f = ((value + HALF) / CHUNK).floor();
    if carry_f < i64::MIN as f64 || carry_f > i64::MAX as f64 {
        return None;
    }
    let mut chunk = carry_f as i64;
    let remainder = value - chunk as f64 * CHUNK;
    let stored = remainder as f32;
    if !stored.is_finite() {
        return None;
    }
    let carry2 = ((f64::from(stored) + HALF) / CHUNK).floor() as i64;
    let mut local = (f64::from(stored) - carry2 as f64 * CHUNK) as f32;
    chunk = chunk.checked_add(carry2)?;

    if local >= 500.0 {
        local -= 1_000.0;
        chunk = chunk.checked_add(1)?;
    } else if local < -500.0 {
        local += 1_000.0;
        chunk = chunk.checked_sub(1)?;
    }
    Some((chunk, local))
}

fn balanced_digits(mut value: i64, count: usize) -> Option<[i32; HASH_DIGIT_WINDOW]> {
    let mut out = [0_i32; HASH_DIGIT_WINDOW];
    for slot in out.iter_mut().take(count) {
        let parent = (i128::from(value) + 5).div_euclid(10);
        let digit = i128::from(value) - parent * 10;
        if !(-5..5).contains(&digit) {
            return None;
        }
        *slot = digit as i32;
        value = i64::try_from(parent).ok()?;
    }
    (value == 0).then_some(out)
}

fn pack_digits(values: [i32; HASH_DIGIT_WINDOW]) -> [IVec4; HASH_DIGIT_PACKS] {
    std::array::from_fn(|pack| {
        let base = pack * 4;
        IVec4::new(
            values[base],
            values[base + 1],
            values[base + 2],
            values[base + 3],
        )
    })
}

fn fine_band(
    anchor_pre_fine_surface: DVec3,
    level: SpatialScale,
    amplitude_metres: f64,
    seed: u32,
) -> Option<GpuFineBand> {
    let metres_per_native = level.metres_per_native();
    let native = anchor_pre_fine_surface / metres_per_native;
    let (cx, ox) = canonical_axis(native.x)?;
    let (cy, oy) = canonical_axis(native.y)?;
    let (cz, oz) = canonical_axis(native.z)?;

    let digit_count = (i16::from(SPATIAL_SCALE_MAX) - i16::from(level.exponent()) + 1) as usize;
    let digit_len = digit_count.min(HASH_DIGIT_WINDOW);
    let prefix_count = digit_count.saturating_sub(digit_len);

    let x = balanced_digits(cx, digit_len)?;
    let y = balanced_digits(cy, digit_len)?;
    let z = balanced_digits(cz, digit_len)?;

    let broad_seed = seed ^ 0xA341_316C;
    let fine_seed = seed ^ 0xC801_3EA4;

    Some(GpuFineBand {
        params: Vec4::new(metres_per_native as f32, amplitude_metres as f32, 0.0, 0.0),
        base_offset: Vec4::new(ox, oy, oz, 0.0),
        meta: IVec4::new(i32::from(level.exponent()), digit_len as i32, 0, 0),
        prefix: UVec4::new(
            zero_prefix_state(broad_seed, prefix_count),
            zero_prefix_state(fine_seed, prefix_count),
            0,
            0,
        ),
        digits_x: pack_digits(x),
        digits_y: pack_digits(y),
        digits_z: pack_digits(z),
    })
}

fn cave_chart(anchor: DVec3, wavelength: f32, offset: Vec3, seed: u32) -> Option<GpuCaveChart> {
    let phase = anchor / f64::from(wavelength)
        + DVec3::new(
            f64::from(offset.x),
            f64::from(offset.y),
            f64::from(offset.z),
        );
    let floor = phase.floor();
    let cell = IVec4::new(
        i32::try_from(floor.x as i64).ok()?,
        i32::try_from(floor.y as i64).ok()?,
        i32::try_from(floor.z as i64).ok()?,
        0,
    );
    let fraction = phase - floor;
    Some(GpuCaveChart {
        base_cell: cell,
        fraction_and_wavelength: Vec4::new(
            fraction.x as f32,
            fraction.y as f32,
            fraction.z as f32,
            wavelength,
        ),
        seed: UVec4::new(seed, 0, 0, 0),
    })
}

fn finite_vec3(value: DVec3) -> Option<Vec3> {
    let out = Vec3::new(value.x as f32, value.y as f32, value.z as f32);
    out.is_finite().then_some(out)
}

#[derive(Debug, Clone, Copy)]
struct GpuTerrainReference {
    center: DVec3,
    anchor_direction: Vec3,
    anchor_pre_fine_surface: DVec3,
    anchor_radius: f64,
    chart_origin_delta: Vec3,
    extent_f32: f32,
    spacing_f32: f32,
}

fn terrain_reference(
    field: CelestialVoxelField,
    origin_local_metres: DVec3,
    extent_metres: f64,
    spacing_metres: f64,
) -> Option<GpuTerrainReference> {
    if !origin_local_metres.is_finite()
        || !extent_metres.is_finite()
        || extent_metres <= 0.0
        || !spacing_metres.is_finite()
        || spacing_metres <= 0.0
        || extent_metres > f64::from(f32::MAX)
    {
        return None;
    }

    let center = origin_local_metres + DVec3::splat(extent_metres * 0.5);
    let center_radius = center.length();
    let mut anchor_direction = if center_radius > f64::EPSILON {
        finite_vec3(center / center_radius)?.normalize_or_zero()
    } else {
        Vec3::Y
    };
    if anchor_direction == Vec3::ZERO {
        anchor_direction = Vec3::Y;
    }

    let sampler = field.presentation_sampler(spacing_metres)?;
    let anchor_pre_fine_surface = sampler
        .pre_fine_surface_local_metres(anchor_direction)
        .ok()?;
    let anchor_radius = anchor_pre_fine_surface.length();
    if !anchor_radius.is_finite() || anchor_radius <= f64::EPSILON {
        return None;
    }

    let chart_origin_delta = finite_vec3(origin_local_metres - anchor_pre_fine_surface)?;
    let extent_f32 = extent_metres as f32;
    let spacing_f32 = spacing_metres as f32;
    if !extent_f32.is_finite() || !spacing_f32.is_finite() {
        return None;
    }

    debug_assert!(anchor_direction.is_finite());
    debug_assert!(chart_origin_delta.is_finite());

    Some(GpuTerrainReference {
        center,
        anchor_direction,
        anchor_pre_fine_surface,
        anchor_radius,
        chart_origin_delta,
        extent_f32,
        spacing_f32,
    })
}

fn build_coarse_bands(
    field: CelestialVoxelField,
    floor: i8,
    root: i8,
) -> Option<([GpuCoarseBand; MAX_COARSE_BANDS], usize)> {
    let mut bands = [GpuCoarseBand::default(); MAX_COARSE_BANDS];
    let lower = floor.max(1);
    if lower > root {
        return Some((bands, 0));
    }

    let (frequency_factor, amplitude, growth, salt) = detail_parameters(field.profile());
    let mut count = 0usize;

    for raw in (lower..=root).rev() {
        let level = SpatialScale::new(raw)?;
        let metres_per_native = level.metres_per_native();
        let depth = i32::from(root - raw).max(0);
        let amplitude_native = amplitude * growth.powi(depth);
        let angular_frequency = (field.radius_metres() / metres_per_native * frequency_factor)
            .max(4.0)
            .min(f64::from(f32::MAX)) as f32;

        let slot = bands.get_mut(count)?;
        *slot = GpuCoarseBand {
            params: Vec4::new(
                (amplitude_native * metres_per_native) as f32,
                angular_frequency,
                0.0,
                0.0,
            ),
            seeds: UVec4::new(scale_layer_seed(field.seed() ^ salt, raw), 0, 0, 0),
        };
        count += 1;
    }

    debug_assert!(count <= MAX_COARSE_BANDS);
    debug_assert!(bands.iter().take(count).all(|band| band.params.is_finite()));
    Some((bands, count))
}

fn build_fine_bands(
    field: CelestialVoxelField,
    anchor_pre_fine_surface: DVec3,
    floor: i8,
    root: i8,
) -> Option<([GpuFineBand; MAX_FINE_BANDS], usize)> {
    let mut bands = [GpuFineBand::default(); MAX_FINE_BANDS];
    let upper = root.min(0);
    if floor > upper {
        return Some((bands, 0));
    }

    let (_, amplitude, growth, salt) = detail_parameters(field.profile());
    let mut count = 0usize;

    for raw in (floor..=upper).rev() {
        let level = SpatialScale::new(raw)?;
        let metres_per_native = level.metres_per_native();
        let depth = i32::from(root - raw).max(0);
        let amplitude_native = amplitude * growth.powi(depth);
        let band = fine_band(
            anchor_pre_fine_surface,
            level,
            amplitude_native * metres_per_native,
            scale_layer_seed(field.seed() ^ salt, raw),
        )?;

        *bands.get_mut(count)? = band;
        count += 1;
    }

    debug_assert!(count <= MAX_FINE_BANDS);
    debug_assert!(bands.iter().take(count).all(|band| band.params.is_finite()));
    Some((bands, count))
}

fn build_cave_charts(
    field: CelestialVoxelField,
    reference: GpuTerrainReference,
) -> Option<([GpuCaveChart; 5], bool)> {
    let half_extent =
        DVec3::splat(f64::from(reference.extent_f32) * 0.5 + f64::from(reference.spacing_f32));
    let include = field.profile() == CelestialBodyProfile::Rocky
        && field.presentation_caves_may_intersect_aabb(reference.center, half_extent);

    let mut caves = [GpuCaveChart::default(); 5];
    if !include {
        return Some((caves, false));
    }

    let seed = field.seed();
    let definitions = [
        (520.0, Vec3::new(13.7, -5.1, 8.9), seed ^ 0x4341_5645),
        (390.0, Vec3::new(-7.4, 19.2, -11.6), seed ^ 0x5455_4E4C),
        (240.0, Vec3::new(-21.3, 4.8, 15.2), seed ^ 0x4252_414E),
        (310.0, Vec3::new(6.6, -17.9, 2.7), seed ^ 0x4348_4D42),
        (680.0, Vec3::new(31.7, -14.1, 9.3), seed ^ 0x4348_414D),
    ];

    for (slot, (wavelength, offset, keyed_seed)) in caves.iter_mut().zip(definitions) {
        *slot = cave_chart(
            reference.anchor_pre_fine_surface,
            wavelength,
            offset,
            keyed_seed,
        )?;
    }

    debug_assert!(include);
    debug_assert!(
        caves
            .iter()
            .all(|chart| { chart.fraction_and_wavelength.is_finite() })
    );
    Some((caves, true))
}

pub(crate) fn descriptor_for_block(
    field: CelestialVoxelField,
    origin_local_metres: DVec3,
    extent_metres: f64,
    spacing_metres: f64,
    transition_bits: u8,
) -> Option<GpuTerrainDescriptor> {
    assert!(
        transition_bits & !0b00_111111 == 0,
        "binary terrain transition bits must describe only six block faces"
    );
    assert!(
        field.surface_detail_scale() <= field.coarsest_detail_scale(),
        "celestial terrain detail floor must not exceed its detail root"
    );

    let reference = terrain_reference(field, origin_local_metres, extent_metres, spacing_metres)?;
    let floor = field.surface_detail_scale().exponent();
    let root = field.coarsest_detail_scale().exponent();
    let (coarse, coarse_count) = build_coarse_bands(field, floor, root)?;
    let (fine, fine_count) =
        build_fine_bands(field, reference.anchor_pre_fine_surface, floor, root)?;
    let (caves, include_caves) = build_cave_charts(field, reference)?;

    let body_radius = field.radius_metres().clamp(0.0, f64::from(f32::MAX)) as f32;
    let anchor_relief = (reference.anchor_radius - field.radius_metres()) as f32;
    if !body_radius.is_finite() || !anchor_relief.is_finite() {
        return None;
    }

    let uv_x = (origin_local_metres.x.rem_euclid(UV_PHASE_WRAP_METRES) * 0.5) as f32;
    let uv_z = (origin_local_metres.z.rem_euclid(UV_PHASE_WRAP_METRES) * 0.5) as f32;

    debug_assert!(coarse_count <= MAX_COARSE_BANDS);
    debug_assert!(fine_count <= MAX_FINE_BANDS);

    Some(GpuTerrainDescriptor {
        chart_origin_and_spacing: reference.chart_origin_delta.extend(reference.spacing_f32),
        anchor_direction_and_inverse_radius: reference
            .anchor_direction
            .extend((1.0 / reference.anchor_radius) as f32),
        extent_uv_radius: Vec4::new(reference.extent_f32, uv_x, uv_z, body_radius),
        reference_relief_and_cave_depths: Vec4::new(
            anchor_relief,
            CAVE_START_DEPTH_METRES as f32,
            CAVE_MAX_DEPTH_METRES as f32,
            0.0,
        ),
        meta: UVec4::new(
            profile_id(field.profile()),
            u32::from(transition_bits),
            coarse_count as u32,
            fine_count as u32,
        ),
        semantic_meta: IVec4::new(i32::from(floor), if include_caves { 1 } else { 0 }, 0, 0),
        seed_meta: UVec4::new(field.seed(), 0, 0, 0),
        coarse,
        fine,
        caves,
    })
}
