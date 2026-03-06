use crate::bevy::prelude::*;
use std::collections::HashMap;

use super::components::{Follower, FollowerTarget};
use super::messages::FollowerTargetLifecycleMessage;

pub(crate) fn update_follower_system(
    mut follower_target_lifecycle_message_reader: MessageReader<FollowerTargetLifecycleMessage>,
    mut param_set: ParamSet<(Query<(Entity, &mut Transform, &mut Follower)>, Query<(&FollowerTarget, &Transform)>)>,
    time: Res<Time<Virtual>>,
    mut previous_target_positions: Local<HashMap<String, Vec2>>,
) {
    process_lifecycle_messages(&mut follower_target_lifecycle_message_reader, &mut param_set.p0());
    let targets = collect_target_positions(&mut param_set.p1());
    update_followers(
        &mut param_set.p0(),
        &targets,
        &time,
        &mut previous_target_positions,
    );
}

fn process_lifecycle_messages(messages: &mut MessageReader<FollowerTargetLifecycleMessage>, followers_query: &mut Query<(Entity, &mut Transform, &mut Follower)>) {
    for message in messages.read() {
        match message {
            FollowerTargetLifecycleMessage::Add { follow_id, followed_entity } => {
                assign_follower(followers_query, follow_id, followed_entity);
            }
            FollowerTargetLifecycleMessage::Remove { follow_id, .. } => {
                unassign_follower(followers_query, follow_id);
            }
        }
    }
}

fn assign_follower(followers_query: &mut Query<(Entity, &mut Transform, &mut Follower)>, follow_id: &String, followed_entity: &Entity) {
    for (follower_entity, _, mut follower) in followers_query.iter_mut() {
        if follower.get_followed_entity().is_some() || follow_id != &follower.follow_id {
            continue;
        }
        if *followed_entity == follower_entity {
            warn!("Entity '{}' attempted to follow itself. Ignoring.", follower_entity);
            return;
        }
        *follower.get_followed_entity_mut() = Some(*followed_entity);
        return;
    }
}

fn unassign_follower(followers_query: &mut Query<(Entity, &mut Transform, &mut Follower)>, follow_id: &String) {
    for (_, _, mut follower) in followers_query.iter_mut() {
        if follower.get_followed_entity().is_some() && follow_id == &follower.follow_id {
            *follower.get_followed_entity_mut() = None;
            return;
        }
    }
}

fn collect_target_positions(targets_query: &mut Query<(&FollowerTarget, &Transform)>) -> HashMap<String, Vec3> {
    targets_query
        .iter()
        .map(|(target, transform)| (target.id.clone(), transform.translation))
        .collect()
}

fn update_followers(
    followers_query: &mut Query<(Entity, &mut Transform, &mut Follower)>,
    targets: &HashMap<String, Vec3>,
    time: &Res<Time<Virtual>>,
    previous_target_positions: &mut HashMap<String, Vec2>,
) {
    // Track jump discontinuities in followed targets (e.g. USF border fold) so followers
    // preserve their current lag offset instead of gliding back toward a wrapped position.
    const TARGET_JUMP_SNAP_DISTANCE: f32 = 400.0;

    for (_, mut follower_transform, mut follower) in followers_query.iter_mut() {
        if let Some(target_position) = follower.get_followed_entity().and_then(|_| targets.get(&follower.follow_id)) {
            let target_pos_2d = target_position.truncate();
            let previous_target_pos = previous_target_positions.insert(follower.follow_id.clone(), target_pos_2d);

            match previous_target_pos {
                Some(previous_target_pos) => {
                    let target_jump_delta = target_pos_2d - previous_target_pos;
                    if target_jump_delta.length_squared() >= TARGET_JUMP_SNAP_DISTANCE * TARGET_JUMP_SNAP_DISTANCE {
                        // Apply the same target jump to the camera to keep relative lag intact.
                        follower_transform.translation.x += target_jump_delta.x;
                        follower_transform.translation.y += target_jump_delta.y;
                        continue;
                    }
                }
                None => {
                    // First frame after assignment should align with current target.
                    let aligned_target = target_pos_2d + follower.offset;
                    follower_transform.translation.x = aligned_target.x;
                    follower_transform.translation.y = aligned_target.y;
                    continue;
                }
            };

            update_follower_position(
                &mut follower,
                &mut follower_transform,
                target_pos_2d,
                time,
            );
        }
    }
}

fn update_follower_position(
    follower: &mut Follower,
    follower_transform: &mut Transform,
    target_position: Vec2,
    time: &Res<Time<Virtual>>,
) {
    if follower.smoothness < 0.0 {
        warn!("Smoothness value for follower '{}' is less than 0. Clamping to 0.", follower.follow_id);
        follower.smoothness = 0.0;
    }

    let target_position_2d = target_position + follower.offset;
    // Clamp smoothing delta so post-load frame hitches don't collapse the intended follow lag.
    const MAX_SMOOTHING_DT_SECS: f32 = 1.0 / 30.0;
    let smoothing_dt_secs = time.delta_secs().min(MAX_SMOOTHING_DT_SECS);

    let interpolation_factor = if follower.smoothness == 0.0 {
        1.0
    } else {
        1.0 - (-smoothing_dt_secs / follower.smoothness).exp()
    };

    follower_transform.translation = follower_transform
        .translation
        .lerp(target_position_2d.extend(follower_transform.translation.z), interpolation_factor);
}
