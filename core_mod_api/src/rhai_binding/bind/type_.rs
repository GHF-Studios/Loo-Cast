use crate::rhai_binding::meta::monomorphized::type_::*;

impl TypeMetadata {
    pub(super) fn register_type(mut self, parent_module: &mut rhai::Module) {
        let parent_module = unsafe { std::mem::transmute::<&mut rhai::Module, &'static mut rhai::Module>(parent_module) };
        self.registrator.call_(parent_module);
    }
}