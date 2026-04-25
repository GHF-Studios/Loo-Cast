use crate::bevy::prelude::World as BevyWorld;

use crate::rhai_binding::runtime::ecs::dispatch_policy::validate_type_path_id;
use crate::rhai_binding::runtime::ecs::system::query::bindings::types::Query;

pub type QueryDispatchKey = (String, String);
pub type QueryDispatchFn = fn(&mut BevyWorld) -> Query;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QueryDispatchAccess {
    Value,
    Ref,
    Mut,
}

impl QueryDispatchAccess {
    fn as_str(self) -> &'static str {
        match self {
            Self::Value => "value",
            Self::Ref => "ref",
            Self::Mut => "mut",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct QueryDispatchTerm {
    pub type_id: &'static str,
    pub access: QueryDispatchAccess,
}

pub fn query_dispatch_key(data_key: &str, filter_key: &str) -> QueryDispatchKey {
    (data_key.to_string(), filter_key.to_string())
}

pub fn query_data_key(terms: &[QueryDispatchTerm]) -> String {
    if terms.is_empty() {
        panic!("query_data_key requires at least one term");
    }

    terms
        .iter()
        .map(|term| {
            validate_type_path_id("query_data_key::term.type_id", term.type_id);
            format!("{}:{}", term.access.as_str(), term.type_id)
        })
        .collect::<Vec<_>>()
        .join("|")
}

pub fn query_filter_key(with: &'static [&'static str], without: &'static [&'static str]) -> String {
    let mut with = with.iter().map(|value| value.to_string()).collect::<Vec<_>>();
    let mut without = without.iter().map(|value| value.to_string()).collect::<Vec<_>>();

    with.sort_unstable();
    with.dedup();
    without.sort_unstable();
    without.dedup();

    for type_id in &with {
        validate_type_path_id("query_filter_key::with", type_id.as_str());
    }
    for type_id in &without {
        validate_type_path_id("query_filter_key::without", type_id.as_str());
    }

    let overlap = with
        .iter()
        .filter(|type_id| without.binary_search(type_id).is_ok())
        .cloned()
        .collect::<Vec<_>>();
    if !overlap.is_empty() {
        panic!("query_filter_key received overlapping `with` and `without` ids: {:?}", overlap);
    }

    format!("with=[{}];without=[{}]", with.join(","), without.join(","))
}

inventory::collect!(QueryDispatchEntry);
pub struct QueryDispatchEntry {
    pub signature_id: &'static str,
    pub data_terms: &'static [QueryDispatchTerm],
    pub filter_with: &'static [&'static str],
    pub filter_without: &'static [&'static str],
    pub dispatch: QueryDispatchFn,
}
