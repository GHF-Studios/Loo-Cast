use rhai::ImmutableString;

use super::{ids::TypeId, names::{FieldName, VariantName}};

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