use crate::bevy::prelude::Entity as BevyEntity;

#[derive(Clone, Default)]
pub struct Query {
    pub(crate) entities: Vec<BevyEntity>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryDataKind {
    Entities,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QueryData {
    pub(crate) kind: QueryDataKind,
}

impl QueryData {
    pub fn entities() -> Self {
        Self { kind: QueryDataKind::Entities }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QueryFilterKind {
    None,
    WithPlayer,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QueryFilter {
    pub(crate) kind: QueryFilterKind,
}

impl QueryFilter {
    pub fn none() -> Self {
        Self { kind: QueryFilterKind::None }
    }

    pub fn with_player() -> Self {
        Self {
            kind: QueryFilterKind::WithPlayer,
        }
    }
}
