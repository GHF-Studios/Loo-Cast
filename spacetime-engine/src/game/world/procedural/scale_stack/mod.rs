//! Sparse hierarchical stack of actual scale-local voxel worlds.
//!
//! Higher scales remain resident when finer scales are introduced. The result is
//! a vertical multi-scale realization spine rather than mutually exclusive worlds.
//! Voxels are the current scale-local realizer; semantic worldgen remains
//! authoritative and contextualizes every finer level.
//!
//! Materialization consumes interaction demand and presentation refinement as
//! independent inputs. A view transition never implicitly becomes physical
//! interaction authority.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    spatial::{
        SpatialScale, UsfChunkAddress, UsfPosition, UsfPrimaryInteractionSlice, UsfScaleLayer,
        UsfViewContext, UsfViewRenderAnchor,
    },
    voxel::{ProceduralVolume, VoxelBase, VoxelPresentationMaterial, VoxelStreaming, VoxelWorld},
    worldgen::{PhenomenonRegistry, WorldgenEvaluationKey, WorldgenStore},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScaleStackDemandKey {
    interaction_target: SpatialScale,
    presentation_refinement: Option<SpatialScale>,
    target_scope: UsfChunkAddress,
}

impl ScaleStackDemandKey {
    fn from_context(
        view: &UsfViewContext,
        interaction: UsfPrimaryInteractionSlice,
    ) -> Option<Self> {
        // A pending interaction target is materialization demand, not current
        // interaction ownership. Realizing it can eventually make the handoff legal.
        let interaction_target = interaction.target_scale();

        let presentation_refinement = view
            .active_scale_demands()
            .into_iter()
            .flatten()
            .filter(|demand| demand.contribution() > 0.001)
            .map(|demand| demand.scale())
            .filter(|scale| *scale < interaction_target)
            .min();

        let scope_scale = presentation_refinement.unwrap_or(interaction_target);
        let target = target_at_scale(view, scope_scale)?;
        let target_scope = UsfChunkAddress::containing(target, scope_scale).ok()?;

        Some(Self {
            interaction_target,
            presentation_refinement,
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
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut stacks: Query<(Entity, &mut ProceduralScaleStack)>,
) {
    let Some(demand) = ScaleStackDemandKey::from_context(&view, *interaction) else {
        error!("observer position could not be represented in requested scale-stack demand");
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
            let target = target_at_scale(&view, scale)
                .expect("requested scale must remain canonically representable");
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
                "extended hierarchical USF voxel realization spine"
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

fn target_at_scale(
    view: &UsfViewContext,
    target_scale: SpatialScale,
) -> Option<UsfPosition> {
    view.anchor().reexpressed_at(target_scale).ok()
}

fn desired_scales(demand: ScaleStackDemandKey) -> impl Iterator<Item = SpatialScale> {
    let ancestors = (demand.interaction_target.exponent()..=MAX_VOXEL_REALIZER_SCALE)
        .rev()
        .filter_map(SpatialScale::new);

    ancestors.chain(
        demand
            .presentation_refinement
            .filter(|scale| scale.exponent() <= MAX_VOXEL_REALIZER_SCALE),
    )
}

fn scale_is_desired(demand: ScaleStackDemandKey, scale: SpatialScale) -> bool {
    scale.exponent() <= MAX_VOXEL_REALIZER_SCALE
        && (scale >= demand.interaction_target
            || demand.presentation_refinement == Some(scale))
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
