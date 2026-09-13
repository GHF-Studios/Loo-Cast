//! USF manifestation observability adapter.

use bevy::prelude::*;

use crate::{
    devtools::{DeveloperSet, DrawDepth, WorldDrawBatch, WorldDrawFrame},
    observability::{
        AppObservabilityExt, DebugChoiceOption, DebugControlSpec, DebugControls, DebugId,
        CATEGORY_WORLD,
    },
};

use super::{UsfEntity, UsfManifestationAuthority, UsfManifestations};

const TOOL: DebugId = DebugId("world.usf_manifestations");
const MARKERS: DebugId = DebugId("world.usf_manifestations.markers");
const LINKS: DebugId = DebugId("world.usf_manifestations.links");
const SCOPE: DebugId = DebugId("world.usf_manifestations.scope");

pub(crate) fn configure(app: &mut App) {
    app.register_debug_control(
        DebugControlSpec::tool(
            TOOL,
            Some(CATEGORY_WORLD),
            "USF manifestations",
            0,
            false,
        )
        .described("Semantic entity <-> spatial manifestation identity and authority."),
    )
    .register_debug_control(DebugControlSpec::toggle(
        MARKERS,
        Some(TOOL),
        "Markers",
        0,
        true,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        LINKS,
        Some(TOOL),
        "Semantic links",
        1,
        true,
    ))
    .register_debug_control(DebugControlSpec::choice(
        SCOPE,
        Some(TOOL),
        "Scope",
        2,
        [
            DebugChoiceOption::new("all", "All manifestations"),
            DebugChoiceOption::new("authority", "Authority only"),
        ],
        0,
    ))
    .add_systems(
        PostUpdate,
        collect_manifestations.in_set(DeveloperSet::CollectWorldDraw),
    );
}

fn collect_manifestations(
    controls: Res<DebugControls>,
    semantic_entities: Query<(Entity, &UsfManifestations), With<UsfEntity>>,
    manifestations: Query<(&GlobalTransform, Has<UsfManifestationAuthority>)>,
    frame: Res<WorldDrawFrame>,
) {
    if !controls.active(TOOL) {
        return;
    }

    let mut batch = WorldDrawBatch::default();
    let markers = controls.active(MARKERS);
    let links = controls.active(LINKS);
    let authority_only = controls.choice_value(SCOPE) == Some("authority");

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
            if authority_only && !authority {
                continue;
            }

            let color = if authority {
                Color::srgb(0.2, 1.0, 0.35)
            } else {
                Color::srgb(0.2, 0.65, 1.0)
            };
            let position = transform.translation();

            if markers {
                batch.cross(
                    Isometry3d::new(position, Quat::IDENTITY),
                    0.12,
                    color,
                    DrawDepth::World,
                );
            }
            if links && position.distance_squared(anchor) > 1.0e-6 {
                batch.line(anchor, position, color, DrawDepth::World);
            }
        }
    }

    frame.submit(batch);
}
