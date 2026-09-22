//! Sparse hierarchical stack of actual scale-local voxel worlds.
//!
//! Which Scale Slices exist as voxel realizers is driven by explicit spatial
//! demand plus independent presentation demand. Interaction focus is merely one
//! demand source; camera state never becomes physical authority.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    spatial::{
        SpatialDemandSnapshot, SpatialScale, UsfChunkAddress, UsfPosition,
        UsfPrimaryInteractionSlice, UsfScaleLayer, UsfViewContext, UsfViewRenderAnchor,
    },
    voxel::{ProceduralVolume, VoxelBase, VoxelPresentationMaterial, VoxelStreaming, VoxelWorld},
    worldgen::{PhenomenonRegistry, WorldgenEvaluationKey, WorldgenStore},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScaleStackDemandKey {
    finest_requested: SpatialScale,
    target_scope: UsfChunkAddress,
}

impl ScaleStackDemandKey {
    fn from_context(
        view: &UsfViewContext,
        interaction: UsfPrimaryInteractionSlice,
        spatial_demand: &SpatialDemandSnapshot,
    ) -> Option<Self> {
        let mut finest_requested = interaction.target_scale();
        let mut target = *view.anchor();

        for scope in spatial_demand.iter() {
            if scope.scale() < finest_requested {
                finest_requested = scope.scale();
                target = scope.center();
            }
        }

        for demand in view.active_scale_demands().into_iter().flatten() {
            if demand.contribution() > 0.001 && demand.scale() < finest_requested {
                finest_requested = demand.scale();
                target = *view.anchor();
            }
        }

        let target_scope =
            UsfChunkAddress::containing(target, finest_requested).ok()?;

        Some(Self {
            finest_requested,
            target_scope,
        })
    }
}

#[derive(Component)]
pub(super) struct ProceduralScaleStack {
    root: WorldgenEvaluationKey,
    base_material: Handle<StandardMaterial>,
    scale_materials: HashMap<SpatialScale, Handle<StandardMaterial>>,
    active: HashMap<SpatialScale, Entity>,
    last_demand: Option<ScaleStackDemandKey>,
}

impl ProceduralScaleStack {
    pub(super) fn new(
        root: WorldgenEvaluationKey,
        base_material: Handle<StandardMaterial>,
    ) -> Self {
        Self {
            root,
            base_material,
            scale_materials: HashMap::new(),
            active: HashMap::new(),
            last_demand: None,
        }
    }

    pub(super) const fn root(&self) -> WorldgenEvaluationKey {
        self.root
    }

    fn material_for_scale(
        &mut self,
        scale: SpatialScale,
        view_scale: SpatialScale,
        materials: &mut Assets<StandardMaterial>,
    ) -> Option<Handle<StandardMaterial>> {
        if let Some(handle) = self.scale_materials.get(&scale) {
            return Some(handle.clone());
        }

        let mut material = materials.get(&self.base_material)?.clone();
        material.depth_bias = scale_depth_bias(scale, view_scale);
        let handle = materials.add(material);
        self.scale_materials.insert(scale, handle.clone());
        Some(handle)
    }

    fn update_material_depth_biases(
        &self,
        view_scale: SpatialScale,
        materials: &mut Assets<StandardMaterial>,
    ) {
        for (&scale, handle) in &self.scale_materials {
            if let Some(mut material) = materials.get_mut(handle) {
                material.depth_bias = scale_depth_bias(scale, view_scale);
            }
        }
    }
}

pub(super) fn sync_scale_stack(
    config: Res<EngineConfig>,
    mut commands: Commands,
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    spatial_demand: Res<SpatialDemandSnapshot>,
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut stacks: Query<(Entity, &mut ProceduralScaleStack)>,
) {
    let Some(demand) =
        ScaleStackDemandKey::from_context(&view, *interaction, &spatial_demand)
    else {
        error!("USF scale-stack demand could not be resolved");
        return;
    };

    for (stack_entity, mut stack) in &mut stacks {
        if stack.last_demand == Some(demand) {
            continue;
        }

        stack.update_material_depth_biases(view.scale(), &mut materials);

        stack.active.retain(|scale, entity| {
            if scale_is_desired(demand, *scale) {
                true
            } else {
                commands.entity(*entity).despawn();
                false
            }
        });

        for scale in desired_scales(demand) {
            let target = view
                .anchor()
                .reexpressed_at(scale)
                .expect("canonical observer must remain representable at demanded scale");
            let key = worldgen
                .contextualize_to(stack.root, target, scale, &registry)
                .expect("scale-local refinement must remain canonically addressable")
                .expect("root context must refine to requested scale");

            if stack.active.contains_key(&scale) {
                continue;
            }

            let volume = volume_for_scale_context(&worldgen, key);
            let material = stack
                .material_for_scale(scale, view.scale(), &mut materials)
                .expect("procedural voxel base material must remain available");

            let entity = commands
                .spawn((
                    Name::new(format!("USF Scale {scale} Voxel World")),
                    ChildOf(stack_entity),
                    UsfScaleLayer::new(scale),
                    VoxelWorld::new_at(VoxelBase::Volume(volume), UsfPosition::zero(scale)),
                    VoxelStreaming::new(config.voxel.streaming.default_load_budget_per_frame),
                    VoxelPresentationMaterial::new(material),
                    Transform::IDENTITY,
                    Visibility::Inherited,
                ))
                .id();

            stack.active.insert(scale, entity);
            debug!(
                scale = %scale,
                resident_scale_worlds = stack.active.len(),
                "extended demand-driven hierarchical USF voxel realization spine"
            );
        }

        stack.last_demand = Some(demand);
    }
}

const MAX_VOXEL_REALIZER_SCALE: i8 = 4;
const MULTISCALE_DEPTH_BIAS_PER_DECADE: f32 = 32.0;

fn scale_depth_bias(scale: SpatialScale, view_scale: SpatialScale) -> f32 {
    let coarser_decades =
        (scale.exponent() as i16 - view_scale.exponent() as i16).max(0) as f32;
    -coarser_decades * MULTISCALE_DEPTH_BIAS_PER_DECADE
}

fn desired_scales(demand: ScaleStackDemandKey) -> impl Iterator<Item = SpatialScale> {
    (demand.finest_requested.exponent()..=MAX_VOXEL_REALIZER_SCALE)
        .rev()
        .filter_map(SpatialScale::new)
}

fn scale_is_desired(demand: ScaleStackDemandKey, scale: SpatialScale) -> bool {
    scale.exponent() <= MAX_VOXEL_REALIZER_SCALE
        && scale >= demand.finest_requested
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
