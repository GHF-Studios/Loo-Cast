//! Character-motor state visualized independently from Avian collider debug.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::debug::{
    AppDebugExt, DebugCamera, DebugGizmos, DebugOverlayGizmos, DebugView, DebugViews,
    billboard_text,
};

use super::{CharacterGroundState, CharacterLocomotionFrame, CharacterMotor};

struct CharacterMotorDebugView;

impl DebugView for CharacterMotorDebugView {
    const NAME: &'static str = "Character / Motor";
}

pub(crate) fn configure(app: &mut App) {
    app.register_debug_view::<CharacterMotorDebugView>()
        .add_systems(PostUpdate, draw_character_motor_debug);
}

fn draw_character_motor_debug(
    views: Res<DebugViews>,
    camera: Query<&Transform, With<DebugCamera>>,
    characters: Query<
        (
            Entity,
            &Transform,
            &LinearVelocity,
            &CharacterGroundState,
            &CharacterLocomotionFrame,
        ),
        With<CharacterMotor>,
    >,
    mut gizmos: Gizmos<DebugGizmos>,
    mut overlay: Gizmos<DebugOverlayGizmos>,
) {
    if !views.enabled::<CharacterMotorDebugView>() {
        return;
    }
    let Some(camera) = camera.iter().next() else {
        return;
    };

    for (entity, transform, velocity, ground, frame) in &characters {
        let position = transform.translation;
        let up = frame.up();
        let velocity_vector = velocity.0.clamp_length_max(20.0) * 0.15;

        if velocity_vector.length_squared() > 1.0e-6 {
            gizmos.arrow(
                position,
                position + velocity_vector,
                Color::srgb(1.0, 0.75, 0.1),
            );
        }

        gizmos.arrow(
            position,
            position + up * 0.8,
            Color::srgb(0.25, 0.7, 1.0),
        );

        if ground.grounded {
            gizmos.arrow(
                position,
                position + ground.ground_normal.normalize_or_zero() * 0.9,
                Color::srgb(0.2, 1.0, 0.35),
            );
        }

        let label = format!(
            "{entity:?}\nspeed {:.2} m/s\n{}",
            velocity.0.length(),
            if ground.grounded { "grounded" } else { "airborne" },
        );
        billboard_text(
            &mut overlay,
            camera,
            position + up * 1.25,
            &label,
            14.0,
            Vec2::new(-0.5, -0.5),
            if ground.grounded {
                Color::srgb(0.2, 1.0, 0.35)
            } else {
                Color::WHITE
            },
        );
    }
}
