use super::components::*;
use bevy::prelude::*;
use crate::entity::resources::*;

pub(in crate) fn update(
    mut translation_lerp_follower_query: Query<(&mut Transform, &TranslationLerpFollower)>,
    target_query: Query<&Transform, Without<TranslationLerpFollower>>,
    entity_registry: Res<EntityRegistry>
) {
    for (mut transform, translation_lerp_follower) in translation_lerp_follower_query.iter_mut() {
        let target_entity = match entity_registry.get_loaded_entity_reference(&translation_lerp_follower.target) {
            Some(target_entity) => target_entity,
            None => continue,
        };

        let target_transform = match target_query.get(target_entity) {
            Ok(target_transform) => target_transform,
            Err(_) => continue,
        };
        
        let target_position = target_transform.translation;
        transform.translation = transform.translation.lerp(target_position, 1.0 - translation_lerp_follower.smoothness);
    }
}