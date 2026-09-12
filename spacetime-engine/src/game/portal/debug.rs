//! Portal-specific interpretation of the generic debug substrate.

use bevy::prelude::*;

use crate::debug::{
    AppDebugExt, DebugCamera, DebugGizmos, DebugOverlayGizmos, DebugView, DebugViews,
    billboard_text,
};

use super::{Portal, PortalActive, PortalEndpoint, PortalSplitTraveler};

struct PortalTopologyDebugView;

impl DebugView for PortalTopologyDebugView {
    const NAME: &'static str = "Portal / Topology";
}

pub(crate) fn configure(app: &mut App) {
    app.register_debug_view::<PortalTopologyDebugView>()
        .add_systems(PostUpdate, draw_portal_debug);
}

fn draw_portal_debug(
    views: Res<DebugViews>,
    camera: Query<&Transform, With<DebugCamera>>,
    portals: Query<(Entity, &Portal, &PortalActive, &Transform)>,
    split_travelers: Query<(Entity, &Transform, &PortalSplitTraveler)>,
    transforms: Query<&Transform>,
    mut gizmos: Gizmos<DebugGizmos>,
    mut overlay: Gizmos<DebugOverlayGizmos>,
) {
    if !views.enabled::<PortalTopologyDebugView>() {
        return;
    }
    let Some(camera) = camera.iter().next() else {
        return;
    };

    for (entity, portal, active, transform) in &portals {
        if !active.0 {
            continue;
        }

        let color = match portal.endpoint {
            PortalEndpoint::First => Color::srgb(0.05, 0.8, 1.0),
            PortalEndpoint::Second => Color::srgb(1.0, 0.25, 0.8),
        };
        let isometry = Isometry3d::new(transform.translation, transform.rotation);

        gizmos.rect(isometry, portal.half_size * 2.0, color);
        gizmos.axes(*transform, 0.55);
        gizmos.arrow(
            transform.translation,
            transform.translation + transform.rotation * Vec3::Z * 0.8,
            color,
        );

        if let Ok((_, _, destination_active, destination)) = portals.get(portal.destination) {
            if destination_active.0 {
                overlay.line(transform.translation, destination.translation, color);
            }
        }

        let label = format!(
            "portal {:?} {entity:?}\n-> {:?}",
            portal.endpoint, portal.destination,
        );
        billboard_text(
            &mut overlay,
            camera,
            transform.translation + transform.rotation * Vec3::Y * (portal.half_size.y + 0.25),
            &label,
            14.0,
            Vec2::new(-0.5, -0.5),
            color,
        );
    }

    for (entity, transform, split) in &split_travelers {
        let Some(active) = split.active else {
            continue;
        };
        let Ok(peer) = transforms.get(split.peer()) else {
            continue;
        };

        let color = Color::srgb(1.0, 0.65, 0.05);
        overlay.line(transform.translation, peer.translation, color);
        overlay.cross(
            Isometry3d::new(peer.translation, Quat::IDENTITY),
            0.16,
            color,
        );

        let label = format!(
            "split {entity:?}\n{:?} -> {:?}\npeer {:?}",
            active.source,
            active.destination,
            split.peer(),
        );
        billboard_text(
            &mut overlay,
            camera,
            transform.translation + Vec3::Y * 1.0,
            &label,
            13.0,
            Vec2::new(-0.5, -0.5),
            color,
        );
    }
}
