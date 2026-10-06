//! Named preset definitions, registry and active baseline state.

use super::*;

#[derive(Debug, Clone, Copy)]
pub(crate) struct DeveloperPresetAssignment {
    pub path: &'static str,
    pub value: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct DeveloperPresetSpec {
    pub name: &'static str,
    pub summary: &'static str,
    pub assignments: &'static [DeveloperPresetAssignment],
}

#[derive(Resource, Default)]
pub(crate) struct DeveloperPresetRegistry {
    pub(super) presets: BTreeMap<String, DeveloperPresetSpec>,
}

impl DeveloperPresetRegistry {
    pub(super) fn register(&mut self, spec: DeveloperPresetSpec) {
        let name = normalize(spec.name);
        assert!(!name.is_empty(), "developer preset names must not be empty");
        assert!(
            !self.presets.contains_key(&name),
            "duplicate developer preset `{name}`"
        );
        self.presets.insert(name, spec);
    }

    pub(super) fn spec(&self, name: &str) -> Option<DeveloperPresetSpec> {
        self.presets.get(&normalize(name)).copied()
    }
}

#[derive(Resource, Debug, Default, Clone)]
pub(crate) struct DeveloperPresetState {
    pub(super) active: Vec<String>,
    pub(super) baselines: BTreeMap<String, String>,
}

impl DeveloperPresetState {
    pub(crate) fn is_active(&self, name: &str) -> bool {
        let name = normalize(name);
        self.active.iter().any(|active| active == &name)
    }

    pub(crate) fn active(&self) -> &[String] {
        &self.active
    }

    pub(crate) fn overridden_path_count(&self) -> usize {
        self.baselines.len()
    }
}

const FREECAM_ASSIGNMENTS: &[DeveloperPresetAssignment] = &[
    DeveloperPresetAssignment {
        path: "debug.freecam.enabled",
        value: "true",
    },
    DeveloperPresetAssignment {
        path: "debug.freecam.control_policy",
        value: "exclusive",
    },
    DeveloperPresetAssignment {
        path: "debug.freecam.projection_policy",
        value: "follow",
    },
    DeveloperPresetAssignment {
        path: "debug.freecam.view_demand",
        value: "frozen",
    },
];

pub(super) const FREECAM_PRESET: DeveloperPresetSpec = DeveloperPresetSpec {
    name: "freecam",
    summary: "Detached view-only camera with exclusive controls and frozen sparse view demand.",
    assignments: FREECAM_ASSIGNMENTS,
};
