//! Typed variable contract and authority restrictions.

use super::*;

pub type RuntimeVariableGetter = fn(&mut World) -> Result<String, String>;
pub type RuntimeVariableSetter = fn(&mut World, &str) -> Result<(), String>;
pub type RuntimeVariableResetter = fn(&mut World) -> Result<(), String>;
pub type RuntimeVariableDefault = fn() -> String;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeVariableAuthority {
    EngineConfigOverride,
    LocalDeveloperControl,
    SemanticOperation,
}

impl RuntimeVariableAuthority {
    pub const fn label(self) -> &'static str {
        match self {
            Self::EngineConfigOverride => "engine-config-override",
            Self::LocalDeveloperControl => "local-debug",
            Self::SemanticOperation => "semantic-operation",
        }
    }

    const fn directly_mutable(self) -> bool {
        !matches!(self, Self::SemanticOperation)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuntimeVariableValueType {
    Boolean,
    Enum,
    F32,
    OptionalF32,
    Usize,
}

impl RuntimeVariableValueType {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Boolean => "bool",
            Self::Enum => "enum",
            Self::F32 => "f32",
            Self::OptionalF32 => "optional-f32",
            Self::Usize => "usize",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum RuntimeVariableDomain {
    Any,
    Range { minimum: f64, maximum: Option<f64> },
    Choices(&'static [&'static str]),
}

impl RuntimeVariableDomain {
    pub(super) fn completions(self, prefix: &str) -> Vec<String> {
        match self {
            Self::Choices(choices) => choices
                .iter()
                .copied()
                .filter(|choice| choice.starts_with(prefix))
                .map(ToOwned::to_owned)
                .collect(),
            Self::Any | Self::Range { .. } => Vec::new(),
        }
    }

    pub(super) fn label(self) -> Option<String> {
        match self {
            Self::Any => None,
            Self::Range {
                minimum,
                maximum: Some(maximum),
            } => Some(format!("[{minimum}, {maximum}]")),
            Self::Range {
                minimum,
                maximum: None,
            } => Some(format!("[{minimum}, +inf)")),
            Self::Choices(choices) => Some(choices.join("|")),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RuntimeVariableSpec {
    pub path: &'static str,
    pub summary: &'static str,
    pub value_type: RuntimeVariableValueType,
    pub units: Option<&'static str>,
    pub authority: RuntimeVariableAuthority,
    pub domain: RuntimeVariableDomain,
}

#[derive(Debug, Clone, Copy)]
pub struct RuntimeVariableBinding {
    spec: RuntimeVariableSpec,
    getter: RuntimeVariableGetter,
    setter: RuntimeVariableSetter,
    resetter: RuntimeVariableResetter,
    default: RuntimeVariableDefault,
}

impl RuntimeVariableBinding {
    pub const fn new(
        spec: RuntimeVariableSpec,
        getter: RuntimeVariableGetter,
        setter: RuntimeVariableSetter,
        resetter: RuntimeVariableResetter,
        default: RuntimeVariableDefault,
    ) -> Self {
        Self {
            spec,
            getter,
            setter,
            resetter,
            default,
        }
    }

    pub const fn spec(self) -> RuntimeVariableSpec {
        self.spec
    }

    pub(crate) fn current(self, world: &mut World) -> Result<String, String> {
        (self.getter)(world)
    }

    pub(crate) fn set(self, world: &mut World, raw: &str) -> Result<(), String> {
        if !self.spec.authority.directly_mutable() {
            return Err(format!(
                "`{}` is a semantic operation; route it through typed causal ingress (#17), not generic runtime config",
                self.spec.path
            ));
        }
        (self.setter)(world, raw)
    }

    pub(crate) fn reset(self, world: &mut World) -> Result<(), String> {
        if !self.spec.authority.directly_mutable() {
            return Err(format!(
                "`{}` is a semantic operation; route it through typed causal ingress (#17), not generic runtime config",
                self.spec.path
            ));
        }
        (self.resetter)(world)
    }

    pub(crate) fn default_value(self) -> String {
        (self.default)()
    }
}
