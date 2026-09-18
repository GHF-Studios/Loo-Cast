//! Derives portal presentation visibility from domain activation state.

use bevy::{
    camera::visibility::RenderLayers,
    light::{DirectionalLight, PointLight, SpotLight},
    prelude::*,
};

use crate::game::portal::{Portal, PortalActive, PortalPair};

use super::DERIVED_VIEW_LAYER;

/// Recursive rendering represents a complete pair, so a lone active endpoint
/// remains logically placed but visually hidden until its partner exists too.
pub fn sync_portal_visibility(
    pair: Res<PortalPair>,
    mut portals: Query<(Ref<PortalActive>, &mut Visibility), With<Portal>>,
) {
    let changed = pair.is_changed()
        || [pair.first, pair.second].into_iter().any(|entity| {
            portals
                .get(entity)
                .is_ok_and(|(active, _)| active.is_changed())
        });
    if !changed {
        return;
    }

    let pair_ready = [pair.first, pair.second]
        .into_iter()
        .all(|entity| portals.get(entity).is_ok_and(|(active, _)| active.0));
    let desired = if pair_ready {
        Visibility::Inherited
    } else {
        Visibility::Hidden
    };

    for entity in [pair.first, pair.second] {
        let Ok((_, mut visibility)) = portals.get_mut(entity) else {
            continue;
        };
        if *visibility != desired {
            *visibility = desired;
        }
    }
}

/// Keep ordinary world lights available to derived world views.
///
/// Bevy applies `RenderLayers` to lights as well as cameras/meshes. Preserve a
/// light's authored layers and mirror only lights that already affect the
/// ordinary world onto the derived-view layer.
pub fn sync_derived_view_lights(
    mut commands: Commands,
    lights: Query<
        (Entity, Option<&RenderLayers>),
        (
            Or<(With<PointLight>, With<DirectionalLight>, With<SpotLight>)>,
            Or<(
                Added<PointLight>,
                Added<DirectionalLight>,
                Added<SpotLight>,
                Changed<RenderLayers>,
            )>,
        ),
    >,
) {
    let world = RenderLayers::default();
    let derived = RenderLayers::layer(DERIVED_VIEW_LAYER);

    for (entity, layers) in &lights {
        let current = layers.cloned().unwrap_or_default();
        if current.intersects(&world) && !current.intersects(&derived) {
            commands
                .entity(entity)
                .insert(current.with(DERIVED_VIEW_LAYER));
        }
    }
}
