use std::sync::Arc;

use crate::rhai_binding::{
    internals::statics::RUNTIME_BINDING_GRAPH,
    meta::{
        generic::{function::*, impl_::*, module::*, trait_::*, type_::*},
        monomorphized::{function::*, impl_::*, module::*, trait_::*, type_::*},
    },
    path::type_path::TypePath,
};

impl TopLevelModuleMetadata {
    pub(super) fn register_top_level_module(&self, engine: &mut rhai::Engine) {
        let graph = RUNTIME_BINDING_GRAPH();
        let mut top_level_module = rhai::Module::new();
        let id_path = self.id_path().get();
        top_level_module.set_id(id_path.module_name());

        for path in self.sub_modules().get().into_iter() {
            let sub_module = graph
                .sub_modules
                .get(&path)
                .unwrap_or_else(|| {
                    panic!(
                        "Missing SubModuleMetadata '{}' while registering top-level module '{}'",
                        path,
                        id_path.module_name()
                    )
                })
                .clone();
            sub_module.register_sub_module(engine, &mut top_level_module);
        }

        for path in self.traits().get().into_iter() {
            let (trait_, trait_object) = graph
                .traits
                .get(&path)
                .unwrap_or_else(|| {
                    panic!(
                        "Missing TraitMetadata '{}' while registering top-level module '{}'",
                        path,
                        id_path.module_name()
                    )
                })
                .clone();
            trait_.register_trait(&mut top_level_module);
            trait_object.register_trait_object(&mut top_level_module);
        }

        for path in self.types().get().into_iter() {
            let type_ = graph
                .types
                .get(&path)
                .unwrap_or_else(|| panic!("Missing TypeMetadata '{}' while registering top-level module '{}'", path, id_path.module_name()))
                .clone();
            type_.register_type(&mut top_level_module);
        }

        for type_binding_module_path in self.type_binding_modules().get().into_iter() {
            let type_path: TypePath = type_binding_module_path.clone().into();

            let type_binding_module = graph
                .type_binding_modules
                .get(&type_binding_module_path)
                .unwrap_or_else(|| panic!("Missing TypeBindingModule '{}'", type_binding_module_path))
                .clone();

            // === Inherent Impl ===

            let inherent_impl_paths: Vec<_> = graph.inherent_impls.keys().filter(|p| p.type_path() == &type_path).cloned().collect();

            let inherent_impl = match inherent_impl_paths.len() {
                0 => None,
                1 => Some(
                    graph
                        .inherent_impls
                        .get(&inherent_impl_paths[0])
                        .unwrap_or_else(|| panic!("Missing InherentImpl '{}'", inherent_impl_paths[0]))
                        .clone(),
                ),
                _ => panic!("Multiple inherent impls for type '{}'", type_path),
            };

            // === Trait Impls ===

            let trait_impls: Vec<_> = graph
                .trait_impls
                .keys()
                .filter(|p| p.type_path() == &type_path)
                .cloned()
                .map(|p| graph.trait_impls.get(&p).unwrap_or_else(|| panic!("Missing TraitImpl '{}'", p)).clone())
                .collect();

            type_binding_module.register_type_binding_module(engine, &mut top_level_module, inherent_impl, trait_impls);
        }

        for path in self.module_associated_functions().get().into_iter() {
            let module_associated_function = graph
                .module_associated_functions
                .get(&path)
                .unwrap_or_else(|| {
                    panic!(
                        "Missing ModuleAssociatedFunctionMetadata '{}' while registering top-level module '{}'",
                        path,
                        id_path.module_name()
                    )
                })
                .clone();
            module_associated_function.register_module_associated_function(&mut top_level_module);
        }

        engine.register_static_module(id_path.module_name(), Arc::new(top_level_module));
    }
}

