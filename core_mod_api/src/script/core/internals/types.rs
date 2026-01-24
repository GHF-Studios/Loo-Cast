use rhai::{ImmutableString, Shared};
use std::sync::{Arc, RwLock};

use crate::script::core::internals::functions::{assert_pascal_case_clean_string, assert_snake_case_clean_string};

/// Rhai-safe handle for scoped access. Rhai should never touch this directly.
pub struct ScopedAccess<T> {
    value: Option<T>,
}

pub type ScopedAccessHandle<T> = Shared<RwLock<ScopedAccess<T>>>;

impl<T> ScopedAccess<T> {
    /// Creates a new ScopedAccess wrapping the given value.
    pub fn new(value: T) -> Self {
        Self {
            value: Some(value),
        }
    }

    /// Internal use only. Grants read-only access to the value via a closure.
    pub fn read<R>(&self, f: impl FnOnce(&T) -> R) -> Result<R, &'static str> {
        if self.value.is_none() {
            return Err("Handle has been invalidated");
        }

        match self.value.as_ref() {
            Some(val) => Ok(f(val)),
            None => Err("Value has already been taken"),
        }
    }

    /// Internal use only. Grants mutable access to the value via a closure.
    pub fn write<R>(&mut self, f: impl FnOnce(&mut T) -> R) -> Result<R, &'static str> {
        if self.value.is_none() {
            return Err("Handle has been invalidated");
        }

        match self.value.as_mut() {
            Some(val) => Ok(f(val)),
            None => Err("Value has already been taken"),
        }
    }

    /// Invalidates the handle and extracts the value, for return to Rust-side.
    pub fn invalidate(&mut self) -> Result<T, &'static str> {
        if self.value.is_none() {
            return Err("Handle has already been invalidated");
        }

        self.value.take().ok_or("Value has already been taken")
    }

    /// Checks if the access is still valid.
    pub fn is_valid(&self) -> bool {
        self.value.is_some()
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypeName {
    pub name: ImmutableString
}
impl TypeName {
    pub fn new(name: impl Into<TypeName>) -> Self {
        name.into()
    }
}
impl From<ImmutableString> for TypeName {
    fn from(name: ImmutableString) -> Self {
        assert_pascal_case_clean_string(&name, "TypeName");

        TypeName {
            name,
        }
    }
}
impl From<TypeName> for ImmutableString {
    fn from(type_name: TypeName) -> Self {
        type_name.name
    }
}
impl std::fmt::Debug for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl std::fmt::Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ModuleName {
    pub name: ImmutableString
}
impl ModuleName {
    pub fn new(name: impl Into<ModuleName>) -> Self {
        name.into()
    }
}
impl From<ImmutableString> for ModuleName {
    fn from(name: ImmutableString) -> Self {
        assert_snake_case_clean_string(&name, "ModuleName");

        ModuleName {
            name,
        }
    }
}
impl From<ModuleName> for ImmutableString {
    fn from(module_name: ModuleName) -> Self {
        module_name.name
    }
}
impl std::fmt::Debug for ModuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl std::fmt::Display for ModuleName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
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
pub struct FieldName {
    pub name: ImmutableString,
}
impl FieldName {
    pub fn new(name: impl Into<FieldName>) -> Self {
        name.into()
    }
}
impl From<ImmutableString> for FieldName {
    fn from(name: ImmutableString) -> Self {
        assert_snake_case_clean_string(&name, "FieldName");

        FieldName {
            name,
        }
    }
}
impl From<FieldName> for ImmutableString {
    fn from(field_name: FieldName) -> Self {
        field_name.name
    }
}
impl std::fmt::Debug for FieldName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl std::fmt::Display for FieldName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VariantName {
    pub name: ImmutableString,
}
impl VariantName {
    pub fn new(name: impl Into<VariantName>) -> Self {
        name.into()
    }
}
impl From<ImmutableString> for VariantName {
    fn from(name: ImmutableString) -> Self {
        assert_pascal_case_clean_string(&name, "VariantName");

        VariantName {
            name,
        }
    }
}
impl From<VariantName> for ImmutableString {
    fn from(variant_name: VariantName) -> Self {
        variant_name.name
    }
}
impl std::fmt::Debug for VariantName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl std::fmt::Display for VariantName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ArgName {
    pub name: ImmutableString,
}
impl ArgName {
    pub fn new(name: impl Into<ArgName>) -> Self {
        name.into()
    }
}
impl From<ImmutableString> for ArgName {
    fn from(name: ImmutableString) -> Self {
        assert_snake_case_clean_string(&name, "ArgName");

        ArgName {
            name,
        }
    }
}
impl From<ArgName> for ImmutableString {
    fn from(arg_name: ArgName) -> Self {
        arg_name.name
    }
}
impl std::fmt::Debug for ArgName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl std::fmt::Display for ArgName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CtorName {
    pub name: ImmutableString,
}
impl CtorName {
    pub fn new(name: impl Into<CtorName>) -> Self {
        name.into()
    }
}
impl From<ImmutableString> for CtorName {
    fn from(name: ImmutableString) -> Self {
        assert_snake_case_clean_string(&name, "CtorName");

        CtorName {
            name,
        }
    }
}
impl From<CtorName> for ImmutableString {
    fn from(ctor_name: CtorName) -> Self {
        ctor_name.name
    }
}
impl std::fmt::Debug for CtorName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl std::fmt::Display for CtorName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MethodName {
    pub name: ImmutableString,
}
impl MethodName {
    pub fn new(name: impl Into<MethodName>) -> Self {
        name.into()
    }
}
impl From<ImmutableString> for MethodName {
    fn from(name: ImmutableString) -> Self {
        assert_snake_case_clean_string(&name, "MethodName");

        MethodName {
            name,
        }
    }
}
impl From<MethodName> for ImmutableString {
    fn from(method_name: MethodName) -> Self {
        method_name.name
    }
}
impl std::fmt::Debug for MethodName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl std::fmt::Display for MethodName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FieldInfo {
    pub name: FieldName,
    pub type_id: TypeId,
}
impl FieldInfo {
    pub fn new(name: impl Into<FieldName>, type_path: impl Into<TypeId>) -> Self {
        FieldInfo {
            name: name.into(),
            type_id: type_path.into(),
        }
    }
}
impl From<ImmutableString> for FieldInfo {
    fn from(field_signature: ImmutableString) -> Self {
        let parts: Vec<&str> = field_signature.split(": ").collect();

        FieldInfo {
            name: FieldName::from(ImmutableString::from(parts[0])),
            type_id: TypeId::from(ImmutableString::from(parts[1])),
        }
    }
}
impl From<FieldInfo> for ImmutableString {
    fn from(field_info: FieldInfo) -> Self {
        let type_path: ImmutableString = field_info.type_id.into();

        ImmutableString::from(format!("{}: {}", field_info.name.name, type_path))
    }
}
impl std::fmt::Debug for FieldInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let field_signature: ImmutableString = self.clone().into();
        write!(f, "{}", field_signature)
    }
}
impl std::fmt::Display for FieldInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let field_signature: ImmutableString = self.clone().into();
        write!(f, "{}", field_signature)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VariantInfo {
    pub name: VariantName,
    pub field_infos: Vec<FieldInfo>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TypeDataInfo {
    Struct {
        field_infos: Vec<FieldInfo>,
    },
    Enum {
        variant_infos: Vec<VariantInfo>,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeKindInfo {
    Struct,
    Enum,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypeLayoutInfo {
    pub inner: TypeDataInfo,
    pub kind: TypeKindInfo,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ArgInfo {
    pub name: ArgName,
    pub type_id: TypeId,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CtorInfo {
    pub name: CtorName,
    pub arg_infos: Vec<ArgInfo>,
    pub fn_ptr: fn(Vec<rhai::Dynamic>) -> rhai::Dynamic,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MethodInfo {
    pub name: MethodName,
    pub arg_infos: Vec<ArgInfo>,
    pub return_type_id: TypeId,
    pub fn_ptr: fn(Vec<rhai::Dynamic>) -> rhai::Dynamic,
}

inventory::collect!(TypeInfo);
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypeInfo {
    pub type_id: TypeId,
    pub type_shape: TypeLayoutInfo,
    pub ctor_infos: Vec<CtorInfo>,
    pub method_infos: Vec<MethodInfo>,
}