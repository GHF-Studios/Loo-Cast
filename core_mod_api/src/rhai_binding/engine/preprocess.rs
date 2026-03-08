use std::collections::{HashMap, HashSet};

use once_cell::sync::Lazy;

use crate::rhai_binding::internals::statics::RUNTIME_BINDING_GRAPH;

#[derive(Clone, Debug)]
struct UseAlias {
    line: usize,
    full_path: String,
    alias: String,
}

fn is_ident_start(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphabetic()
}

fn is_ident_continue(ch: char) -> bool {
    ch == '_' || ch.is_ascii_alphanumeric()
}

fn is_valid_identifier(raw: &str) -> bool {
    let mut chars = raw.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !is_ident_start(first) {
        return false;
    }
    chars.all(is_ident_continue)
}

fn is_valid_path(raw: &str) -> bool {
    if raw.trim().is_empty() {
        return false;
    }
    if raw.starts_with("::") || raw.ends_with("::") {
        return false;
    }
    let parts = raw.split("::").collect::<Vec<_>>();
    if parts.is_empty() {
        return false;
    }
    parts.iter().all(|part| !part.is_empty() && is_valid_identifier(part))
}

fn insert_known_symbol_name(map: &mut HashMap<String, HashSet<String>>, full_path: String) {
    let short_name = full_path.rsplit("::").next().unwrap_or(full_path.as_str()).to_string();
    map.entry(short_name).or_default().insert(full_path);
}

fn build_known_global_symbols_by_name() -> HashMap<String, HashSet<String>> {
    let graph = RUNTIME_BINDING_GRAPH();
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();

    for path in graph.top_level_modules.keys() {
        let raw = path.as_module_path().as_path().to_string().to_string();
        insert_known_symbol_name(&mut map, raw);
    }
    for path in graph.sub_modules.keys() {
        let raw = path.as_module_path().as_path().to_string().to_string();
        insert_known_symbol_name(&mut map, raw);
    }
    for path in graph.types.keys() {
        let raw = path.as_path().to_string().to_string();
        insert_known_symbol_name(&mut map, raw);
    }
    for path in graph.traits.keys() {
        let raw = path.as_path().to_string().to_string();
        insert_known_symbol_name(&mut map, raw);
    }

    map
}

static KNOWN_GLOBAL_SYMBOLS_BY_NAME: Lazy<HashMap<String, HashSet<String>>> = Lazy::new(build_known_global_symbols_by_name);

static RESERVED_RHAI_KEYWORDS: &[&str] = &[
    "if", "else", "while", "loop", "for", "in", "switch", "case", "default", "break", "continue", "return", "throw", "try", "catch", "do", "until", "let",
    "const", "fn", "private", "this", "true", "false", "null",
];

fn validate_alias_name(use_alias: &UseAlias, source_name: &str) {
    if RESERVED_RHAI_KEYWORDS.iter().any(|keyword| *keyword == use_alias.alias) {
        panic!(
            "Invalid Rhai use alias declaration in `{}` at line {}: alias `{}` is a reserved Rhai keyword",
            source_name, use_alias.line, use_alias.alias
        );
    }

    if let Some(existing_paths) = KNOWN_GLOBAL_SYMBOLS_BY_NAME.get(&use_alias.alias) {
        if !existing_paths.contains(&use_alias.full_path) {
            let mut existing_paths = existing_paths.iter().cloned().collect::<Vec<_>>();
            existing_paths.sort_unstable();
            panic!(
                "Invalid Rhai use alias declaration in `{}` at line {}: alias `{}` conflicts with existing global symbol name(s) [{}]",
                source_name,
                use_alias.line,
                use_alias.alias,
                existing_paths.join(", ")
            );
        }
    }
}

