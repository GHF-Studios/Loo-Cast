//! Sparse hierarchical stack of actual scale-local voxel worlds.
//!
//! Higher scales remain resident when finer scales are introduced. The result is
//! a vertical multi-scale realization spine rather than mutually exclusive worlds.
//! Voxels are the current scale-local realizer; semantic worldgen remains
//! authoritative and contextualizes every finer level.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    spatial::{
        SPATIAL_SCALE_MAX, SpatialScale, UsfActiveScaleLayer, UsfChunkAddress, UsfPosition,
        UsfScaleLayer, UsfScaleLayerFrames, UsfViewFrame,
    },
    voxel::{ProceduralVolume, VoxelBase, VoxelPresentationMaterial, VoxelStreaming, VoxelWorld},
    worldgen::{PhenomenonRegistry, WorldgenEvaluationKey, WorldgenStore},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ScaleStackDemandKey {
    interaction: SpatialScale,
    refinement: Option<SpatialScale>,
    target_scope: UsfChunkAddress,
}

impl ScaleStackDemandKey {
    fn from_view(
        view: &UsfViewFrame,
        active: UsfActiveScaleLayer,
        frames: &UsfScaleLayerFrames,
    ) -> Option<Self> {
        let interaction = view.interaction_scale();
        let refinement = (view.scale() < interaction && view.contribution(view.scale()) > 0.001)
            .then_some(view.scale());
        let target = view_target_at_scale(view, active, frames, view.scale())?;
        let target_scope = UsfChunkAddress::containing(target, view.scale()).ok()?;

        Some(Self {
            interaction,
            refinement,
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
        root_material: Handle<StandardMaterial>,
        root_world: Entity,
    ) -> Self {
        let mut active = HashMap::new();
        active.insert(SpatialScale::MAX, root_world);
        let mut scale_materials = HashMap::new();
        scale_materials.insert(SpatialScale::MAX, root_material);

        Self {
            root,
            base_material,
            scale_materials,
            active,
            last_demand: None,
        }
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
    view: Res<UsfViewFrame>,
    active_layer: Res<UsfActiveScaleLayer>,
    frames: Res<UsfScaleLayerFrames>,
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut stacks: Query<(Entity, &mut ProceduralScaleStack)>,
) {
    let Some(demand) = ScaleStackDemandKey::from_view(&view, *active_layer, &frames) else {
        error!("observer position could not be represented in the requested refinement scale");
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
            let target = view_target_at_scale(&view, *active_layer, &frames, scale)
                .expect("requested scale must remain representable near the observer");
            let key = worldgen
                .contextualize_to(stack.root, target, scale, &registry)
                .expect("scale-local refinement must remain canonically addressable")
                .expect("root context must refine to requested scale");

            // Refresh semantic refinement even when this scale-local realizer
            // already exists. A voxel realizer spans many semantic scopes and
            // must not be owned by one child's context identity.
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

const MULTISCALE_DEPTH_BIAS_PER_DECADE: f32 = 32.0;

fn scale_depth_bias(scale: SpatialScale, view_scale: SpatialScale) -> f32 {
    let coarser_decades =
        (scale.exponent() as i16 - view_scale.exponent() as i16).max(0) as f32;
    -coarser_decades * MULTISCALE_DEPTH_BIAS_PER_DECADE
}

/// Re-expresses the bounded observer position in any requested scale chart.
///
/// During a +35 -> +34 transition, +34 worldgen therefore already receives the
/// actual +34 region the observer is approaching even though +35 still owns
/// physical interaction.
fn view_target_at_scale(
    view: &UsfViewFrame,
    active: UsfActiveScaleLayer,
    frames: &UsfScaleLayerFrames,
    target_scale: SpatialScale,
) -> Option<UsfPosition> {
    let active_scale = active.scale();
    let active_absolute = frames.absolute(active_scale, view.runtime_anchor());
    let target_absolute = frames.convert_absolute(active_absolute, active_scale, target_scale);
    let target_local = Vec3::new(
        target_absolute.x as f32,
        target_absolute.y as f32,
        target_absolute.z as f32,
    );
    if !target_local.is_finite() {
        return None;
    }

    UsfPosition::zero(target_scale)
        .translated_native(target_local)
        .ok()
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
