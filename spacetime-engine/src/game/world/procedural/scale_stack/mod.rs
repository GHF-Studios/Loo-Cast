//! Sparse hierarchical stack of actual scale-local voxel worlds.
//!
//! Higher scales remain resident when finer scales are introduced. The result is
//! a vertical multi-scale realization spine rather than mutually exclusive worlds.
//! Voxels at every scale are intentionally a testing realizer; semantic worldgen
//! remains authoritative and contextualizes every finer level.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    spatial::{SPATIAL_SCALE_MAX, SpatialScale, UsfPosition, UsfScaleLayer, UsfViewFrame},
    voxel::{ProceduralVolume, VoxelBase, VoxelPresentationMaterial, VoxelStreaming, VoxelWorld},
    worldgen::{PhenomenonRegistry, WorldgenEvaluationKey, WorldgenStore},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScaleStackDemandKey {
    interaction: SpatialScale,
    refinement: Option<SpatialScale>,
}

impl ScaleStackDemandKey {
    fn from_view(view: &UsfViewFrame) -> Self {
        let interaction = view.interaction_scale();
        let refinement = (view.scale() < interaction && view.contribution(view.scale()) > 0.001)
            .then_some(view.scale());

        Self {
            interaction,
            refinement,
        }
    }
}

#[derive(Component)]
pub(super) struct ProceduralScaleStack {
    semantic_target: UsfPosition,
    root: WorldgenEvaluationKey,
    material: Handle<StandardMaterial>,
    active: HashMap<SpatialScale, Entity>,
    last_demand: Option<ScaleStackDemandKey>,
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
            last_demand: None,
        }
    }
}

pub(super) fn sync_scale_stack(
    config: Res<EngineConfig>,
    mut commands: Commands,
    view: Res<UsfViewFrame>,
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
    mut stacks: Query<(Entity, &mut ProceduralScaleStack)>,
) {
    let demand = ScaleStackDemandKey::from_view(&view);

    for (stack_entity, mut stack) in &mut stacks {
        if stack.last_demand == Some(demand) {
            continue;
        }

        stack.active.retain(|scale, entity| {
            if scale_is_desired(demand, *scale) {
                true
            } else {
                commands.entity(*entity).despawn();
                false
            }
        });

        for scale in desired_scales(demand) {
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
                    VoxelStreaming::new(config.voxel.streaming.default_load_budget_per_frame),
                    VoxelPresentationMaterial::new(stack.material.clone()),
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

        stack.last_demand = Some(demand);
    }
}

fn desired_scales(demand: ScaleStackDemandKey) -> impl Iterator<Item = SpatialScale> {
    let ancestors = (demand.interaction.exponent()..=SPATIAL_SCALE_MAX)
        .rev()
        .map(|raw| SpatialScale::new(raw).expect("validated scale"));

    ancestors.chain(demand.refinement)
}

fn scale_is_desired(demand: ScaleStackDemandKey, scale: SpatialScale) -> bool {
    scale >= demand.interaction || demand.refinement == Some(scale)
}

pub(super) fn volume_for_scale_context(
    worldgen: &WorldgenStore,
    key: WorldgenEvaluationKey,
) -> ProceduralVolume {
    worldgen
        .node(key)
        .expect("requested scale context must already exist");
    let lineage = worldgen
        .lineage(key)
        .into_iter()
        .map(|node| (node.context().spatial_scale(), node.context().seed()))
        .collect::<Vec<_>>();

    ProceduralVolume::scale_refinement(worldgen.universe_seed(), key.scope().scale(), &lineage)
}
