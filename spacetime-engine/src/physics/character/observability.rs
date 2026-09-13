//! Character-controller observability.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::observability::{
    AppObservabilityExt, DebugControlSpec, DebugControls, DebugDepth, DebugFrame, DebugFrameBatch,
    DebugId, ObservabilitySet, CATEGORY_PHYSICS,
};

use super::{CharacterGroundState, CharacterLocomotionFrame, CharacterMotor};

const TOOL: DebugId = DebugId("physics.character");
const VELOCITY: DebugId = DebugId("physics.character.velocity");
const UP: DebugId = DebugId("physics.character.up");
const GROUND_NORMAL: DebugId = DebugId("physics.character.ground_normal");
const LABELS: DebugId = DebugId("physics.character.labels");

pub(crate) fn configure(app: &mut App) {
    app.register_debug_control(
        DebugControlSpec::tool(
            TOOL,
            Some(CATEGORY_PHYSICS),
            "Character controller",
            10,
            false,
        )
        .described("Engine-owned locomotion state, independent of collider rendering."),
    )
    .register_debug_control(DebugControlSpec::toggle(
        VELOCITY,
        Some(TOOL),
        "Velocity",
        0,
        true,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        UP,
        Some(TOOL),
        "Locomotion up",
        1,
        true,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        GROUND_NORMAL,
        Some(TOOL),
        "Ground normal",
        2,
        true,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        LABELS,
        Some(TOOL),
        "State labels",
        3,
        true,
    ))
    .add_systems(
        PostUpdate,
        collect_character_state.in_set(ObservabilitySet::Collect),
    );
}

fn collect_character_state(
    controls: Res<DebugControls>,
    characters: Query<
        (
            Entity,
            &GlobalTransform,
            &LinearVelocity,
            &CharacterGroundState,
            &CharacterLocomotionFrame,
        ),
        With<CharacterMotor>,
    >,
    frame: Res<DebugFrame>,
) {
    if !controls.active(TOOL) {
        return;
    }

    let mut batch = DebugFrameBatch::default();

    for (entity, transform, velocity, ground, locomotion) in &characters {
        let position = transform.translation();
        let up = locomotion.up();

        if controls.active(VELOCITY) {
            let rendered = velocity.0.clamp_length_max(20.0) * 0.15;
            if rendered.length_squared() > 1.0e-6 {
                batch.arrow(
                    position,
                    position + rendered,
                    Color::srgb(1.0, 0.75, 0.1),
                    DebugDepth::World,
                );
            }
        }

        if controls.active(UP) {
            batch.arrow(
                position,
                position + up * 0.8,
                Color::srgb(0.25, 0.7, 1.0),
                DebugDepth::World,
            );
        }

        if controls.active(GROUND_NORMAL) && ground.grounded {
            batch.arrow(
                position,
                position + ground.ground_normal.normalize_or_zero() * 0.9,
                Color::srgb(0.2, 1.0, 0.35),
                DebugDepth::World,
            );
        }

        if controls.active(LABELS) {
            batch.label_for(
                entity,
                position + up * 1.25,
                format!(
                    "{entity:?}\nspeed {:.2} m/s\n{}",
                    velocity.0.length(),
                    if ground.grounded { "grounded" } else { "airborne" },
                ),
                DebugFrameBatch::DEFAULT_LABEL_FONT_SIZE,
                if ground.grounded {
                    Color::srgb(0.2, 1.0, 0.35)
                } else {
                    Color::WHITE
                },
            );
        }
    }

    frame.submit(batch);
}
