use std::collections::HashMap;

use crate::{reflection::internals::traits::*, utils::string::*};

pub struct ReflectionRegistry {
    pub top_level_modules: HashMap<TopLevelModulePath, Box<dyn TopLevelModule + 'static>>,
    pub sub_modules: HashMap<SubModulePath, Box<dyn SubModule + 'static>>,
    pub type_proxy_modules: HashMap<TypeProxyModulePath, Box<dyn TypeProxyModule + 'static>>,
    pub traits: HashMap<TraitPath, (Box<dyn Trait + 'static>, Box<dyn TraitObject + 'static>)>,
    pub types: HashMap<TypePath, Box<dyn Type + 'static>>,
    pub module_associated_functions: HashMap<ModuleAssociatedFunctionPath, Box<dyn ModuleAssociatedFunction + 'static>>,
    pub type_associated_functions: HashMap<TypeAssociatedFunctionPath, Box<dyn TypeAssociatedFunction + 'static>>,
    pub constructor_functions: HashMap<ConstructorFunctionPath, Box<dyn ConstructorFunction + 'static>>,
    pub method_functions: HashMap<MethodFunctionPath, Box<dyn MethodFunction + 'static>>,
}
impl ReflectionRegistry {
    pub fn build() -> Self {
        let mut top_level_modules: HashMap<TopLevelModulePath, Box<dyn TopLevelModule + 'static>> = Default::default();
        for entry in inventory::iter::<TopLevelModuleMetadata> {
            let id = (entry.id_thunk)();
            let inner = (entry.inner_thunk)();
            if top_level_modules.insert(id.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id}'!")
            }
        }
        let mut sub_modules: HashMap<SubModulePath, Box<dyn SubModule + 'static>> = Default::default();
        for entry in inventory::iter::<SubModuleMetadata> {
            let id = (entry.id_thunk)();
            let inner = (entry.inner_thunk)();
            if sub_modules.insert(id.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id}'!")
            }
        }
        let mut type_proxy_modules: HashMap<TypeProxyModulePath, Box<dyn TypeProxyModule + 'static>> = Default::default();
        for entry in inventory::iter::<TypeProxyModuleMetadata> {
            let id = (entry.id_thunk)();
            let inner = (entry.inner_thunk)();
            if type_proxy_modules.insert(id.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id}'!")
            }
        }
        
        let mut traits_raw: HashMap<TraitPath, Box<dyn Trait>> = Default::default();
        let mut trait_objects_raw: HashMap<TraitPath, Box<dyn TraitObject>> = Default::default();
        for entry in inventory::iter::<TraitMetadata> {
            let id = (entry.id_thunk)();
            let inner = (entry.inner_thunk)();
        
            if traits_raw.insert(id.clone(), inner).is_some() {
                panic!("Duplicate trait '{id}'!");
            }
        }
        for entry in inventory::iter::<TraitObjectMetadata> {
            let id = (entry.id_thunk)();
            let inner = (entry.inner_thunk)();
        
            if trait_objects_raw.insert(id.clone(), inner).is_some() {
                panic!("Duplicate trait object '{id}'!");
            }
        }
        let mut traits: HashMap<
            TraitPath,
            (Box<dyn Trait>, Box<dyn TraitObject>)
        > = HashMap::with_capacity(traits_raw.len());
        for (id, trait_impl) in traits_raw {
            let trait_object = trait_objects_raw
                .remove(&id)
                .unwrap_or_else(|| {
                    panic!("Missing trait object for trait '{id}'!");
                });
            
            traits.insert(id, (trait_impl, trait_object));
        }
        if !trait_objects_raw.is_empty() {
            for leftover in trait_objects_raw.keys() {
                panic!("Missing trait for trait object '{leftover}'!");
            }
        }
        
        let mut types: HashMap<TypePath, Box<dyn Type + 'static>> = Default::default();
        for entry in inventory::iter::<TypeMetadata> {
            let id = (entry.id_thunk)();
            let inner = (entry.inner_thunk)();
            if types.insert(id.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id}'!")
            }
        }
        
        let mut module_associated_functions: HashMap<ModuleAssociatedFunctionPath, Box<dyn ModuleAssociatedFunction + 'static>> = Default::default();
        for entry in inventory::iter::<ModuleAssociatedFunctionMetadata> {
            let id = (entry.id_thunk)();
            let inner = (entry.inner_thunk)();
            if module_associated_functions.insert(id.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id}'!")
            }
        }
        let mut type_associated_functions: HashMap<TypeAssociatedFunctionPath, Box<dyn TypeAssociatedFunction + 'static>> = Default::default();
        for entry in inventory::iter::<TypeAssociatedFunctionMetadata> {
            let id = (entry.id_thunk)();
            let inner = (entry.inner_thunk)();
            if type_associated_functions.insert(id.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id}'!")
            }
        }
        let mut constructor_functions: HashMap<ConstructorFunctionPath, Box<dyn ConstructorFunction + 'static>> = Default::default();
        for entry in inventory::iter::<ConstructorFunctionMetadata> {
            let id = (entry.id_thunk)();
            let inner = (entry.inner_thunk)();
            if constructor_functions.insert(id.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id}'!")
            }
        }
        let mut method_functions: HashMap<MethodFunctionPath, Box<dyn MethodFunction + 'static>> = Default::default();
        for entry in inventory::iter::<MethodFunctionMetadata> {
            let id = (entry.id_thunk)();
            let inner = (entry.inner_thunk)();
            if method_functions.insert(id.clone(), inner).is_some() {
                panic!("Duplicate top level module '{id}'!")
            }
        }

        Self {
            top_level_modules,
            sub_modules,
            type_proxy_modules,
            traits,
            types,
            module_associated_functions,
            type_associated_functions,
            constructor_functions,
            method_functions
        } 
    }
}