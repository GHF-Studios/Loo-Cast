//! Expansion of repeated/parametric authored geometry.

use super::*;

pub(super) fn compile_staircase(output: &mut Vec<CompiledNode>, value: &StaircaseDef) {
    let origin = v3(value.origin);
    let yaw = value.yaw_degrees.to_radians();
    let rotation = Quat::from_rotation_y(yaw);
    let forward = rotation * Vec3::Z;
    let width = value.width;
    let step_height = value.step_height;
    let step_depth = value.step_depth;

    for index in 0..value.steps {
        let height = step_height * (index + 1) as f32;
        let center =
            origin + forward * (step_depth * (index as f32 + 0.5)) + Vec3::Y * (height * 0.5);

        push_geometry(
            output,
            CompiledGeometry {
                id: format!("{}/{index:03}", value.id),
                zone: value.zone.clone(),
                tags: value.tags.clone(),
                transform: Transform::from_translation(center).with_rotation(rotation),
                shape: CompiledShape::Box {
                    size: Vec3::new(width, height, step_depth),
                },
                material: value.material.clone(),
                solid: true,
                motion: None,
            },
        );
    }
}

pub(super) fn compile_step_sweep(output: &mut Vec<CompiledNode>, value: &StepSweepDef) {
    let origin = v3(value.origin);
    let rotation = Quat::from_rotation_y(value.yaw_degrees.to_radians());
    let right = rotation * Vec3::X;
    let width = value.width;
    let depth = value.depth;
    let gap = value.gap;

    let mut height = value.start_height;
    let mut index = 0usize;
    while height <= value.end_height + value.increment * 0.25 {
        let world_height = height;
        let center =
            origin + right * ((width + gap) * index as f32) + Vec3::Y * (world_height * 0.5);

        push_geometry(
            output,
            CompiledGeometry {
                id: format!("{}/{index:03}_{height:.3}", value.id),
                zone: value.zone.clone(),
                tags: value.tags.clone(),
                transform: Transform::from_translation(center).with_rotation(rotation),
                shape: CompiledShape::Box {
                    size: Vec3::new(width, world_height, depth),
                },
                material: value.material.clone(),
                solid: true,
                motion: None,
            },
        );

        index += 1;
        height += value.increment;
    }
}

pub(super) fn compile_slope_sweep(output: &mut Vec<CompiledNode>, value: &SlopeSweepDef) {
    let base_origin = v3(value.origin);
    let mut angle = value.start_angle_degrees;
    let mut index = 0usize;

    while angle <= value.end_angle_degrees + value.angle_increment_degrees * 0.25 {
        let rise = value.run * angle.to_radians().tan();
        let lateral = index as f32 * (value.width + value.gap);
        let yaw_rotation = Quat::from_rotation_y(value.yaw_degrees.to_radians());
        let lateral_world = yaw_rotation * Vec3::X * lateral;
        let start = base_origin + lateral_world;

        let ramp = RampDef {
            id: format!("{}/{index:03}_{angle:.3}deg", value.id),
            zone: value.zone.clone(),
            start: tuple3(start),
            yaw_degrees: value.yaw_degrees,
            width: value.width,
            run: value.run,
            rise,
            thickness: value.thickness,
            material: value.material.clone(),
            solid: true,
            tags: value.tags.clone(),
        };

        push_geometry(output, compile_ramp(&ramp));

        index += 1;
        angle += value.angle_increment_degrees;
    }
}

pub(super) fn compile_pillar_grid(output: &mut Vec<CompiledNode>, value: &PillarGridDef) {
    let origin = v3(value.origin);
    let rotation = Quat::from_rotation_y(value.yaw_degrees.to_radians());
    let right = rotation * Vec3::X;
    let forward = rotation * Vec3::Z;

    let width = value.pillar_width;
    let depth = value.pillar_depth;
    let height = value.pillar_height;
    let spacing_x = value.spacing_x;
    let spacing_z = value.spacing_z;

    for column in 0..value.columns {
        for row in 0..value.rows {
            let center = origin
                + right * (column as f32 * spacing_x)
                + forward * (row as f32 * spacing_z)
                + Vec3::Y * (height * 0.5);

            push_geometry(
                output,
                CompiledGeometry {
                    id: format!("{}/{column:02}_{row:02}", value.id),
                    zone: value.zone.clone(),
                    tags: value.tags.clone(),
                    transform: Transform::from_translation(center).with_rotation(rotation),
                    shape: CompiledShape::Box {
                        size: Vec3::new(width, height, depth),
                    },
                    material: value.material.clone(),
                    solid: true,
                    motion: None,
                },
            );
        }
    }
}
