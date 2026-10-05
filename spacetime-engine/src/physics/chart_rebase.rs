//! Coherent floating-origin rebasing for the Avian backend.
//!
//! A USF chart rebase changes numerical representation, not physical motion.
//! Feeding that shift through Avian's ordinary `Position` change path makes
//! every collider look independently moved, which can force collider-tree
//! optimization despite unchanged relative geometry.

use avian3d::{
    collider_tree::{ColliderTreeProxyKey, ColliderTreeType, ColliderTrees},
    collision::collider::EnlargedAabb,
    prelude::{
        Collider, ColliderAabb, ColliderOf, Position, RigidBody,
    },
};
use bevy::prelude::*;

use crate::spatial::{UsfOriginRebased, UsfScaleLayer, resolved_rebase_scale};

const fn tree_index(tree_type: ColliderTreeType) -> usize {
    match tree_type {
        ColliderTreeType::Dynamic => 0,
        ColliderTreeType::Kinematic => 1,
        ColliderTreeType::Static => 2,
        ColliderTreeType::Standalone => 3,
    }
}

/// Applies chart-origin shifts directly to Avian's local representation.
///
/// Leaves move at their Scale Slice and each affected BVH is refitted once.
/// No proxy is marked moved merely because the coordinate chart changed.
pub(super) fn apply_usf_rebases_to_avian(
    mut rebases: MessageReader<UsfOriginRebased>,
    body_layers: Query<&UsfScaleLayer, With<RigidBody>>,
    mut positions: Query<(
        &mut Position,
        Option<&UsfScaleLayer>,
        Option<&ColliderOf>,
    )>,
    mut colliders: Query<
        (
            &mut ColliderAabb,
            &mut EnlargedAabb,
            &ColliderTreeProxyKey,
            Option<&UsfScaleLayer>,
            Option<&ColliderOf>,
        ),
        With<Collider>,
    >,
    mut trees: ResMut<ColliderTrees>,
) {
    for rebase in rebases.read() {
        let _span = bevy::log::info_span!("usf_rebase.avian_backend").entered();
        let delta = rebase.delta;
        let fallback = delta.source_scale();

        {
            let _span = bevy::log::info_span!("usf_rebase.avian_positions").entered();
            for (mut position, direct_layer, attached) in &mut positions {
                let scale = resolved_rebase_scale(direct_layer, attached, &body_layers, fallback);
                let local_shift = delta
                    .at_scale(scale)
                    .expect("USF rebase scale conversion was preflighted");

                // Preserve any earlier genuine-motion tick. The chart shift itself
                // is representation synchronization, not body motion.
                position.bypass_change_detection().0 -= local_shift;
            }
        }

        let mut touched_trees = [false; 4];
        {
            let _span = bevy::log::info_span!("usf_rebase.avian_leaves").entered();

            for (
                mut tight,
                mut enlarged,
                proxy_key,
                direct_layer,
                attached,
            ) in &mut colliders
            {
                let scale = resolved_rebase_scale(direct_layer, attached, &body_layers, fallback);
                let local_shift = delta
                    .at_scale(scale)
                    .expect("USF rebase scale conversion was preflighted");

                {
                    let tight = tight.bypass_change_detection();
                    tight.min -= local_shift;
                    tight.max -= local_shift;
                }

                let old = enlarged.get();
                let shifted = ColliderAabb::from_min_max(
                    old.min - local_shift,
                    old.max - local_shift,
                );
                *enlarged.bypass_change_detection() = EnlargedAabb::new(shifted);

                if *proxy_key == ColliderTreeProxyKey::PLACEHOLDER {
                    continue;
                }

                let tree_type = proxy_key.tree_type();
                trees
                    .tree_for_type_mut(tree_type)
                    .set_proxy_aabb(proxy_key.id(), shifted.into());
                touched_trees[tree_index(tree_type)] = true;
            }
        }

        {
            let _span = bevy::log::info_span!("usf_rebase.avian_refit").entered();
            for tree_type in ColliderTreeType::ALL {
                if touched_trees[tree_index(tree_type)] {
                    trees.tree_for_type_mut(tree_type).refit_all();
                }
            }
        }
    }
}
