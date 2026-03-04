use crate::bevy::prelude::Entity as BevyEntity;
use rhai::{Array, Dynamic};

pub trait QueryApi {
    fn len(&self) -> i64;
    fn is_empty(&self) -> bool;
    fn to_array(&self) -> Array;
    fn first_or_unit(&self) -> Dynamic;
    fn single(&self) -> BevyEntity;
    fn try_single(&self) -> Dynamic;
}
