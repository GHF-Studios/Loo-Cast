use rhai::Dynamic;

use super::function_ids::{CtorId, MethodId, StaticFunctionId};

inventory::collect!(CtorRegistryEntry);
#[derive(Clone)]
pub struct CtorRegistryEntry {
    pub id: CtorId,
    pub fn_ptr: fn(Vec<Dynamic>) -> Dynamic,
}
impl PartialEq for CtorRegistryEntry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for CtorRegistryEntry {}
impl std::hash::Hash for CtorRegistryEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

inventory::collect!(MethodRegistryEntry);
#[derive(Clone)]
pub struct MethodRegistryEntry {
    pub id: MethodId,
    pub fn_ptr: fn(Vec<Dynamic>) -> Dynamic,
}
impl PartialEq for MethodRegistryEntry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for MethodRegistryEntry {}
impl std::hash::Hash for MethodRegistryEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

inventory::collect!(StaticFunctionRegistryEntry);
#[derive(Clone)]
pub struct StaticFunctionRegistryEntry {
    pub id: StaticFunctionId,
    pub fn_ptr: fn(Vec<Dynamic>) -> Dynamic,
}
impl PartialEq for StaticFunctionRegistryEntry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for StaticFunctionRegistryEntry {}
impl std::hash::Hash for StaticFunctionRegistryEntry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
