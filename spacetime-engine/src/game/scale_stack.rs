//! Demand-driven scale-local voxel worlds.
//!
//! Voxels are intentionally allowed at every scale as a torture test of the
//! scale-layer architecture. Semantic worldgen remains authoritative.

use bevy::prelude::*;
use std::collections::HashMap;

use crate::{
    spatial::{SpatialScale, UsfPosition, UsfScaleLayer, UsfViewFrame},
    voxel::{ProceduralVolume, VoxelBase, VoxelStreaming, VoxelWorld},
    worldgen::{PhenomenonRegistry, WorldgenEvaluationKey, WorldgenStore},
};

#[derive(Component)]
pub(super) struct ProceduralScaleStack {
    semantic_target: UsfPosition,
    root: WorldgenEvaluationKey,
    material: Handle<StandardMaterial>,
    active: HashMap<SpatialScale, Entity>,
}

impl ProceduralScaleStack {
    pub(super) fn new(
        semantic_target: UsfPosition,
        root: WorldgenEvaluationKey,
        material: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            semantic_target,
            root,
            material,
            active: HashMap::new(),
        }
    }
}

pub(super) fn sync_scale_stack(
    mut commands: Commands,
    view: Res<UsfViewFrame>,
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
    mut stacks: Query<(Entity, &mut ProceduralScaleStack)>,
) {
    for (stack_entity, mut stack) in &mut stacks {
        let desired = view
            .active_scale_demands()
            .into_iter()
            .flatten()
            .filter(|demand| demand.scale() >= SpatialScale::ZERO && demand.contribution() > 0.001)
            .map(|demand| demand.scale())
            .collect::<Vec<_>>();

        let stale = stack
            .active
            .iter()
            .filter_map(|(scale, entity)| (!desired.contains(scale)).then_some((*scale, *entity)))
            .collect::<Vec<_>>();
        for (scale, entity) in stale {
            stack.active.remove(&scale);
            commands.entity(entity).despawn();
        }

        for scale in desired {
            if stack.active.contains_key(&scale) {
                continue;
            }
            let key = worldgen
                .contextualize_to(stack.root, stack.semantic_target, scale, &registry)
                .expect("scale-local refinement must remain addressable")
                .expect("root context must refine to requested scale");
            let volume = volume_for_scale_context(&worldgen, key);
            let entity = commands
                .spawn((
                    Name::new(format!("USF Scale {scale} Voxel World")),
                    ChildOf(stack_entity),
                    UsfScaleLayer::new(scale),
                    VoxelWorld::new_at(VoxelBase::Volume(volume), UsfPosition::zero(scale)),
                    VoxelStreaming::new(24, stack.material.clone()),
                    Transform::IDENTITY,
                    Visibility::Inherited,
                ))
                .id();
            stack.active.insert(scale, entity);
            info!(scale = %scale, "instantiated scale-local voxel simulation layer");
        }
    }
}

pub(super) fn volume_for_scale_context(
    worldgen: &WorldgenStore,
    key: WorldgenEvaluationKey,
) -> ProceduralVolume {
    let node = worldgen
        .node(key)
        .expect("requested scale context must exist");
    ProceduralVolume::scale_layer(
        worldgen.universe_seed(),
        node.context().seed(),
        key.scope().scale(),
    )
}
