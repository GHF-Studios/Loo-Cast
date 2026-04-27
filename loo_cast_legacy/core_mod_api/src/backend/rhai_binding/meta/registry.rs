use std::collections::HashMap;

use crate::rhai_binding::meta::monomorphized::{function::*, generic_::*, impl_::*, module::*, trait_::*, type_::*};
use crate::{
    rhai_binding::path::{function_path::*, impl_path::*, module_path::*, trait_path::*, type_path::*},
    utils::clone_lazy::CloneLazy,
};

inventory::collect!(TopLevelModuleMetadataEntry);
pub struct TopLevelModuleMetadataEntry(pub &'static CloneLazy<TopLevelModuleMetadata>);
inventory::collect!(SubModuleMetadataEntry);
pub struct SubModuleMetadataEntry(pub &'static CloneLazy<SubModuleMetadata>);
inventory::collect!(TypeBindingModuleMetadataEntry);
pub struct TypeBindingModuleMetadataEntry(pub &'static CloneLazy<TypeBindingModuleMetadata>);
inventory::collect!(TraitMetadataEntry);
pub struct TraitMetadataEntry(pub &'static CloneLazy<TraitMetadata>);
inventory::collect!(TraitObjectMetadataEntry);
pub struct TraitObjectMetadataEntry(pub &'static CloneLazy<TraitObjectMetadata>);
inventory::collect!(TypeMetadataEntry);
pub struct TypeMetadataEntry(pub &'static CloneLazy<TypeMetadata>);
inventory::collect!(InherentImplMetadataEntry);
pub struct InherentImplMetadataEntry(pub &'static CloneLazy<InherentImplMetadata>);
inventory::collect!(TraitImplMetadataEntry);
pub struct TraitImplMetadataEntry(pub &'static CloneLazy<TraitImplMetadata>);
inventory::collect!(ModuleAssociatedFunctionMetadataEntry);
pub struct ModuleAssociatedFunctionMetadataEntry(pub &'static CloneLazy<ModuleAssociatedFunctionMetadata>);
inventory::collect!(ItemAssociatedFunctionMetadataEntry);
pub struct ItemAssociatedFunctionMetadataEntry(pub &'static CloneLazy<ItemAssociatedFunctionMetadata>);
inventory::collect!(ConstructorFunctionMetadataEntry);
pub struct ConstructorFunctionMetadataEntry(pub &'static CloneLazy<ConstructorFunctionMetadata>);
inventory::collect!(MethodFunctionMetadataEntry);
pub struct MethodFunctionMetadataEntry(pub &'static CloneLazy<MethodFunctionMetadata>);
inventory::collect!(GenericDefinitionMetadataEntry);
pub struct GenericDefinitionMetadataEntry(pub &'static CloneLazy<GenericDefinitionMetadata>);
inventory::collect!(GenericInstantiationMetadataEntry);
pub struct GenericInstantiationMetadataEntry(pub &'static CloneLazy<GenericInstantiationMetadata>);

pub struct RuntimeBindingGraph {
    pub top_level_modules: HashMap<TopLevelModulePath, TopLevelModuleMetadata>,
    pub sub_modules: HashMap<SubModulePath, SubModuleMetadata>,
    pub type_binding_modules: HashMap<TypeBindingModulePath, TypeBindingModuleMetadata>,

    pub traits: HashMap<TraitPath, (TraitMetadata, TraitObjectMetadata)>,
    pub types: HashMap<TypePath, TypeMetadata>,

    pub inherent_impls: HashMap<InherentImplPath, InherentImplMetadata>,
    pub trait_impls: HashMap<TraitImplPath, TraitImplMetadata>,

    pub module_associated_functions: HashMap<ModuleAssociatedFunctionPath, ModuleAssociatedFunctionMetadata>,
    pub item_associated_functions: HashMap<ItemAssociatedFunctionPath, ItemAssociatedFunctionMetadata>,
    pub constructor_functions: HashMap<ConstructorFunctionPath, ConstructorFunctionMetadata>,
    pub method_functions: HashMap<MethodFunctionPath, MethodFunctionMetadata>,
    pub generic_definitions: HashMap<rhai::ImmutableString, GenericDefinitionMetadata>,
    pub generic_instantiations: HashMap<rhai::ImmutableString, GenericInstantiationMetadata>,
}
impl RuntimeBindingGraph {
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
        let mut type_binding_modules = HashMap::default();
        for entry in inventory::iter::<TypeBindingModuleMetadataEntry> {
            let value = entry.0.get();
            let id_path = value.id_path.get();
            if let Some(value) = type_binding_modules.insert(id_path, value) {
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
            let trait_object_meta = trait_objects_raw.remove(&id_path).unwrap_or_else(|| {
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
            if inherent_impls.contains_key(&id_path) {
                panic!("Duplicate inherent impl '{id_path}'!");
            }
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

        // Generic metadata
        let mut generic_definitions = HashMap::default();
        for entry in inventory::iter::<GenericDefinitionMetadataEntry> {
            let value = entry.0.get();
            let id = value.id.get();
            if let Some(existing) = generic_definitions.insert(id.clone(), value) {
                let existing_id = existing.id.get();
                panic!("Duplicate generic definition '{existing_id}'!");
            }
        }
        let mut generic_instantiations = HashMap::default();
        for entry in inventory::iter::<GenericInstantiationMetadataEntry> {
            let value = entry.0.get();
            let id = value.id.get();
            if let Some(existing) = generic_instantiations.insert(id.clone(), value) {
                let existing_id = existing.id.get();
                panic!("Duplicate generic instantiation '{existing_id}'!");
            }
        }

        Self {
            top_level_modules,
            sub_modules,
            type_binding_modules,
            traits,
            types,
            inherent_impls,
            trait_impls,
            module_associated_functions,
            item_associated_functions,
            constructor_functions,
            method_functions,
            generic_definitions,
            generic_instantiations,
        }
    }

    pub fn log_contents(self) -> Self {
        let top_level_modules_string = self.top_level_modules.keys().collect::<Vec<_>>();
        let sub_modules_string = self.sub_modules.keys().collect::<Vec<_>>();
        let type_binding_modules_string = self.type_binding_modules.keys().collect::<Vec<_>>();
        let traits_string = self.traits.keys().collect::<Vec<_>>();
        let types_string = self.types.keys().collect::<Vec<_>>();
        let inherent_impls_string = self.inherent_impls.keys().collect::<Vec<_>>();
        let trait_impls_string = self.trait_impls.keys().collect::<Vec<_>>();
        let module_associated_functions_string = self.module_associated_functions.keys().collect::<Vec<_>>();
        let item_associated_functions_string = self.item_associated_functions.keys().collect::<Vec<_>>();
        let constructor_functions_string = self.constructor_functions.keys().collect::<Vec<_>>();
        let method_functions_string = self.method_functions.keys().collect::<Vec<_>>();
        let generic_definitions_string = self
            .generic_definitions
            .iter()
            .map(|(id, metadata)| {
                let owner_kind = metadata.owner_kind.get();
                let params = metadata.params.get();
                let bounds = metadata.param_trait_bounds.get();
                let notes = metadata.notes.get();

                let params_with_bounds = params
                    .iter()
                    .enumerate()
                    .map(|(idx, param)| {
                        let param_bounds = bounds.get(idx).cloned().unwrap_or_default();
                        format!("{param}: {:?}", param_bounds)
                    })
                    .collect::<Vec<_>>();

                format!("{id} [{owner_kind:?}] params={params_with_bounds:?} notes={notes:?}")
            })
            .collect::<Vec<_>>();
        let generic_instantiations_string = self
            .generic_instantiations
            .iter()
            .map(|(id, metadata)| {
                let generic_id = metadata.generic_id.get();
                let type_arguments = metadata.type_arguments.get();
                let concrete_item = metadata.concrete_item_path.get();
                let value_semantics = metadata.value_semantics.get();
                format!("{id} => generic={generic_id}, args={type_arguments:?}, concrete={concrete_item}, semantics={value_semantics:?}")
            })
            .collect::<Vec<_>>();
        let generic_bound_links_string = self
            .generic_instantiations
            .iter()
            .map(|(id, inst)| {
                let generic_id = inst.generic_id.get();
                let type_arguments = inst.type_arguments.get();
                let Some(definition) = self.generic_definitions.get(&generic_id) else {
                    return format!("{id}: unresolved generic definition '{generic_id}'");
                };

                let params = definition.params.get();
                let bounds = definition.param_trait_bounds.get();
                let mut links = Vec::new();
                for (idx, param) in params.iter().enumerate() {
                    let arg = type_arguments
                        .get(idx)
                        .map(|path| format!("{path}"))
                        .unwrap_or_else(|| "<missing-arg>".to_string());
                    let required_bounds = bounds.get(idx).cloned().unwrap_or_default();
                    links.push(format!("{param} -> {arg} requires {:?}", required_bounds));
                }

                if type_arguments.len() > params.len() {
                    for extra in type_arguments.iter().skip(params.len()) {
                        links.push(format!("<extra-arg> -> {extra} (no generic parameter slot)"));
                    }
                }

                format!("{id}: {}", links.join(" | "))
            })
            .collect::<Vec<_>>();

        println!("top_level_modules: {:?}", top_level_modules_string);
        println!("sub_modules: {:?}", sub_modules_string);
        println!("type_binding_modules: {:?}", type_binding_modules_string);
        println!("traits: {:?}", traits_string);
        println!("types: {:?}", types_string);
        println!("inherent_impls: {:?}", inherent_impls_string);
        println!("trait_impls: {:?}", trait_impls_string);
        println!("module_associated_functions: {:?}", module_associated_functions_string);
        println!("item_associated_functions: {:?}", item_associated_functions_string);
        println!("constructor_functions: {:?}", constructor_functions_string);
        println!("method_functions: {:?}", method_functions_string);
        println!("generic_definitions: {:?}", generic_definitions_string);
        println!("generic_instantiations: {:?}", generic_instantiations_string);
        println!("generic_bound_links: {:?}", generic_bound_links_string);

        self
    }
}
