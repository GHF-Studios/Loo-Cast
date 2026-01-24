use rhai::{ImmutableString, Shared};
use std::sync::{Arc, RwLock};

use crate::script::core::internals::functions::{assert_pascal_case_clean_string, assert_snake_case_clean_string};

pub type ScopedAccessHandle<T> = Shared<RwLock<ScopedAccess<T>>>;

/// Rhai-safe handle for scoped access. Rhai should never touch this directly.
#[repr(transparent)]
pub struct ScopedAccess<T> {
    value: Option<T>,
}
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
pub struct TraitName {
    pub name: ImmutableString
}
impl TraitName {
    pub fn new(name: impl Into<TraitName>) -> Self {
        name.into()
    }
}
impl From<ImmutableString> for TraitName {
    fn from(name: ImmutableString) -> Self {
        assert_pascal_case_clean_string(&name, "TraitName");

        TraitName {
            name,
        }
    }
}
impl From<TraitName> for ImmutableString {
    fn from(trait_name: TraitName) -> Self {
        trait_name.name
    }
}
impl std::fmt::Debug for TraitName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl std::fmt::Display for TraitName {
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
pub struct TraitId {
    pub module_id: ModuleId,
    pub trait_name: TraitName,
}
impl TraitId {
    pub fn new(trait_path: impl Into<TraitId>) -> Self {
        trait_path.into()
    }
}
impl From<ImmutableString> for TraitId {
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

        TraitId {
            module_id,
            trait_name,
        }
    }
}
impl From<TraitId> for ImmutableString {
    fn from(trait_id: TraitId) -> Self {
        let module_path: ImmutableString = trait_id.module_id.into();
        let trait_name: ImmutableString = trait_id.trait_name.into();

        ImmutableString::from(format!("{}::{}", module_path, trait_name))
    }
}
impl std::fmt::Debug for TraitId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let full_path: ImmutableString = self.clone().into();
        write!(f, "{}", full_path)
    }
}
impl std::fmt::Display for TraitId {
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
#[repr(transparent)]
pub struct StaticFunctionName {
    pub name: ImmutableString,
}
impl StaticFunctionName {
    pub fn new(name: impl Into<StaticFunctionName>) -> Self {
        name.into()
    }
}
impl From<ImmutableString> for StaticFunctionName {
    fn from(name: ImmutableString) -> Self {
        assert_snake_case_clean_string(&name, "StaticFunctionName");

        StaticFunctionName {
            name,
        }
    }
}
impl From<StaticFunctionName> for ImmutableString {
    fn from(static_function_name: StaticFunctionName) -> Self {
        static_function_name.name
    }
}
impl std::fmt::Debug for StaticFunctionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl std::fmt::Display for StaticFunctionName {
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

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum FunctionOrigin {
    Inherent,
    ViaTrait { trait_id: TraitId },
}
impl From<FunctionOrigin> for ImmutableString {
    fn from(function_origin: FunctionOrigin) -> Self {
        match function_origin {
            FunctionOrigin::Inherent => ImmutableString::new(),
            FunctionOrigin::ViaTrait { trait_id } => {
                let trait_path: ImmutableString = trait_id.into();
                ImmutableString::from(format!(" via {}", trait_path))
            }
        }
    }
}
impl std::fmt::Debug for FunctionOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let origin_signature: ImmutableString = self.clone().into();
        write!(f, "{}", origin_signature)
    }
}
impl std::fmt::Display for FunctionOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let origin_signature: ImmutableString = self.clone().into();
        write!(f, "{}", origin_signature)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CtorSignature {
    pub name: CtorName,
    pub arg_infos: Vec<ArgInfo>,
}
impl From<ImmutableString> for CtorSignature {
    fn from(src: ImmutableString) -> Self {
        let src = src.trim();
        let open_paren = src.find('(').expect("missing '(' in constructor signature");
        let close_paren = src.rfind(')').expect("missing ')' in constructor signature");

        let name = &src[..open_paren].trim();
        let args = &src[open_paren + 1..close_paren];

        let arg_infos = if args.is_empty() {
            Vec::new()
        } else {
            args.split(',')
                .map(|arg| ArgInfo::from(ImmutableString::from(arg.trim())))
                .collect()
        };

        CtorSignature {
            name: CtorName::from(ImmutableString::from(*name)),
            arg_infos,
        }
    }
}

