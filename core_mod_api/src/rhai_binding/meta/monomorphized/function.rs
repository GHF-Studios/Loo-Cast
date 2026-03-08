use rhai::ImmutableString;

use crate::{
    rhai_binding::{
        meta::generic::{abstract_primitive::ConstDynMetadata, function::*},
        path::function_path::*,
    },
    utils::{clone_closure::CloneClosure, clone_lazy::CloneLazy},
};

#[derive(Clone)]
pub struct ModuleAssociatedFunctionMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<ModuleAssociatedFunctionPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    pub registrator: CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)>,
}
impl ConstDynMetadata for ModuleAssociatedFunctionMetadata {
    fn raw_rust_module_path(&self) -> &'static str {
        self.raw_rust_module_path
    }
}
impl ModuleAssociatedFunctionConstDynMetadata for ModuleAssociatedFunctionMetadata {
    fn id_path(&self) -> CloneLazy<ModuleAssociatedFunctionPath> {
        self.id_path.clone()
    }
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)> {
        self.registrator
    }
}
#[derive(Clone)]
pub struct ItemAssociatedFunctionMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<ItemAssociatedFunctionPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    pub registrator: CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)>,
}
impl ConstDynMetadata for ItemAssociatedFunctionMetadata {
    fn raw_rust_module_path(&self) -> &'static str {
        self.raw_rust_module_path
    }
}
impl ItemAssociatedFunctionConstDynMetadata for ItemAssociatedFunctionMetadata {
    fn id_path(&self) -> CloneLazy<ItemAssociatedFunctionPath> {
        self.id_path.clone()
    }
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)> {
        self.registrator
    }
}
#[derive(Clone)]
pub struct ConstructorFunctionMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<ConstructorFunctionPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    pub registrator: CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)>,
}
impl ConstDynMetadata for ConstructorFunctionMetadata {
    fn raw_rust_module_path(&self) -> &'static str {
        self.raw_rust_module_path
    }
}
impl ConstructorFunctionConstDynMetadata for ConstructorFunctionMetadata {
    fn id_path(&self) -> CloneLazy<ConstructorFunctionPath> {
        self.id_path.clone()
    }
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)> {
        self.registrator
    }
}
#[derive(Clone)]
pub struct MethodFunctionMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<MethodFunctionPath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    pub registrator: CloneClosure<ImmutableString, &'static mut rhai::Engine, (), fn(ImmutableString, &mut rhai::Engine)>,
}
impl ConstDynMetadata for MethodFunctionMetadata {
    fn raw_rust_module_path(&self) -> &'static str {
        self.raw_rust_module_path
    }
}
impl MethodFunctionConstDynMetadata for MethodFunctionMetadata {
    fn id_path(&self) -> CloneLazy<MethodFunctionPath> {
        self.id_path.clone()
    }
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Engine, (), fn(ImmutableString, &mut rhai::Engine)> {
        self.registrator
    }
}
