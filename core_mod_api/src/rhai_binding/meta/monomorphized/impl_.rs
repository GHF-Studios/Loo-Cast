use crate::{
    rhai_binding::{
        meta::generic::{
            abstract_primitive::ConstDynMetadata,
            impl_::{InherentImplConstDynMetadata, TraitImplConstDynMetadata},
        },
        path::{
            function_path::*,
            impl_path::{InherentImplPath, TraitImplPath},
        },
    },
    utils::clone_lazy::CloneLazy,
};

#[derive(Clone)]
pub struct InherentImplMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<InherentImplPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub constructor_functions: CloneLazy<Vec<ConstructorFunctionPath>>,
    pub method_functions: CloneLazy<Vec<MethodFunctionPath>>,
    pub item_associated_functions: CloneLazy<Vec<ItemAssociatedFunctionPath>>,
}
impl ConstDynMetadata for InherentImplMetadata {
    fn raw_rust_module_path(&self) -> &'static str {
        self.raw_rust_module_path
    }
}
impl InherentImplConstDynMetadata for InherentImplMetadata {
    fn id_path(&self) -> CloneLazy<InherentImplPath> {
        self.id_path.clone()
    }
    fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>> {
        self.constructor_functions.clone()
    }
    fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>> {
        self.method_functions.clone()
    }
    fn item_associated_functions(&self) -> CloneLazy<Vec<ItemAssociatedFunctionPath>> {
        self.item_associated_functions.clone()
    }
}

#[derive(Clone)]
pub struct TraitImplMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<TraitImplPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,

    pub constructor_functions: CloneLazy<Vec<ConstructorFunctionPath>>,
    pub method_functions: CloneLazy<Vec<MethodFunctionPath>>,
    pub item_associated_functions: CloneLazy<Vec<ItemAssociatedFunctionPath>>,
}
impl ConstDynMetadata for TraitImplMetadata {
    fn raw_rust_module_path(&self) -> &'static str {
        self.raw_rust_module_path
    }
}
impl TraitImplConstDynMetadata for TraitImplMetadata {
    fn id_path(&self) -> CloneLazy<TraitImplPath> {
        self.id_path.clone()
    }
    fn constructor_functions(&self) -> CloneLazy<Vec<ConstructorFunctionPath>> {
        self.constructor_functions.clone()
    }
    fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>> {
        self.method_functions.clone()
    }
    fn item_associated_functions(&self) -> CloneLazy<Vec<ItemAssociatedFunctionPath>> {
        self.item_associated_functions.clone()
    }
}
