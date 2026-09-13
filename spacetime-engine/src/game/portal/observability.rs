//! Portal observability adapter.

use bevy::prelude::*;

use crate::observability::{
    AppObservabilityExt, DebugContext, DebugControlSpec, DebugControls, DebugDepth, DebugFrame,
    DebugFrameBatch, DebugId, DebugInspector, DebugInspectorSection, ObservabilitySet,
    CATEGORY_WORLD, format_quantity, scientific_unit,
};

use super::{Portal, PortalActive, PortalEndpoint, PortalSplitTraveler};

const TOOL: DebugId = DebugId("world.portal");
const APERTURES: DebugId = DebugId("world.portal.apertures");
const FRAMES: DebugId = DebugId("world.portal.frames");
const PAIR_LINKS: DebugId = DebugId("world.portal.pair_links");
const SPLITS: DebugId = DebugId("world.portal.splits");
const LABELS: DebugId = DebugId("world.portal.labels");

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
    .register_debug_control(DebugControlSpec::toggle(
        LABELS,
        Some(TOOL),
        "Labels",
        4,
        true,
    ))
    .add_systems(
        PostUpdate,
        collect_portal_state.in_set(ObservabilitySet::Collect),
    );
}

fn collect_portal_state(
    controls: Res<DebugControls>,
    context: Res<DebugContext>,
    portals: Query<(Entity, &Portal, &PortalActive, &GlobalTransform)>,
    split_travelers: Query<(Entity, &GlobalTransform, &PortalSplitTraveler)>,
    transforms: Query<&GlobalTransform>,
    frame: Res<DebugFrame>,
    inspector: Res<DebugInspector>,
) {
    if !controls.active(TOOL) {
        return;
    }

    let mut batch = DebugFrameBatch::default();

    if let Some(selection) = context.selection
        && let Ok((_, portal, active, _)) = portals.get(selection.entity)
        && active.0
    {
        let aperture = portal.half_size * 2.0;
        inspector.submit(
            DebugInspectorSection::new(DebugId("inspector.portal"), "Portal", 10)
                .row("Endpoint", format!("{:?}", portal.endpoint))
                .row("Destination", format!("{:?}", portal.destination))
                .row(
                    "Aperture",
                    format!(
                        "{} × {}",
                        format_quantity(aperture.x, scientific_unit::METER, 4),
                        format_quantity(aperture.y, scientific_unit::METER, 4),
                    ),
                )
                .row("Sidedness", format!("{:?}", portal.sidedness)),
        );
    }

    for (entity, portal, active, transform) in &portals {
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
                DebugDepth::World,
            );
        }

        if controls.active(FRAMES) {
            batch.axes(transform, 0.55, DebugDepth::World);
            batch.arrow(
                transform.translation,
                transform.translation + transform.rotation * Vec3::Z * 0.8,
                color,
                DebugDepth::World,
            );
        }

        if controls.active(PAIR_LINKS) {
            if let Ok((_, _, destination_active, destination)) = portals.get(portal.destination) {
                if destination_active.0 {
                    batch.line(
                        transform.translation,
                        destination.translation(),
                        color,
                        DebugDepth::Overlay,
                    );
                }
            }
        }

        if controls.active(LABELS) {
            batch.label_for(
                entity,
                transform.translation
                    + transform.rotation * Vec3::Y * (portal.half_size.y + 0.25),
                format!(
                    "portal {:?} {entity:?}\n-> {:?}",
                    portal.endpoint, portal.destination
                ),
                DebugFrameBatch::DEFAULT_LABEL_FONT_SIZE,
                color,
            );
        }
    }

    if !controls.active(SPLITS) {
        frame.submit(batch);
        return;
    }

    for (entity, transform, split) in &split_travelers {
        let Some(active) = split.active else {
            continue;
        };
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
            DebugDepth::Overlay,
        );
        batch.cross(
            Isometry3d::new(peer_position, Quat::IDENTITY),
            0.16,
            color,
            DebugDepth::Overlay,
        );

        if controls.active(LABELS) {
            batch.label_for(
                entity,
                transform.translation + Vec3::Y,
                format!(
                    "split {entity:?}\n{:?} -> {:?}\npeer {:?}",
                    active.source,
                    active.destination,
                    split.peer()
                ),
                DebugFrameBatch::DEFAULT_LABEL_FONT_SIZE,
                color,
            );
        }
    }

    frame.submit(batch);
}