impl From<CtorSignature> for ImmutableString {
    fn from(ctor_sig: CtorSignature) -> Self {
        let arg_signatures: Vec<ImmutableString> = ctor_sig
            .arg_infos
            .into_iter()
            .map(|ai| ai.into())
            .collect();

        ImmutableString::from(format!(
            "ctor {}({})",
            ctor_sig.name.name,
            arg_signatures.join(", ")
        ))
    }
}
impl std::fmt::Debug for CtorSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ctor_signature: ImmutableString = self.clone().into();
        write!(f, "{}", ctor_signature)
    }
}
impl std::fmt::Display for CtorSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ctor_signature: ImmutableString = self.clone().into();
        write!(f, "{}", ctor_signature)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MethodSignature {
    pub name: MethodName,
    pub arg_infos: Vec<ArgInfo>,
    pub return_type_id: TypeId,
}
impl From<ImmutableString> for MethodSignature {
    fn from(src: ImmutableString) -> Self {
        let src = src.trim();

        let parts: Vec<&str> = src.split("->").map(str::trim).collect();
        if parts.len() != 2 {
            panic!("method signature must contain '->'");
        }

        let decl_part = parts[0];
        let return_type_str = parts[1];

        let open_paren = decl_part.find('(').expect("missing '(' in method signature");
        let close_paren = decl_part.rfind(')').expect("missing ')' in method signature");

        let name = &decl_part[..open_paren].trim();
        let args = &decl_part[open_paren + 1..close_paren];

        let arg_infos = if args.is_empty() {
            Vec::new()
        } else {
            args.split(',')
                .map(|arg| ArgInfo::from(ImmutableString::from(arg.trim())))
                .collect()
        };

        MethodSignature {
            name: MethodName::from(ImmutableString::from(*name)),
            arg_infos,
            return_type_id: TypeId::from(ImmutableString::from(return_type_str)),
        }
    }
}
impl From<MethodSignature> for ImmutableString {
    fn from(method_sig: MethodSignature) -> Self {
        let arg_signatures: Vec<ImmutableString> = method_sig
            .arg_infos
            .into_iter()
            .map(|ai| ai.into())
            .collect();
        let return_type_path: ImmutableString = method_sig.return_type_id.into();

        ImmutableString::from(format!(
            "fn {}({}) -> {}",
            method_sig.name.name,
            arg_signatures.join(", "),
            return_type_path
        ))
    }
}
impl std::fmt::Debug for MethodSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_signature: ImmutableString = self.clone().into();
        write!(f, "{}", method_signature)
    }
}
impl std::fmt::Display for MethodSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_signature: ImmutableString = self.clone().into();
        write!(f, "{}", method_signature)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StaticFunctionSignature {
    pub name: StaticFunctionName,
    pub arg_infos: Vec<ArgInfo>,
    pub return_type_id: TypeId,
}
impl From<ImmutableString> for StaticFunctionSignature {
    fn from(src: ImmutableString) -> Self {
        let src = src.trim();

        let parts: Vec<&str> = src.split("->").map(str::trim).collect();
        if parts.len() != 2 {
            panic!("static function signature must contain '->'");
        }

        let decl_part = parts[0];
        let return_type_str = parts[1];

        let open_paren = decl_part.find('(').expect("missing '(' in static fn signature");
        let close_paren = decl_part.rfind(')').expect("missing ')' in static fn signature");

        let name = &decl_part[..open_paren].trim();
        let args = &decl_part[open_paren + 1..close_paren];

        let arg_infos = if args.is_empty() {
            Vec::new()
        } else {
            args.split(',')
                .map(|arg| ArgInfo::from(ImmutableString::from(arg.trim())))
                .collect()
        };

        StaticFunctionSignature {
            name: StaticFunctionName::from(ImmutableString::from(*name)),
            arg_infos,
            return_type_id: TypeId::from(ImmutableString::from(return_type_str)),
        }
    }
}
impl From<StaticFunctionSignature> for ImmutableString {
    fn from(static_fn_sig: StaticFunctionSignature) -> Self {
        let arg_signatures: Vec<ImmutableString> = static_fn_sig
            .arg_infos
            .into_iter()
            .map(|ai| ai.into())
            .collect();
        let return_type_path: ImmutableString = static_fn_sig.return_type_id.into();

        ImmutableString::from(format!(
            "fn {}({}) -> {}",
            static_fn_sig.name.name,
            arg_signatures.join(", "),
            return_type_path
        ))
    }
}
impl std::fmt::Debug for StaticFunctionSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fn_signature: ImmutableString = self.clone().into();
        write!(f, "{}", fn_signature)
    }
}
impl std::fmt::Display for StaticFunctionSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fn_signature: ImmutableString = self.clone().into();
        write!(f, "{}", fn_signature)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CtorId {
    pub sig: CtorSignature,
    pub origin: FunctionOrigin,
}
impl From<CtorId> for ImmutableString {
    fn from(id: CtorId) -> Self {
        let sig: ImmutableString = id.sig.into();
        let origin: ImmutableString = id.origin.clone().into();

        ImmutableString::from(format!(
            "{}{}",
            sig,
            origin
        ))
    }
}
impl std::fmt::Debug for CtorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ctor_signature: ImmutableString = self.clone().into();
        write!(f, "{}", ctor_signature)
    }
}
impl std::fmt::Display for CtorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ctor_signature: ImmutableString = self.clone().into();
        write!(f, "{}", ctor_signature)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MethodId {
    pub sig: MethodSignature,
    pub origin: FunctionOrigin,
}
impl From<MethodId> for ImmutableString {
    fn from(id: MethodId) -> Self {
        let sig: ImmutableString = id.sig.into();
        let origin: ImmutableString = id.origin.clone().into();

        ImmutableString::from(format!(
            "{}{}",
            sig,
            origin
        ))
    }
}
impl std::fmt::Debug for MethodId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_signature: ImmutableString = self.clone().into();
        write!(f, "{}", method_signature)
    }
}
impl std::fmt::Display for MethodId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_signature: ImmutableString = self.clone().into();
        write!(f, "{}", method_signature)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StaticFunctionId {
    pub sig: StaticFunctionSignature,
    pub origin: FunctionOrigin,
}
impl From<StaticFunctionId> for ImmutableString {
    fn from(id: StaticFunctionId) -> Self {
        let sig: ImmutableString = id.sig.into();
        let origin: ImmutableString = id.origin.clone().into();

        ImmutableString::from(format!(
            "{}{}",
            sig,
            origin
        ))
    }
}
impl std::fmt::Debug for StaticFunctionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fn_signature: ImmutableString = self.clone().into();
        write!(f, "{}", fn_signature)
    }
}
impl std::fmt::Display for StaticFunctionId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fn_signature: ImmutableString = self.clone().into();
        write!(f, "{}", fn_signature)
    }
}

