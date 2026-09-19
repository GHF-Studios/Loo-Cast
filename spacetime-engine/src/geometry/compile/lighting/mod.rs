//! Compilation of authored light definitions.

use super::*;

pub(super) fn compile_point_light(value: &PointLightDef) -> CompiledPointLight {
    CompiledPointLight {
        id: value.id.clone(),
        zone: value.zone.clone(),
        position: v3(value.position),
        color: Color::srgb(value.color.0, value.color.1, value.color.2),
        intensity: value.intensity,
        range: value.range,
        shadows: value.shadows,
    }
}

pub(super) fn compile_directional_light(value: &DirectionalLightDef) -> CompiledDirectionalLight {
    CompiledDirectionalLight {
        id: value.id.clone(),
        zone: value.zone.clone(),
        rotation: authored_rotation(value.rotation_degrees),
        color: Color::srgb(value.color.0, value.color.1, value.color.2),
        illuminance: value.illuminance,
        shadows: value.shadows,
    }
}
