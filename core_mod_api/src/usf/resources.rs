use bevy::prelude::*;
use std::collections::HashMap;

use crate::utils::types::I128Vec2;

use super::pos::GridOffset;
use super::scale::Scale;

#[derive(Resource)]
pub struct ScaleOrigins {
    inner: [GridOffset; 71]
}
impl Default for ScaleOrigins {
    fn default() -> Self {
        let mut inner = Vec::with_capacity(71);
        let mut current_origin = GridOffset::new_origin(None, Scale::MAX);
        inner.push(current_origin.clone());
        for scale_index in (0..=70).rev() {
            let scale = Scale::from_index_from_top(scale_index).unwrap();
            current_origin = GridOffset::new(Some(Box::new(current_origin)), scale, I128Vec2::ZERO);
            inner.push(current_origin.clone());
        }
        inner.reverse();
        Self { inner: inner.try_into().unwrap() }
    }
}