//! Bounded canonical lattice point preparation.

use super::*;

#[derive(Debug, Clone, Copy)]
pub(crate) struct PreparedSemanticNoisePoint {
    pub(super) leaf_exponent: i8,
    pub(super) chunk_coordinate: [i64; 3],
    pub(super) offset: Vec3,
}

impl PreparedSemanticNoisePoint {
    pub(crate) fn from_native_f64(native: DVec3, scale: SpatialScale) -> Option<Self> {
        SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
            SemanticNoiseDiagnosticsStorage::bump(&diagnostics.prepared_point_attempts);
        });

        if !native.is_finite() {
            SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                SemanticNoiseDiagnosticsStorage::bump(
                    &diagnostics.prepared_point_nonfinite_failures,
                );
            });
            return None;
        }

        #[inline]
        fn canonical_axis(value: f64) -> Option<(i64, f32)> {
            let chunk_size = SEMANTIC_NATIVE_CHUNK_SIZE as f64;
            let carry_f = ((value + SEMANTIC_NATIVE_HALF_CHUNK) / chunk_size).floor();
            if carry_f < i64::MIN as f64 || carry_f > i64::MAX as f64 {
                SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                    SemanticNoiseDiagnosticsStorage::bump(
                        &diagnostics.prepared_point_range_failures,
                    );
                });
                return None;
            }

            let mut chunk = carry_f as i64;
            let remainder = value - chunk as f64 * chunk_size;
            let stored = remainder as f32;
            if !stored.is_finite() {
                SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                    SemanticNoiseDiagnosticsStorage::bump(
                        &diagnostics.prepared_point_nonfinite_failures,
                    );
                });
                return None;
            }

            let stored_f64 = f64::from(stored);
            let carry2_f = ((stored_f64 + SEMANTIC_NATIVE_HALF_CHUNK) / chunk_size).floor();
            let carry2 = carry2_f as i64;
            let mut local = (stored_f64 - carry2 as f64 * chunk_size) as f32;
            let Some(next_chunk) = chunk.checked_add(carry2) else {
                SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                    SemanticNoiseDiagnosticsStorage::bump(
                        &diagnostics.prepared_point_overflow_failures,
                    );
                });
                return None;
            };
            chunk = next_chunk;

            if local >= 500.0 {
                local -= 1_000.0;
                let Some(next_chunk) = chunk.checked_add(1) else {
                    SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                        SemanticNoiseDiagnosticsStorage::bump(
                            &diagnostics.prepared_point_overflow_failures,
                        );
                    });
                    return None;
                };
                chunk = next_chunk;
            } else if local < -500.0 {
                local += 1_000.0;
                let Some(next_chunk) = chunk.checked_sub(1) else {
                    SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                        SemanticNoiseDiagnosticsStorage::bump(
                            &diagnostics.prepared_point_overflow_failures,
                        );
                    });
                    return None;
                };
                chunk = next_chunk;
            }

            Some((chunk, local))
        }

        let (cx, ox) = canonical_axis(native.x)?;
        let (cy, oy) = canonical_axis(native.y)?;
        let (cz, oz) = canonical_axis(native.z)?;

        SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
            SemanticNoiseDiagnosticsStorage::bump(&diagnostics.prepared_point_successes);
        });

        Some(Self {
            leaf_exponent: scale.exponent(),
            chunk_coordinate: [cx, cy, cz],
            offset: Vec3::new(ox, oy, oz),
        })
    }

    #[inline]
    pub(super) fn cell_key_and_smooth(
        self,
        cell_size: i64,
        seed: u32,
    ) -> Option<(SemanticNoiseCellKey, Vec3)> {
        if cell_size <= 0
            || cell_size > i64::from(i32::MAX)
            || SEMANTIC_NATIVE_CHUNK_SIZE % cell_size != 0
        {
            SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                SemanticNoiseDiagnosticsStorage::bump(&diagnostics.prepared_cell_address_failures);
            });
            return None;
        }

        let size = cell_size as f32;
        let remainder = Vec3::new(
            self.offset.x.rem_euclid(size),
            self.offset.y.rem_euclid(size),
            self.offset.z.rem_euclid(size),
        );
        let fraction = remainder / size;
        let smooth = fraction * fraction * (Vec3::splat(3.0) - fraction * 2.0);

        #[inline]
        fn lower_axis(chunk: i64, offset: f32, remainder: f32) -> Option<i64> {
            let local = offset - remainder;
            let rounded = local.round();
            if (local - rounded).abs() > 1.0e-4 {
                return None;
            }
            chunk
                .checked_mul(SEMANTIC_NATIVE_CHUNK_SIZE)?
                .checked_add(rounded as i64)
        }

        let base = [
            lower_axis(self.chunk_coordinate[0], self.offset.x, remainder.x),
            lower_axis(self.chunk_coordinate[1], self.offset.y, remainder.y),
            lower_axis(self.chunk_coordinate[2], self.offset.z, remainder.z),
        ];

        let [Some(x), Some(y), Some(z)] = base else {
            SEMANTIC_NOISE_DIAGNOSTICS.with(|diagnostics| {
                SemanticNoiseDiagnosticsStorage::bump(&diagnostics.prepared_cell_address_failures);
            });
            return None;
        };

        Some((
            SemanticNoiseCellKey {
                leaf_exponent: self.leaf_exponent,
                cell_size: cell_size as i32,
                seed,
                x,
                y,
                z,
            },
            smooth,
        ))
    }
}

//
// `SemanticNoiseCornerKey` is already the complete canonical identity of one
// integer-aligned body-local noise corner. Reconstructing a UsfPosition,
// translating it, normalizing it, then walking its digit array on every cache
// miss is redundant. Reproduce the exact balanced-decimal digit stream directly
// from the compact integer native coordinate.
