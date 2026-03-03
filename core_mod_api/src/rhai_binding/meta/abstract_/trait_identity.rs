use crate::reflection::traits::StaticTraitObject;

// TODO: Consolidate Type::name() into here as TYPE_NAME similarly to GetTraitId
// TODO: Add string-format documentation or newtype with invariant-enforcing on construction
pub trait GetTypeId: Sized + 'static {
    const TYPE_ID: &'static str;
}
pub trait GetTraitId: Clone + Sized + 'static {
    const TRAIT_ID: &'static str;
}
pub trait GetTraitObjectId: Clone + Sized + 'static {
    const TRAIT_OBJECT_ID: &'static str;
}

// TODO: MAJOR: REFACTOR: But where to?
// TODO: MINOR: Add string-format documentation or newtype with invariant-enforcing on construction
pub trait GetTraitName: Clone + Sized + 'static {
    const TRAIT_NAME: &'static str;
}
pub trait GetTraitObjectName: Clone + Sized + 'static {
    const TRAIT_OBJECT_NAME: &'static str;
}
pub trait DynGetTraitName: 'static {
    fn trait_name(&self) -> &'static str;
}
pub trait DynGetTraitObjectName: 'static {
    fn trait_object_name(&self) -> &'static str;
}
pub trait ToTraitObject<T: GetTraitId>: Sized {
    fn cast_to(self) -> StaticTraitObject<T>;
    fn cast_from(obj: StaticTraitObject<T>) -> Self;
}
