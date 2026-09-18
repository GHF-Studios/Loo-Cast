//! Sparse world-generation cache and refinement orchestration.

use std::{any::Any, collections::HashMap};

use bevy::prelude::*;

use crate::spatial::{SpatialScale, UsfChunkAddress, UsfPosition, UsfPositionError};

use super::{
    model::{
        DEFAULT_UNIVERSE_SEED, PhenomenonEvaluationContext, PhenomenonId, TemporalScale,
        WorldgenEpoch, WorldgenEvaluationKey, WorldgenNode,
    },
    phenomenon::PhenomenonRegistry,
    seed::scope_seed,
};

/// Sparse semantic generation cache. Addressability of the USF hierarchy does
/// not imply entries here: only requested/refined scopes are allocated.
#[derive(Resource)]
pub struct WorldgenStore {
    universe_seed: u64,
    nodes: HashMap<WorldgenEvaluationKey, WorldgenNode>,
}

impl Default for WorldgenStore {
    fn default() -> Self {
        Self {
            universe_seed: DEFAULT_UNIVERSE_SEED,
            nodes: HashMap::new(),
        }
    }
}

impl WorldgenStore {
    pub fn new(universe_seed: u64) -> Self {
        Self {
            universe_seed,
            nodes: HashMap::new(),
        }
    }

    pub const fn universe_seed(&self) -> u64 {
        self.universe_seed
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
        Ok(WorldgenEvaluationKey {
            scope: UsfChunkAddress::containing(position, scale)?,
            temporal_scale,
            epoch: epoch.id(),
        })
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
        let key = WorldgenEvaluationKey {
            scope,
            temporal_scale,
            epoch: epoch.id(),
        };
        if self.nodes.contains_key(&key) {
            return Ok(key);
        }

        let seed = scope_seed(self.universe_seed, scope);
        let context = PhenomenonEvaluationContext { key, epoch, seed };
        let phenomena = registry.evaluate(&context, None);
        debug_assert!(
            !phenomena.is_empty(),
            "root context must be semantically interpreted"
        );
        self.nodes.insert(
            key,
            WorldgenNode {
                context,
                parent: None,
                phenomena,
            },
        );
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

        let key = WorldgenEvaluationKey {
            scope: child_scope,
            temporal_scale: parent.temporal_scale(),
            epoch: parent.epoch(),
        };
        if self.nodes.contains_key(&key) {
            return Ok(Some(key));
        }

        let epoch = parent_node.context().epoch();
        let seed = scope_seed(self.universe_seed, child_scope);
        let context = PhenomenonEvaluationContext { key, epoch, seed };
        let phenomena = registry.evaluate(&context, Some(parent_node));
        debug_assert!(
            !phenomena.is_empty(),
            "every currently modeled +35 -> 0 scale should be semantically interpreted"
        );
        self.nodes.insert(
            key,
            WorldgenNode {
                context,
                parent: Some(parent),
                phenomena,
            },
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

    /// Compatibility convenience implemented in terms of coarse bootstrap and
    /// explicit downward contextual refinement.
    pub fn ensure_branch(
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

    pub fn node(&self, key: WorldgenEvaluationKey) -> Option<&WorldgenNode> {
        self.nodes.get(&key)
    }

    pub fn state<T: Any>(&self, key: WorldgenEvaluationKey, id: PhenomenonId) -> Option<&T> {
        self.node(key)?.state::<T>(id)
    }

    /// Returns leaf -> root ancestry for one generated evaluation.
    pub fn lineage(&self, mut key: WorldgenEvaluationKey) -> Vec<&WorldgenNode> {
        let mut lineage = Vec::new();
        while let Some(node) = self.nodes.get(&key) {
            lineage.push(node);
            let Some(parent) = node.parent else {
                break;
            };
            key = parent;
        }
        lineage
    }
}

pub struct WorldGenerationPlugin;

impl Plugin for WorldGenerationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PhenomenonRegistry>()
            .init_resource::<WorldgenStore>();
    }
}
