use crate::rhai_binding::{
    internals::statics::RUNTIME_BINDING_GRAPH,
    meta::{
        generic::impl_::*,
        monomorphized::impl_::*
    }
};

impl InherentImplMetadata {
    pub(super) fn register_inherent_impl(&self, engine: &mut rhai::Engine, type_binding_module: &mut rhai::Module) {
        let graph = RUNTIME_BINDING_GRAPH();

        for path in self.constructor_functions().get().into_iter() {
            let constructor_function = graph.constructor_functions.get(&path).unwrap().clone();
            constructor_function.register_constructor_function(type_binding_module);
        }

        for path in self.method_functions().get().into_iter() {
            let method_function = graph.method_functions.get(&path).unwrap().clone();
            method_function.register_method_function(engine);
        }

        for path in self.item_associated_functions().get().into_iter() {
            let item_associated_function = graph.item_associated_functions.get(&path).unwrap().clone();
            item_associated_function.register_item_associated_function(type_binding_module);
        }
    }
}

impl TraitImplMetadata {
    pub(super) fn register_trait_impl(&self, engine: &mut rhai::Engine, type_binding_module: &mut rhai::Module) {
        let graph = RUNTIME_BINDING_GRAPH();

        for path in self.constructor_functions().get().into_iter() {
            let constructor_function = graph.constructor_functions.get(&path).unwrap().clone();
            constructor_function.register_constructor_function(type_binding_module);
        }

        for path in self.method_functions().get().into_iter() {
            let method_function = graph.method_functions.get(&path).unwrap().clone();
            method_function.register_method_function(engine);
        }

        for path in self.item_associated_functions().get().into_iter() {
            let item_associated_function = graph.item_associated_functions.get(&path).unwrap().clone();
            item_associated_function.register_item_associated_function(type_binding_module);
        }
    }
}