use std::collections::HashMap;

use crate::{reflection::internals::traits::*, utils::string::*};

pub struct ReflectionRegistry {
    pub top_level_modules: HashMap<TopLevelModulePath, Box<dyn TopLevelModule + 'static>>,
    pub sub_modules: HashMap<SubModulePath, Box<dyn SubModule + 'static>>,
    pub type_proxy_modules: HashMap<TypeProxyModulePath, Box<dyn TypeProxyModule + 'static>>,
    pub traits: HashMap<TraitPath, (Box<dyn Trait + 'static>, Box<dyn TraitObject + 'static>)>,
    pub types: HashMap<TypePath, Box<dyn Type + 'static>>,
    pub module_associated_functions: HashMap<ModuleAssociatedFunctionPath, Box<dyn ModuleAssociatedFunction + 'static>>,
    pub item_associated_functions: HashMap<ItemAssociatedFunctionPath, Box<dyn ItemAssociatedFunction + 'static>>,
    pub constructor_functions: HashMap<ConstructorFunctionPath, Box<dyn ConstructorFunction + 'static>>,
    pub method_functions: HashMap<MethodFunctionPath, Box<dyn MethodFunction + 'static>>,
}
impl ReflectionRegistry {
    pub fn build() -> Self {
        // Module
        let mut top_level_modules: HashMap<TopLevelModulePath, Box<dyn TopLevelModule + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<TopLevelModuleLinkedMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
            if top_level_modules.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id_path}'!")
            }
        }
        let mut sub_modules: HashMap<SubModulePath, Box<dyn SubModule + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<SubModuleLinkedMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
            if sub_modules.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id_path}'!")
            }
        }
        let mut type_proxy_modules: HashMap<TypeProxyModulePath, Box<dyn TypeProxyModule + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<TypeProxyModuleLinkedMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
            if type_proxy_modules.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id_path}'!")
            }
        }
        
        // Trait
        let mut traits_raw: HashMap<TraitPath, Box<dyn Trait>> = Default::default();
        let mut trait_objects_raw: HashMap<TraitPath, Box<dyn TraitObject>> = Default::default();
        for linked_metadata in inventory::iter::<TraitLinkedMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
        
            if traits_raw.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate trait '{id_path}'!");
            }
        }
        for linked_metadata in inventory::iter::<TraitObjectLinkedMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
        
            if trait_objects_raw.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate trait object '{id_path}'!");
            }
        }
        let mut traits: HashMap<
            TraitPath,
            (Box<dyn Trait>, Box<dyn TraitObject>)
        > = HashMap::with_capacity(traits_raw.len());
        for (id_path, trait_inner) in traits_raw {
            let trait_object_inner = trait_objects_raw
                .remove(&id_path)
                .unwrap_or_else(|| {
                    panic!("Missing trait object for trait '{id_path}'!");
                });
            
            traits.insert(id_path, (trait_inner, trait_object_inner));
        }
        if !trait_objects_raw.is_empty() {
            for leftover in trait_objects_raw.keys() {
                panic!("Missing trait for trait object '{leftover}'!");
            }
        }
        
        // Type
        let mut types: HashMap<TypePath, Box<dyn Type + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<TypeLinkedMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
            if types.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id_path}'!")
            }
        }
        
        // Function
        let mut module_associated_functions: HashMap<ModuleAssociatedFunctionPath, Box<dyn ModuleAssociatedFunction + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<ModuleAssociatedFunctionLinkedMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
            if module_associated_functions.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id_path}'!")
            }
        }
        let mut item_associated_functions: HashMap<ItemAssociatedFunctionPath, Box<dyn ItemAssociatedFunction + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<ItemAssociatedFunctionLinkedMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
            if item_associated_functions.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id_path}'!")
            }
        }
        let mut constructor_functions: HashMap<ConstructorFunctionPath, Box<dyn ConstructorFunction + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<ConstructorFunctionLinkedMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
            if constructor_functions.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id_path}'!")
            }
        }
        let mut method_functions: HashMap<MethodFunctionPath, Box<dyn MethodFunction + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<MethodFunctionLinkedMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
            if method_functions.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id_path}'!")
            }
        }

        Self {
            top_level_modules,
            sub_modules,
            type_proxy_modules,
            traits,
            types,
            module_associated_functions,
            item_associated_functions,
            constructor_functions,
            method_functions
        } 
    }
}