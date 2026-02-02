use std::hash::Hash;

use crate::reflection::traits::StaticTraitObject;

pub trait GetTypeId: Sized + 'static {
    const TYPE_ID: &'static str;
}

pub trait Trait: Clone + PartialEq + Eq + Hash + Sized + 'static {
    const TRAIT_ID: &'static str;
}
pub trait ToTraitObject<T: Trait>: Sized {
    fn cast_to(self) -> StaticTraitObject<T>;
    fn cast_from(obj: StaticTraitObject<T>) -> Self;
}