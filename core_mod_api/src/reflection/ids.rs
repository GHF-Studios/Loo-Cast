use rhai::ImmutableString;
use std::hash::Hash;
use std::marker::PhantomData;

use super::{
    internals::traits::{Trait, GetTypeId},
    names::{ModuleName, TypeName, TraitName}
};

#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ModuleId {
    pub module_path: Vec<ModuleName>
}
impl ModuleId {
    pub fn new(module_path: impl Into<ModuleId>) -> Self {
        module_path.into()
    }
}
impl From<ImmutableString> for ModuleId {
    fn from(module_path: ImmutableString) -> Self {
        if module_path.is_empty() {
            panic!("ModuleId strings must not be empty");
        }

        let segments: Vec<ModuleName> = module_path
            .split("::")
            .map(|s| ModuleName::new(ImmutableString::from(s)))
            .collect();

        if segments.is_empty() {
            panic!("ModuleId must have at least one segment, got '{}'", module_path);
        }

        ModuleId {
            module_path: segments,
        }
    }
}
impl From<ModuleId> for ImmutableString {
    fn from(module_id: ModuleId) -> Self {
        let segments: Vec<ImmutableString> = module_id
            .module_path
            .into_iter()
            .map(|mn| mn.name)
            .collect();

        ImmutableString::from(segments.join("::"))
    }
}
impl std::fmt::Debug for ModuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let module_path: ImmutableString = self.clone().into();
        write!(f, "{}", module_path)
    }
}
impl std::fmt::Display for ModuleId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let module_path: ImmutableString = self.clone().into();
        write!(f, "{}", module_path)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypeId {
    pub module_id: ModuleId,
    pub type_name: TypeName,
}
impl TypeId {
    pub fn new(type_path: impl Into<TypeId>) -> Self {
        type_path.into()
    }

    pub fn of<T: GetTypeId>() -> Self {
        Self::new(ImmutableString::from(<T as GetTypeId>::TYPE_ID))
    }
}
impl From<ImmutableString> for TypeId {
    fn from(full_path: ImmutableString) -> Self {
        if full_path.is_empty() {
            panic!("TypeId strings must not be empty");
        }

        let parts: Vec<&str> = full_path.rsplitn(2, "::").collect();
        if parts.len() != 2 {
            panic!("TypeId strings must be in the format 'module_id::TypeName', got '{}'", full_path);
        }

        let type_name = TypeName::new(ImmutableString::from(parts[0]));
        let module_id = ModuleId::from(ImmutableString::from(parts[1]));

        TypeId {
            module_id,
            type_name,
        }
    }
}
impl From<TypeId> for ImmutableString {
    fn from(type_id: TypeId) -> Self {
        let module_path: ImmutableString = type_id.module_id.into();
        let type_name: ImmutableString = type_id.type_name.into();

        ImmutableString::from(format!("{}::{}", module_path, type_name))
    }
}
impl std::fmt::Debug for TypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let full_path: ImmutableString = self.clone().into();
        write!(f, "{}", full_path)
    }
}
impl std::fmt::Display for TypeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let full_path: ImmutableString = self.clone().into();
        write!(f, "{}", full_path)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StaticTraitId<T: Trait> {
    pub id: DynamicTraitId,
    pub _phantom: PhantomData<T>,
}
impl<T: Trait> StaticTraitId<T> {
    pub fn new() -> Self {
        Self {
            id: DynamicTraitId::new(ImmutableString::from(T::TRAIT_ID)),
            _phantom: PhantomData,
        }
    }
}
impl<T: Trait> std::fmt::Debug for StaticTraitId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.id)
    }
}
impl<T: Trait> std::fmt::Display for StaticTraitId<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct DynamicTraitId {
    pub module_id: ModuleId,
    pub trait_name: TraitName,
}
impl DynamicTraitId {
    pub fn new(trait_path: impl Into<DynamicTraitId>) -> Self {
        trait_path.into()
    }
}
impl From<ImmutableString> for DynamicTraitId {
    fn from(full_path: ImmutableString) -> Self {
        if full_path.is_empty() {
            panic!("TraitId strings must not be empty");
        }

        let parts: Vec<&str> = full_path.rsplitn(2, "::").collect();
        if parts.len() != 2 {
            panic!("TraitId strings must be in the format 'module_id::TraitName', got '{}'", full_path);
        }

        let trait_name = TraitName::new(ImmutableString::from(parts[0]));
        let module_id = ModuleId::from(ImmutableString::from(parts[1]));

        DynamicTraitId {
            module_id,
            trait_name,
        }
    }
}
impl From<DynamicTraitId> for ImmutableString {
    fn from(trait_id: DynamicTraitId) -> Self {
        let module_path: ImmutableString = trait_id.module_id.into();
        let trait_name: ImmutableString = trait_id.trait_name.into();

        ImmutableString::from(format!("{}::{}", module_path, trait_name))
    }
}
impl std::fmt::Debug for DynamicTraitId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let full_path: ImmutableString = self.clone().into();
        write!(f, "{}", full_path)
    }
}
impl std::fmt::Display for DynamicTraitId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let full_path: ImmutableString = self.clone().into();
        write!(f, "{}", full_path)
    }
}