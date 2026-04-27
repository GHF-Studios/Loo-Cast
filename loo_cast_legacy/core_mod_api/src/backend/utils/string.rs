use heck::{ToPascalCase, ToSnakeCase};
use rhai::ImmutableString;

pub fn assert_pascal_case_clean_string(s: &ImmutableString, string_type_name: &'static str) {
    if s.is_empty() {
        panic!("{string_type_name} strings must not be empty");
    }

    if s.chars().any(|c| c.is_whitespace()) {
        panic!("{string_type_name} strings must not contain whitespace, found '{}'", s);
    }

    if s.chars().any(|c| !c.is_ascii_alphanumeric()) {
        panic!("{string_type_name} strings must be alphanumeric ASCII, found '{}'", s);
    }

    if s.chars().next().unwrap().is_ascii_digit() {
        panic!("{string_type_name} strings must not start with a digit, found '{}'", s);
    }

    if s != s.to_pascal_case() {
        panic!("{string_type_name}s must be in 'PascalCase' format, found '{}'", s);
    }
}

pub fn assert_snake_case_clean_string(s: &ImmutableString, string_type_name: &'static str) {
    if s.is_empty() {
        panic!("{string_type_name} strings must not be empty");
    }

    if s.chars().any(|c| c.is_whitespace()) {
        panic!("{string_type_name} strings must not contain whitespace, found '{}'", s);
    }

    if s.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
        panic!("{string_type_name} strings must be alphanumeric ASCII or underscores, found '{}'", s);
    }

    if s.chars().next().unwrap().is_ascii_digit() {
        panic!("{string_type_name} strings must not start with a digit, found '{}'", s);
    }

    let s = if s.chars().last().unwrap() == '_' {
        s.rsplit_once('_').unwrap().0
    } else {
        s.as_str()
    };

    if s != s.to_snake_case() {
        panic!("{string_type_name}s must be in 'snake_case' format, found '{}'", s);
    }
}
