use std::collections::{HashMap, HashSet};

use crate::{reflection::internals::traits::*, utils::{clone_lazy::CloneLazy, string::*}};

pub struct RawReflectionMetadata {
    pub top_level_modules: HashMap<TopLevelModulePath, CloneLazy<TopLevelModuleMetadata>>,
    pub sub_modules: HashMap<SubModulePath, CloneLazy<SubModuleMetadata>>,
    pub type_proxy_modules: HashMap<TypeProxyModulePath, CloneLazy<TypeProxyModuleMetadata>>,
    pub traits: HashMap<TraitPath, CloneLazy<(TraitMetadata, TraitObjectMetadata)>>,
    pub types: HashMap<TypePath, CloneLazy<TypeMetadata>>,
    pub module_associated_functions: HashMap<ModuleAssociatedFunctionPath, CloneLazy<ModuleAssociatedFunctionMetadata>>,
    pub item_associated_functions: HashMap<ItemAssociatedFunctionPath, CloneLazy<ItemAssociatedFunctionMetadata>>,
    pub constructor_functions: HashMap<ConstructorFunctionPath, CloneLazy<ConstructorFunctionMetadata>>,
    pub method_functions: HashMap<MethodFunctionPath, CloneLazy<MethodFunctionMetadata>>,
}
impl RawReflectionMetadata {
    pub fn build() -> Self {
        let mut top_level_modules_raw: HashSet<TopLevelModulePath> = inventory::iter::<TopLevelModuleMetadata>
            .into_iter()
            .map(|d| d.id_path.clone())
            .collect();

        // Module
        let mut top_level_modules: HashMap<TopLevelModulePath, Box<dyn TopLevelModuleConstDynMetadata + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<TopLevelModuleMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
            if top_level_modules.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id_path}'!")
            }
        }
        let mut sub_modules: HashMap<SubModulePath, Box<dyn SubModuleConstDynMetadata + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<SubModuleMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
            if sub_modules.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id_path}'!")
            }
        }
        let mut type_proxy_modules: HashMap<TypeProxyModulePath, Box<dyn TypeProxyModuleConstDynMetadata + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<TypeProxyModuleMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
            if type_proxy_modules.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id_path}'!")
            }
        }
        
        // Trait
        let mut traits_raw: HashMap<TraitPath, Box<dyn TraitConstDynMetadata>> = Default::default();
        let mut trait_objects_raw: HashMap<TraitPath, Box<dyn TraitObjectConstDynMetadata>> = Default::default();
        for linked_metadata in inventory::iter::<TraitMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
        
            if traits_raw.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate trait '{id_path}'!");
            }
        }
        for linked_metadata in inventory::iter::<TraitObjectMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
        
            if trait_objects_raw.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate trait object '{id_path}'!");
            }
        }
        let mut traits: HashMap<
            TraitPath,
            (Box<dyn TraitConstDynMetadata>, Box<dyn TraitObjectConstDynMetadata>)
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
        let mut types: HashMap<TypePath, Box<dyn TypeConstDynMetadata + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<TypeMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
            if types.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id_path}'!")
            }
        }
        
        // Function
        let mut module_associated_functions: HashMap<ModuleAssociatedFunctionPath, Box<dyn ModuleAssociatedFunctionConstDynMetadata + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<ModuleAssociatedFunctionMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
            if module_associated_functions.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id_path}'!")
            }
        }
        let mut item_associated_functions: HashMap<ItemAssociatedFunctionPath, Box<dyn ItemAssociatedFunctionConstDynMetadata + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<ItemAssociatedFunctionMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
            if item_associated_functions.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id_path}'!")
            }
        }
        let mut constructor_functions: HashMap<ConstructorFunctionPath, Box<dyn ConstructorFunctionConstDynMetadata + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<ConstructorFunctionMetadata> {
            let id_path = (linked_metadata.id_path_thunk)();
            let inner = (linked_metadata.inner_thunk)();
            if constructor_functions.insert(id_path.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id_path}'!")
            }
        }
        let mut method_functions: HashMap<MethodFunctionPath, Box<dyn MethodFunctionConstDynMetadata + 'static>> = Default::default();
        for linked_metadata in inventory::iter::<MethodFunctionMetadata> {
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