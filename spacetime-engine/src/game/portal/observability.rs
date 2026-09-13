//! Portal observability adapter.

use bevy::prelude::*;

use crate::{
    devtools::{DeveloperSet, DrawDepth, WorldDrawBatch, WorldDrawFrame},
    observability::{
        AppObservabilityExt, DebugControlSpec, DebugControls, DebugId, CATEGORY_WORLD,
    },
};

use super::{Portal, PortalActive, PortalEndpoint, PortalSplitTraveler};

const TOOL: DebugId = DebugId("world.portal");
const APERTURES: DebugId = DebugId("world.portal.apertures");
const FRAMES: DebugId = DebugId("world.portal.frames");
const PAIR_LINKS: DebugId = DebugId("world.portal.pair_links");
const SPLITS: DebugId = DebugId("world.portal.splits");

pub(crate) fn configure(app: &mut App) {
    app.register_debug_control(
        DebugControlSpec::tool(
            TOOL,
            Some(CATEGORY_WORLD),
            "Portals",
            10,
            false,
        )
        .described("Physical apertures, mappings and active split manifestations."),
    )
    .register_debug_control(DebugControlSpec::toggle(
        APERTURES,
        Some(TOOL),
        "Apertures",
        0,
        true,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        FRAMES,
        Some(TOOL),
        "Frames / normals",
        1,
        true,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        PAIR_LINKS,
        Some(TOOL),
        "Pair mapping links",
        2,
        true,
    ))
    .register_debug_control(DebugControlSpec::toggle(
        SPLITS,
        Some(TOOL),
        "Active splits",
        3,
        true,
    ))
    .add_systems(
        PostUpdate,
        collect_portal_state.in_set(DeveloperSet::CollectWorldDraw),
    );
}

fn collect_portal_state(
    controls: Res<DebugControls>,
    portals: Query<(&Portal, &PortalActive, &GlobalTransform)>,
    split_travelers: Query<(&GlobalTransform, &PortalSplitTraveler)>,
    transforms: Query<&GlobalTransform>,
    frame: Res<WorldDrawFrame>,
) {
    if !controls.active(TOOL) {
        return;
    }

    let mut batch = WorldDrawBatch::default();

    for (portal, active, transform) in &portals {
        if !active.0 {
            continue;
        }

        let color = match portal.endpoint {
            PortalEndpoint::First => Color::srgb(0.05, 0.8, 1.0),
            PortalEndpoint::Second => Color::srgb(1.0, 0.25, 0.8),
        };
        let transform = transform.compute_transform();

        if controls.active(APERTURES) {
            batch.rect(
                Isometry3d::new(transform.translation, transform.rotation),
                portal.half_size * 2.0,
                color,
                DrawDepth::World,
            );
        }

        if controls.active(FRAMES) {
            batch.axes(transform, 0.55, DrawDepth::World);
            batch.arrow(
                transform.translation,
                transform.translation + transform.rotation * Vec3::Z * 0.8,
                color,
                DrawDepth::World,
            );
        }

        if controls.active(PAIR_LINKS)
            && let Ok((_, destination_active, destination)) = portals.get(portal.destination)
            && destination_active.0
        {
            batch.line(
                transform.translation,
                destination.translation(),
                color,
                DrawDepth::Overlay,
            );
        }
    }

    if !controls.active(SPLITS) {
        frame.submit(batch);
        return;
    }

    for (transform, split) in &split_travelers {
        if split.active.is_none() {
            continue;
        }
        let Ok(peer) = transforms.get(split.peer()) else {
            continue;
        };

        let transform = transform.compute_transform();
        let peer_position = peer.translation();
        let color = Color::srgb(1.0, 0.65, 0.05);
        batch.line(
            transform.translation,
            peer_position,
            color,
            DrawDepth::Overlay,
        );
        batch.cross(
            Isometry3d::new(peer_position, Quat::IDENTITY),
            0.16,
            color,
            DrawDepth::Overlay,
        );
    }

    frame.submit(batch);
}
