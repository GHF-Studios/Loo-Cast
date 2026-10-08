//! Cursor-aware tokenization, quoting, and replacement site discovery.

#[derive(Debug, Clone)]
struct CompletionToken {
    start: usize,
    end: usize,
    value: String,
}

#[derive(Debug)]
pub(super) struct CompletionSite {
    pub(super) start: usize,
    pub(super) end: usize,
    pub(super) prefix: String,
    pub(super) preceding: Vec<String>,
    pub(super) preserve_leading_slash: bool,
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

pub(super) fn completion_site(source: &str, cursor: usize) -> CompletionSite {
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
