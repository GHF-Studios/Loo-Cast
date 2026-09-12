//! Derives portal presentation visibility from domain activation state.

use bevy::prelude::*;

use crate::game::portal::{
    Portal,
    PortalActive,
    PortalPair,
};

/// Recursive rendering represents a complete pair, so a lone active endpoint
/// remains logically placed but visually hidden until its partner exists too.
pub fn sync_portal_visibility(
    pair: Res<PortalPair>,
    mut portals: Query<
        (&PortalActive, &mut Visibility),
        With<Portal>,
    >,
) {
    let pair_ready = [pair.first, pair.second]
        .into_iter()
        .all(|entity| {
            portals
                .get(entity)
                .is_ok_and(|(active, _)| active.0)
        });

    for entity in [pair.first, pair.second] {
        let Ok((_, mut visibility)) = portals.get_mut(entity) else {
            continue;
        };

        *visibility = if pair_ready {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        };
    }
}