inventory::collect!(TypeInfo);
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypeInfo {
    pub type_id: TypeId,
    pub type_layout_info: TypeLayoutInfo,
    pub ctor_ids: Vec<CtorId>,
    pub method_ids: Vec<MethodId>,
    pub static_function_ids: Vec<StaticFunctionId>,
}
impl From<ImmutableString> for TypeInfo {
    fn from(input: ImmutableString) -> Self {
        fn parse_module_line(line: &str) -> &str {
            if !line.starts_with("#[module =") {
                panic!("expected #[module = \"...\"]");
            }
            line.split('"').nth(1).expect("invalid module syntax")
        }

        fn parse_type_declaration<'a>(
            line: &'a str,
            lines: &mut impl Iterator<Item = &'a str>,
        ) -> (TypeFormInfo, &'a str, Vec<&'a str>) {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.len() < 2 {
                panic!("invalid type declaration");
            }

            let kind = tokens[0];
            let name = tokens[1];

            let mut body = Vec::new();
            for line in lines {
                if line == "}" {
                    break;
                }
                body.push(line);
            }

            match kind {
                "struct" => (TypeFormInfo::Struct, name, body),
                "enum" => (TypeFormInfo::Enum, name, body),
                _ => panic!("unknown type form: {}", kind),
            }
        }

        fn parse_fields(lines: Vec<&str>) -> Vec<FieldInfo> {
            lines
                .into_iter()
                .map(|line| {
                    let line = line.trim_end_matches(',');
                    FieldInfo::from(ImmutableString::from(line))
                })
                .collect()
        }

        fn parse_variants(lines: Vec<&str>) -> Vec<VariantInfo> {
            lines
                .into_iter()
                .map(|line| {
                    let line = line.trim_end_matches(',');
                    VariantInfo::from(ImmutableString::from(line))
                })
                .collect()
        }

        fn parse_ctor_id(line: &str, type_id: &TypeId) -> CtorId {
            let line = line.strip_prefix("ctor ").unwrap();
            let (sig, origin) = split_origin(line);
            CtorId {
                sig: CtorSignature::from(ImmutableString::from(sig)),
                origin,
            }
        }

        fn parse_method_id(line: &str, type_id: &TypeId) -> MethodId {
            let line = line.strip_prefix("fn ").unwrap();
            let (sig, origin) = split_origin(line);
            MethodId {
                sig: MethodSignature::from(ImmutableString::from(sig)),
                origin,
            }
        }

        fn parse_static_fn_id(line: &str, type_id: &TypeId) -> StaticFunctionId {
            let line = line.strip_prefix("static fn ").unwrap();
            let (sig, origin) = split_origin(line);
            StaticFunctionId {
                sig: StaticFunctionSignature::from(ImmutableString::from(sig)),
                origin,
            }
        }

