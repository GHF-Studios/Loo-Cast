use rhai::{ImmutableString, Shared};
use std::sync::{Arc, RwLock};

use crate::script::core::internals::functions::{assert_pascal_case_clean_string, assert_snake_case_clean_string};

/// Rhai-safe handle for scoped access. Rhai should never touch this directly.
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
#[repr(transparent)]
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
impl VariantInfo {
    pub fn new(name: impl Into<VariantName>, fields: Vec<impl Into<FieldInfo>>) -> Self {
        VariantInfo {
            name: name.into(),
            field_infos: fields.into_iter().map(|f| f.into()).collect(),
        }
    }
}
impl From<ImmutableString> for VariantInfo {
    fn from(variant_signature: ImmutableString) -> Self {
        let parts: Vec<&str> = variant_signature.splitn(2, '(').collect();

        if parts.len() != 2 || !variant_signature.ends_with(')') {
            panic!("VariantInfo strings must be in the format 'VariantName(field1: Type1, field2: Type2)', got '{}'", variant_signature);
        }

        let name = VariantName::from(ImmutableString::from(parts[0].trim()));
        let fields_str = &parts[1][..parts[1].len() - 1]; // Remove the trailing ')'
        let field_infos: Vec<FieldInfo> = if fields_str.trim().is_empty() {
            Vec::new()
        } else {
            fields_str
                .split(',')
                .map(|s| FieldInfo::from(ImmutableString::from(s.trim())))
                .collect()
        };

        VariantInfo {
            name,
            field_infos,
        }
    }
}
impl From<VariantInfo> for ImmutableString {
    fn from(variant_info: VariantInfo) -> Self {
        let field_signatures: Vec<ImmutableString> = variant_info
            .field_infos
            .into_iter()
            .map(|fi| fi.into())
            .collect();

        ImmutableString::from(format!("{} {{ {} }}", variant_info.name.name, field_signatures.join(", ")))
    }
}
impl std::fmt::Debug for VariantInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant_signature: ImmutableString = self.clone().into();
        write!(f, "{}", variant_signature)
    }
}
impl std::fmt::Display for VariantInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let variant_signature: ImmutableString = self.clone().into();
        write!(f, "{}", variant_signature)
    }
}

/// Not intended to be constructed directly. See TypeInfo
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum TypeDataInfo {
    Struct {
        field_infos: Vec<FieldInfo>,
    },
    Enum {
        variant_infos: Vec<VariantInfo>,
    },
}
impl From<TypeDataInfo> for ImmutableString {
    fn from(type_data_info: TypeDataInfo) -> Self {
        match type_data_info {
            TypeDataInfo::Struct { field_infos } => {
                let field_signatures: Vec<ImmutableString> = field_infos
                    .into_iter()
                    .map(|fi| fi.into())
                    .collect();

                ImmutableString::from(format!(
                    "Struct {{\n\t{}\n}}",
                    field_signatures.join(",\n\t")
                ))
            }
            TypeDataInfo::Enum { variant_infos } => {
                let variant_signatures: Vec<ImmutableString> = variant_infos
                    .into_iter()
                    .map(|vi| vi.into())
                    .collect();

                ImmutableString::from(format!(
                    "Enum {{\n\t{}\n}}",
                    variant_signatures.join(",\n\t")
                ))
            }
        }
    }
}
impl std::fmt::Debug for TypeDataInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_signature: ImmutableString = self.clone().into();
        write!(f, "{}", type_signature)
    }
}
impl std::fmt::Display for TypeDataInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_signature: ImmutableString = self.clone().into();
        write!(f, "{}", type_signature)
    }
}

/// Not intended to be constructed directly. See TypeInfo
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeFormInfo {
    Struct,
    Enum,
}
impl From<TypeFormInfo> for ImmutableString {
    fn from(type_form_info: TypeFormInfo) -> Self {
        match type_form_info {
            TypeFormInfo::Struct => ImmutableString::from("Struct"),
            TypeFormInfo::Enum => ImmutableString::from("Enum"),
        }
    }
}
impl std::fmt::Debug for TypeFormInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_form_signature: ImmutableString = self.clone().into();
        write!(f, "{}", type_form_signature)
    }
}
impl std::fmt::Display for TypeFormInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_form_signature: ImmutableString = self.clone().into();
        write!(f, "{}", type_form_signature)
    }
}

/// Not intended to be constructed directly. See TypeInfo
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypeLayoutInfo {
    pub data_info: TypeDataInfo,
    pub form_info: TypeFormInfo,
}
impl From<TypeLayoutInfo> for ImmutableString {
    fn from(type_layout_info: TypeLayoutInfo) -> Self {
        match type_layout_info.data_info {
            TypeDataInfo::Struct { field_infos } => {
                let field_signatures: Vec<ImmutableString> = field_infos
                    .into_iter()
                    .map(|fi| fi.into())
                    .collect();

                ImmutableString::from(format!(
                    "Struct {{\n\t{}\n}}",
                    field_signatures.join(",\n\t")
                ))
            }
            TypeDataInfo::Enum { variant_infos } => {
                let variant_signatures: Vec<ImmutableString> = variant_infos
                    .into_iter()
                    .map(|vi| vi.into())
                    .collect();

                ImmutableString::from(format!(
                    "Enum {{\n\t{}\n}}",
                    variant_signatures.join(",\n\t")
                ))
            }
        }
    }
}
impl std::fmt::Debug for TypeLayoutInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_layout_signature: ImmutableString = self.clone().into();
        write!(f, "{}", type_layout_signature)
    }
}
impl std::fmt::Display for TypeLayoutInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_layout_signature: ImmutableString = self.clone().into();
        write!(f, "{}", type_layout_signature)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ArgInfo {
    pub name: ArgName,
    pub type_id: TypeId,
}
impl ArgInfo {
    pub fn new(name: impl Into<ArgName>, type_path: impl Into<TypeId>) -> Self {
        ArgInfo {
            name: name.into(),
            type_id: type_path.into(),
        }
    }
}
impl From<ImmutableString> for ArgInfo {
    fn from(arg_signature: ImmutableString) -> Self {
        let parts: Vec<&str> = arg_signature.split(": ").collect();

        ArgInfo {
            name: ArgName::from(ImmutableString::from(parts[0])),
            type_id: TypeId::from(ImmutableString::from(parts[1])),
        }
    }
}
impl From<ArgInfo> for ImmutableString {
    fn from(arg_info: ArgInfo) -> Self {
        let type_path: ImmutableString = arg_info.type_id.into();

        ImmutableString::from(format!("{}: {}", arg_info.name.name, type_path))
    }
}
impl std::fmt::Debug for ArgInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arg_signature: ImmutableString = self.clone().into();
        write!(f, "{}", arg_signature)
    }
}
impl std::fmt::Display for ArgInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arg_signature: ImmutableString = self.clone().into();
        write!(f, "{}", arg_signature)
    }
}

