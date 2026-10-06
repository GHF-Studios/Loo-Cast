//! Plain decimal and stack representation of canonical coordinates.

use super::*;

impl UsfCoordinate {
    pub const fn leaf_scale(self) -> SpatialScale {
        self.leaf_scale
    }

    pub const fn offset(self) -> f32 {
        self.offset
    }

    pub fn digit(self, scale: SpatialScale) -> i8 {
        self.digits[scale.index_from_top()]
    }

    fn plain_decimal(self) -> String {
        let mut terms = Vec::<(i32, i32)>::with_capacity(SPATIAL_SCALE_COUNT + 12);

        for raw_scale in self.leaf_scale.exponent()..=SPATIAL_SCALE_MAX {
            let scale = SpatialScale::new(raw_scale).expect("validated spatial scale");
            let digit = i32::from(self.digit(scale));
            if digit != 0 {
                // One chunk digit at Sx is 1000 * 10^x S0 units.
                terms.push((i32::from(raw_scale) + 3, digit));
            }
        }

        push_float_decimal_terms(
            &mut terms,
            self.offset,
            i32::from(self.leaf_scale.exponent()),
        );

        format_decimal_terms(&terms)
    }
}

impl Display for UsfCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.plain_decimal())
    }
}

impl std::fmt::Debug for UsfCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UsfCoordinate({self})")
    }
}

impl Display for UsfPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

impl std::fmt::Debug for UsfPosition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "UsfPosition({},{},{})", self.x(), self.y(), self.z())
    }
}

impl UsfPosition {
    pub fn nonzero_digits(&self) -> impl Iterator<Item = (SpatialScale, IVec3)> + '_ {
        (self.leaf_scale.exponent()..=SPATIAL_SCALE_MAX)
            .rev()
            .filter_map(|raw| {
                let scale = SpatialScale::new(raw).expect("range is validated");
                let digit = self.digit(scale);
                (digit != IVec3::ZERO).then_some((scale, digit))
            })
    }

    pub fn format_stack(&self) -> String {
        let mut parts = self
            .nonzero_digits()
            .map(|(scale, digit)| format!("S{}=({}, {}, {})", scale, digit.x, digit.y, digit.z))
            .collect::<Vec<_>>();

        if parts.is_empty() {
            parts.push("digits=0".to_string());
        }

        format!(
            "{} | S{} local=({:.3}, {:.3}, {:.3})",
            parts.join(" "),
            self.leaf_scale,
            self.offset.x,
            self.offset.y,
            self.offset.z,
        )
    }
}
