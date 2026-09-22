//! First-class canonical spatial transitions.
//!
//! Runtime transforms are projections into one bounded active chart. They are
//! never authoritative universe coordinates. Ordinary movement is committed to
//! canonical [`UsfPosition`] first; scale changes and discontinuous relocation
//! then rebuild the runtime chart from canonical state.

use std::collections::VecDeque;

use avian3d::prelude::{LinearVelocity, Position};
use bevy::prelude::*;

use crate::ecs::{UsfLogicalProjection, UsfManifestationOf};

use super::{
    SpatialScale, UsfActiveScaleLayer, UsfFollowsActiveScale, UsfPosition, UsfScaleLayer,
    UsfScaleLayerFrames, UsfSpatialAnchor, UsfSpatialFrame, UsfViewContext, UsfViewRenderAnchor,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsfTransitionVelocity {
    /// Preserve the numeric velocity vector while changing scale-local charts.
    ///
    /// `6.0` therefore remains `6.0`, but those units are reinterpreted in the
    /// destination scale. This is the default for scale-local physics.
    PreserveNative,
    /// Preserve canonical physical velocity across a chart change by rescaling
    /// the numeric vector between native-unit systems.
    PreserveCanonical,
    Zero,
}

#[derive(Debug, Clone)]
pub struct UsfSpatialTransition {
    subject: Entity,
    position: UsfPosition,
    target_scale: Option<SpatialScale>,
    view_exponent: Option<f32>,
    velocity: UsfTransitionVelocity,
}

impl UsfSpatialTransition {
    pub const fn new(subject: Entity, position: UsfPosition) -> Self {
        Self {
            subject,
            position,
            target_scale: None,
            view_exponent: None,
            velocity: UsfTransitionVelocity::PreserveNative,
        }
    }

    /// Explicitly selects the destination interaction Scale Slice.
    /// View zoom never selects this implicitly.
    pub const fn with_scale(mut self, scale: SpatialScale) -> Self {
        self.target_scale = Some(scale);
        self
    }

    pub fn with_view_exponent(mut self, exponent: f32) -> Self {
        self.view_exponent = Some(exponent);
        self
    }

    pub const fn with_velocity(mut self, velocity: UsfTransitionVelocity) -> Self {
        self.velocity = velocity;
        self
    }

    pub const fn subject(&self) -> Entity {
        self.subject
    }

    pub const fn position(&self) -> UsfPosition {
        self.position
    }

    pub const fn view_exponent(&self) -> Option<f32> {
        self.view_exponent
    }
}

#[derive(Resource, Default)]
pub struct UsfSpatialTransitionQueue {
    pending: VecDeque<UsfSpatialTransition>,
}

impl UsfSpatialTransitionQueue {
    pub fn request(&mut self, transition: UsfSpatialTransition) {
        self.pending.push_back(transition);
    }

    fn take_latest_for(&mut self, subject: Entity) -> Option<UsfSpatialTransition> {
        let mut latest = None;
        let mut retained = VecDeque::with_capacity(self.pending.len());

        while let Some(request) = self.pending.pop_front() {
            if request.subject == subject {
                latest = Some(request);
            } else {
                retained.push_back(request);
            }
        }

        self.pending = retained;
        latest
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsfSpatialTransitionCause {
    ViewScale,
    Requested,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct UsfSpatialTransitionApplied {
    pub subject: Entity,
    pub anchor: Entity,
    pub previous_scale: SpatialScale,
    pub active_scale: SpatialScale,
    pub cause: UsfSpatialTransitionCause,
}

pub(super) fn apply_spatial_transitions(
    mut view: Single<&mut UsfViewContext, With<UsfViewRenderAnchor>>,
    mut active: ResMut<UsfActiveScaleLayer>,
    mut layer_frames: ResMut<UsfScaleLayerFrames>,
    mut frame: ResMut<UsfSpatialFrame>,
    mut queue: ResMut<UsfSpatialTransitionQueue>,
    mut participants: ParamSet<(
        Query<
            (Entity, &Transform, &UsfScaleLayer, &UsfManifestationOf),
            (
                With<UsfSpatialAnchor>,
                With<UsfLogicalProjection>,
                With<UsfFollowsActiveScale>,
                Without<ChildOf>,
            ),
        >,
        Query<
            (
                Entity,
                &mut Transform,
                &mut UsfScaleLayer,
                Option<&mut Position>,
                Option<&mut LinearVelocity>,
                Option<&UsfManifestationOf>,
            ),
            (With<UsfFollowsActiveScale>, Without<ChildOf>),
        >,
    )>,
    mut semantic_positions: Query<&mut UsfPosition>,
    mut applied: MessageWriter<UsfSpatialTransitionApplied>,
) {
    let (anchor_entity, old_anchor_runtime, previous_scale, subject) = {
        let anchors = participants.p0();
        let Some((entity, transform, layer, manifestation)) = anchors.iter().next() else {
            return;
        };
        (entity, transform.translation, layer.scale(), manifestation.0)
    };

    let Some(request) = queue.take_latest_for(subject) else {
        return;
    };
    if let Some(exponent) = request.view_exponent {
        view.set_continuous_exponent(exponent);
    }

    let target_scale = request.target_scale.unwrap_or(previous_scale);
    let requested_relocation = true;

    let Ok(mut semantic) = semantic_positions.get_mut(subject) else {
        return;
    };

    *semantic = request.position;

    // Changing the runtime chart must never change semantic precision.
    // The exact canonical subject position becomes the frame origin. Pass 2
    // removes the remaining DVec3 projection used only by legacy frame metadata.
    let chart_origin = *semantic;
    let Ok(chart_absolute) = semantic.coordinate_at_scale_f64(target_scale) else {
        error!(
            subject = ?subject,
            scale = %target_scale,
            "USF target runtime chart origin could not be projected for legacy frame metadata"
        );
        return;
    };

    let transition_factor =
        10.0_f32.powi(previous_scale.exponent() as i32 - target_scale.exponent() as i32);
    if !transition_factor.is_finite() {
        error!(
            previous_scale = %previous_scale,
            target_scale = %target_scale,
            "USF runtime chart transition factor is non-finite"
        );
        return;
    }

    let velocity_policy = request.velocity;

    for (_entity, mut transform, mut layer, position, velocity, manifestation) in
        &mut participants.p1()
    {
        if manifestation.is_none_or(|manifestation| manifestation.0 != subject) {
            continue;
        }

        // Followers belong to the current observer-local chart. Preserve only
        // their bounded offset from the primary anchor; never reinterpret an old
        // absolute runtime coordinate as a new-scale universe coordinate.
        let local_offset = transform.translation - old_anchor_runtime;

        // Preserve the numeric local offset while reinterpreting the chart.
        // 0.78 local units stays 0.78; only UsfScaleLayer changes what it means.
        let translated = local_offset;
        transform.translation = translated;
        layer.set_scale(target_scale);

        if let Some(mut position) = position {
            position.0 = translated;
        }

        if let Some(mut velocity) = velocity {
            let belongs_to_subject =
                manifestation.is_some_and(|manifestation| manifestation.0 == subject);

            if requested_relocation
                && belongs_to_subject
                && velocity_policy == UsfTransitionVelocity::Zero
            {
                velocity.0 = Vec3::ZERO;
            } else if velocity_policy == UsfTransitionVelocity::PreserveCanonical {
                velocity.0 *= transition_factor;
            }
            // PreserveNative deliberately leaves the numeric vector untouched:
            // the destination UsfScaleLayer changes what one local unit means.
        }
    }

    layer_frames.set_origin(target_scale, chart_absolute);
    frame.origin = chart_origin;
    frame.last_shift = Vec3::ZERO;
    active.set_scale(target_scale);

    applied.write(UsfSpatialTransitionApplied {
        subject,
        anchor: anchor_entity,
        previous_scale,
        active_scale: target_scale,
        cause: UsfSpatialTransitionCause::Requested,
    });

    debug!(
        subject = ?subject,
        previous_scale = %previous_scale,
        active_scale = %target_scale,
        requested = requested_relocation,
        "applied canonical USF spatial transition"
    );
}
