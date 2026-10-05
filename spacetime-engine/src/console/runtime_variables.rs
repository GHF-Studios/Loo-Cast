//! Typed runtime-variable adapters for the shared developer console.
//!
//! #56 typed-runtime-variable-registry-v1
//!
//! This registry is metadata and typed ingress, not a second configuration
//! authority. Bindings adapt existing resources/components. Ordinary semantic
//! simulation operations are deliberately rejected here and remain owned by
//! the causal ingress tracked by #17.

use std::collections::BTreeMap;

use bevy::prelude::*;

use super::{
    AppConsoleExt, ConsoleArgumentCompletion, ConsoleCommandInvocation,
    ConsoleCommandResult, ConsoleCommandSpec,
};

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
    Range {
        minimum: f64,
        maximum: Option<f64>,
    },
    Choices(&'static [&'static str]),
}

impl RuntimeVariableDomain {
    fn completions(self, prefix: &str) -> Vec<String> {
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

    fn label(self) -> Option<String> {
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

#[derive(Resource, Default)]
pub struct RuntimeVariableRegistry {
    bindings: BTreeMap<String, RuntimeVariableBinding>,
}

impl RuntimeVariableRegistry {
    pub fn register(&mut self, binding: RuntimeVariableBinding) {
        let path = normalize_path(binding.spec.path);
        assert!(!path.is_empty(), "runtime-variable paths must not be empty");
        assert!(
            !self.bindings.contains_key(&path),
            "duplicate runtime-variable path `{path}`"
        );
        self.bindings.insert(path, binding);
    }

    pub(crate) fn binding(&self, path: &str) -> Option<RuntimeVariableBinding> {
        self.bindings.get(&normalize_path(path)).copied()
    }

    pub(crate) fn path_completions(&self, prefix: &str) -> Vec<String> {
        let prefix = normalize_path(prefix);
        self.bindings
            .keys()
            .filter(|path| path.starts_with(&prefix))
            .cloned()
            .collect()
    }

    pub(crate) fn value_completions(&self, path: &str, prefix: &str) -> Vec<String> {
        self.binding(path)
            .map_or_else(Vec::new, |binding| binding.spec.domain.completions(prefix))
    }

}

pub trait AppRuntimeVariableExt {
    fn register_runtime_variable(&mut self, binding: RuntimeVariableBinding) -> &mut Self;
}

impl AppRuntimeVariableExt for App {
    fn register_runtime_variable(&mut self, binding: RuntimeVariableBinding) -> &mut Self {
        self.init_resource::<RuntimeVariableRegistry>();
        self.world_mut()
            .resource_mut::<RuntimeVariableRegistry>()
            .register(binding);
        self
    }
}

pub(super) fn configure(app: &mut App) {
    app.register_console_command_with_completion(
        ConsoleCommandSpec {
            name: "config get",
            aliases: &["cvar get"],
            usage: "config get <path>",
            summary: "Read a typed runtime variable and its compiled default.",
        },
        &[ConsoleArgumentCompletion::RuntimeVariablePath],
        config_get,
    )
    .register_console_command_with_completion(
        ConsoleCommandSpec {
            name: "config set",
            aliases: &["cvar set"],
            usage: "config set <path> <value>",
            summary: "Set a typed engine override or local debug control.",
        },
        &[
            ConsoleArgumentCompletion::RuntimeVariablePath,
            ConsoleArgumentCompletion::RuntimeVariableValue { path_argument: 0 },
        ],
        config_set,
    )
    .register_console_command_with_completion(
        ConsoleCommandSpec {
            name: "config reset",
            aliases: &["cvar reset"],
            usage: "config reset <path>",
            summary: "Reset a typed runtime override/control to its owning default.",
        },
        &[ConsoleArgumentCompletion::RuntimeVariablePath],
        config_reset,
    )
    .register_console_command_with_completion(
        ConsoleCommandSpec {
            name: "config list",
            aliases: &["cvar list", "cvars"],
            usage: "config list [prefix]",
            summary: "List registered typed runtime variables and metadata.",
        },
        &[ConsoleArgumentCompletion::RuntimeVariablePath],
        config_list,
    );
}

fn config_get(world: &mut World, invocation: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    if invocation.args().len() != 1 {
        return ConsoleCommandResult::error("usage: config get <path>");
    }
    let path = &invocation.args()[0];
    let binding = {
        let registry = world.resource::<RuntimeVariableRegistry>();
        registry.binding(path)
    };
    let Some(binding) = binding else {
        return ConsoleCommandResult::error(format!("unknown runtime variable `{path}`"));
    };

    match binding.current(world) {
        Ok(current) => {
            let spec = binding.spec();
            ConsoleCommandResult::success(format!(
                "{} = {} | default={} | type={}{} | authority={}",
                spec.path,
                current,
                binding.default_value(),
                spec.value_type.label(),
                spec.units.map_or(String::new(), |units| format!(" {units}")),
                spec.authority.label(),
            ))
        }
        Err(error) => ConsoleCommandResult::error(error),
    }
}

fn config_set(world: &mut World, invocation: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    if invocation.args().len() != 2 {
        return ConsoleCommandResult::error("usage: config set <path> <value>");
    }
    let path = &invocation.args()[0];
    let raw = &invocation.args()[1];
    let binding = {
        let registry = world.resource::<RuntimeVariableRegistry>();
        registry.binding(path)
    };
    let Some(binding) = binding else {
        return ConsoleCommandResult::error(format!("unknown runtime variable `{path}`"));
    };

    if let Err(error) = binding.set(world, raw) {
        return ConsoleCommandResult::error(error);
    }
    match binding.current(world) {
        Ok(current) => ConsoleCommandResult::success(format!(
            "{} = {}",
            binding.spec().path,
            current
        )),
        Err(error) => ConsoleCommandResult::error(error),
    }
}

fn config_reset(world: &mut World, invocation: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    if invocation.args().len() != 1 {
        return ConsoleCommandResult::error("usage: config reset <path>");
    }
    let path = &invocation.args()[0];
    let binding = {
        let registry = world.resource::<RuntimeVariableRegistry>();
        registry.binding(path)
    };
    let Some(binding) = binding else {
        return ConsoleCommandResult::error(format!("unknown runtime variable `{path}`"));
    };

    match binding.reset(world) {
        Ok(()) => ConsoleCommandResult::success(format!(
            "{} reset; layered/effective state reconciles at its owning runtime boundary",
            binding.spec().path
        )),
        Err(error) => ConsoleCommandResult::error(error),
    }
}

fn config_list(world: &mut World, invocation: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: config list [prefix]");
    }
    let prefix = invocation
        .args()
        .first()
        .map_or(String::new(), |value| normalize_path(value));

    let lines = {
        let registry = world.resource::<RuntimeVariableRegistry>();
        registry
            .bindings
            .iter()
            .filter(|(path, _)| path.starts_with(&prefix))
            .map(|(_, binding)| {
                let spec = binding.spec();
                let domain = spec
                    .domain
                    .label()
                    .map_or(String::new(), |domain| format!(" {domain}"));
                let units = spec.units.map_or("", |units| units);
                format!(
                    "{:<52} {:<12} {:<22} {}{} — {}",
                    spec.path,
                    spec.value_type.label(),
                    spec.authority.label(),
                    units,
                    domain,
                    spec.summary,
                )
            })
            .collect::<Vec<_>>()
    };

    if lines.is_empty() {
        ConsoleCommandResult::error(format!(
            "no runtime variables match `{}`",
            invocation.args().first().map_or("", String::as_str)
        ))
    } else {
        ConsoleCommandResult::lines(lines)
    }
}

fn normalize_path(path: &str) -> String {
    path.trim()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}
