//! Gravity alignment and transient control-basis synchronization.

use super::super::{CharacterGroundState, CharacterMotor};
use super::{CharacterControlFrame, CharacterLocomotionFrame, GravityAlignedLocomotionFrame};
use crate::physics::gravity::GravitySample;
use bevy::prelude::*;

pub(in crate::physics::character) fn sync_gravity_aligned_locomotion_frames(
    mut frames: Query<
        (&GravitySample, &mut CharacterLocomotionFrame),
        With<GravityAlignedLocomotionFrame>,
    >,
) {
    for (gravity, mut frame) in &mut frames {
        let acceleration = gravity.acceleration_metres_per_second2();
        if acceleration.length_squared() <= 1.0e-12 {
            continue;
        }

        let down = Vec3::new(
            acceleration.x as f32,
            acceleration.y as f32,
            acceleration.z as f32,
        )
        .normalize_or_zero();
        if down != Vec3::ZERO {
            frame.up = -down;
        }
    }
}

pub(in crate::physics::character) fn sync_character_body_alignment(
    mut characters: Query<
        (
            &CharacterLocomotionFrame,
            &CharacterGroundState,
            &mut CharacterControlFrame,
            &mut Transform,
        ),
        With<CharacterMotor>,
    >,
) {
    for (frame, ground, mut control, mut transform) in &mut characters {
        control.follow_locomotion_frame(frame);
        if ground.is_grounded() {
            transform.rotation = frame.aligned_rotation(transform.rotation);
        }
    }
}

pub(in crate::physics::character) fn settle_character_control_frames(
    time: Res<Time>,
    mut frames: Query<&mut CharacterControlFrame>,
) {
    for mut frame in &mut frames {
        frame.tick(time.delta_secs());
    }
}
