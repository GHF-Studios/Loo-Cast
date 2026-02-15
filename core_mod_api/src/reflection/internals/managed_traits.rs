use rhai::Dynamic;

use crate::{player::bundles::PlayerBundle, reflection::{ids::{StaticTraitId, TypeId}, internals::traits::{ToTraitObject, Trait}, traits::StaticTraitObject}, script::access::ScopedAccessHandle};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BundleTrait;
impl Trait for BundleTrait {
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