//! Command contracts, registry, parsing, and completion.

use super::RuntimeVariableRegistry;
use bevy::prelude::*;
use std::collections::BTreeMap;

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

#[derive(Debug, Clone, Copy)]
pub(super) struct RegisteredConsoleCommand {
    spec: ConsoleCommandSpec,
    pub(super) handler: ConsoleCommandHandler,
    completion: &'static [ConsoleArgumentCompletion],
}

impl RegisteredConsoleCommand {
    /// A command-tail descriptor also owns every later argument of the nested
    /// command; other descriptors apply to one position only.
    fn completion_for(self, argument_index: usize) -> Option<(ConsoleArgumentCompletion, usize)> {
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
    commands: BTreeMap<String, RegisteredConsoleCommand>,
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

    pub(super) fn resolve_tokens(
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

    pub(super) fn command_segment_completions(
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleCommandSource {
    Overlay,
    Terminal,
    Binding,
}

impl ConsoleCommandSource {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::Overlay => "overlay",
            Self::Terminal => "stdin",
            Self::Binding => "bind",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConsoleCommandInvocation {
    raw: String,
    name: String,
    args: Vec<String>,
    source: ConsoleCommandSource,
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

    pub(super) fn tokens(&self) -> Vec<String> {
        let mut tokens = Vec::with_capacity(1 + self.args.len());
        tokens.push(self.name.clone());
        tokens.extend(self.args.iter().cloned());
        tokens
    }

    pub(super) fn resolved(mut self, canonical: String, consumed: usize) -> Self {
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

#[derive(Debug, Clone)]
struct CompletionToken {
    start: usize,
    end: usize,
    value: String,
}

#[derive(Debug)]
struct CompletionSite {
    start: usize,
    end: usize,
    prefix: String,
    preceding: Vec<String>,
    preserve_leading_slash: bool,
}

fn scan_completion_tokens(source: &str) -> Vec<CompletionToken> {
    let mut tokens = Vec::new();
    let mut start = None;
    let mut quote = None;
    let mut escaped = false;

    for (index, character) in source.char_indices() {
        if start.is_none() {
            if character.is_whitespace() {
                continue;
            }
            start = Some(index);
        }

        if escaped {
            escaped = false;
            continue;
        }
        if character == '\\' {
            escaped = true;
            continue;
        }
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            }
            continue;
        }
        if matches!(character, '"' | '\'') {
            quote = Some(character);
            continue;
        }
        if character.is_whitespace() {
            let token_start = start.take().expect("token start exists");
            let raw = &source[token_start..index];
            tokens.push(CompletionToken {
                start: token_start,
                end: index,
                value: decode_completion_fragment(raw),
            });
        }
    }

    if let Some(token_start) = start {
        let raw = &source[token_start..];
        tokens.push(CompletionToken {
            start: token_start,
            end: source.len(),
            value: decode_completion_fragment(raw),
        });
    }

    tokens
}

fn decode_completion_fragment(raw: &str) -> String {
    let mut value = String::new();
    let mut quote = None;
    let mut escaped = false;

    for character in raw.chars() {
        if escaped {
            value.push(character);
            escaped = false;
            continue;
        }
        if character == '\\' {
            escaped = true;
            continue;
        }
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else {
                value.push(character);
            }
            continue;
        }
        if matches!(character, '"' | '\'') {
            quote = Some(character);
        } else {
            value.push(character);
        }
    }

    if escaped {
        value.push('\\');
    }
    value
}

fn completion_site(source: &str, cursor: usize) -> CompletionSite {
    let cursor = cursor.min(source.len());
    let tokens = scan_completion_tokens(source);

    if let Some((index, token)) = tokens
        .iter()
        .enumerate()
        .find(|(_, token)| cursor >= token.start && cursor <= token.end)
    {
        let raw_prefix = &source[token.start..cursor];
        let raw_token = &source[token.start..token.end];
        return CompletionSite {
            start: token.start,
            end: token.end,
            prefix: decode_completion_fragment(raw_prefix),
            preceding: tokens[..index]
                .iter()
                .map(|token| token.value.clone())
                .collect(),
            preserve_leading_slash: index == 0 && raw_token.trim_start().starts_with('/'),
        };
    }

    let preceding = tokens
        .iter()
        .filter(|token| token.end <= cursor)
        .map(|token| token.value.clone())
        .collect();
    CompletionSite {
        start: cursor,
        end: cursor,
        prefix: String::new(),
        preceding,
        preserve_leading_slash: false,
    }
}

pub(super) fn completion_candidates(
    input: &str,
    cursor: usize,
    registry: &ConsoleCommandRegistry,
    runtime_variables: &RuntimeVariableRegistry,
) -> Vec<String> {
    let site = completion_site(input, cursor);
    completion_candidates_for_tokens(&site.preceding, &site.prefix, registry, runtime_variables)
}

fn completion_candidates_for_tokens(
    preceding: &[String],
    prefix: &str,
    registry: &ConsoleCommandRegistry,
    runtime_variables: &RuntimeVariableRegistry,
) -> Vec<String> {
    let command_matches = registry.command_segment_completions(preceding, prefix);
    if !command_matches.is_empty() {
        return command_matches;
    }

    let Some((command, consumed, _)) = registry.resolve_tokens(preceding) else {
        return Vec::new();
    };
    let argument_index = preceding.len().saturating_sub(consumed);
    let Some((completion, completion_index)) = command.completion_for(argument_index) else {
        return Vec::new();
    };

    let prefix_lower = prefix.to_ascii_lowercase();
    let mut values = match completion {
        ConsoleArgumentCompletion::CommandPath => {
            registry.command_segment_completions(&preceding[consumed..], prefix)
        }
        ConsoleArgumentCompletion::CommandTail(static_candidates) => {
            let tail_start = consumed + completion_index;
            let tail_preceding = preceding.get(tail_start..).unwrap_or(&[]);
            let mut values = completion_candidates_for_tokens(
                tail_preceding,
                prefix,
                registry,
                runtime_variables,
            );
            if tail_preceding.is_empty() {
                values.extend(
                    static_candidates
                        .iter()
                        .copied()
                        .filter(|candidate| candidate.starts_with(&prefix_lower))
                        .map(ToOwned::to_owned),
                );
            }
            values
        }
        ConsoleArgumentCompletion::RuntimeVariablePath => {
            runtime_variables.path_completions(&prefix_lower)
        }
        ConsoleArgumentCompletion::RuntimeVariableValue { path_argument } => {
            let arguments = &preceding[consumed..];
            arguments.get(path_argument).map_or_else(Vec::new, |path| {
                runtime_variables.value_completions(path, &prefix_lower)
            })
        }
        ConsoleArgumentCompletion::Static(candidates) => candidates
            .iter()
            .copied()
            .filter(|candidate| candidate.starts_with(&prefix_lower))
            .map(ToOwned::to_owned)
            .collect(),
    };
    values.sort();
    values.dedup();
    values
}

pub(super) fn complete_command_input(
    input: &mut String,
    cursor: usize,
    registry: &ConsoleCommandRegistry,
    runtime_variables: &RuntimeVariableRegistry,
) -> usize {
    let site = completion_site(input, cursor);
    let completions = completion_candidates(input, cursor, registry, runtime_variables);
    if completions.is_empty() {
        return cursor.min(input.len());
    }

    let completed = if completions.len() == 1 {
        completions[0].clone()
    } else {
        common_prefix(&completions)
    };
    let prefix = if site.preserve_leading_slash {
        site.prefix.trim_start_matches('/')
    } else {
        site.prefix.as_str()
    };

    if completed == prefix && completions.len() == 1 && site.end == cursor {
        if input[site.end..]
            .chars()
            .next()
            .is_none_or(|character| !character.is_whitespace())
        {
            input.insert(site.end, ' ');
            return site.end + 1;
        }
        return cursor;
    }
    if completed.len() <= prefix.len() {
        return cursor.min(input.len());
    }

    let replacement = if site.preserve_leading_slash {
        format!("/{completed}")
    } else {
        completed
    };
    input.replace_range(site.start..site.end, &replacement);
    let mut next_cursor = site.start + replacement.len();

    if completions.len() == 1
        && input[next_cursor..]
            .chars()
            .next()
            .is_none_or(|character| !character.is_whitespace())
    {
        input.insert(next_cursor, ' ');
        next_cursor += 1;
    }

    next_cursor
}

pub(super) fn char_to_byte_index(text: &str, char_index: usize) -> usize {
    text.char_indices()
        .nth(char_index)
        .map_or(text.len(), |(index, _)| index)
}

fn common_prefix(values: &[String]) -> String {
    let Some(first) = values.first() else {
        return String::new();
    };
    let mut prefix = first.clone();
    for value in &values[1..] {
        prefix = prefix
            .chars()
            .zip(value.chars())
            .take_while(|(left, right)| left == right)
            .map(|(character, _)| character)
            .collect();
        if prefix.is_empty() {
            break;
        }
    }
    prefix
}

pub(super) fn help_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    let registry = world.resource::<ConsoleCommandRegistry>();

    if !invocation.args().is_empty() {
        let tokens = invocation.args().to_vec();
        let Some((command, consumed, _)) = registry.resolve_tokens(&tokens) else {
            return ConsoleCommandResult::error(format!(
                "unknown command `{}`",
                invocation.args().join(" ")
            ));
        };
        if consumed != tokens.len() {
            return ConsoleCommandResult::error(format!(
                "unknown command `{}`",
                invocation.args().join(" ")
            ));
        }

        let mut lines = vec![format!("{} — {}", command.spec.usage, command.spec.summary)];
        if !command.spec.aliases.is_empty() {
            lines.push(format!("aliases: {}", command.spec.aliases.join(", ")));
        }
        return ConsoleCommandResult::lines(lines);
    }

    ConsoleCommandResult::lines(
        registry
            .commands
            .values()
            .map(|command| format!("{:<44} {}", command.spec.usage, command.spec.summary)),
    )
}

pub(super) fn clear_command(_: &mut World, _: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    ConsoleCommandResult::clear()
}

pub(super) fn echo_command(
    _: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    ConsoleCommandResult::success(invocation.args().join(" "))
}

pub(super) fn parse_command(
    raw: &str,
    source: ConsoleCommandSource,
) -> Result<ConsoleCommandInvocation, String> {
    let source_text = raw.trim();
    let source_text = source_text.strip_prefix('/').unwrap_or(source_text).trim();
    if source_text.is_empty() {
        return Err("empty command".to_string());
    }

    let tokens = tokenize(source_text)?;
    let Some((name, args)) = tokens.split_first() else {
        return Err("empty command".to_string());
    };

    Ok(ConsoleCommandInvocation {
        raw: raw.to_string(),
        name: normalize_name(name),
        args: args.to_vec(),
        source,
    })
}

fn tokenize(source: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quote = None;
    let mut escaped = false;

    for character in source.chars() {
        if escaped {
            current.push(character);
            escaped = false;
            continue;
        }
        if character == '\\' {
            escaped = true;
            continue;
        }
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else {
                current.push(character);
            }
            continue;
        }

        match character {
            '"' | '\'' => quote = Some(character),
            character if character.is_whitespace() => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(character),
        }
    }

    if escaped {
        current.push('\\');
    }
    if quote.is_some() {
        return Err("unterminated quoted argument".to_string());
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    Ok(tokens)
}

fn normalize_name(name: &str) -> String {
    name.trim().trim_start_matches('/').to_ascii_lowercase()
}

fn normalize_path(path: &str) -> String {
    path.split_whitespace()
        .map(normalize_name)
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
