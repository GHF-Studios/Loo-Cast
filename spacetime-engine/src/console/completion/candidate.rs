//! Resolve command-path and argument completion candidates.

use super::super::{
    RuntimeVariableRegistry,
    command::{ConsoleArgumentCompletion, ConsoleCommandRegistry},
};
use super::scan::completion_site;

pub(in crate::console) fn completion_candidates(
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
