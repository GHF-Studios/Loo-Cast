use bevy::prelude::*;

use super::components::{FollowerComponent, FollowerTargetComponent};

pub(crate) fn observe_on_add_follower(
    trigger: Trigger<OnAdd, FollowerComponent>,
    mut param_set: ParamSet<(Query<(Entity, &mut FollowerComponent)>, Query<(Entity, &FollowerTargetComponent, &Transform)>)>,
) {
    let follower_entity = trigger.entity();

    let mut follower_query = param_set.p0();
    let follow_id = match follower_query.get_mut(follower_entity) {
        Ok((_, follower)) => follower.follow_id.clone(),
        Err(_) => {
            warn!("Failed to process new follower: {:?} not found in query.", follower_entity);
            return;
        }
    };

    let target_query = param_set.p1();
    let matching_target = target_query.iter().find(|(_, target_component, _)| target_component.id == follow_id);

    if let Some((target_entity, _, _)) = matching_target {
        let mut follower_query = param_set.p0();
        if let Ok((_, mut follower)) = follower_query.get_mut(follower_entity) {
            *follower.get_followed_entity_mut() = Some(target_entity);
            info!("Assigned follower '{}' to target '{}'.", follower.follow_id, follow_id);
        }
    } else {
        warn!("No matching target found for new follower '{}'.", follow_id);
    }
}
