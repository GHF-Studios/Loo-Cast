use crate::{
    rhai_binding::{
        meta::{
            abstract_::trait_identity::{DynGetTraitName, DynGetTraitObjectName},
            monomorphized::trait_::*,
        },
    },
};

impl TraitMetadata {
    pub(super) fn register_trait(&self, parent_module: &mut rhai::Module) where Self: Sized {
        parent_module.set_custom_type::<Self>(self.trait_name());
    }
}

impl TraitObjectMetadata {
    pub(super) fn register_trait_object(&self, parent_module: &mut rhai::Module) where Self: Sized {
        parent_module.set_custom_type::<Self>(self.trait_object_name());
    }
}
