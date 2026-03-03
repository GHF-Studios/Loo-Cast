use crate::rhai_binding::meta::monomorphized::function::*;

impl ModuleAssociatedFunctionMetadata {
    pub(super) fn register_module_associated_function(mut self, parent_module: &mut rhai::Module) {
        let parent_module = unsafe { std::mem::transmute::<&mut rhai::Module, &'static mut rhai::Module>(parent_module) };
        self.registrator.call_(parent_module);
    }
}

impl ItemAssociatedFunctionMetadata {
    pub(super) fn register_item_associated_function(mut self, type_binding_module: &mut rhai::Module) {
        let type_binding_module = unsafe { std::mem::transmute::<&mut rhai::Module, &'static mut rhai::Module>(type_binding_module) };
        self.registrator.call_(type_binding_module);
    }
}

impl ConstructorFunctionMetadata {
    pub(super) fn register_constructor_function(mut self, parent_module: &mut rhai::Module) {
        let parent_module = unsafe { std::mem::transmute::<&mut rhai::Module, &'static mut rhai::Module>(parent_module) };
        self.registrator.call_(parent_module);
    }
}

impl MethodFunctionMetadata {
    pub(super) fn register_method_function(mut self, engine: &mut rhai::Engine) {
        let engine = unsafe { std::mem::transmute::<&mut rhai::Engine, &'static mut rhai::Engine>(engine) };
        self.registrator.call_(engine);
    }
}