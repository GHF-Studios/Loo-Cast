//! Sparse phenomenon-evaluation cache and contextual refinement facility.

use std::{any::Any, collections::HashMap};

use bevy::prelude::*;

use crate::spatial::{SpatialScale, UsfChunkAddress, UsfPosition, UsfPositionError};

use super::{
    model::{
        DEFAULT_WORLDGEN_NOISE_KEY, PhenomenonEvaluationContext, PhenomenonId, TemporalScale,
        WorldgenEpoch, WorldgenEvaluation, WorldgenEvaluationKey,
    },
    phenomenon::PhenomenonRegistry,
    seed::scope_noise_key,
};

/// Sparse semantic-generation capability cache over canonical USF context
/// addresses. The parent relation comes from [`UsfChunkAddress`], the same
/// topology used by runtime context residency; this store does not define a
/// competing spatial tree. Addressability alone does not allocate semantic
/// state: only requested/refined scopes are cached here.
#[derive(Resource)]
pub struct WorldgenEvaluationCache {
    noise_key: u64,
    nodes: HashMap<WorldgenEvaluationKey, WorldgenEvaluation>,
}

impl Default for WorldgenEvaluationCache {
    fn default() -> Self {
        Self {
            noise_key: DEFAULT_WORLDGEN_NOISE_KEY,
            nodes: HashMap::new(),
        }
    }
}

impl WorldgenEvaluationCache {
    pub fn new(noise_key: u64) -> Self {
        Self {
            noise_key,
            nodes: HashMap::new(),
        }
    }

    pub const fn noise_key(&self) -> u64 {
        self.noise_key
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn key_for(
        &self,
        position: UsfPosition,
        scale: SpatialScale,
        temporal_scale: TemporalScale,
        epoch: WorldgenEpoch,
    ) -> Result<WorldgenEvaluationKey, UsfPositionError> {
        Ok(WorldgenEvaluationKey::new(
            UsfChunkAddress::containing(position, scale)?,
            temporal_scale,
            epoch.id(),
        ))
    }

    /// Bootstraps exactly one root context at Scale +35.
    /// Lower scales remain virtual until context is refined downward.
    pub fn bootstrap_root(
        &mut self,
        target: UsfPosition,
        temporal_scale: TemporalScale,
        epoch: WorldgenEpoch,
        registry: &PhenomenonRegistry,
    ) -> Result<WorldgenEvaluationKey, UsfPositionError> {
        let scope = UsfChunkAddress::containing(target, SpatialScale::MAX)?;
        let key = WorldgenEvaluationKey::new(scope, temporal_scale, epoch.id());
        if self.nodes.contains_key(&key) {
            return Ok(key);
        }

        let noise_key = scope_noise_key(self.noise_key, scope);
        let context = PhenomenonEvaluationContext::new(key, epoch, noise_key);
        let phenomena = registry.evaluate(&context, None);
        debug_assert!(
            !phenomena.is_empty(),
            "root context must be semantically interpreted"
        );
        self.nodes
            .insert(key, WorldgenEvaluation::new(context, None, phenomena));
        Ok(key)
    }

    /// Contextualizes exactly one finer child containing `target`.
    /// The parent supplies inherited conditions; child rules still specialize
    /// and contribute genuinely local emergence.
    pub fn contextualize_child(
        &mut self,
        parent: WorldgenEvaluationKey,
        target: UsfPosition,
        registry: &PhenomenonRegistry,
    ) -> Result<Option<WorldgenEvaluationKey>, UsfPositionError> {
        let Some(parent_node) = self.nodes.get(&parent) else {
            return Ok(None);
        };
        let parent_scope = parent_node.context().spatial_scope();
        let parent_scale = parent_scope.scale();
        if parent_scale == SpatialScale::MIN {
            return Ok(None);
        }

        let child_scale =
            SpatialScale::new(parent_scale.exponent() - 1).expect("non-minimum scale has child");
        let child_scope = UsfChunkAddress::containing(target, child_scale)?;
        if child_scope.parent() != Some(parent_scope) {
            return Ok(None);
        }

        let key = WorldgenEvaluationKey::new(child_scope, parent.temporal_scale(), parent.epoch());
        if self.nodes.contains_key(&key) {
            return Ok(Some(key));
        }

        let epoch = parent_node.context().epoch();
        let noise_key = scope_noise_key(self.noise_key, child_scope);
        let context = PhenomenonEvaluationContext::new(key, epoch, noise_key);
        let phenomena = registry.evaluate(&context, Some(parent_node));
        debug_assert!(
            !phenomena.is_empty(),
            "every currently modeled +35 -> 0 scale should be semantically interpreted"
        );
        self.nodes.insert(
            key,
            WorldgenEvaluation::new(context, Some(parent), phenomena),
        );
        Ok(Some(key))
    }

    /// Refines an already bootstrapped root downward until `target_scale`.
    pub fn contextualize_to(
        &mut self,
        root: WorldgenEvaluationKey,
        target: UsfPosition,
        target_scale: SpatialScale,
        registry: &PhenomenonRegistry,
    ) -> Result<Option<WorldgenEvaluationKey>, UsfPositionError> {
        let mut current = root;
        while current.scope().scale() > target_scale {
            let Some(child) = self.contextualize_child(current, target, registry)? else {
                return Ok(None);
            };
            current = child;
        }
        Ok((current.scope().scale() == target_scale).then_some(current))
    }

    /// Evaluate one sparse canonical branch down to the requested Scale Slice.
    ///
    /// This is phenomenon evaluation only. Semantic construction must consume
    /// typed results through a separate construction boundary.
    pub fn evaluate_branch(
        &mut self,
        target: UsfPosition,
        target_scale: SpatialScale,
        temporal_scale: TemporalScale,
        epoch: WorldgenEpoch,
        registry: &PhenomenonRegistry,
    ) -> Result<WorldgenEvaluationKey, UsfPositionError> {
        let root = self.bootstrap_root(target, temporal_scale, epoch, registry)?;
        Ok(self
            .contextualize_to(root, target, target_scale, registry)?
            .expect("root and target define one canonical refinement branch"))
    }

    pub fn node(&self, key: WorldgenEvaluationKey) -> Option<&WorldgenEvaluation> {
        self.nodes.get(&key)
    }

    pub fn state<T: Any>(&self, key: WorldgenEvaluationKey, id: PhenomenonId) -> Option<&T> {
        self.node(key)?.state::<T>(id)
    }

    /// Returns leaf -> root ancestry for one generated evaluation.
    pub fn lineage(&self, mut key: WorldgenEvaluationKey) -> Vec<&WorldgenEvaluation> {
        let mut lineage = Vec::new();
        while let Some(node) = self.nodes.get(&key) {
            lineage.push(node);
            let Some(parent) = node.parent() else {
                break;
            };
            key = parent;
        }
        lineage
    }
}

pub struct WorldgenEvaluationPlugin;

impl Plugin for WorldgenEvaluationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhenomenonRegistry>()
            .init_resource::<WorldgenEvaluationCache>();
    }
}
