//! Apply a selected completion to the command input.

use super::super::{RuntimeVariableRegistry, command::ConsoleCommandRegistry};
use super::{candidate::completion_candidates, scan::completion_site};

pub(in crate::console) fn complete_command_input(
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

pub(in crate::console) fn char_to_byte_index(text: &str, char_index: usize) -> usize {
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
