use bevy::prelude::*;
use std::collections::HashMap;

use super::pos::GridOffset;
use super::scale::Scale;

#[derive(Resource, Default)]
pub struct ScaleOrigins {
    inner: HashMap<Scale, GridOffset>
}