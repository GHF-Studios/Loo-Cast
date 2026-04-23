use crate::{
    rhai_binding::{
        meta::generic::{abstract_primitive::ConstDynMetadata, module::*},
        path::{function_path::*, impl_path::*, module_path::*, trait_path::TraitPath, type_path::TypePath},
    },
    utils::clone_lazy::CloneLazy,
};

#[derive(Clone)]
pub struct TopLevelModuleMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<TopLevelModulePath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub traits: CloneLazy<Vec<TraitPath>>,
    pub types: CloneLazy<Vec<TypePath>>,
    pub inherent_impls: CloneLazy<Vec<InherentImplPath>>,
    pub trait_impls: CloneLazy<Vec<TraitImplPath>>,

    pub sub_modules: CloneLazy<Vec<SubModulePath>>,
    pub type_binding_modules: CloneLazy<Vec<TypeBindingModulePath>>,
    pub module_associated_functions: CloneLazy<Vec<ModuleAssociatedFunctionPath>>,
}
impl ConstDynMetadata for TopLevelModuleMetadata {
    fn raw_rust_module_path(&self) -> &'static str {
        self.raw_rust_module_path
    }
}
impl NativeModuleConstDynMetadata for TopLevelModuleMetadata {
    fn traits(&self) -> CloneLazy<Vec<TraitPath>> {
        self.traits.clone()
    }
    fn types(&self) -> CloneLazy<Vec<TypePath>> {
        self.types.clone()
    }
    fn inherent_impls(&self) -> CloneLazy<Vec<InherentImplPath>> {
        self.inherent_impls.clone()
    }
    fn trait_impls(&self) -> CloneLazy<Vec<TraitImplPath>> {
        self.trait_impls.clone()
    }
}
impl TopLevelModuleConstDynMetadata for TopLevelModuleMetadata {
    fn id_path(&self) -> CloneLazy<TopLevelModulePath> {
        self.id_path.clone()
    }
    fn sub_modules(&self) -> CloneLazy<Vec<SubModulePath>> {
        self.sub_modules.clone()
    }
    fn type_binding_modules(&self) -> CloneLazy<Vec<TypeBindingModulePath>> {
        self.type_binding_modules.clone()
    }
    fn module_associated_functions(&self) -> CloneLazy<Vec<ModuleAssociatedFunctionPath>> {
        self.module_associated_functions.clone()
    }
}

#[derive(Clone)]
pub struct SubModuleMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<SubModulePath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub traits: CloneLazy<Vec<TraitPath>>,
    pub types: CloneLazy<Vec<TypePath>>,
    pub inherent_impls: CloneLazy<Vec<InherentImplPath>>,
    pub trait_impls: CloneLazy<Vec<TraitImplPath>>,

    pub sub_modules: CloneLazy<Vec<SubModulePath>>,
    pub type_binding_modules: CloneLazy<Vec<TypeBindingModulePath>>,
    pub module_associated_functions: CloneLazy<Vec<ModuleAssociatedFunctionPath>>,
}
impl ConstDynMetadata for SubModuleMetadata {
    fn raw_rust_module_path(&self) -> &'static str {
        self.raw_rust_module_path
    }
}
impl NativeModuleConstDynMetadata for SubModuleMetadata {
    fn traits(&self) -> CloneLazy<Vec<TraitPath>> {
        self.traits.clone()
    }
    fn types(&self) -> CloneLazy<Vec<TypePath>> {
        self.types.clone()
    }
    fn inherent_impls(&self) -> CloneLazy<Vec<InherentImplPath>> {
        self.inherent_impls.clone()
    }
    fn trait_impls(&self) -> CloneLazy<Vec<TraitImplPath>> {
        self.trait_impls.clone()
    }
}
impl SubModuleConstDynMetadata for SubModuleMetadata {
    fn id_path(&self) -> CloneLazy<SubModulePath> {
        self.id_path.clone()
    }
    fn sub_modules(&self) -> CloneLazy<Vec<SubModulePath>> {
        self.sub_modules.clone()
    }
    fn type_binding_modules(&self) -> CloneLazy<Vec<TypeBindingModulePath>> {
        self.type_binding_modules.clone()
    }
    fn module_associated_functions(&self) -> CloneLazy<Vec<ModuleAssociatedFunctionPath>> {
        self.module_associated_functions.clone()
    }
}

#[derive(Clone)]
pub struct TypeBindingModuleMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<TypeBindingModulePath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub item_associated_functions: CloneLazy<Vec<ItemAssociatedFunctionPath>>,
    pub constructor_functions: CloneLazy<Vec<ConstructorFunctionPath>>,
    pub method_functions: CloneLazy<Vec<MethodFunctionPath>>,
}
impl ConstDynMetadata for TypeBindingModuleMetadata {
    fn raw_rust_module_path(&self) -> &'static str {
        self.raw_rust_module_path
    }
}
impl TypeBindingModuleConstDynMetadata for TypeBindingModuleMetadata {
    fn id_path(&self) -> CloneLazy<TypeBindingModulePath> {
        self.id_path.clone()
    }
    fn item_associated_functions(&self) -> CloneLazy<Vec<ItemAssociatedFunctionPath>> {
        self.item_associated_functions.clone()
    }
    fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>> {
        self.constructor_functions.clone()
    }
    fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>> {
        self.method_functions.clone()
    }
}
