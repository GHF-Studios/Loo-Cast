//! Floating-origin rebasing of the bounded local runtime chart.

use super::*;

/// Canonical chart-origin displacement authored in one Scale Slice's native units.
///
/// A rebase is one physical/canonical displacement, not one universally reusable
/// `Vec3`. Every runtime cache must project this delta into its own chart before
/// mutating local coordinates.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfChartDelta {
    source_scale: SpatialScale,
    local_shift: Vec3,
}

impl UsfChartDelta {
    pub const fn new(source_scale: SpatialScale, local_shift: Vec3) -> Self {
        Self { source_scale, local_shift }
    }

    pub const fn source_scale(self) -> SpatialScale {
        self.source_scale
    }

    pub const fn local_shift(self) -> Vec3 {
        self.local_shift
    }

    pub fn at_scale(self, target_scale: SpatialScale) -> Result<Vec3, UsfPositionError> {
        if !self.local_shift.is_finite() {
            return Err(UsfPositionError::NonFiniteTranslation);
        }

        let exponent_delta =
            i32::from(self.source_scale.exponent()) - i32::from(target_scale.exponent());
        let factor = 10.0_f64.powi(exponent_delta);
        let converted = [
            f64::from(self.local_shift.x) * factor,
            f64::from(self.local_shift.y) * factor,
            f64::from(self.local_shift.z) * factor,
        ];

        if converted
            .iter()
            .any(|value| !value.is_finite() || value.abs() > f64::from(f32::MAX))
        {
            return Err(UsfPositionError::TranslationTooLarge);
        }

        Ok(Vec3::new(
            converted[0] as f32,
            converted[1] as f32,
            converted[2] as f32,
        ))
    }
}

pub(super) fn rebase_local_frame(
    mut frame: ResMut<UsfSpatialFrame>,
    mut transforms: ParamSet<(
        Query<
            (Entity, &Transform, &UsfScaleLayer),
            (With<UsfSpatialAnchor>, With<UsfLogicalProjection>),
        >,
        Query<(&mut Transform, Option<&UsfScaleLayer>), Without<ChildOf>>,
    )>,
    mut physics_positions: Query<(&mut Position, Option<&UsfScaleLayer>)>,
    mut rebased: MessageWriter<UsfOriginRebased>,
) {
    let (anchor_translation, anchor_scale) = {
        let anchors = transforms.p0();
        let mut anchors = anchors.iter();
        let Some((anchor_entity, anchor, layer)) = anchors.next() else {
            return;
        };
        if let Some((other_entity, _, _)) = anchors.next() {
            error!(
                first = ?anchor_entity,
                second = ?other_entity,
                "USF local frame has multiple spatial anchors; refusing ambiguous rebase"
            );
            return;
        }
        (anchor.translation, layer.scale())
    };

    let shift = rebase_shift(anchor_translation);
    if shift == Vec3::ZERO {
        return;
    }
    let delta = UsfChartDelta::new(anchor_scale, shift);

    // Preflight every scale conversion before changing canonical or runtime
    // state. A rebase is a transaction; partial chart mutation is invalid.
    {
        let mut runtime_transforms = transforms.p1();
        for (_, layer) in &mut runtime_transforms {
            let target_scale = layer.map_or(anchor_scale, |layer| layer.scale());
            if let Err(error) = delta.at_scale(target_scale) {
                error!(
                    ?error,
                    source_scale = %anchor_scale,
                    target_scale = %target_scale,
                    ?shift,
                    "USF rebase cannot be represented in one resident runtime chart"
                );
                return;
            }
        }
    }
    for (_, layer) in &mut physics_positions {
        let target_scale = layer.map_or(anchor_scale, |layer| layer.scale());
        if let Err(error) = delta.at_scale(target_scale) {
            error!(
                ?error,
                source_scale = %anchor_scale,
                target_scale = %target_scale,
                ?shift,
                "USF rebase cannot be represented in one resident physics chart"
            );
            return;
        }
    }

    let Ok(new_origin) = frame.origin.translated_at_scale(anchor_scale, shift) else {
        error!(?shift, "USF canonical translation failed during local-origin rebase");
        return;
    };

    frame.origin = new_origin;
    frame.rebase_count = frame.rebase_count.wrapping_add(1);
    frame.last_shift = shift;

    {
        let mut runtime_transforms = transforms.p1();
        for (mut transform, layer) in &mut runtime_transforms {
            let target_scale = layer.map_or(anchor_scale, |layer| layer.scale());
            let local_shift = delta
                .at_scale(target_scale)
                .expect("rebase scale conversion was preflighted");
            transform.translation -= local_shift;
        }
    }

    for (mut position, layer) in &mut physics_positions {
        let target_scale = layer.map_or(anchor_scale, |layer| layer.scale());
        let local_shift = delta
            .at_scale(target_scale)
            .expect("rebase scale conversion was preflighted");
        position.0 -= local_shift;
    }

    debug!(
        source_scale = %anchor_scale,
        ?shift,
        rebase_count = frame.rebase_count,
        "rebased canonical USF runtime chart"
    );
    rebased.write(UsfOriginRebased { delta });
}

pub(super) fn rebase_shift(position: Vec3) -> Vec3 {
    Vec3::new(
        rebase_axis(position.x),
        rebase_axis(position.y),
        rebase_axis(position.z),
    )
}

fn rebase_axis(value: f32) -> f32 {
    if value.abs() < REBASE_THRESHOLD_NATIVE {
        0.0
    } else {
        (value / REBASE_QUANTUM_NATIVE).trunc() * REBASE_QUANTUM_NATIVE
    }
}
