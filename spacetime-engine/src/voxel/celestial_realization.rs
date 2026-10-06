//! Lazy celestial voxel-realization lifecycle.
//!
//! Semantic celestial authority exists independently of scale-local voxel worlds.
//! This module owns only the disposable authority+Scale -> `VoxelWorld` mapping.
//! Demand policy stays in `realization`; dense chunk residency stays in `streaming`.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    ecs::{UsfAuthorityPartitions, UsfLogicalRealizationOf},
    spatial::{UsfPosition, UsfScaleLayer, UsfSemanticFrame},
};

use super::{
    CelestialVoxelField, VoxelBase, VoxelCollisionDisabled, VoxelEditingDisabled,
    VoxelPresentationMaterial, VoxelScaleDomain, VoxelStreaming, VoxelWorld,
    realization::{VoxelRealizationIntentSnapshot, VoxelRealizationTarget},
};

#[derive(Component, Debug, Clone)]
pub struct CelestialVoxelRealizationPolicy {
    presentation_material: Handle<StandardMaterial>,
}

impl CelestialVoxelRealizationPolicy {
    pub fn new(presentation_material: Handle<StandardMaterial>) -> Self {
        Self {
            presentation_material,
        }
    }

    pub(crate) fn presentation_material(&self) -> &Handle<StandardMaterial> {
        &self.presentation_material
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::voxel) struct CelestialVoxelRealization {
    target: VoxelRealizationTarget,
}

impl CelestialVoxelRealization {
    const fn new(target: VoxelRealizationTarget) -> Self {
        Self { target }
    }

    pub(in crate::voxel) const fn target(self) -> VoxelRealizationTarget {
        self.target
    }

    pub(in crate::voxel) const fn authority(self) -> Entity {
        self.target.authority()
    }
}

/// Pose binding between a scale-local voxel world and semantic celestial frame.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub(in crate::voxel) struct CelestialVoxelFrameBinding {
    origin: UsfPosition,
    frame: UsfSemanticFrame,
    revision: u64,
}
impl CelestialVoxelFrameBinding {
    fn new(origin: UsfPosition, frame: UsfSemanticFrame) -> Self { Self { origin, frame, revision: 0 } }
    pub(in crate::voxel) const fn revision(self) -> u64 { self.revision }
    fn translated(&mut self, origin: UsfPosition) {
        if self.origin != origin { self.origin=origin; self.revision=self.revision.wrapping_add(1); }
    }
}

#[derive(Resource, Debug, Default)]
pub(in crate::voxel) struct CelestialVoxelRealizationRegistry {
    worlds: HashMap<VoxelRealizationTarget, Entity>,
}

impl CelestialVoxelRealizationRegistry {
    pub(in crate::voxel) fn world_for(
        &self,
        target: VoxelRealizationTarget,
    ) -> Option<Entity> {
        self.worlds.get(&target).copied()
    }
}

/// Reconcile scale-local realization containers against semantic authority.
///
/// Demand controls active residency inside a `VoxelWorld`; it does not own the
/// lifetime of the scale-world container itself. A temporarily undemanded
/// realization is therefore parked with zero active residency while its bounded
/// warm materialization cache remains reusable.
///
/// This is essential for make-before-break refinement: transient intent changes
/// must not erase already-generated terrain and restart expensive reconstruction.
pub(super) fn sync_celestial_voxel_realizations(
    config: Res<EngineConfig>,
    intents: Res<VoxelRealizationIntentSnapshot>,
    mut registry: ResMut<CelestialVoxelRealizationRegistry>,
    mut commands: Commands,
    authorities: Query<(
        Option<&Name>, &UsfPosition, &UsfSemanticFrame, &CelestialVoxelField,
        &VoxelScaleDomain, &CelestialVoxelRealizationPolicy, &UsfAuthorityPartitions,
    )>,
    mut existing: Query<(Entity, &CelestialVoxelRealization, &mut VoxelWorld, &mut CelestialVoxelFrameBinding)>,
) {
    let desired = intents.celestial_targets().collect::<HashSet<_>>();
    registry.worlds.clear();

    for (entity, realization, mut world, mut binding) in &mut existing {
        let target=realization.target();
        let Ok((_,body_origin,body_frame,field,domain,_,_))=authorities.get(target.authority()) else { commands.entity(entity).despawn(); continue; };
        if !domain.realizes(target.scale()) { commands.entity(entity).despawn(); continue; }

        // Translation preserves body-local dense cache identity. The present
        // canonical-axis lattice cannot preserve arbitrary rotation honestly.
        if binding.frame != *body_frame { commands.entity(entity).despawn(); continue; }
        if binding.origin != *body_origin {
            let Ok(grid_origin)=body_origin.reexpressed_at(target.scale()) else { commands.entity(entity).despawn(); continue; };
            let base=field.realization(*body_origin,*body_frame,target.scale());
            if world.reanchor_reconstructible(VoxelBase::celestial_body(base),grid_origin).is_err() { commands.entity(entity).despawn(); continue; }
            binding.translated(*body_origin);
        }

        if let Some(duplicate_of)=registry.worlds.get(&target).copied() {
            warn!(?entity,?duplicate_of,authority=?target.authority(),scale=%target.scale(),"duplicate celestial voxel realization retired");
            commands.entity(entity).despawn(); continue;
        }
        registry.worlds.insert(target,entity);
    }

    for target in desired {
        if registry.world_for(target).is_some(){continue;}
        let Ok((name,body_origin,body_frame,field,domain,policy,partitions))=authorities.get(target.authority()) else {continue;};
        if !domain.realizes(target.scale()) { warn!(authority=?target.authority(),scale=%target.scale(),"celestial voxel realization demand targets unsupported scale"); continue; }
        let mut partitions=partitions.iter();
        let Some(partition)=partitions.next() else { warn!(authority=?target.authority(),scale=%target.scale(),"celestial voxel realization has no authority partition"); continue; };
        if partitions.next().is_some(){ error!(authority=?target.authority(),scale=%target.scale(),"celestial voxel realization requires one unambiguous ordinary authority partition"); continue; }
        let Ok(grid_origin)=body_origin.reexpressed_at(target.scale()) else { error!(authority=?target.authority(),scale=%target.scale(),"celestial voxel realization origin cannot re-express at requested scale"); continue; };
        let base=field.realization(*body_origin,*body_frame,target.scale());
        let body_name=name.map(|v|v.as_str()).unwrap_or("Celestial Body");
        let mut world=commands.spawn((
            Name::new(format!("{body_name} S{} Terrain",target.scale())), UsfScaleLayer::new(target.scale()),
            UsfLogicalRealizationOf(partition), CelestialVoxelRealization::new(target),
            CelestialVoxelFrameBinding::new(*body_origin,*body_frame),
            VoxelWorld::new_at(VoxelBase::celestial_body(base),grid_origin),
            VoxelStreaming::new(config.voxel.streaming.default_load_budget_per_frame),
            VoxelPresentationMaterial::new(policy.presentation_material().clone()), Transform::IDENTITY, Visibility::Inherited,
        ));
        if !domain.collides(target.scale()){world.insert(VoxelCollisionDisabled);}
        if !domain.editable(target.scale()){world.insert(VoxelEditingDisabled);}
        registry.worlds.insert(target,world.id());
    }
}
