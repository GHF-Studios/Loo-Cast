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
    UsfScaleLayerFrames, UsfSpatialAnchor, UsfSpatialFrame, UsfViewFrame,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsfTransitionVelocity {
    Preserve,
    Zero,
}

#[derive(Debug, Clone)]
pub struct UsfSpatialTransition {
    subject: Entity,
    position: UsfPosition,
    view_exponent: Option<f32>,
    velocity: UsfTransitionVelocity,
}

impl UsfSpatialTransition {
    pub const fn new(subject: Entity, position: UsfPosition) -> Self {
        Self {
            subject,
            position,
            view_exponent: None,
            velocity: UsfTransitionVelocity::Preserve,
        }
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
    mut view: ResMut<UsfViewFrame>,
    mut active: ResMut<UsfActiveScaleLayer>,
    mut layer_frames: ResMut<UsfScaleLayerFrames>,
    mut frame: ResMut<UsfSpatialFrame>,
    mut queue: ResMut<UsfSpatialTransitionQueue>,
    mut participants: ParamSet<(
        Query<
            (Entity, &Transform, &UsfManifestationOf),
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
    let (anchor_entity, old_anchor_runtime, subject) = {
        let anchors = participants.p0();
        let Some((entity, transform, manifestation)) = anchors.iter().next() else {
            return;
        };
        (entity, transform.translation, manifestation.0)
    };

    let request = queue.take_latest_for(subject);
    if let Some(request) = &request
        && let Some(exponent) = request.view_exponent
    {
        view.set_continuous_exponent(exponent);
    }

    let previous_scale = active.scale();
    let target_scale = view.dominant_scale();
    let requested_relocation = request.is_some();

    if !requested_relocation && target_scale == previous_scale {
        return;
    }

    let Ok(mut semantic) = semantic_positions.get_mut(subject) else {
        return;
    };

    if let Some(request) = &request {
        *semantic = request.position;
    }

    // Entering a finer chart increases resolved semantic precision. Zooming back
    // out never coarsens the semantic identity.
    if target_scale < semantic.leaf_scale() {
        let Ok(refined) = semantic.reexpressed_at(target_scale) else {
            error!(
                subject = ?subject,
                scale = %target_scale,
                "USF semantic position could not refine for spatial transition"
            );
            return;
        };
        *semantic = refined;
    }

    let Ok(chart_origin) = semantic.reexpressed_at(target_scale) else {
        error!(
            subject = ?subject,
            scale = %target_scale,
            "USF semantic position could not enter target runtime chart"
        );
        return;
    };
    let Ok(chart_absolute) = chart_origin.coordinate_at_scale_f64(target_scale) else {
        error!(
            subject = ?subject,
            scale = %target_scale,
            "USF target runtime chart origin could not be projected"
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

    let velocity_policy = request
        .as_ref()
        .map_or(UsfTransitionVelocity::Preserve, |request| request.velocity);

    for (_entity, mut transform, mut layer, position, velocity, manifestation) in
        &mut participants.p1()
    {
        // Followers belong to the current observer-local chart. Preserve only
        // their bounded offset from the primary anchor; never reinterpret an old
        // absolute runtime coordinate as a new-scale universe coordinate.
        let local_offset = transform.translation - old_anchor_runtime;
        let translated = local_offset * transition_factor;
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
            } else {
                velocity.0 *= transition_factor;
            }
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
        cause: if requested_relocation {
            UsfSpatialTransitionCause::Requested
        } else {
            UsfSpatialTransitionCause::ViewScale
        },
    });

    debug!(
        subject = ?subject,
        previous_scale = %previous_scale,
        active_scale = %target_scale,
        requested = requested_relocation,
        "applied canonical USF spatial transition"
    );
}
