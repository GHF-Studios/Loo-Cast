use std::marker::PhantomData;

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

#[derive(Clone)]
pub struct StaticTraitObject<T: GetTraitId> {
    pub value: rhai::Dynamic,
    pub trait_id: &'static str,
    pub instance_type_id: &'static str,
    pub _phantom: PhantomData<T>,
}
impl<T: GetTraitId> StaticTraitObject<T> {
    pub fn new(value: rhai::Dynamic, instance_type_id: &'static str) -> Self {
        Self {
            value,
            trait_id: T::TRAIT_ID,
            instance_type_id,
            _phantom: PhantomData,
        }
    }
}

pub trait ToTraitObject<T: GetTraitId>: Sized {
    fn cast_to(self) -> StaticTraitObject<T>;
    fn cast_from(obj: StaticTraitObject<T>) -> Self;
}
