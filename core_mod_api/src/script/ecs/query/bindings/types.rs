use rhai::{Dynamic, ImmutableString};

pub const QUERY_FILTER_NONE_ID: &str = "ecs::query::filter::none";

#[derive(Clone, Default)]
pub struct Query {
    pub(crate) values: Vec<Dynamic>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QueryData {
    pub(crate) id: String,
}

impl QueryData {
    pub fn of(id: ImmutableString) -> Self {
        let id = id.to_string();
        if id.trim().is_empty() {
            panic!("QueryData::of requires a non-empty id");
        }
        Self { id }
    }

    pub fn id(&self) -> ImmutableString {
        self.id.clone().into()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QueryFilter {
    pub(crate) id: String,
}

impl QueryFilter {
    pub fn none() -> Self {
        Self {
            id: QUERY_FILTER_NONE_ID.to_string(),
        }
    }

    pub fn of(id: ImmutableString) -> Self {
        let id = id.to_string();
        if id.trim().is_empty() {
            panic!("QueryFilter::of requires a non-empty id");
        }
        Self { id }
    }

    pub fn id(&self) -> ImmutableString {
        self.id.clone().into()
    }
}
