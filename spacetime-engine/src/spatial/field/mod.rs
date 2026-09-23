//! Generic hierarchical field representation primitives.
//!
//! A field's semantic source type is domain-specific (gravity, radiation,
//! temperature, etc.). The spatial representation mechanism is not. This module
//! supplies the shared ancestor-context partition used by exact field backends
//! today and approximate/multipole/grid backends later.

use std::collections::HashMap;

use bevy::prelude::Entity;

use super::{UsfChunkAddress, UsfContextTopology, UsfPosition};

pub(crate) trait UsfFieldSourceLocation {
    fn field_position(&self) -> UsfPosition;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct UsfFieldSource<S> {
    entity: Entity,
    source: S,
}

impl<S> UsfFieldSource<S> {
    pub(crate) const fn new(entity: Entity, source: S) -> Self {
        Self { entity, source }
    }

    pub(crate) const fn entity(&self) -> Entity {
        self.entity
    }

}

impl<S: Copy> UsfFieldSource<S> {
    pub(crate) const fn copied_source(self) -> S {
        self.source
    }
}

#[derive(Debug, Clone)]
pub(crate) struct UsfFieldContext<S> {
    inherited_residual: Vec<UsfFieldSource<S>>,
    refinement_sources: Vec<UsfFieldSource<S>>,
}

impl<S> Default for UsfFieldContext<S> {
    fn default() -> Self {
        Self {
            inherited_residual: Vec::new(),
            refinement_sources: Vec::new(),
        }
    }
}

impl<S> UsfFieldContext<S> {
    pub(crate) fn inherited_residual(&self) -> &[UsfFieldSource<S>] {
        &self.inherited_residual
    }

    pub(crate) fn refinement_sources(&self) -> &[UsfFieldSource<S>] {
        &self.refinement_sources
    }
}

#[derive(Debug)]
pub(crate) struct UsfHierarchicalFieldCache<S> {
    revision: u64,
    topology_revision: u64,
    global_sources: Vec<UsfFieldSource<S>>,
    contexts: HashMap<UsfChunkAddress, UsfFieldContext<S>>,
}

impl<S> Default for UsfHierarchicalFieldCache<S> {
    fn default() -> Self {
        Self {
            revision: 0,
            topology_revision: 0,
            global_sources: Vec::new(),
            contexts: HashMap::new(),
        }
    }
}

impl<S> UsfHierarchicalFieldCache<S>
where
    S: Copy + PartialEq + UsfFieldSourceLocation,
{
    pub(crate) const fn revision(&self) -> u64 {
        self.revision
    }

    pub(crate) const fn topology_revision(&self) -> u64 {
        self.topology_revision
    }

    pub(crate) fn global_sources(&self) -> &[UsfFieldSource<S>] {
        &self.global_sources
    }

    pub(crate) fn context(&self, scope: UsfChunkAddress) -> Option<&UsfFieldContext<S>> {
        self.contexts.get(&scope)
    }

    pub(crate) fn rebuild(
        &mut self,
        topology: &UsfContextTopology,
        mut sources: Vec<UsfFieldSource<S>>,
    ) {
        sources.sort_by_key(|source| source.entity().to_bits());

        self.contexts.clear();
        self.global_sources = sources;
        self.topology_revision = topology.revision();

        let mut scopes = topology.iter().map(|node| node.scope()).collect::<Vec<_>>();
        scopes.sort_by(|a, b| b.scale().cmp(&a.scale()));

        for scope in scopes {
            let Some(node) = topology.node(scope) else {
                continue;
            };

            let (inherited_residual, refinement_sources) = if let Some(parent) = node.parent() {
                let Some(parent_cache) = self.contexts.get(&parent) else {
                    continue;
                };
                partition_for_child(parent_cache.refinement_sources(), scope)
            } else {
                (Vec::new(), self.global_sources.clone())
            };

            self.contexts.insert(
                scope,
                UsfFieldContext {
                    inherited_residual,
                    refinement_sources,
                },
            );
        }

        self.revision = self.revision.wrapping_add(1).max(1);
    }
}

fn partition_for_child<S>(
    parent_sources: &[UsfFieldSource<S>],
    child: UsfChunkAddress,
) -> (Vec<UsfFieldSource<S>>, Vec<UsfFieldSource<S>>)
where
    S: Copy + UsfFieldSourceLocation,
{
    let mut residual = Vec::new();
    let mut refinement = Vec::new();

    for source in parent_sources.iter().copied() {
        let inside_child =
            UsfChunkAddress::containing(source.copied_source().field_position(), child.scale())
                .is_ok_and(|scope| scope == child);

        if inside_child {
            refinement.push(source);
        } else {
            residual.push(source);
        }
    }

    (residual, refinement)
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum UsfFieldSampleQuality {
    ExactHierarchical,
    #[default]
    ExactGlobalFallback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UsfFieldSampleMetadata {
    deepest_context: Option<UsfChunkAddress>,
    context_depth: u8,
    representation_revision: u64,
    quality: UsfFieldSampleQuality,
}

impl Default for UsfFieldSampleMetadata {
    fn default() -> Self {
        Self::exact_global_fallback(0)
    }
}

impl UsfFieldSampleMetadata {
    pub(crate) const fn exact_global_fallback(representation_revision: u64) -> Self {
        Self {
            deepest_context: None,
            context_depth: 0,
            representation_revision,
            quality: UsfFieldSampleQuality::ExactGlobalFallback,
        }
    }

    pub(crate) const fn exact_hierarchical(
        deepest_context: UsfChunkAddress,
        context_depth: usize,
        representation_revision: u64,
    ) -> Self {
        Self {
            deepest_context: Some(deepest_context),
            context_depth: if context_depth > u8::MAX as usize {
                u8::MAX
            } else {
                context_depth as u8
            },
            representation_revision,
            quality: UsfFieldSampleQuality::ExactHierarchical,
        }
    }

    pub const fn deepest_context(self) -> Option<UsfChunkAddress> {
        self.deepest_context
    }

    pub const fn context_depth(self) -> u8 {
        self.context_depth
    }

    pub const fn representation_revision(self) -> u64 {
        self.representation_revision
    }

    pub const fn quality(self) -> UsfFieldSampleQuality {
        self.quality
    }
}
