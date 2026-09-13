//! Runtime control graph backing developer tools and their nested configuration.
//!
//! The graph models user intent, not rendering implementation. Groups organize tools,
//! toggles enable semantic features, choices are inherently mutually exclusive, and
//! explicit requirements/conflicts cover the rarer cross-tool relationship.

use std::collections::{HashMap, HashSet};

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DebugId(pub &'static str);

impl std::fmt::Display for DebugId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DebugChoiceOption {
    pub value: &'static str,
    pub label: &'static str,
}

impl DebugChoiceOption {
    pub const fn new(value: &'static str, label: &'static str) -> Self {
        Self { value, label }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum DebugCondition {
    /// The referenced control must currently be selected/effective.
    Selected(DebugId),
    /// The referenced choice must currently have the requested value.
    ChoiceEquals(DebugId, &'static str),
}

#[derive(Debug, Clone)]
pub enum DebugControlKind {
    Group,
    /// Equipable semantic developer tool. Tools may own nested settings.
    Tool { equipped: bool },
    Toggle { value: bool },
    Choice {
        selected: usize,
        options: Vec<DebugChoiceOption>,
    },
    Scalar {
        value: f32,
        minimum: f32,
        maximum: f32,
        step: f32,
        unit: &'static str,
    },
    Integer {
        value: u32,
        minimum: u32,
        maximum: u32,
        step: u32,
        unit: &'static str,
    },
}

#[derive(Debug, Clone)]
pub struct DebugControlSpec {
    pub id: DebugId,
    pub parent: Option<DebugId>,
    pub label: &'static str,
    pub description: &'static str,
    pub order: i32,
    pub kind: DebugControlKind,
    pub requires: Vec<DebugId>,
    pub conflicts: Vec<DebugId>,
    /// Applicability conditions gate effective state and menu visibility while
    /// preserving the underlying value. This is how configuration remains
    /// stateful without presenting nonsense controls (for example a height
    /// scale while a scalar field is in flat-heatmap mode).
    pub conditions: Vec<DebugCondition>,
}

impl DebugControlSpec {
    pub fn group(id: DebugId, parent: Option<DebugId>, label: &'static str, order: i32) -> Self {
        Self::new(id, parent, label, order, DebugControlKind::Group)
    }

    pub fn tool(
        id: DebugId,
        parent: Option<DebugId>,
        label: &'static str,
        order: i32,
        equipped: bool,
    ) -> Self {
        Self::new(
            id,
            parent,
            label,
            order,
            DebugControlKind::Tool { equipped },
        )
    }

    pub fn toggle(
        id: DebugId,
        parent: Option<DebugId>,
        label: &'static str,
        order: i32,
        enabled: bool,
    ) -> Self {
        Self::new(
            id,
            parent,
            label,
            order,
            DebugControlKind::Toggle { value: enabled },
        )
    }

    pub fn choice(
        id: DebugId,
        parent: Option<DebugId>,
        label: &'static str,
        order: i32,
        options: impl IntoIterator<Item = DebugChoiceOption>,
        selected: usize,
    ) -> Self {
        let options = options.into_iter().collect::<Vec<_>>();
        assert!(!options.is_empty(), "debug choice {id} has no options");
        assert!(
            selected < options.len(),
            "debug choice {id} default index {selected} is out of bounds"
        );
        Self::new(
            id,
            parent,
            label,
            order,
            DebugControlKind::Choice { selected, options },
        )
    }

    pub fn scalar(
        id: DebugId,
        parent: Option<DebugId>,
        label: &'static str,
        order: i32,
        value: f32,
        minimum: f32,
        maximum: f32,
        step: f32,
        unit: &'static str,
    ) -> Self {
        assert!(minimum <= maximum);
        assert!(step > 0.0);
        Self::new(
            id,
            parent,
            label,
            order,
            DebugControlKind::Scalar {
                value: value.clamp(minimum, maximum),
                minimum,
                maximum,
                step,
                unit,
            },
        )
    }

    pub fn integer(
        id: DebugId,
        parent: Option<DebugId>,
        label: &'static str,
        order: i32,
        value: u32,
        minimum: u32,
        maximum: u32,
        step: u32,
        unit: &'static str,
    ) -> Self {
        assert!(minimum <= maximum);
        assert!(step > 0);
        Self::new(
            id,
            parent,
            label,
            order,
            DebugControlKind::Integer {
                value: value.clamp(minimum, maximum),
                minimum,
                maximum,
                step,
                unit,
            },
        )
    }

    fn new(
        id: DebugId,
        parent: Option<DebugId>,
        label: &'static str,
        order: i32,
        kind: DebugControlKind,
    ) -> Self {
        Self {
            id,
            parent,
            label,
            description: "",
            order,
            kind,
            requires: Vec::new(),
            conflicts: Vec::new(),
            conditions: Vec::new(),
        }
    }

    pub fn described(mut self, description: &'static str) -> Self {
        self.description = description;
        self
    }

    pub fn requires(mut self, requirement: DebugId) -> Self {
        self.requires.push(requirement);
        self
    }

    pub fn conflicts_with(mut self, conflict: DebugId) -> Self {
        self.conflicts.push(conflict);
        self
    }

    pub fn when_selected(mut self, control: DebugId) -> Self {
        self.conditions.push(DebugCondition::Selected(control));
        self
    }

    pub fn when_choice(mut self, choice: DebugId, value: &'static str) -> Self {
        self.conditions.push(DebugCondition::ChoiceEquals(choice, value));
        self
    }
}

#[derive(Resource, Debug)]
pub struct DebugControls {
    master_enabled: bool,
    nodes: Vec<DebugControlSpec>,
    index: HashMap<DebugId, usize>,
    generation: u64,
    revision: u64,
}

impl Default for DebugControls {
    fn default() -> Self {
        Self {
            master_enabled: true,
            nodes: Vec::new(),
            index: HashMap::new(),
            generation: 0,
            revision: 0,
        }
    }
}

impl DebugControls {
    pub fn register(&mut self, spec: DebugControlSpec) {
        assert!(
            !self.index.contains_key(&spec.id),
            "duplicate debug control id {}",
            spec.id
        );
        let index = self.nodes.len();
        self.index.insert(spec.id, index);
        self.nodes.push(spec);
        self.generation = self.generation.wrapping_add(1);
    }

    pub fn validate(&self) {
        for node in &self.nodes {
            if let Some(parent) = node.parent {
                assert!(
                    self.index.contains_key(&parent),
                    "debug control {} references missing parent {}",
                    node.id,
                    parent
                );
            }
            for requirement in &node.requires {
                let Some(required) = self.node(*requirement) else {
                    panic!(
                        "debug control {} requires missing control {}",
                        node.id, requirement
                    );
                };
                assert_ne!(node.id, *requirement, "debug control cannot require itself");
                assert!(
                    matches!(&required.kind, DebugControlKind::Tool { .. } | DebugControlKind::Toggle { .. }),
                    "debug control {} requires non-toggle control {}",
                    node.id,
                    requirement
                );
            }
            for conflict in &node.conflicts {
                let Some(conflicting) = self.node(*conflict) else {
                    panic!(
                        "debug control {} conflicts with missing control {}",
                        node.id, conflict
                    );
                };
                assert_ne!(node.id, *conflict, "debug control cannot conflict with itself");
                assert!(
                    matches!(&conflicting.kind, DebugControlKind::Tool { .. } | DebugControlKind::Toggle { .. }),
                    "debug control {} conflicts with non-toggle control {}",
                    node.id,
                    conflict
                );
            }
            if self.raw_toggle_value(node.id) == Some(true) {
                for conflict in &node.conflicts {
                    assert_ne!(
                        self.raw_toggle_value(*conflict),
                        Some(true),
                        "debug controls {} and {} conflict but are both enabled by default",
                        node.id,
                        conflict
                    );
                }
            }
            for condition in &node.conditions {
                match *condition {
                    DebugCondition::Selected(control) => {
                        assert!(
                            self.index.contains_key(&control),
                            "debug control {} conditions on missing control {}",
                            node.id,
                            control
                        );
                        assert_ne!(node.id, control, "debug control cannot condition on itself");
                    }
                    DebugCondition::ChoiceEquals(choice, value) => {
                        let Some(choice_node) = self.node(choice) else {
                            panic!(
                                "debug control {} conditions on missing choice {}",
                                node.id, choice
                            );
                        };
                        let DebugControlKind::Choice { options, .. } = &choice_node.kind else {
                            panic!(
                                "debug control {} conditions on non-choice control {}",
                                node.id, choice
                            );
                        };
                        assert!(
                            options.iter().any(|option| option.value == value),
                            "debug control {} conditions on unknown value {value:?} of choice {}",
                            node.id,
                            choice
                        );
                    }
                }
            }
        }

        for node in &self.nodes {
            let mut visiting = HashSet::new();
            self.assert_effective_dependencies_acyclic(node.id, &mut visiting);

            if matches!(
                &node.kind,
                DebugControlKind::Tool { .. } | DebugControlKind::Toggle { .. }
            ) {
                let mut simultaneously_required = HashSet::new();
                simultaneously_required.insert(node.id);
                self.collect_required_toggles(node.id, &mut simultaneously_required);

                for first in &simultaneously_required {
                    for second in &simultaneously_required {
                        if first >= second {
                            continue;
                        }
                        let first_conflicts = self
                            .node(*first)
                            .is_some_and(|candidate| candidate.conflicts.contains(second));
                        let second_conflicts = self
                            .node(*second)
                            .is_some_and(|candidate| candidate.conflicts.contains(first));
                        assert!(
                            !first_conflicts && !second_conflicts,
                            "debug control {} has an impossible requirement set: {} conflicts with {}",
                            node.id,
                            first,
                            second
                        );
                    }
                }
            }
        }
    }

    fn collect_required_toggles(&self, id: DebugId, output: &mut HashSet<DebugId>) {
        let Some(node) = self.node(id) else {
            return;
        };

        for requirement in &node.requires {
            if output.insert(*requirement) {
                self.collect_required_toggles(*requirement, output);
            }
            self.collect_toggle_ancestors(*requirement, output);
        }
    }

    fn collect_toggle_ancestors(&self, id: DebugId, output: &mut HashSet<DebugId>) {
        let mut parent = self.node(id).and_then(|node| node.parent);
        while let Some(parent_id) = parent {
            let Some(parent_node) = self.node(parent_id) else {
                break;
            };
            if matches!(
                &parent_node.kind,
                DebugControlKind::Tool { .. } | DebugControlKind::Toggle { .. }
            ) && output.insert(parent_id)
            {
                self.collect_required_toggles(parent_id, output);
            }
            parent = parent_node.parent;
        }
    }

    fn assert_effective_dependencies_acyclic(
        &self,
        id: DebugId,
        visiting: &mut HashSet<DebugId>,
    ) {
        assert!(
            visiting.insert(id),
            "debug control effective-state dependency cycle contains {id}"
        );

        if let Some(node) = self.node(id) {
            if let Some(parent) = node.parent {
                self.assert_effective_dependencies_acyclic(parent, visiting);
            }
            for requirement in &node.requires {
                self.assert_effective_dependencies_acyclic(*requirement, visiting);
            }
            for condition in &node.conditions {
                let dependency = match *condition {
                    DebugCondition::Selected(control) => control,
                    DebugCondition::ChoiceEquals(choice, _) => choice,
                };
                self.assert_effective_dependencies_acyclic(dependency, visiting);
            }
        }

        visiting.remove(&id);
    }

    pub fn master_enabled(&self) -> bool {
        self.master_enabled
    }

    pub fn toggle_master(&mut self) -> bool {
        self.master_enabled = !self.master_enabled;
        self.revision = self.revision.wrapping_add(1);
        self.master_enabled
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn nodes(&self) -> impl Iterator<Item = &DebugControlSpec> {
        self.nodes.iter()
    }

    pub fn node(&self, id: DebugId) -> Option<&DebugControlSpec> {
        self.index.get(&id).and_then(|index| self.nodes.get(*index))
    }

    pub fn children(&self, parent: DebugId) -> Vec<&DebugControlSpec> {
        let mut children = self
            .nodes
            .iter()
            .filter(|node| node.parent == Some(parent))
            .collect::<Vec<_>>();
        children.sort_by_key(|node| (node.order, node.label));
        children
    }

    /// User selection state ignoring the global output master switch.
    pub fn selected(&self, id: DebugId) -> bool {
        self.selected_inner(id, &mut HashSet::new())
    }

    fn selected_inner(&self, id: DebugId, visiting: &mut HashSet<DebugId>) -> bool {
        if !visiting.insert(id) {
            return false;
        }

        let result = if let Some(node) = self.node(id) {
            let own = match &node.kind {
                DebugControlKind::Tool { equipped } => *equipped,
                DebugControlKind::Toggle { value } => *value,
                _ => true,
            };

            own
                && node
                    .parent
                    .map_or(true, |parent| self.selected_inner(parent, visiting))
                && node
                    .requires
                    .iter()
                    .all(|requirement| self.selected_inner(*requirement, visiting))
                && node.conditions.iter().all(|condition| match *condition {
                    DebugCondition::Selected(control) => self.selected_inner(control, visiting),
                    DebugCondition::ChoiceEquals(choice, value) => {
                        self.choice_value(choice) == Some(value)
                            && self.selected_inner(choice, visiting)
                    }
                })
        } else {
            false
        };

        visiting.remove(&id);
        result
    }

    /// Effective output state including the global F3 master switch.
    pub fn active(&self, id: DebugId) -> bool {
        self.master_enabled && self.selected(id)
    }

    /// Whether a control is contextually relevant enough to present in the
    /// configuration UI. Parent enablement is intentionally ignored so nested
    /// values stay visible/configurable while their owning tool is disabled;
    /// explicit conditions, however, hide settings that do not apply to the
    /// currently selected mode.
    pub fn visible(&self, id: DebugId) -> bool {
        let Some(node) = self.node(id) else {
            return false;
        };
        node.conditions.iter().all(|condition| match *condition {
            DebugCondition::Selected(control) => self.selected(control),
            DebugCondition::ChoiceEquals(choice, value) => {
                self.choice_value(choice) == Some(value)
            }
        })
    }

    fn raw_toggle_value(&self, id: DebugId) -> Option<bool> {
        match &self.node(id)?.kind {
            DebugControlKind::Tool { equipped } => Some(*equipped),
            DebugControlKind::Toggle { value } => Some(*value),
            _ => None,
        }
    }

    pub fn toggle_value(&self, id: DebugId) -> Option<bool> {
        self.raw_toggle_value(id)
    }

    pub fn choice_value(&self, id: DebugId) -> Option<&'static str> {
        match &self.node(id)?.kind {
            DebugControlKind::Choice { selected, options } => {
                options.get(*selected).map(|option| option.value)
            }
            _ => None,
        }
    }

    pub fn choice_label(&self, id: DebugId) -> Option<&'static str> {
        match &self.node(id)?.kind {
            DebugControlKind::Choice { selected, options } => {
                options.get(*selected).map(|option| option.label)
            }
            _ => None,
        }
    }

    pub fn scalar_value(&self, id: DebugId) -> Option<f32> {
        match &self.node(id)?.kind {
            DebugControlKind::Scalar { value, .. } => Some(*value),
            _ => None,
        }
    }

    pub fn integer_value(&self, id: DebugId) -> Option<u32> {
        match &self.node(id)?.kind {
            DebugControlKind::Integer { value, .. } => Some(*value),
            _ => None,
        }
    }

    pub fn set_toggle(&mut self, id: DebugId, enabled: bool) {
        self.set_toggle_inner(id, enabled, &mut HashSet::new());
    }

    fn set_toggle_inner(
        &mut self,
        id: DebugId,
        enabled: bool,
        visiting: &mut HashSet<DebugId>,
    ) {
        if !visiting.insert(id) {
            return;
        }
        let Some(index) = self.index.get(&id).copied() else {
            visiting.remove(&id);
            return;
        };
        if !matches!(
            &self.nodes[index].kind,
            DebugControlKind::Tool { .. } | DebugControlKind::Toggle { .. }
        ) {
            visiting.remove(&id);
            return;
        }

        if enabled {
            let requirements = self.nodes[index].requires.clone();
            for requirement in requirements {
                self.enable_requirement(requirement, visiting);
            }

            let mut conflicts = self.nodes[index].conflicts.clone();
            conflicts.extend(
                self.nodes
                    .iter()
                    .filter(|node| node.conflicts.contains(&id))
                    .map(|node| node.id),
            );
            conflicts.sort();
            conflicts.dedup();
            for conflict in conflicts {
                if let Some(conflict_index) = self.index.get(&conflict).copied() {
                    let changed = match &mut self.nodes[conflict_index].kind {
                        DebugControlKind::Tool { equipped } => {
                            let changed = *equipped;
                            *equipped = false;
                            changed
                        }
                        DebugControlKind::Toggle { value } => {
                            let changed = *value;
                            *value = false;
                            changed
                        }
                        _ => false,
                    };
                    if changed {
                        self.revision = self.revision.wrapping_add(1);
                    }
                }
            }
        }

        let changed = match &mut self.nodes[index].kind {
            DebugControlKind::Tool { equipped } => {
                let changed = *equipped != enabled;
                *equipped = enabled;
                changed
            }
            DebugControlKind::Toggle { value } => {
                let changed = *value != enabled;
                *value = enabled;
                changed
            }
            _ => false,
        };
        if changed {
            self.revision = self.revision.wrapping_add(1);
        }
        visiting.remove(&id);
    }

    fn enable_requirement(&mut self, id: DebugId, visiting: &mut HashSet<DebugId>) {
        let mut toggle_ancestors = Vec::new();
        let mut parent = self.node(id).and_then(|node| node.parent);
        while let Some(parent_id) = parent {
            let Some(parent_node) = self.node(parent_id) else {
                break;
            };
            if self.toggle_value(parent_id).is_some() {
                toggle_ancestors.push(parent_id);
            }
            parent = parent_node.parent;
        }

        // Root-most owner first, then the required control itself. This makes a
        // requirement effective rather than merely setting a hidden raw bit under
        // a disabled parent tool.
        for ancestor in toggle_ancestors.into_iter().rev() {
            self.set_toggle_inner(ancestor, true, visiting);
        }
        self.set_toggle_inner(id, true, visiting);
    }

    pub fn toggle(&mut self, id: DebugId) {
        if let Some(value) = self.toggle_value(id) {
            self.set_toggle(id, !value);
        }
    }

    pub fn cycle_choice(&mut self, id: DebugId, direction: i32) {
        let Some(index) = self.index.get(&id).copied() else {
            return;
        };
        let DebugControlKind::Choice { selected, options } = &mut self.nodes[index].kind else {
            return;
        };
        if options.is_empty() {
            return;
        }
        let len = options.len() as i32;
        let next = ((*selected as i32 + direction).rem_euclid(len)) as usize;
        if next != *selected {
            *selected = next;
            self.revision = self.revision.wrapping_add(1);
        }
    }

    pub fn adjust_scalar(&mut self, id: DebugId, steps: i32) {
        let Some(index) = self.index.get(&id).copied() else {
            return;
        };
        let DebugControlKind::Scalar {
            value,
            minimum,
            maximum,
            step,
            ..
        } = &mut self.nodes[index].kind
        else {
            return;
        };
        let next = (*value + *step * steps as f32).clamp(*minimum, *maximum);
        if next != *value {
            *value = next;
            self.revision = self.revision.wrapping_add(1);
        }
    }

    pub fn adjust_integer(&mut self, id: DebugId, steps: i32) {
        let Some(index) = self.index.get(&id).copied() else {
            return;
        };
        let DebugControlKind::Integer {
            value,
            minimum,
            maximum,
            step,
            ..
        } = &mut self.nodes[index].kind
        else {
            return;
        };
        let next = *value as i64 + *step as i64 * steps as i64;
        let next = next.clamp(*minimum as i64, *maximum as i64) as u32;
        if next != *value {
            *value = next;
            self.revision = self.revision.wrapping_add(1);
        }
    }
}

pub trait AppObservabilityExt {
    fn register_debug_control(&mut self, spec: DebugControlSpec) -> &mut Self;
}

impl AppObservabilityExt for App {
    fn register_debug_control(&mut self, spec: DebugControlSpec) -> &mut Self {
        self.init_resource::<DebugControls>();
        self.world_mut().resource_mut::<DebugControls>().register(spec);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROOT: DebugId = DebugId("root");
    const A: DebugId = DebugId("a");
    const B: DebugId = DebugId("b");
    const CHILD: DebugId = DebugId("child");
    const CHOICE: DebugId = DebugId("choice");
    const CONDITIONAL: DebugId = DebugId("conditional");

    fn controls() -> DebugControls {
        let mut controls = DebugControls::default();
        controls.register(DebugControlSpec::group(ROOT, None, "Root", 0));
        controls.register(
            DebugControlSpec::toggle(A, Some(ROOT), "A", 0, true).conflicts_with(B),
        );
        controls.register(DebugControlSpec::toggle(B, Some(ROOT), "B", 1, false));
        controls.register(DebugControlSpec::toggle(CHILD, Some(A), "Child", 0, true));
        controls.register(DebugControlSpec::choice(
            CHOICE,
            Some(ROOT),
            "Choice",
            2,
            [
                DebugChoiceOption::new("one", "One"),
                DebugChoiceOption::new("two", "Two"),
            ],
            0,
        ));
        controls.register(
            DebugControlSpec::toggle(CONDITIONAL, Some(ROOT), "Conditional", 3, true)
                .when_choice(CHOICE, "two"),
        );
        controls.validate();
        controls
    }

    #[test]
    fn child_selection_is_gated_but_preserved() {
        let mut controls = controls();
        assert!(controls.selected(CHILD));
        controls.set_toggle(A, false);
        assert!(!controls.selected(CHILD));
        assert_eq!(controls.toggle_value(CHILD), Some(true));
        controls.set_toggle(A, true);
        assert!(controls.selected(CHILD));
    }

    #[test]
    fn conflicts_resolve_when_enabling() {
        let mut controls = controls();
        controls.set_toggle(B, true);
        assert_eq!(controls.toggle_value(B), Some(true));
        assert_eq!(controls.toggle_value(A), Some(false));
    }

    #[test]
    fn choices_are_intrinsically_exclusive() {
        let mut controls = controls();
        assert_eq!(controls.choice_value(CHOICE), Some("one"));
        controls.cycle_choice(CHOICE, 1);
        assert_eq!(controls.choice_value(CHOICE), Some("two"));
        controls.cycle_choice(CHOICE, 1);
        assert_eq!(controls.choice_value(CHOICE), Some("one"));
    }



    #[test]
    fn requirements_enable_owning_tools_so_the_requirement_is_effective() {
        const OWNER: DebugId = DebugId("owner");
        const REQUIRED: DebugId = DebugId("required");
        const CONSUMER: DebugId = DebugId("consumer");

        let mut controls = DebugControls::default();
        controls.register(DebugControlSpec::group(ROOT, None, "Root", 0));
        controls.register(DebugControlSpec::tool(OWNER, Some(ROOT), "Owner", 0, false));
        controls.register(DebugControlSpec::toggle(REQUIRED, Some(OWNER), "Required", 0, false));
        controls.register(
            DebugControlSpec::toggle(CONSUMER, Some(ROOT), "Consumer", 1, false)
                .requires(REQUIRED),
        );
        controls.validate();

        controls.set_toggle(CONSUMER, true);
        assert_eq!(controls.toggle_value(OWNER), Some(true));
        assert_eq!(controls.toggle_value(REQUIRED), Some(true));
        assert!(controls.selected(CONSUMER));
    }

    #[test]
    #[should_panic(expected = "impossible requirement set")]
    fn validation_rejects_required_controls_that_conflict() {
        let mut controls = DebugControls::default();
        controls.register(DebugControlSpec::group(ROOT, None, "Root", 0));
        controls.register(DebugControlSpec::toggle(A, Some(ROOT), "A", 0, false).requires(B));
        controls.register(DebugControlSpec::toggle(B, Some(ROOT), "B", 1, false).conflicts_with(A));
        controls.validate();
    }

    #[test]
    #[should_panic(expected = "both enabled by default")]
    fn validation_rejects_conflicting_defaults() {
        let mut controls = DebugControls::default();
        controls.register(DebugControlSpec::group(ROOT, None, "Root", 0));
        controls.register(DebugControlSpec::toggle(A, Some(ROOT), "A", 0, true).conflicts_with(B));
        controls.register(DebugControlSpec::toggle(B, Some(ROOT), "B", 1, true));
        controls.validate();
    }

    #[test]
    fn conditions_gate_effective_state_and_visibility_without_losing_raw_state() {
        let mut controls = controls();
        assert_eq!(controls.toggle_value(CONDITIONAL), Some(true));
        assert!(!controls.selected(CONDITIONAL));
        assert!(!controls.visible(CONDITIONAL));

        controls.cycle_choice(CHOICE, 1);
        assert!(controls.selected(CONDITIONAL));
        assert!(controls.visible(CONDITIONAL));
        assert_eq!(controls.toggle_value(CONDITIONAL), Some(true));
    }
}
