//! Temporary high-signal profiling probe for Avian collider-tree optimization.
//!
//! Emits one compact line only on physics ticks where Avian actually has moved
//! proxies to optimize. Remove once the burst source is identified.

use avian3d::{
    collider_tree::{
        ColliderTreeOptimization, ColliderTreeSystems, ColliderTreeType, ColliderTrees,
    },
    prelude::PhysicsSchedule,
};
use bevy::prelude::*;

pub(super) fn configure(app: &mut App) {
    app.add_systems(
        PhysicsSchedule,
        trace_tree_pressure
            .before(ColliderTreeSystems::BeginOptimize)
            .ambiguous_with_all(),
    );
}

fn trace_tree_pressure(
    trees: Res<ColliderTrees>,
    optimization: Res<ColliderTreeOptimization>,
    names: Query<&Name>,
) {
    for tree_type in ColliderTreeType::ALL {
        let tree = trees.tree_for_type(tree_type);
        let total = tree.proxies.len();
        let moved = tree.moved_proxies.len();
        if moved == 0 {
            continue;
        }

        let ratio = moved as f32 / total.max(1) as f32;
        let strategy = optimization.optimization_mode.resolve(ratio);

        let mut voxel_collision_aggregates = 0_usize;
        let mut named_other = 0_usize;
        let mut unnamed = 0_usize;

        for &proxy_id in &tree.moved_proxies {
            let Some(proxy) = tree.get_proxy(proxy_id) else {
                continue;
            };
            match names.get(proxy.collider) {
                Ok(name) if name.as_str() == "Voxel Collision Aggregate" => {
                    voxel_collision_aggregates += 1;
                }
                Ok(_) => named_other += 1,
                Err(_) => unnamed += 1,
            }
        }

        info!(
            ?tree_type,
            total_proxies = total,
            moved_proxies = moved,
            moved_ratio = ratio,
            ?strategy,
            voxel_collision_aggregates,
            named_other,
            unnamed,
            optimize_in_place = optimization.optimize_in_place,
            async_tasks = optimization.use_async_tasks,
            "AVIAN_TREE_PRESSURE"
        );
    }
}
