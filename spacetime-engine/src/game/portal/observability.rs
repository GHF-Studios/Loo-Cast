//! Portal topology developer visualization.

use bevy::prelude::*;

use crate::devtools::{
    AppDeveloperToolsExt, DeveloperSet, DeveloperTools, DrawDepth, VisualizationId,
    VisualizationSpec, WorldDrawBatch, WorldDrawFrame,
};

use super::{Portal, PortalActive, PortalEndpoint, PortalSplitTraveler};

const VISUALIZATION: VisualizationId = VisualizationId("world.portal");

pub(crate) fn configure(app: &mut App) {
    app.register_developer_visualization(VisualizationSpec::new(
        VISUALIZATION,
        "Portal topology",
        30,
        false,
    ))
    .add_systems(
        PostUpdate,
        collect_portal_state.in_set(DeveloperSet::CollectWorldDraw),
    );
}

fn collect_portal_state(
    tools: Res<DeveloperTools>,
    portals: Query<(&Portal, &PortalActive, &GlobalTransform)>,
    split_travelers: Query<(&GlobalTransform, &PortalSplitTraveler)>,
    transforms: Query<&GlobalTransform>,
    frame: Res<WorldDrawFrame>,
) {
    if !tools.visualization_enabled(VISUALIZATION) {
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

        batch.rect(
            Isometry3d::new(transform.translation, transform.rotation),
            portal.half_size * 2.0,
            color,
            DrawDepth::World,
        );
        batch.axes(transform, 0.55, DrawDepth::World);
        batch.arrow(
            transform.translation,
            transform.translation + transform.rotation * Vec3::Z * 0.8,
            color,
            DrawDepth::World,
        );

        if let Ok((_, destination_active, destination)) = portals.get(portal.destination)
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
