//! Observer-derived sparse presentation demand over the USF Scale Stack.
//!
//! Camera capture publishes relevance only. Semantic residency and physical
//! interaction are owned by their own demand and capability paths.

use bevy::{camera::visibility::VisibilitySystems, prelude::*};

mod capture;
mod model;
mod policy;

pub use model::{UsfViewDemand, UsfViewDemandSnapshot};
pub use policy::{UsfViewDemandMode, UsfViewDemandPolicy};

pub(super) fn configure(app: &mut App) {
    app.init_resource::<UsfViewDemandPolicy>()
        .init_resource::<UsfViewDemandSnapshot>()
        .add_systems(
            PostUpdate,
            capture::capture_view_demand.after(VisibilitySystems::UpdateFrusta),
        );
}
