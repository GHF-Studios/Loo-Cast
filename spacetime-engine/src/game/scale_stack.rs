//! Sparse hierarchical stack of actual scale-local voxel worlds.
//!
//! Higher scales remain resident when finer scales are introduced. The result is
//! a vertical multi-scale realization spine rather than mutually exclusive worlds.
//! Voxels at every scale are intentionally a testing realizer; semantic worldgen
//! remains authoritative and contextualizes every finer level.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    spatial::{SPATIAL_SCALE_MAX, SpatialScale, UsfPosition, UsfScaleLayer, UsfViewFrame},
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
        root_world: Entity,
    ) -> Self {
        let mut active = HashMap::new();
        active.insert(SpatialScale::MAX, root_world);
        Self {
            semantic_target,
            root,
            material,
            active,
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
        let desired = desired_scales(&view);

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
                .expect("scale-local refinement must remain canonically addressable")
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
            info!(
                scale = %scale,
                resident_scale_worlds = stack.active.len(),
                "extended hierarchical USF voxel realization spine"
            );
        }
    }
}

fn desired_scales(view: &UsfViewFrame) -> Vec<SpatialScale> {
    let interaction = view.interaction_scale();
    let mut desired = (interaction.exponent()..=SPATIAL_SCALE_MAX)
        .rev()
        .map(|raw| SpatialScale::new(raw).expect("validated scale"))
        .collect::<Vec<_>>();

    if view.scale() < interaction && view.contribution(view.scale()) > 0.001 {
        desired.push(view.scale());
    }

    desired
}

pub(super) fn volume_for_scale_context(
    worldgen: &WorldgenStore,
    key: WorldgenEvaluationKey,
) -> ProceduralVolume {
    let node = worldgen
        .node(key)
        .expect("requested scale context must already exist");
    ProceduralVolume::scale_layer(
        worldgen.universe_seed(),
        node.context().seed(),
        key.scope().scale(),
    )
}
