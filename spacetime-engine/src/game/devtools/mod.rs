//! Test-game adapters for developer focus, inspection and domain tooling.

use avian3d::prelude::{SpatialQuery, SpatialQueryFilter};
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    devtools::{
        DeveloperFocus, DeveloperSet, DeveloperTools, DeveloperView, FocusHit, FocusTarget,
        InspectField, InspectSection, InspectSectionId, InspectValue, InspectionFrame,
    },
    ecs::{UsfManifestationAuthority, UsfManifestationOf, UsfManifestations},
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive},
    view::{PrimaryGameView, PrimaryViewPresentation, ViewRay, ViewportSpace},
};

use super::{
    player::{Player, cursor::CursorCapture},
    portal::{Portal, PortalActive},
};

const FOCUS_RANGE_METERS: f32 = 250.0;
const IDENTITY_SECTION: InspectSectionId = InspectSectionId("identity");

mod focus;
mod inspection;
mod view;

use focus::{handle_focus_pin, resolve_player_focus};
use inspection::collect_identity_inspection;
use view::resolve_developer_view;

pub struct TestGameDeveloperToolsPlugin;

impl Plugin for TestGameDeveloperToolsPlugin {
    fn build(&self, app: &mut App) {
        super::portal::devtools::configure(app);
        crate::thermal::devtools::configure(app);
        crate::thermal::world_draw::configure(app);

        app.add_systems(
            PostUpdate,
            (
                resolve_developer_view,
                resolve_player_focus,
                handle_focus_pin,
            )
                .chain()
                .in_set(DeveloperSet::ResolveFocus),
        )
        .add_systems(
            PostUpdate,
            collect_identity_inspection.in_set(DeveloperSet::CollectInspection),
        );
    }
}
