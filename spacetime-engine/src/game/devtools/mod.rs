//! Loo Cast adapters into generic developer focus, inspection and tooling facilities.
//!
//! ## Module map
//!
//! - `focus`: Game-specific spatial focus resolution, portal aperture hits and focus pinning.
//! - `inspection`: Identity/manifestation inspection contributed by the test game.
//! - `lab`: Live developer override/preset composition.
//! - `view`: Resolution of the game camera/cursor into the generic developer interaction view.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

use avian3d::prelude::SpatialQuery;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    devtools::{
        DeveloperFocus, DeveloperSet, DeveloperTools, DeveloperView, FocusHit, FocusTarget,
        InspectField, InspectSection, InspectSectionId, InspectValue, InspectionFrame,
    },
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive, UsfRuntimeOwnershipQuery},
    portal::{Portal, PortalActive},
    view::{PrimaryGameView, PrimaryViewPresentation, ViewRay, ViewportSpace},
};

use super::player::{Player, cursor::CursorCapture};

const FOCUS_RANGE_METRES: f32 = 250.0;
const IDENTITY_SECTION: InspectSectionId = InspectSectionId("identity");

mod focus;
mod inspection;
pub(crate) mod lab;
mod runtime_audit;
mod view;

use focus::{handle_focus_pin, resolve_player_focus};
use inspection::collect_identity_inspection;
use runtime_audit::audit_primary_runtime_coherence;
use view::resolve_developer_view;

pub struct LooCastDeveloperAdaptersPlugin;

impl Plugin for LooCastDeveloperAdaptersPlugin {
    fn build(&self, app: &mut App) {
        crate::portal::devtools::configure(app);
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
        )
        .add_systems(
            PostUpdate,
            audit_primary_runtime_coherence.after(crate::spatial::UsfSpatialSet::ViewProjection),
        );
    }
}
