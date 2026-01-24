use heck::ToPascalCase;
use rhai::{ImmutableString, Shared};
use std::sync::{Arc, RwLock};

use crate::script::core::internals::traits::IntoTypeName;

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
    pub fn new(name: impl IntoTypeName) -> Self {
        name.into_type_name()
    }
}
impl From<ImmutableString> for TypeName {
    fn from(name: ImmutableString) -> Self {
        if name.is_empty() {
            panic!("TypeName strings must not be empty");
        }
        if name.chars().any(|c| c.is_whitespace()) {
            panic!("TypeName strings must not contain whitespace, found '{}'", name);
        }
        if name.chars().any(|c| !c.is_ascii_alphanumeric()) {
            panic!("TypeName strings must be alphanumeric ASCII, found '{}'", name);
        }
        if name.chars().next().unwrap().is_ascii_digit() {
            panic!("TypeName strings must not start with a digit, found '{}'", name);
        }
        if name != name.to_pascal_case() {
            panic!("Type names must be in PascalCase format, found '{}'", name);
        }

        TypeName {
            name,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ModuleName {
    pub name: ImmutableString
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ModuleId {
    pub segments: Vec<ModuleName>
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ModulePath {
    pub segments: Vec<ImmutableString>
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeKind {
    Struct,
    Enum,
}

/// A type identifier, consisting of a module path and a type name.
/// # Limitations
/// 
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypeId {
    pub module_path: ModulePath,
    pub type_name: TypeName,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FieldInfo {
    pub name: ImmutableString,
    pub type_id: TypeId,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct VariantInfo {
    pub name: ImmutableString,
    pub field_infos: Vec<FieldInfo>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TypeDataLayout {
    Struct {
        field_infos: Vec<FieldInfo>,
    },
    Enum {
        variant_infos: Vec<VariantInfo>,
    },
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypeShape {
    pub kind: TypeKind,
    pub inner: TypeDataLayout,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ArgInfo {
    pub name: ImmutableString,
    pub type_id: TypeId,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CtorInfo {
    pub name: ImmutableString,
    pub arg_infos: Vec<ArgInfo>,
    pub fn_ptr: fn(Vec<rhai::Dynamic>) -> rhai::Dynamic,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MethodInfo {
    pub name: ImmutableString,
    pub arg_infos: Vec<ArgInfo>,
    pub return_type_id: TypeId,
    pub fn_ptr: fn(Vec<rhai::Dynamic>) -> rhai::Dynamic,
}

inventory::collect!(TypeInfo);
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypeInfo {
    pub type_id: TypeId,
    pub type_shape: TypeShape,
    pub ctor_infos: Vec<CtorInfo>,
    pub method_infos: Vec<MethodInfo>,
}