//! Phenomenon rule contract and registry.

use bevy::prelude::Resource;

use super::model::{PhenomenonEvaluationContext, PhenomenonId, PhenomenonSnapshot, WorldgenNode};

/// A self-contained domain rule that can continue/refine state or introduce a
/// new Phenomenon when its spatial domain becomes meaningful.
pub trait PhenomenonRule: Send + Sync + 'static {
    fn id(&self) -> PhenomenonId;

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
        current: &[PhenomenonSnapshot],
    ) -> Option<PhenomenonSnapshot>;
}

/// Ordered only for same-scope handoff dependencies in this first implementation.
/// The states themselves remain independently typed and registered.
#[derive(Resource)]
pub struct PhenomenonRegistry {
    rules: Vec<Box<dyn PhenomenonRule>>,
}

impl Default for PhenomenonRegistry {
    fn default() -> Self {
        let mut registry = Self { rules: Vec::new() };
        super::builtin::register_builtin_rules(&mut registry);
        registry
    }
}

impl PhenomenonRegistry {
    pub fn register<R: PhenomenonRule>(&mut self, rule: R) {
        self.rules.push(Box::new(rule));
    }

    fn evaluate(
        &self,
        context: &PhenomenonEvaluationContext,
        parent: Option<&WorldgenNode>,
    ) -> Vec<PhenomenonSnapshot> {
        let mut snapshots = Vec::new();
        for rule in &self.rules {
            if let Some(snapshot) = rule.evaluate(context, parent, &snapshots) {
                debug_assert_eq!(snapshot.id(), rule.id());
                snapshots.push(snapshot);
            }
        }
        snapshots
    }
}