impl SubModuleMetadata {
    pub(super) fn register_sub_module(&self, engine: &mut rhai::Engine, parent_module: &mut rhai::Module) {
        let graph = RUNTIME_BINDING_GRAPH();
        let mut origin_sub_module = rhai::Module::new();
        let id_path = self.id_path().get();
        origin_sub_module.set_id(id_path.module_name());

        for path in self.sub_modules().get().into_iter() {
            let sub_module = graph
                .sub_modules
                .get(&path)
                .unwrap_or_else(|| panic!("Missing SubModuleMetadata '{}' while registering sub-module '{}'", path, id_path.module_name()))
                .clone();
            sub_module.register_sub_module(engine, &mut origin_sub_module);
        }

        for path in self.traits().get().into_iter() {
            let (trait_, trait_object) = graph
                .traits
                .get(&path)
                .unwrap_or_else(|| panic!("Missing TraitMetadata '{}' while registering sub-module '{}'", path, id_path.module_name()))
                .clone();
            trait_.register_trait(&mut origin_sub_module);
            trait_object.register_trait_object(&mut origin_sub_module);
        }

        for path in self.types().get().into_iter() {
            let type_ = graph
                .types
                .get(&path)
                .unwrap_or_else(|| panic!("Missing TypeMetadata '{}' while registering sub-module '{}'", path, id_path.module_name()))
                .clone();
            type_.register_type(&mut origin_sub_module);
        }

        for type_binding_module_path in self.type_binding_modules().get().into_iter() {
            let type_path: TypePath = type_binding_module_path.clone().into();

            let type_binding_module = graph
                .type_binding_modules
                .get(&type_binding_module_path)
                .unwrap_or_else(|| panic!("Missing TypeBindingModule '{}'", type_binding_module_path))
                .clone();

            // === Inherent Impl ===

            let inherent_impl_paths: Vec<_> = graph.inherent_impls.keys().filter(|p| p.type_path() == &type_path).cloned().collect();

            let inherent_impl = match inherent_impl_paths.len() {
                0 => None,
                1 => Some(
                    graph
                        .inherent_impls
                        .get(&inherent_impl_paths[0])
                        .unwrap_or_else(|| panic!("Missing InherentImpl '{}'", inherent_impl_paths[0]))
                        .clone(),
                ),
                _ => panic!("Multiple inherent impls for type '{}'", type_path),
            };

            // === Trait Impls ===

            let trait_impls: Vec<_> = graph
                .trait_impls
                .keys()
                .filter(|p| p.type_path() == &type_path)
                .cloned()
                .map(|p| graph.trait_impls.get(&p).unwrap_or_else(|| panic!("Missing TraitImpl '{}'", p)).clone())
                .collect();

            type_binding_module.register_type_binding_module(engine, &mut origin_sub_module, inherent_impl, trait_impls);
        }

        for path in self.module_associated_functions().get().into_iter() {
            let module_associated_function = graph
                .module_associated_functions
                .get(&path)
                .unwrap_or_else(|| {
                    panic!(
                        "Missing ModuleAssociatedFunctionMetadata '{}' while registering sub-module '{}'",
                        path,
                        id_path.module_name()
                    )
                })
                .clone();
            module_associated_function.register_module_associated_function(&mut origin_sub_module);
        }

        parent_module.set_sub_module(id_path.module_name(), origin_sub_module);
    }
}

impl TypeBindingModuleMetadata {
    fn register_type_binding_module(
        &self,
        engine: &mut rhai::Engine,
        parent_module: &mut rhai::Module,
        inherent_impl: Option<InherentImplMetadata>,
        trait_impls: Vec<TraitImplMetadata>,
    ) {
        let graph = RUNTIME_BINDING_GRAPH();
        let mut type_binding_module = rhai::Module::new();
        let id_path = self.id_path().get();
        type_binding_module.set_id(id_path.type_name());

        if let Some(inherent_impl) = inherent_impl {
            inherent_impl.register_inherent_impl(engine, &mut type_binding_module);
        }

        for trait_impl in trait_impls {
            trait_impl.register_trait_impl(engine, &mut type_binding_module);
        }

        for path in self.item_associated_functions().get().into_iter() {
            let item_associated_function = graph
                .item_associated_functions
                .get(&path)
                .unwrap_or_else(|| panic!("Failed to find item associated function '{}'", path))
                .clone();
            item_associated_function.register_item_associated_function(&mut type_binding_module);
        }

        for path in self.constructor_functions().get().into_iter() {
            let constructor_function = graph.constructor_functions.get(&path).unwrap().clone();
            constructor_function.register_constructor_function(&mut type_binding_module);
        }

        for path in self.method_functions().get().into_iter() {
            let method_function = graph.method_functions.get(&path).unwrap().clone();
            method_function.register_method_function(engine);
        }

        parent_module.set_sub_module(id_path.type_name(), type_binding_module);
    }
}
