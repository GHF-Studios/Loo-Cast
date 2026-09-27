//! USF manifestation developer visualization.

use bevy::prelude::*;

use crate::devtools::{
    AppDeveloperToolsExt, DeveloperSet, DeveloperTools, DrawDepth, VisualizationId,
    VisualizationSpec, WorldDrawBatch, WorldDrawFrame,
};

use super::{UsfAuthorityPartitions, UsfEntity, UsfLogicalRealizations};

const VISUALIZATION: VisualizationId = VisualizationId("world.usf_manifestations");

pub(crate) fn configure(app: &mut App) {
    app.register_developer_visualization(VisualizationSpec::new(
        VISUALIZATION,
        "USF manifestations",
        20,
        false,
    ))
    .add_systems(
        PostUpdate,
        collect_manifestations.in_set(DeveloperSet::CollectWorldDraw),
    );
}

fn collect_manifestations(
    tools: Res<DeveloperTools>,
    semantic_entities: Query<&UsfAuthorityPartitions, With<UsfEntity>>,
    partitions: Query<&UsfLogicalRealizations>,
    realizations: Query<&GlobalTransform>,
    frame: Res<WorldDrawFrame>,
) {
    if !tools.visualization_enabled(VISUALIZATION) {
        return;
    }

    let mut batch = WorldDrawBatch::default();

    for semantic_partitions in &semantic_entities {
        let logical_realizations = semantic_partitions
            .iter()
            .filter_map(|partition| partitions.get(partition).ok())
            .flat_map(|realizations| realizations.iter())
            .collect::<Vec<_>>();

        let Some(anchor) = logical_realizations
            .iter()
            .find_map(|entity| realizations.get(*entity).ok())
            .map(GlobalTransform::translation)
        else {
            continue;
        };

        for realization in logical_realizations {
            let Ok(transform) = realizations.get(realization) else {
                continue;
            };

            let color = Color::srgb(0.2, 0.65, 1.0);
            let position = transform.translation();

            batch.cross(
                Isometry3d::new(position, Quat::IDENTITY),
                0.12,
                color,
                DrawDepth::World,
            );
            if position.distance_squared(anchor) > 1.0e-6 {
                batch.line(anchor, position, color, DrawDepth::World);
            }
        }
    }

    frame.submit(batch);
}
