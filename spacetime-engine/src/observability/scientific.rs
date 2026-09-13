//! Small, renderer-independent formatting helpers for scientific debug text.
//!
//! Debug output should not invent ASCII approximations of physical notation in
//! every domain. Keep typography and significant-digit policy centralized here.

pub mod unit {
    pub const KELVIN: &str = "K";
    pub const JOULE: &str = "J";
    pub const WATT: &str = "W";
    pub const METER: &str = "m";
    pub const METERS_PER_SECOND: &str = "m/s";
    pub const METERS_PER_SECOND_SQUARED: &str = "m/s²";
    pub const DENSITY: &str = "kg/m³";
    pub const SPECIFIC_HEAT_CAPACITY: &str = "J/(kg·K)";
    pub const THERMAL_CONDUCTIVITY: &str = "W/(m·K)";
    pub const THERMAL_DIFFUSIVITY: &str = "m²/s";
}

/// Formats a finite scalar with approximately `significant_digits` useful
/// digits. Very large/small values use engineering powers of ten and Unicode
/// superscripts rather than raw `e±NN` notation.
pub fn format_number(value: f32, significant_digits: usize) -> String {
    if !value.is_finite() {
        return "—".to_owned();
    }
    if value == 0.0 {
        return "0".to_owned();
    }

    let significant_digits = significant_digits.clamp(1, 7);
    let magnitude = value.abs();
    let exponent = magnitude.log10().floor() as i32;

    if exponent >= 6 || exponent <= -4 {
        let engineering_exponent = exponent.div_euclid(3) * 3;
        let scaled = value / 10.0_f32.powi(engineering_exponent);
        let scaled_exponent = scaled.abs().log10().floor() as i32;
        let decimals = (significant_digits as i32 - 1 - scaled_exponent)
            .clamp(0, 6) as usize;
        return format!(
            "{} × 10{}",
            trim_decimal_zeros(format!("{scaled:.decimals$}")),
            superscript_integer(engineering_exponent),
        );
    }

    let decimals = (significant_digits as i32 - 1 - exponent).clamp(0, 6) as usize;
    trim_decimal_zeros(format!("{value:.decimals$}"))
}

pub fn format_quantity(value: f32, unit: &str, significant_digits: usize) -> String {
    format!(
        "{}\u{202f}{}",
        format_number(value, significant_digits),
        unit
    )
}

pub fn format_quantity_range(
    minimum: f32,
    maximum: f32,
    unit: &str,
    significant_digits: usize,
) -> String {
    format!(
        "{} … {}",
        format_quantity(minimum, unit, significant_digits),
        format_quantity(maximum, unit, significant_digits),
    )
}

fn trim_decimal_zeros(mut value: String) -> String {
    if !value.contains('.') {
        return value;
    }
    while value.ends_with('0') {
        value.pop();
    }
    if value.ends_with('.') {
        value.pop();
    }
    value
}

fn superscript_integer(value: i32) -> String {
    let mut result = String::new();
    if value < 0 {
        result.push('⁻');
    }
    for character in value.unsigned_abs().to_string().chars() {
        result.push(match character {
            '0' => '⁰',
            '1' => '¹',
            '2' => '²',
            '3' => '³',
            '4' => '⁴',
            '5' => '⁵',
            '6' => '⁶',
            '7' => '⁷',
            '8' => '⁸',
            '9' => '⁹',
            _ => unreachable!(),
        });
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantity_text_uses_real_scientific_typography() {
        assert_eq!(format_quantity(293.15, unit::KELVIN, 5), "293.15\u{202f}K");
        assert_eq!(
            format_quantity(0.000_012_3, unit::THERMAL_DIFFUSIVITY, 3),
            "12.3 × 10⁻⁶\u{202f}m²/s"
        );
    }
}
