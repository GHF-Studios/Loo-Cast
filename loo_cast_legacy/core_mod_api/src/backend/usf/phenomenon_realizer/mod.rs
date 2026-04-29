use crate::bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
pub struct PhenomenonRealizerId(pub String);

impl PhenomenonRealizerId {
    pub fn new(raw: impl AsRef<str>) -> Self {
        Self(raw.as_ref().trim().to_ascii_lowercase())
    }
}
