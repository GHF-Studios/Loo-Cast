use rhai::Dynamic;

use crate::{
    player::bundles::PlayerBundle,
    rhai_binding::{
        meta::abstract_::trait_identity::{GetTraitId, GetTraitName, GetTraitObjectName, ToTraitObject},
        value_semantics::{
            ids::{StaticTraitId, TypeId},
            scoped_access::ScopedAccessHandle,
            trait_object::StaticTraitObject,
        },
    },
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BundleTrait;
impl GetTraitName for BundleTrait {
    const TRAIT_NAME: &'static str = "Bundle";
}
impl GetTraitObjectName for BundleTrait {
    const TRAIT_OBJECT_NAME: &'static str = "BundleTraitObject";
}
impl GetTraitId for BundleTrait {
    const TRAIT_ID: &'static str = "ecs::bundle::Bundle";
}

#[repr(transparent)]
pub struct BundleTraitObject(pub StaticTraitObject<BundleTrait>);
impl ToTraitObject<BundleTrait> for ScopedAccessHandle<PlayerBundle> {
    fn cast_to(self) -> StaticTraitObject<BundleTrait> {
        StaticTraitObject {
            value: Dynamic::from(self.0),
            trait_id: StaticTraitId::new(),
            instance_type_id: TypeId::of::<PlayerBundle>(),
        }
    }

    fn cast_from(obj: StaticTraitObject<BundleTrait>) -> Self {
        ScopedAccessHandle(obj.value.cast())
    }
}
