//! View-only canonical observer projection and reversible policy override.

use super::super::PlayerCamera;
use super::{DebugFreecam, FreecamProjectionPolicy};
use crate::{
    game::control::LocalViewTarget,
    physics::topology::UsfRuntimeOwnershipQuery,
    spatial::{
        UsfPosition, UsfScaleLayer, UsfViewDemandMode, UsfViewDemandPolicy,
        UsfViewObservationOverride,
    },
};
use bevy::prelude::*;

/// Reconciles freecam with the generic observer/view-demand policy owned by #40.
///
/// The canonical gameplay subject and its `SpatialDemandSource` / refinement
/// components are never edited here. Follow mode derives a *view-only* canonical
/// observer position from the subject anchor plus the bounded freecam offset.
pub(in crate::game::player) fn sync_freecam_observer_policy(
    settings: Res<DebugFreecam>,
    runtime_ownership: UsfRuntimeOwnershipQuery,
    subject: Single<(Entity, &Transform, &UsfScaleLayer), With<LocalViewTarget>>,
    semantic_positions: Query<&UsfPosition>,
    camera: Single<&Transform, With<PlayerCamera>>,
    mut observation: ResMut<UsfViewObservationOverride>,
    mut demand_policy: ResMut<UsfViewDemandPolicy>,
    mut previous_observation: Local<Option<UsfViewObservationOverride>>,
    mut previous_demand_mode: Local<Option<UsfViewDemandMode>>,
) {
    if !settings.enabled() {
        if let Some(previous) = previous_observation.take() {
            *observation = previous;
        }
        if let Some(previous) = previous_demand_mode.take() {
            demand_policy.set_mode(previous);
        }
        return;
    }

    if previous_observation.is_none() {
        *previous_observation = Some(*observation);
    }
    if previous_demand_mode.is_none() {
        *previous_demand_mode = Some(demand_policy.mode());
    }
    demand_policy.set_mode(settings.view_demand_mode());

    if matches!(
        settings.projection_policy(),
        FreecamProjectionPolicy::Disabled
    ) {
        observation.clear();
        return;
    }
    if matches!(
        settings.projection_policy(),
        FreecamProjectionPolicy::Frozen
    ) && observation.current().is_some()
    {
        return;
    }

    let (subject_entity, subject_transform, layer) = subject.into_inner();
    let Some(semantic_entity) = runtime_ownership.semantic_of(subject_entity) else {
        warn!(?subject_entity, "freecam view target has no semantic owner");
        observation.clear();
        return;
    };
    let Ok(&subject_anchor) = semantic_positions.get(semantic_entity) else {
        warn!(
            ?semantic_entity,
            "freecam semantic owner has no canonical position"
        );
        observation.clear();
        return;
    };

    let runtime_anchor = camera.translation;
    let runtime_offset = runtime_anchor - subject_transform.translation;
    match subject_anchor.translated_at_scale(layer.scale(), runtime_offset) {
        Ok(anchor) => observation.set(anchor, runtime_anchor),
        Err(error) => {
            warn!(
                ?error,
                "freecam observer offset could not enter canonical USF space"
            );
            observation.clear();
        }
    }
}
