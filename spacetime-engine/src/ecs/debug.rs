//! Debug interpretation of semantic entities and their manifestations.

use bevy::prelude::*;

use crate::debug::{
    AppDebugExt, DebugCamera, DebugGizmos, DebugOverlayGizmos, DebugView, DebugViews,
    billboard_text,
};

use super::{UsfEntity, UsfManifestationAuthority, UsfManifestations};

struct UsfManifestationDebugView;

impl DebugView for UsfManifestationDebugView {
    const NAME: &'static str = "USF / Manifestations";
}

pub(crate) fn configure(app: &mut App) {
    app.register_debug_view::<UsfManifestationDebugView>()
        .add_systems(PostUpdate, draw_manifestations);
}

fn draw_manifestations(
    views: Res<DebugViews>,
    camera: Query<&Transform, With<DebugCamera>>,
    semantic_entities: Query<(Entity, &UsfManifestations), With<UsfEntity>>,
    manifestations: Query<(Option<&Name>, &Transform, Has<UsfManifestationAuthority>)>,
    mut gizmos: Gizmos<DebugGizmos>,
    mut overlay: Gizmos<DebugOverlayGizmos>,
) {
    if !views.enabled::<UsfManifestationDebugView>() {
        return;
    }
    let Some(camera) = camera.iter().next() else {
        return;
    };

    for (semantic, linked) in &semantic_entities {
        let Some(anchor) = linked
            .iter()
            .filter_map(|entity| manifestations.get(entity).ok())
            .find(|(_, _, authority)| *authority)
            .or_else(|| linked.iter().find_map(|entity| manifestations.get(entity).ok()))
            .map(|(_, transform, _)| transform.translation)
        else {
            continue;
        };

        for manifestation in linked.iter() {
            let Ok((name, transform, authority)) = manifestations.get(manifestation) else {
                continue;
            };

            let color = if authority {
                Color::srgb(0.2, 1.0, 0.35)
            } else {
                Color::srgb(0.2, 0.65, 1.0)
            };
            let position = transform.translation;

            gizmos.cross(Isometry3d::new(position, Quat::IDENTITY), 0.12, color);
            if position.distance_squared(anchor) > 1.0e-6 {
                gizmos.line(anchor, position, color);
            }

            let label = match name {
                Some(name) => format!(
                    "USF {semantic:?}\n{}{}",
                    name.as_str(),
                    if authority { " [authority]" } else { "" },
                ),
                None => format!(
                    "USF {semantic:?}\n{manifestation:?}{}",
                    if authority { " [authority]" } else { "" },
                ),
            };
            billboard_text(
                &mut overlay,
                camera,
                position + Vec3::Y * 0.35,
                &label,
                13.0,
                Vec2::new(-0.5, -0.5),
                color,
            );
        }
    }
}
