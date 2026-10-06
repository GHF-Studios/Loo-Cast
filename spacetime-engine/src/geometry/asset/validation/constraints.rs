//! Scalar, polygon and generated-count constraints shared by object rules.

use super::super::{V2, V3};

pub(super) fn validate_color(color: V3, context: &str) -> Result<(), String> {
    let (r, g, b) = color;
    if [r, g, b]
        .into_iter()
        .any(|value| !value.is_finite() || !(0.0..=1.0).contains(&value))
    {
        return Err(format!(
            "{context} color channels must be finite and in 0..=1"
        ));
    }
    Ok(())
}

pub(super) fn validate_convex_polygon(points: &[V2], id: &str) -> Result<(), String> {
    let mut winding = 0.0f32;

    for index in 0..points.len() {
        let a = points[index];
        let b = points[(index + 1) % points.len()];
        let c = points[(index + 2) % points.len()];
        let ab = (b.0 - a.0, b.1 - a.1);
        let bc = (c.0 - b.0, c.1 - b.1);
        let cross = ab.0 * bc.1 - ab.1 * bc.0;

        if cross.abs() <= f32::EPSILON {
            continue;
        }

        if winding == 0.0 {
            winding = cross.signum();
        } else if cross.signum() != winding {
            return Err(format!(
                "convex prism {id:?} cross-section is not convex or not in winding order"
            ));
        }
    }

    if winding == 0.0 {
        return Err(format!("convex prism {id:?} cross-section is degenerate"));
    }

    Ok(())
}

pub(super) fn sweep_count(start: f32, end: f32, increment: f32, id: &str) -> Result<usize, String> {
    if !start.is_finite() || !end.is_finite() || !increment.is_finite() || increment <= 0.0 {
        return Err(format!("sweep {id:?} has invalid range"));
    }
    if end < start {
        return Err(format!("sweep {id:?} end must be >= start"));
    }

    let count = ((end - start) / increment).floor() + 1.0;
    if !count.is_finite() || count < 1.0 || count > usize::MAX as f32 {
        return Err(format!("sweep {id:?} object count is invalid"));
    }
    Ok(count as usize)
}

pub(super) fn finite_v3(value: V3, id: &str, field: &str) -> Result<(), String> {
    finite(value.0, id, field)?;
    finite(value.1, id, field)?;
    finite(value.2, id, field)
}

pub(super) fn finite(value: f32, id: &str, field: &str) -> Result<(), String> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(format!("object {id:?} field {field} must be finite"))
    }
}

pub(super) fn positive(value: f32, id: &str, field: &str) -> Result<(), String> {
    finite(value, id, field)?;
    if value > 0.0 {
        Ok(())
    } else {
        Err(format!("object {id:?} field {field} must be positive"))
    }
}

pub(super) fn non_negative(value: f32, id: &str, field: &str) -> Result<(), String> {
    finite(value, id, field)?;
    if value >= 0.0 {
        Ok(())
    } else {
        Err(format!("object {id:?} field {field} must be non-negative"))
    }
}

pub(super) fn positive_usize(value: usize, id: &str, field: &str) -> Result<(), String> {
    if value > 0 {
        Ok(())
    } else {
        Err(format!("object {id:?} field {field} must be positive"))
    }
}

pub(super) fn positive_v3(value: V3, id: &str, field: &str) -> Result<(), String> {
    positive(value.0, id, field)?;
    positive(value.1, id, field)?;
    positive(value.2, id, field)
}
