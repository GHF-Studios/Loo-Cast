//! Character-controller developer visualization.

use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;

use crate::devtools::{
    AppDeveloperToolsExt, DeveloperSet, DeveloperTools, DrawDepth, VisualizationId,
    VisualizationSpec, WorldDrawBatch, WorldDrawFrame,
};

use super::{CharacterGroundState, CharacterLocomotionFrame, CharacterMotor};

const VISUALIZATION: VisualizationId = VisualizationId("physics.character");

pub(crate) fn configure(app: &mut App) {
    app.register_developer_visualization(VisualizationSpec::new(
        VISUALIZATION,
        "Character controller",
        40,
        false,
    ))
    .add_systems(
        PostUpdate,
        collect_character_state.in_set(DeveloperSet::CollectWorldDraw),
    );
}

fn collect_character_state(
    tools: Res<DeveloperTools>,
    characters: Query<
        (
            &GlobalTransform,
            &LinearVelocity,
            &CharacterGroundState,
            &CharacterLocomotionFrame,
        ),
        With<CharacterMotor>,
    >,
    frame: Res<WorldDrawFrame>,
) {
    if !tools.visualization_enabled(VISUALIZATION) {
        return;
    }

    let mut batch = WorldDrawBatch::default();

    for (transform, velocity, ground, locomotion) in &characters {
        let position = transform.translation();
        let up = locomotion.up();

        let rendered = velocity.0.clamp_length_max(20.0) * 0.15;
        if rendered.length_squared() > 1.0e-6 {
            batch.arrow(
                position,
                position + rendered,
                Color::srgb(1.0, 0.75, 0.1),
                DrawDepth::World,
            );
        }

        batch.arrow(
            position,
            position + up * 0.8,
            Color::srgb(0.25, 0.7, 1.0),
            DrawDepth::World,
        );

        if ground.grounded {
            batch.arrow(
                position,
                position + ground.ground_normal.normalize_or_zero() * 0.9,
                Color::srgb(0.2, 1.0, 0.35),
                DrawDepth::World,
            );
        }
    }

    frame.submit(batch);
}
