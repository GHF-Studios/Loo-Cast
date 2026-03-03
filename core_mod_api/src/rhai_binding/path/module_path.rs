use rhai::ImmutableString;

use crate::{rhai_binding::path::{binding_path::{BindingPath, BindingPathSegment}, type_path::TypePath}, utils::string::assert_snake_case_clean_string};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ModulePath(BindingPath);
impl ModulePath {
    pub fn parse(raw: &ImmutableString) -> Self {
        let path = BindingPath::parse_with_classifier(raw, |_i, _total, seg| {
            assert_snake_case_clean_string(seg, "ModuleSegment");
            BindingPathSegment::Module(seg.clone())
        });

        Self(path)
    }

    pub fn as_path(&self) -> &BindingPath {
        &self.0
    }

    pub fn segments(&self) -> &[BindingPathSegment] {
        self.0.segments()
    }

    pub fn module_segments(&self) -> Vec<&ImmutableString> {
        self.0
            .segments()
            .iter()
            .map(|seg| match seg {
                BindingPathSegment::Module(s) => s,
                _ => unreachable!("ModulePath can only contain Module segments"),
            })
            .collect()
    }

    pub fn last(&self) -> &ImmutableString {
        self.module_segments()
            .last()
            .expect("ModulePath must contain at least one segment")
    }

    pub fn parent(&self) -> Option<ModulePath> {
        let segs = self.module_segments();
        if segs.len() <= 1 {
            return None;
        }

        let raw = segs[..segs.len() - 1]
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("::");

        Some(ModulePath::parse(&ImmutableString::from(raw)))
    }
}
impl Into<ModulePath> for &'static str {
    fn into(self) -> ModulePath {
        ModulePath::parse(&ImmutableString::from(self))
    }
}
impl std::fmt::Debug for ModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}
impl std::fmt::Display for ModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.to_string())
    }
}

/// Format: `"snake"`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TopLevelModulePath(ModulePath);
impl TopLevelModulePath {
    pub fn parse(raw: &ImmutableString) -> Self {
        let base = ModulePath::parse(raw);

        if base.as_path().segments().len() != 1 {
            panic!("TopLevelModulePath must contain exactly one segment");
        }

        Self(base)
    }

    pub fn as_module_path(&self) -> &ModulePath {
        &self.0
    }

    pub fn module_name(&self) -> &ImmutableString {
        self.0.last()
    }
}
impl Into<TopLevelModulePath> for &'static str {
    fn into(self) -> TopLevelModulePath {
        TopLevelModulePath::parse(&ImmutableString::from(self))
    }
}
impl std::fmt::Debug for TopLevelModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TopLevelModulePath({})", self.0.to_string())
    }
}
impl std::fmt::Display for TopLevelModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TopLevelModulePath({})", self.0.to_string())
    }
}

/// Format: `"snake::snake::..."`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SubModulePath(ModulePath);
impl SubModulePath {
    pub fn parse(raw: &ImmutableString) -> Self {
        let base = ModulePath::parse(raw);

        if base.as_path().segments().len() < 2 {
            panic!("SubModulePath must contain at least two segments");
        }

        Self(base)
    }

    pub fn as_module_path(&self) -> &ModulePath {
        &self.0
    }

    pub fn parent_module_path(&self) -> ModulePath {
        self.0
            .parent()
            .expect("SubModulePath must have a parent")
    }

    pub fn module_name(&self) -> &ImmutableString {
        self.0.last()
    }
}
impl Into<SubModulePath> for &'static str {
    fn into(self) -> SubModulePath {
        SubModulePath::parse(&ImmutableString::from(self))
    }
}
impl std::fmt::Debug for SubModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SubModulePath({})", self.0.to_string())
    }
}
impl std::fmt::Display for SubModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SubModulePath({})", self.0.to_string())
    }
}

/// Format: `"snake::snake::Pascal"`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypeBindingModulePath(pub(in crate::rhai_binding) TypePath);
impl TypeBindingModulePath {
    pub fn parse(raw: &ImmutableString) -> Self {
        let type_path = TypePath::parse(raw);
        Self(type_path)
    }

    pub fn as_type_path(&self) -> &TypePath {
        &self.0
    }

    pub fn parent_module_path(&self) -> ModulePath {
        self.0.module_path()
    }

    pub fn type_name(&self) -> &ImmutableString {
        self.0.type_name()
    }
}
impl Into<TypeBindingModulePath> for &'static str {
    fn into(self) -> TypeBindingModulePath {
        TypeBindingModulePath::parse(&ImmutableString::from(self))
    }
}
impl Into<TypePath> for TypeBindingModulePath {
    fn into(self) -> TypePath {
        self.0.clone()
    }
}
impl std::fmt::Debug for TypeBindingModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeBindingModulePath({})", self.0.to_string())
    }
}
impl std::fmt::Display for TypeBindingModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeBindingModulePath({})", self.0.to_string())
    }
}
