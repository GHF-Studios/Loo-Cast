//! USF manifestation developer visualization.

use bevy::prelude::*;

use crate::devtools::{
    AppDeveloperToolsExt, DeveloperSet, DeveloperTools, DrawDepth, VisualizationId,
    VisualizationSpec, WorldDrawBatch, WorldDrawFrame,
};

use super::{UsfEntity, UsfManifestationAuthority, UsfManifestations};

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
    semantic_entities: Query<(Entity, &UsfManifestations), With<UsfEntity>>,
    manifestations: Query<(&GlobalTransform, Has<UsfManifestationAuthority>)>,
    frame: Res<WorldDrawFrame>,
) {
    if !tools.visualization_enabled(VISUALIZATION) {
        return;
    }

    let mut batch = WorldDrawBatch::default();

    for (_, linked) in &semantic_entities {
        let Some(anchor) = linked
            .iter()
            .filter_map(|entity| manifestations.get(entity).ok())
            .find(|(_, authority)| *authority)
            .or_else(|| linked.iter().find_map(|entity| manifestations.get(entity).ok()))
            .map(|(transform, _)| transform.translation())
        else {
            continue;
        };

        for manifestation in linked.iter() {
            let Ok((transform, authority)) = manifestations.get(manifestation) else {
                continue;
            };

            let color = if authority {
                Color::srgb(0.2, 1.0, 0.35)
            } else {
                Color::srgb(0.2, 0.65, 1.0)
            };
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
