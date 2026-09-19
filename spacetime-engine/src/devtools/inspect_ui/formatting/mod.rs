//! Compact human-readable formatting for inspection values.

use super::*;

pub fn format_value(value: &InspectValue) -> String {
    match value {
        InspectValue::Text(value) => value.clone(),
        InspectValue::Bool(value) => (if *value { "yes" } else { "no" }).to_owned(),
        InspectValue::Integer(value) => value.to_string(),
        InspectValue::Number { value, format } => format_number(*value, *format),
        InspectValue::Quantity {
            value,
            unit,
            format,
        } => format_quantity(*value, unit.0, *format),
        InspectValue::Range {
            minimum,
            maximum,
            unit,
            format,
        } => format!(
            "{} … {}",
            format_quantity(*minimum, unit.0, *format),
            format_quantity(*maximum, unit.0, *format),
        ),
        InspectValue::Entity(entity) => format!("{entity:?}"),
        InspectValue::Vec3(value) => format!("({:.3}, {:.3}, {:.3})", value.x, value.y, value.z),
    }
}

pub fn format_quantity(value: f64, unit: &str, format: InspectNumberFormat) -> String {
    format!("{} {unit}", format_number(value, format))
}

pub fn format_number(value: f64, format: InspectNumberFormat) -> String {
    if !value.is_finite() {
        return "—".to_owned();
    }
    if value == 0.0 {
        return "0".to_owned();
    }

    let significant_digits = usize::from(format.significant_digits.clamp(1, 12));
    let magnitude = value.abs();
    let exponent = magnitude.log10().floor() as i32;

    if exponent >= 6 || exponent <= -4 {
        let engineering_exponent = exponent.div_euclid(3) * 3;
        let scaled = value / 10.0_f64.powi(engineering_exponent);
        let scaled_exponent = scaled.abs().log10().floor() as i32;
        let decimals = (significant_digits as i32 - 1 - scaled_exponent).clamp(0, 10) as usize;
        return format!(
            "{} × 10{}",
            trim_decimal_zeros(format!("{scaled:.decimals$}")),
            superscript_integer(engineering_exponent),
        );
    }

    let decimals = (significant_digits as i32 - 1 - exponent).clamp(0, 10) as usize;
    trim_decimal_zeros(format!("{value:.decimals$}"))
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
