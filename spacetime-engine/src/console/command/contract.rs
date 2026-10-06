//! Typed command ingress and result contract shared by frontends.

use bevy::prelude::World;

pub type ConsoleCommandHandler = fn(&mut World, &ConsoleCommandInvocation) -> ConsoleCommandResult;

#[derive(Debug, Clone, Copy)]
pub struct ConsoleCommandSpec {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub usage: &'static str,
    pub summary: &'static str,
}

// #56 hierarchical-console-runtime-vars-v1
#[derive(Debug, Clone, Copy)]
pub enum ConsoleArgumentCompletion {
    CommandPath,
    /// Delegate completion to an embedded command line beginning at this argument.
    /// Static candidates are merged at the first embedded token so action-style
    /// targets such as `+forward` coexist with ordinary console commands.
    CommandTail(&'static [&'static str]),
    RuntimeVariablePath,
    RuntimeVariableValue {
        path_argument: usize,
    },
    Static(&'static [&'static str]),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleCommandSource {
    Overlay,
    Terminal,
    Binding,
}

impl ConsoleCommandSource {
    pub(in crate::console) fn label(self) -> &'static str {
        match self {
            Self::Overlay => "overlay",
            Self::Terminal => "stdin",
            Self::Binding => "bind",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConsoleCommandInvocation {
    pub(super) raw: String,
    pub(super) name: String,
    pub(super) args: Vec<String>,
    pub(super) source: ConsoleCommandSource,
}

impl ConsoleCommandInvocation {
    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }

    pub const fn source(&self) -> ConsoleCommandSource {
        self.source
    }

    pub(in crate::console) fn tokens(&self) -> Vec<String> {
        let mut tokens = Vec::with_capacity(1 + self.args.len());
        tokens.push(self.name.clone());
        tokens.extend(self.args.iter().cloned());
        tokens
    }

    pub(in crate::console) fn resolved(mut self, canonical: String, consumed: usize) -> Self {
        let tokens = self.tokens();
        self.name = canonical;
        self.args = tokens.into_iter().skip(consumed).collect();
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleFocusDisposition {
    KeepConsole,
    ReturnToGameplay,
}

#[derive(Debug)]
pub enum ConsoleCommandResult {
    Silent,
    Success {
        lines: Vec<String>,
        focus: ConsoleFocusDisposition,
    },
    Error(String),
    Clear,
}

impl ConsoleCommandResult {
    pub fn success(line: impl Into<String>) -> Self {
        Self::Success {
            lines: vec![line.into()],
            focus: ConsoleFocusDisposition::KeepConsole,
        }
    }

    pub fn success_and_return_to_gameplay(line: impl Into<String>) -> Self {
        Self::Success {
            lines: vec![line.into()],
            focus: ConsoleFocusDisposition::ReturnToGameplay,
        }
    }

    pub fn lines(lines: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::Success {
            lines: lines.into_iter().map(Into::into).collect(),
            focus: ConsoleFocusDisposition::KeepConsole,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::Error(message.into())
    }

    pub const fn clear() -> Self {
        Self::Clear
    }
}
