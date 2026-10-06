//! Console command-line parsing and canonical path normalization.

use super::contract::{ConsoleCommandInvocation, ConsoleCommandSource};

pub(in crate::console) fn parse_command(
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

pub(super) fn normalize_name(name: &str) -> String {
    name.trim().trim_start_matches('/').to_ascii_lowercase()
}

pub(super) fn normalize_path(path: &str) -> String {
    path.split_whitespace()
        .map(normalize_name)
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}
