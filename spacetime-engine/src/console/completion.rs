//! Input completion over registered command paths and runtime variables.

use super::command::{ConsoleArgumentCompletion, ConsoleCommandRegistry};
use super::RuntimeVariableRegistry;

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