fn parse_use_alias_line(line: &str, line_number: usize) -> Option<UseAlias> {
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") || trimmed.starts_with("/*") {
        return None;
    }
    if !trimmed.starts_with("use") {
        return None;
    }

    let mut rest = &trimmed["use".len()..];
    if !rest.starts_with(char::is_whitespace) {
        return None;
    }
    rest = rest.trim_start();

    let path_end = rest
        .find(char::is_whitespace)
        .unwrap_or_else(|| panic!("Invalid Rhai use alias declaration at line {line_number}: expected `use <path> as <alias>;`"));
    let path = rest[..path_end].trim();
    if !is_valid_path(path) {
        panic!("Invalid Rhai use alias declaration at line {line_number}: invalid path `{path}`");
    }
    rest = rest[path_end..].trim_start();

    if !rest.starts_with("as") {
        panic!("Invalid Rhai use alias declaration at line {line_number}: expected `as`");
    }
    rest = &rest["as".len()..];
    if !rest.starts_with(char::is_whitespace) {
        panic!("Invalid Rhai use alias declaration at line {line_number}: expected `as <alias>`");
    }
    rest = rest.trim_start();

    let alias_end = rest.find(|ch: char| ch.is_whitespace() || ch == ';').unwrap_or(rest.len());
    let alias = rest[..alias_end].trim();
    if !is_valid_identifier(alias) {
        panic!("Invalid Rhai use alias declaration at line {line_number}: invalid alias `{alias}`");
    }
    rest = rest[alias_end..].trim_start();

    let Some(trailing) = rest.strip_prefix(';') else {
        panic!("Invalid Rhai use alias declaration at line {line_number}: missing `;` terminator");
    };

    let trailing = trailing.trim_start();
    if !(trailing.is_empty() || trailing.starts_with("//")) {
        panic!("Invalid Rhai use alias declaration at line {line_number}: unexpected trailing content after `;`");
    }

    Some(UseAlias {
        line: line_number,
        full_path: path.to_string(),
        alias: alias.to_string(),
    })
}

fn extract_use_aliases(source: &str, source_name: &str) -> (String, HashMap<String, String>) {
    let mut stripped = String::with_capacity(source.len());
    let mut alias_map: HashMap<String, String> = HashMap::new();
    let mut alias_line_map: HashMap<String, usize> = HashMap::new();

    for (idx, part) in source.split_inclusive('\n').enumerate() {
        let line_number = idx + 1;
        let (line, newline) = if let Some(stripped_newline) = part.strip_suffix('\n') {
            (stripped_newline, "\n")
        } else {
            (part, "")
        };

        if let Some(use_alias) = parse_use_alias_line(line, line_number) {
            validate_alias_name(&use_alias, source_name);
            if let Some(previous_line) = alias_line_map.insert(use_alias.alias.clone(), use_alias.line) {
                panic!(
                    "Duplicate Rhai use alias `{}` in `{}` at line {} (already declared at line {})",
                    use_alias.alias, source_name, use_alias.line, previous_line
                );
            }
            alias_map.insert(use_alias.alias, use_alias.full_path);
            stripped.push_str(newline);
            continue;
        }

        stripped.push_str(line);
        stripped.push_str(newline);
    }

    (stripped, alias_map)
}

fn parse_declaration_name(line: &str) -> Option<&str> {
    fn parse_name_after_prefix<'a>(line: &'a str, prefix: &str) -> Option<&'a str> {
        let rest = line.strip_prefix(prefix)?;
        if rest.is_empty() {
            return None;
        }
        let end = rest
            .find(|ch: char| ch.is_whitespace() || ch == '=' || ch == '(' || ch == '{' || ch == ';' || ch == '|')
            .unwrap_or(rest.len());
        let candidate = rest[..end].trim();
        if candidate.is_empty() {
            return None;
        }
        Some(candidate)
    }

    let trimmed = line.trim_start();
    if trimmed.starts_with("//") || trimmed.starts_with("/*") {
        return None;
    }

    parse_name_after_prefix(trimmed, "private fn ")
        .or_else(|| parse_name_after_prefix(trimmed, "fn "))
        .or_else(|| parse_name_after_prefix(trimmed, "let "))
        .or_else(|| parse_name_after_prefix(trimmed, "const "))
}

fn validate_alias_local_declarations(source: &str, aliases: &HashMap<String, String>, source_name: &str) {
    if aliases.is_empty() {
        return;
    }

    for (idx, line) in source.lines().enumerate() {
        let line_number = idx + 1;
        let Some(name) = parse_declaration_name(line) else {
            continue;
        };
        if aliases.contains_key(name) {
            panic!(
                "Invalid Rhai use alias usage in `{}` at line {}: `{}` is used as a local declaration name, but aliases are reserved identifiers",
                source_name, line_number, name
            );
        }
    }
}

fn quote_string(value: &str) -> String {
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}

fn rewrite_alias_tokens(source: &str, aliases: &HashMap<String, String>) -> String {
    if aliases.is_empty() {
        return source.to_string();
    }

    let mut out = String::with_capacity(source.len());
    let chars = source.chars().collect::<Vec<_>>();
    let mut i = 0;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum State {
        Code,
        LineComment,
        BlockComment,
        DoubleString,
        SingleString,
    }

    let mut state = State::Code;

    while i < chars.len() {
        match state {
            State::Code => {
                let ch = chars[i];

                if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                    out.push('/');
                    out.push('/');
                    i += 2;
                    state = State::LineComment;
                    continue;
                }
                if ch == '/' && i + 1 < chars.len() && chars[i + 1] == '*' {
                    out.push('/');
                    out.push('*');
                    i += 2;
                    state = State::BlockComment;
                    continue;
                }
                if ch == '"' {
                    out.push(ch);
                    i += 1;
                    state = State::DoubleString;
                    continue;
                }
                if ch == '\'' {
                    out.push(ch);
                    i += 1;
                    state = State::SingleString;
                    continue;
                }

                if is_ident_start(ch) {
                    let start = i;
                    i += 1;
                    while i < chars.len() && is_ident_continue(chars[i]) {
                        i += 1;
                    }

                    let token = chars[start..i].iter().collect::<String>();
                    let mut j = i;
                    while j < chars.len() && chars[j].is_whitespace() {
                        j += 1;
                    }

                    if let Some(full_path) = aliases.get(&token) {
                        if j + 1 < chars.len() && chars[j] == ':' && chars[j + 1] == ':' {
                            out.push_str(full_path);
                            continue;
                        }
                        out.push_str(&quote_string(full_path));
                        continue;
                    }

                    out.push_str(&token);
                    continue;
                }

                out.push(ch);
                i += 1;
            }
            State::LineComment => {
                let ch = chars[i];
                out.push(ch);
                i += 1;
                if ch == '\n' {
                    state = State::Code;
                }
            }
            State::BlockComment => {
                let ch = chars[i];
                out.push(ch);
                i += 1;
                if ch == '*' && i < chars.len() && chars[i] == '/' {
                    out.push('/');
                    i += 1;
                    state = State::Code;
                }
            }
            State::DoubleString => {
                let ch = chars[i];
                out.push(ch);
                i += 1;
                if ch == '\\' && i < chars.len() {
                    out.push(chars[i]);
                    i += 1;
                    continue;
                }
                if ch == '"' {
                    state = State::Code;
                }
            }
            State::SingleString => {
                let ch = chars[i];
                out.push(ch);
                i += 1;
                if ch == '\\' && i < chars.len() {
                    out.push(chars[i]);
                    i += 1;
                    continue;
                }
                if ch == '\'' {
                    state = State::Code;
                }
            }
        }
    }

    out
}

pub(super) fn preprocess_script_source(source: &str, source_name: &str) -> String {
    let (stripped, aliases) = extract_use_aliases(source, source_name);
    validate_alias_local_declarations(&stripped, &aliases, source_name);
    rewrite_alias_tokens(&stripped, &aliases)
}

#[cfg(test)]
mod tests {
    use super::preprocess_script_source;

    #[test]
    fn rewrites_alias_roots_and_strips_use_lines() {
        let source = r#"
use core_mod_api::player::bundles::PlayerBundle as PlayerBundle;
let bundle = PlayerBundle::new_default();
"#;

        let output = preprocess_script_source(source, "test.rhai");
        assert!(!output.contains("use core_mod_api::player::bundles::PlayerBundle as PlayerBundle;"));
        assert!(output.contains("core_mod_api::player::bundles::PlayerBundle::new_default()"));
    }

    #[test]
    fn accepts_extra_whitespace_in_use_declaration() {
        let source = "use    bevy::ecs::query::QueryData\tas   QueryData  ;\nlet q = QueryData::single(\"x\");\n";
        let output = preprocess_script_source(source, "test.rhai");
        assert!(output.contains("let q = bevy::ecs::query::QueryData::single(\"x\");"));
    }

    #[test]
    fn rewrites_bare_alias_to_type_id_literal() {
        let source = r#"
use bevy::ecs::entity::Entity as Entity;
let x = Entity;
"#;
        let output = preprocess_script_source(source, "test.rhai");
        assert!(output.contains("let x = \"bevy::ecs::entity::Entity\";"));
    }

    #[test]
    fn does_not_replace_comments_or_strings() {
        let source = r#"
use bevy::ecs::query::QueryData as QueryData;
// QueryData::single("bevy::ecs::entity::Entity")
let s = "QueryData::single";
let data = QueryData::single("bevy::ecs::entity::Entity");
"#;

        let output = preprocess_script_source(source, "test.rhai");
        assert!(output.contains("// QueryData::single(\"bevy::ecs::entity::Entity\")"));
        assert!(output.contains("let s = \"QueryData::single\";"));
        assert!(output.contains("let data = bevy::ecs::query::QueryData::single(\"bevy::ecs::entity::Entity\");"));
    }

    #[test]
    #[should_panic(expected = "conflicts with existing global symbol")]
    fn alias_conflict_with_known_global_symbol_panics() {
        let source = r#"
use my::custom::Whatever as QueryData;
"#;
        let _ = preprocess_script_source(source, "test.rhai");
    }

    #[test]
    #[should_panic(expected = "Duplicate Rhai use alias")]
    fn duplicate_alias_panics() {
        let source = r#"
use test::module::TypeA as AliasA;
use test::module::TypeB as AliasA;
"#;

        let _ = preprocess_script_source(source, "test.rhai");
    }
}