        fn split_origin(entry: &str) -> (&str, FunctionOrigin) {
            if let Some((sig, trait_part)) = entry.split_once(" via ") {
                let trait_id = TraitId::from(ImmutableString::from(trait_part.trim()));
                (sig.trim(), FunctionOrigin::ViaTrait { trait_id })
            } else {
                (entry.trim(), FunctionOrigin::Inherent)
            }
        }

        let input = input.as_str().replace("\r\n", "\n");
        let mut lines = input.lines().map(str::trim).peekable();

        let module_line = lines
            .next()
            .expect("expected #[module = \"...\"] line");

        let module_path = parse_module_line(module_line);
        let module_id = ModuleId::from(ImmutableString::from(module_path));

        let decl_line = lines
            .next()
            .expect("expected type declaration line");

        let (form_info, type_name, data_lines) = parse_type_declaration(decl_line, &mut lines);
        let type_id = TypeId::from(ImmutableString::from(format!("{}::{}", module_path, type_name)));

        let data_info = match form_info {
            TypeFormInfo::Struct => {
                let fields = parse_fields(data_lines);
                TypeDataInfo::Struct { field_infos: fields }
            }
            TypeFormInfo::Enum => {
                let variants = parse_variants(data_lines);
                TypeDataInfo::Enum { variant_infos: variants }
            }
        };

        let mut ctor_ids = Vec::new();
        let mut method_ids = Vec::new();
        let mut static_function_ids = Vec::new();

        while let Some(line) = lines.next() {
            if line.starts_with("impl ") {
                // Consume "impl TypeName {"
                assert!(line.ends_with("{"));

                while let Some(entry) = lines.next() {
                    let entry = entry.trim_end_matches(';').trim();
                    if entry == "}" {
                        break;
                    }

                    if entry.starts_with("ctor ") {
                        ctor_ids.push(parse_ctor_id(entry, &type_id));
                    } else if entry.starts_with("fn ") {
                        method_ids.push(parse_method_id(entry, &type_id));
                    } else if entry.starts_with("static fn ") {
                        static_function_ids.push(parse_static_fn_id(entry, &type_id));
                    } else {
                        panic!("unknown impl entry: {}", entry);
                    }
                }
            }
        }

        TypeInfo {
            type_id,
            type_layout_info: TypeLayoutInfo {
                data_info,
                form_info,
            },
            ctor_ids,
            method_ids,
            static_function_ids,
        }
    }
}
impl From<TypeInfo> for ImmutableString {
    fn from(type_info: TypeInfo) -> Self {
        let type_name: ImmutableString = type_info.type_id.type_name.clone().into();
        let module_path: ImmutableString = type_info.type_id.module_id.clone().into();

        let header = format!(
            "#[module = \"{}\"]",
            module_path
        );

        let layout = match type_info.type_layout_info.data_info {
            TypeDataInfo::Struct { field_infos } => {
                let fields = field_infos
                    .into_iter()
                    .map(|f| format!("    {},", ImmutableString::from(f)))
                    .collect::<Vec<_>>()
                    .join("\n");
            
                format!(
                    "struct {} {{\n{}\n}}",
                    type_name, fields
                )
            }
            TypeDataInfo::Enum { variant_infos } => {
                let variants = variant_infos
                    .into_iter()
                    .map(|v| format!("    {},", ImmutableString::from(v)))
                    .collect::<Vec<_>>()
                    .join("\n");
            
                format!(
                    "enum {} {{\n{}\n}}",
                    type_name, variants
                )
            }
        };

        let impl_block = if type_info.ctor_ids.is_empty() && type_info.method_ids.is_empty() && type_info.static_function_ids.is_empty() {
            String::new()
        } else {
            let ctor_lines = type_info.ctor_ids.into_iter().map(|c| {
                format!("    ctor {};", ImmutableString::from(c))
            });
        
            let method_lines = type_info.method_ids.into_iter().map(|m| {
                format!("    fn {};", ImmutableString::from(m))
            });

            let static_function_lines = type_info.static_function_ids.into_iter().map(|m| {
                format!("    static fn {};", ImmutableString::from(m))
            });
        
            let body = ctor_lines
                .chain(method_lines)
                .chain(static_function_lines)
                .collect::<Vec<_>>()
                .join("\n");
        
            format!(
                "\n\nimpl {} {{\n{}\n}}",
                type_name, body
            )
        };

        ImmutableString::from(format!(
            "{}\n{}\n{}",
            header,
            layout,
            impl_block
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