#[derive(Clone, Eq)]
pub struct CtorInfo {
    pub name: CtorName,
    pub arg_infos: Vec<ArgInfo>,
    pub fn_ptr: fn(Vec<rhai::Dynamic>) -> rhai::Dynamic,
}
impl PartialEq for CtorInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arg_infos == other.arg_infos
    }
}
impl std::hash::Hash for CtorInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.arg_infos.hash(state);
    }
}
impl From<CtorInfo> for ImmutableString {
    fn from(ctor_info: CtorInfo) -> Self {
        let arg_signatures: Vec<ImmutableString> = ctor_info
            .arg_infos
            .into_iter()
            .map(|ai| ai.into())
            .collect();

        ImmutableString::from(format!(
            "ctor {}({})",
            ctor_info.name.name,
            arg_signatures.join(", ")
        ))
    }
}
impl std::fmt::Debug for CtorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ctor_signature: ImmutableString = self.clone().into();
        write!(f, "{}", ctor_signature)
    }
}
impl std::fmt::Display for CtorInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ctor_signature: ImmutableString = self.clone().into();
        write!(f, "{}", ctor_signature)
    }
}

#[derive(Clone, Eq)]
pub struct MethodInfo {
    pub name: MethodName,
    pub arg_infos: Vec<ArgInfo>,
    pub return_type_id: TypeId,
    pub fn_ptr: fn(Vec<rhai::Dynamic>) -> rhai::Dynamic,
}
impl PartialEq for MethodInfo {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.arg_infos == other.arg_infos && self.return_type_id == other.return_type_id
    }
}
impl std::hash::Hash for MethodInfo {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.arg_infos.hash(state);
        self.return_type_id.hash(state);
    }
}
impl From<MethodInfo> for ImmutableString {
    fn from(method_info: MethodInfo) -> Self {
        let arg_signatures: Vec<ImmutableString> = method_info
            .arg_infos
            .into_iter()
            .map(|ai| ai.into())
            .collect();
        let return_type_path: ImmutableString = method_info.return_type_id.into();

        ImmutableString::from(format!(
            "fn {}({}) -> {}",
            method_info.name.name,
            arg_signatures.join(", "),
            return_type_path
        ))
    }
}
impl std::fmt::Debug for MethodInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_signature: ImmutableString = self.clone().into();
        write!(f, "{}", method_signature)
    }
}
impl std::fmt::Display for MethodInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_signature: ImmutableString = self.clone().into();
        write!(f, "{}", method_signature)
    }
}

inventory::collect!(TypeInfo);
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypeInfo {
    pub type_id: TypeId,
    pub type_layout_info: TypeLayoutInfo,
    pub ctor_infos: Vec<CtorInfo>,
    pub method_infos: Vec<MethodInfo>,
}
// Full custom pretty-printer with newlines and identation, essentially should look as close to a 1:1 to a C header file, but for rust
impl From<TypeInfo> for ImmutableString {
    fn from(type_info: TypeInfo) -> Self {
        let type_name: ImmutableString = type_info.type_id.type_name.into();
        let module_path: ImmutableString = type_info.type_id.module_id.into();
        let type_form: ImmutableString = type_info.type_layout_info.form_info.into();
        let type_layout_signature: ImmutableString = type_info.type_layout_info.into();

        let ctor_signatures: Vec<ImmutableString> = type_info
            .ctor_infos
            .into_iter()
            .map(|ci| ci.into())
            .collect();

        let method_signatures: Vec<ImmutableString> = type_info
            .method_infos
            .into_iter()
            .map(|mi| mi.into())
            .collect();

        ImmutableString::from(format!(
            "{} {} in '{}' {{\n\t{}\n\n\tconstructors:\n\t\t{}\n\n\tmethods:\n\t\t{}\n}}",
            type_form,
            type_name,
            module_path,
            type_layout_signature.replace("\n", "\n\t"),
            if ctor_signatures.is_empty() {
                String::from("/* none */")
            } else {
                ctor_signatures.join(",\n\t\t")
            },
            if method_signatures.is_empty() {
                String::from("/* none */")
            } else {
                method_signatures.join(",\n\t\t")
            },
        ))
    }
}
impl std::fmt::Debug for TypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_signature: ImmutableString = self.clone().into();
        write!(f, "{}", type_signature)
    }
}
impl std::fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_signature: ImmutableString = self.clone().into();
        write!(f, "{}", type_signature)
    }
}