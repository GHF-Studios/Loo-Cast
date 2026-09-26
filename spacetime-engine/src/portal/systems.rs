//! Portal runtime adapters for cross-cutting engine events.

use bevy::prelude::*;

use crate::spatial::{UsfOriginRebased, UsfScaleLayer, UsfSpatialTransitionApplied};
use super::{
    PortalSplitTraveler, PortalTraveler,
    simulation::split::retire_split_partition,
};

pub(super) fn rebase_portal_local_caches(
    mut rebases: MessageReader<UsfOriginRebased>,
    mut travelers: Query<(Entity, Option<&UsfScaleLayer>, &mut PortalTraveler)>,
    mut split_travelers: Query<(Entity, Option<&UsfScaleLayer>, &mut PortalSplitTraveler)>,
) {
    for rebase in rebases.read() {
        for (entity, layer, mut traveler) in &mut travelers {
            let scale = layer.map_or(rebase.delta.source_scale(), |layer| layer.scale());
            match rebase.delta.at_scale(scale) {
                Ok(shift) if shift != Vec3::ZERO => traveler.rebase_local_origin(shift),
                Ok(_) => {}
                Err(error) => error!(
                    ?entity,
                    ?error,
                    scale = %scale,
                    "portal traveler cache could not project USF rebase delta"
                ),
            }
        }

        for (entity, layer, mut traveler) in &mut split_travelers {
            let scale = layer.map_or(rebase.delta.source_scale(), |layer| layer.scale());
            match rebase.delta.at_scale(scale) {
                Ok(shift) if shift != Vec3::ZERO => traveler.rebase_local_origin(shift),
                Ok(_) => {}
                Err(error) => error!(
                    ?entity,
                    ?error,
                    scale = %scale,
                    "portal split cache could not project USF rebase delta"
                ),
            }
        }
    }
}

/// A canonical relocation/rechart invalidates local-space portal crossing history.
pub(super) fn reset_portal_spatial_transition_caches(
    mut commands: Commands,
    mut transitions: MessageReader<UsfSpatialTransitionApplied>,
    mut travelers: Query<(&Transform, &mut PortalTraveler)>,
    mut split_travelers: Query<(&Transform, &mut PortalSplitTraveler)>,
) {
    for transition in transitions.read() {
        if let Ok((transform, mut traveler)) = travelers.get_mut(transition.anchor) {
            traveler.reset_spatial_transition(transform.translation);
        }
        if let Ok((transform, mut traveler)) = split_travelers.get_mut(transition.anchor) {
            retire_split_partition(&mut commands, &mut traveler);
            traveler.reset_spatial_transition(*transform);
        }
    }
}
