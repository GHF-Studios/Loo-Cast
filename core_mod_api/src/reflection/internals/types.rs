use std::collections::HashMap;

use crate::{reflection::internals::traits::*, utils::string::*};

pub struct RawReflectionMetadata {
    pub top_level_modules: HashMap<TopLevelModulePath, TopLevelModuleMetadata>,
    pub sub_modules: HashMap<SubModulePath, SubModuleMetadata>,
    pub type_proxy_modules: HashMap<TypeProxyModulePath, TypeProxyModuleMetadata>,

    pub traits: HashMap<TraitPath, (TraitMetadata, TraitObjectMetadata)>,
    pub types: HashMap<TypePath, TypeMetadata>,

    pub inherent_impls: HashMap<InherentImplPath, InherentImplMetadata>,
    pub trait_impls: HashMap<TraitImplPath, TraitImplMetadata>,

    pub module_associated_functions: HashMap<ModuleAssociatedFunctionPath, ModuleAssociatedFunctionMetadata>,
    pub item_associated_functions: HashMap<ItemAssociatedFunctionPath, ItemAssociatedFunctionMetadata>,
    pub constructor_functions: HashMap<ConstructorFunctionPath, ConstructorFunctionMetadata>,
    pub method_functions: HashMap<MethodFunctionPath, MethodFunctionMetadata>,
}
impl RawReflectionMetadata {
    pub fn build() -> Self {
        // Modules
        let mut top_level_modules = HashMap::default();
        for entry in inventory::iter::<TopLevelModuleMetadataEntry> {
            let value = entry.0.get();
            let id_path = value.id_path.get();
            if let Some(value) = top_level_modules.insert(id_path, value) {
                let id_path = value.id_path.get();
                panic!("Duplicate top level module '{id_path}'!");
            }
        }
        let mut sub_modules = HashMap::default();
        for entry in inventory::iter::<SubModuleMetadataEntry> {
            let value = entry.0.get();
            let id_path = value.id_path.get();
            if let Some(value) = sub_modules.insert(id_path, value) {
                let id_path = value.id_path.get();
                panic!("Duplicate sub module '{id_path}'!");
            }
        }
        let mut type_proxy_modules = HashMap::default();
        for entry in inventory::iter::<TypeProxyModuleMetadataEntry> {
            let value = entry.0.get();
            let id_path = value.id_path.get();
            if let Some(value) = type_proxy_modules.insert(id_path, value) {
                let id_path = value.id_path.get();
                panic!("Duplicate type proxy module '{id_path}'!");
            }
        }

        // Traits
        let mut traits_raw: HashMap<TraitPath, TraitMetadata> = Default::default();
        let mut trait_objects_raw: HashMap<TraitPath, TraitObjectMetadata> = Default::default();
        for entry in inventory::iter::<TraitMetadataEntry> {
            let value = entry.0.get();
            let id_path = value.id_path.get();
            if let Some(value) = traits_raw.insert(id_path, value) {
                let id_path = value.id_path.get();
                panic!("Duplicate trait '{id_path}'!");
            }
        }
        for entry in inventory::iter::<TraitObjectMetadataEntry> {
            let value = entry.0.get();
            let id_path = value.id_path.get();
            if let Some(value) = trait_objects_raw.insert(id_path, value) {
                let id_path = value.id_path.get();
                panic!("Duplicate trait object '{id_path}'!");
            }
        }
        let mut traits = HashMap::with_capacity(traits_raw.len());
        for (id_path, trait_meta) in traits_raw {
            let trait_object_meta = trait_objects_raw
                .remove(&id_path)
                .unwrap_or_else(|| {
                    panic!("Missing trait object for trait '{id_path}'!");
                });

            let value = (trait_meta, trait_object_meta);

            traits.insert(id_path, value);
        }

        // Types
        let mut types = HashMap::default();
        for entry in inventory::iter::<TypeMetadataEntry> {
            let value = entry.0.get();
            let id_path = value.id_path.get();
            if let Some(value) = types.insert(id_path, value) {
                let id_path = value.id_path.get();
                panic!("Duplicate type '{id_path}'!");
            }
        }

        // Impls
        let mut inherent_impls = HashMap::default();
        for entry in inventory::iter::<InherentImplMetadataEntry> {
            let value = entry.0.get();
            let id_path = value.id_path.get();
            if let Some(value) = inherent_impls.insert(id_path, value) {
                let id_path = value.id_path.get();
                panic!("Duplicate inherent impl '{id_path}'!");
            }
        }
        let mut trait_impls = HashMap::default();
        for entry in inventory::iter::<TraitImplMetadataEntry> {
            let value = entry.0.get();
            let id_path = value.id_path.get();
            if let Some(value) = trait_impls.insert(id_path, value) {
                let id_path = value.id_path.get();
                panic!("Duplicate trait impl '{id_path}'!");
            }
        }

        // Functions
        let mut module_associated_functions = HashMap::default();
        for entry in inventory::iter::<ModuleAssociatedFunctionMetadataEntry> {
            let value = entry.0.get();
            let id_path = value.id_path.get();
            if let Some(value) = module_associated_functions.insert(id_path, value) {
                let id_path = value.id_path.get();
                panic!("Duplicate module associated function '{id_path}'!");
            }
        }
        let mut item_associated_functions = HashMap::default();
        for entry in inventory::iter::<ItemAssociatedFunctionMetadataEntry> {
            let value = entry.0.get();
            let id_path = value.id_path.get();
            if let Some(value) = item_associated_functions.insert(id_path, value) {
                let id_path = value.id_path.get();
                panic!("Duplicate item associated function '{id_path}'!");
            }
        }
        let mut constructor_functions = HashMap::default();
        for entry in inventory::iter::<ConstructorFunctionMetadataEntry> {
            let value = entry.0.get();
            let id_path = value.id_path.get();
            if let Some(value) = constructor_functions.insert(id_path, value) {
                let id_path = value.id_path.get();
                panic!("Duplicate constructor function '{id_path}'!");
            }
        }
        let mut method_functions = HashMap::default();
        for entry in inventory::iter::<MethodFunctionMetadataEntry> {
            let value = entry.0.get();
            let id_path = value.id_path.get();
            if let Some(value) = method_functions.insert(id_path, value) {
                let id_path = value.id_path.get();
                panic!("Duplicate method function '{id_path}'!");
            }
        }

        Self {
            top_level_modules,
            sub_modules,
            type_proxy_modules,

            traits,
            types,

            inherent_impls,
            trait_impls,

            module_associated_functions,
            item_associated_functions,
            constructor_functions,
            method_functions
        } 
    }
}