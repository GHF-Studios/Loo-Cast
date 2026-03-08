use rhai::ImmutableString;

use crate::{
    rhai_binding::{
        meta::{
            generic::abstract_primitive::ConstDynMetadata,
            monomorphized::function::{ConstructorFunctionMetadata, ItemAssociatedFunctionMetadata, MethodFunctionMetadata, ModuleAssociatedFunctionMetadata},
        },
        path::function_path::*,
    },
    utils::{clone_closure::CloneClosure, clone_lazy::CloneLazy},
};

pub const trait ModuleAssociatedFunctionConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<ModuleAssociatedFunctionPath>;
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)>;
}
pub const trait ItemAssociatedFunctionConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<ItemAssociatedFunctionPath>;
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)>;
}
pub const trait ConstructorFunctionConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<ConstructorFunctionPath>;
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)>;
}
pub const trait MethodFunctionConstDynMetadata: ConstDynMetadata {
    fn id_path(&self) -> CloneLazy<MethodFunctionPath>;
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Engine, (), fn(ImmutableString, &mut rhai::Engine)>;
}

pub trait ModuleAssociatedFunctionDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: ModuleAssociatedFunctionConstDynMetadata>(&self, const_dyn_metadata: &T) -> ModuleAssociatedFunctionMetadata {
        ModuleAssociatedFunctionMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path(),
            registrator: const_dyn_metadata.clone().registrator(),
        }
    }
}
pub trait ItemAssociatedFunctionDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: ItemAssociatedFunctionConstDynMetadata>(&self, const_dyn_metadata: &T) -> ItemAssociatedFunctionMetadata {
        ItemAssociatedFunctionMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path(),
            registrator: const_dyn_metadata.clone().registrator(),
        }
    }
}
pub trait ConstructorFunctionDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: ConstructorFunctionConstDynMetadata>(&self, const_dyn_metadata: &T) -> ConstructorFunctionMetadata {
        ConstructorFunctionMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path(),
            registrator: const_dyn_metadata.clone().registrator(),
        }
    }
}
pub trait MethodFunctionDynamicTypedMetadata {
    fn from_comptime_to_runtime<T: MethodFunctionConstDynMetadata>(&self, const_dyn_metadata: &T) -> MethodFunctionMetadata {
        MethodFunctionMetadata {
            raw_rust_module_path: const_dyn_metadata.raw_rust_module_path(),
            id_path: const_dyn_metadata.id_path(),
            registrator: const_dyn_metadata.clone().registrator(),
        }
    }
}
