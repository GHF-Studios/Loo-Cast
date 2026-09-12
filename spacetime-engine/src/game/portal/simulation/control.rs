//! Applies public portal-domain commands to persistent physical endpoints.
//!
//! Presentation is intentionally absent here. Rendering observes
//! [`PortalActive`] and derives visibility separately.

use bevy::prelude::*;

use crate::game::portal::{Portal, PortalActive, PortalCommand, PortalEndpoint, PortalPair};

pub fn apply_portal_commands(
    pair: Option<Res<PortalPair>>,
    mut commands: MessageReader<PortalCommand>,
    mut portals: Query<(&mut Transform, &mut PortalActive), With<Portal>>,
) {
    let Some(pair) = pair else {
        // Consume commands even if startup has not produced the persistent pair.
        for _ in commands.read() {}
        return;
    };

    for command in commands.read() {
        match command {
            PortalCommand::Place {
                endpoint,
                transform,
            } => {
                // Physical portals are rigid transforms. Invalid scale is a
                // malformed command, not presentation state to compensate for.
                if (transform.scale - Vec3::ONE).length_squared() > 1e-6 {
                    continue;
                }

                if let Ok((mut current, mut active)) = portals.get_mut(pair.entity(*endpoint)) {
                    *current = transform.clone();
                    active.0 = true;
                }
            }
            PortalCommand::Remove { endpoint } => {
                if let Ok((_, mut active)) = portals.get_mut(pair.entity(*endpoint)) {
                    active.0 = false;
                }
            }
            PortalCommand::RemovePair => {
                for endpoint in [PortalEndpoint::First, PortalEndpoint::Second] {
                    if let Ok((_, mut active)) = portals.get_mut(pair.entity(endpoint)) {
                        active.0 = false;
                    }
                }
            }
        }
    }
}
