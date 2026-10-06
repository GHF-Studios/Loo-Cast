//! Registered command paths, aliases and argument-completion ownership.

use super::{
    contract::*,
    parse::{normalize_name, normalize_path},
};
use bevy::prelude::*;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy)]
pub(in crate::console) struct RegisteredConsoleCommand {
    pub(super) spec: ConsoleCommandSpec,
    pub(in crate::console) handler: ConsoleCommandHandler,
    completion: &'static [ConsoleArgumentCompletion],
}

impl RegisteredConsoleCommand {
    /// A command-tail descriptor also owns every later argument of the nested
    /// command; other descriptors apply to one position only.
    pub(in crate::console) fn completion_for(
        self,
        argument_index: usize,
    ) -> Option<(ConsoleArgumentCompletion, usize)> {
        if let Some(completion) = self.completion.get(argument_index) {
            return Some((*completion, argument_index));
        }
        let last = *self.completion.last()?;
        matches!(last, ConsoleArgumentCompletion::CommandTail(_))
            .then_some((last, self.completion.len() - 1))
    }
}

#[derive(Resource, Default)]
pub struct ConsoleCommandRegistry {
    pub(super) commands: BTreeMap<String, RegisteredConsoleCommand>,
    aliases: BTreeMap<String, String>,
}

impl ConsoleCommandRegistry {
    pub fn register(&mut self, spec: ConsoleCommandSpec, handler: ConsoleCommandHandler) {
        self.register_with_completion(spec, &[], handler);
    }

    pub fn register_with_completion(
        &mut self,
        spec: ConsoleCommandSpec,
        completion: &'static [ConsoleArgumentCompletion],
        handler: ConsoleCommandHandler,
    ) {
        let canonical = normalize_path(spec.name);
        assert!(
            !canonical.is_empty(),
            "console command paths must not be empty"
        );
        assert!(
            !self.commands.contains_key(&canonical) && !self.aliases.contains_key(&canonical),
            "duplicate console command path `{canonical}`"
        );

        for &alias in spec.aliases {
            let alias = normalize_path(alias);
            assert!(
                !alias.is_empty()
                    && !self.commands.contains_key(&alias)
                    && !self.aliases.contains_key(&alias),
                "duplicate/empty console command alias `{alias}`"
            );
            self.aliases.insert(alias, canonical.clone());
        }

        self.commands.insert(
            canonical,
            RegisteredConsoleCommand {
                spec,
                handler,
                completion,
            },
        );
    }

    pub(in crate::console) fn resolve_tokens(
        &self,
        tokens: &[String],
    ) -> Option<(RegisteredConsoleCommand, usize, String)> {
        for consumed in (1..=tokens.len()).rev() {
            let candidate = normalize_path(&tokens[..consumed].join(" "));
            if let Some(command) = self.commands.get(&candidate).copied() {
                return Some((command, consumed, candidate));
            }
            if let Some(canonical) = self.aliases.get(&candidate)
                && let Some(command) = self.commands.get(canonical).copied()
            {
                return Some((command, consumed, canonical.clone()));
            }
        }
        None
    }

    pub(in crate::console) fn command_segment_completions(
        &self,
        preceding: &[String],
        prefix: &str,
    ) -> Vec<String> {
        let preceding = preceding
            .iter()
            .map(|segment| normalize_name(segment))
            .collect::<Vec<_>>();
        let prefix = normalize_name(prefix);
        let mut matches = Vec::new();

        for path in self.commands.keys().chain(self.aliases.keys()) {
            let segments = path.split_whitespace().collect::<Vec<_>>();
            if preceding.len() >= segments.len() {
                continue;
            }
            if preceding
                .iter()
                .zip(segments.iter())
                .any(|(left, right)| left != right)
            {
                continue;
            }
            let candidate = segments[preceding.len()];
            if candidate.starts_with(&prefix) {
                matches.push(candidate.to_string());
            }
        }

        matches.sort();
        matches.dedup();
        matches
    }
}

pub trait AppConsoleExt {
    fn register_console_command(
        &mut self,
        spec: ConsoleCommandSpec,
        handler: ConsoleCommandHandler,
    ) -> &mut Self;

    fn register_console_command_with_completion(
        &mut self,
        spec: ConsoleCommandSpec,
        completion: &'static [ConsoleArgumentCompletion],
        handler: ConsoleCommandHandler,
    ) -> &mut Self;
}

impl AppConsoleExt for App {
    fn register_console_command(
        &mut self,
        spec: ConsoleCommandSpec,
        handler: ConsoleCommandHandler,
    ) -> &mut Self {
        self.init_resource::<ConsoleCommandRegistry>();
        self.world_mut()
            .resource_mut::<ConsoleCommandRegistry>()
            .register(spec, handler);
        self
    }

    fn register_console_command_with_completion(
        &mut self,
        spec: ConsoleCommandSpec,
        completion: &'static [ConsoleArgumentCompletion],
        handler: ConsoleCommandHandler,
    ) -> &mut Self {
        self.init_resource::<ConsoleCommandRegistry>();
        self.world_mut()
            .resource_mut::<ConsoleCommandRegistry>()
            .register_with_completion(spec, completion, handler);
        self
    }
}
