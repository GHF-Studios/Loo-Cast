use crate::{
    reflection::internals::traits::*,
    rhai_binding::meta::monomorphized::trait_::*
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