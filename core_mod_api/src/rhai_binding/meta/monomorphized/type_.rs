use rhai::ImmutableString;

use crate::{rhai_binding::{meta::generic::{abstract_primitive::ConstDynMetadata, type_::TypeConstDynMetadata}, path::{function_path::MethodFunctionPath, type_path::TypePath}, value_semantics::modes::TypeValueSemantics}, utils::{clone_closure::CloneClosure, clone_lazy::CloneLazy}};

#[derive(Clone)]
pub struct TypeMetadata {
    /// Primary means of identification
    pub id_path: CloneLazy<TypePath>,
    /// Raw `module_path!()` output to verify physical locations relatively (this is NOT a *globally* unique ID)
    pub raw_rust_module_path: &'static str,
    pub registrator: CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)>,

    pub method_functions: CloneLazy<Vec<MethodFunctionPath>>,
    pub value_semantics: CloneLazy<TypeValueSemantics>,
}
impl ConstDynMetadata for TypeMetadata {
    fn raw_rust_module_path(&self) -> &'static str { self.raw_rust_module_path }
}
impl TypeConstDynMetadata for TypeMetadata {
    fn id_path(&self) -> CloneLazy<TypePath> { self.id_path.clone() }
    fn registrator(self) -> CloneClosure<ImmutableString, &'static mut rhai::Module, (), fn(ImmutableString, &mut rhai::Module)> { self.registrator }
    
    fn method_functions(&self) -> CloneLazy<Vec<MethodFunctionPath>> { self.method_functions.clone() }
    fn value_semantics(&self) -> CloneLazy<TypeValueSemantics> { self.value_semantics.clone() }
}
