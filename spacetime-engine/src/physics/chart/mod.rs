//! USF simulation-chart integration for the physics backend.
//!
//! `UsfScaleLayer` is exact Scale Slice partition identity. `UsfChartMask`
//! selects one or more of the 71 partitions; bounded chart coordinates are
//! derived separately from canonical USF chart algebra. Avian `CollisionLayers`
//! remain a within-slice interaction-category mechanism.

use avian3d::prelude::{
    ActiveCollisionHooks, Collider, ColliderOf, RigidBody, SpatialQueryFilter,
};
use bevy::{ecs::system::SystemParam, prelude::*};

use crate::spatial::{SpatialScale, UsfChartMask, UsfScaleLayer};

#[derive(SystemParam)]
pub struct UsfPhysicsCharts<'w, 's> {
    colliders: Query<
        'w,
        's,
        (
            Entity,
            Option<&'static UsfScaleLayer>,
            Option<&'static ColliderOf>,
        ),
        With<Collider>,
    >,
    bodies: Query<'w, 's, &'static UsfScaleLayer, With<RigidBody>>,
}

impl UsfPhysicsCharts<'_, '_> {
    pub fn collider_scale(&self, collider: Entity) -> Option<SpatialScale> {
        let Ok((_, direct, attached)) = self.colliders.get(collider) else {
            return None;
        };
        direct
            .copied()
            .or_else(|| attached.and_then(|a| self.bodies.get(a.body).ok().copied()))
            .map(UsfScaleLayer::scale)
    }

    pub fn filter(
        &self,
        charts: UsfChartMask,
        excluded: impl IntoIterator<Item = Entity>,
    ) -> SpatialQueryFilter {
        let cross_chart = self.colliders.iter().filter_map(|(entity, direct, attached)| {
            let layer = direct
                .copied()
                .or_else(|| attached.and_then(|a| self.bodies.get(a.body).ok().copied()))?;
            (!charts.contains(layer.scale())).then_some(entity)
        });

        SpatialQueryFilter::from_excluded_entities(excluded.into_iter().chain(cross_chart))
    }

    pub fn filter_for_scale(
        &self,
        scale: SpatialScale,
        excluded: impl IntoIterator<Item = Entity>,
    ) -> SpatialQueryFilter {
        self.filter(UsfChartMask::from_scale(scale), excluded)
    }
}

pub(crate) fn prepare_usf_physics_charts(
    mut commands: Commands,
    bodies: Query<&UsfScaleLayer, With<RigidBody>>,
    colliders: Query<
        (
            Entity,
            Option<&UsfScaleLayer>,
            Option<&ColliderOf>,
            Option<&ActiveCollisionHooks>,
        ),
        With<Collider>,
    >,
) {
    for (entity, direct, attached, hooks) in &colliders {
        let resolved = direct
            .copied()
            .or_else(|| attached.and_then(|a| bodies.get(a.body).ok().copied()));
        let Some(layer) = resolved else {
            continue;
        };

        let mut entity_commands = commands.entity(entity);
        if direct.is_none() {
            entity_commands.insert(layer);
        }

        let hooks = hooks.copied().unwrap_or_default();
        if !hooks.contains(ActiveCollisionHooks::FILTER_PAIRS) {
            entity_commands.insert(hooks | ActiveCollisionHooks::FILTER_PAIRS);
        }
    }
}
