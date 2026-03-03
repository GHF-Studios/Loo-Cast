use rhai::ImmutableString;

use crate::{rhai_binding::path::{binding_path::{BindingPath, BindingPathSegment}, impl_path::InherentImplPath, module_path::{ModulePath, TypeBindingModulePath}}, utils::string::{assert_pascal_case_clean_string, assert_snake_case_clean_string}};


/// Format: `"snake::snake::Type"`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypePath(pub(in crate::rhai_binding) BindingPath);
impl TypePath {
    pub fn parse(raw: &ImmutableString) -> Self {
        let path = BindingPath::parse_with_classifier(raw, |i, total, seg| {
            if i + 1 == total {
                assert_pascal_case_clean_string(seg, "TypeName");
                BindingPathSegment::Type(seg.clone())
            } else {
                assert_snake_case_clean_string(seg, "ModuleSegment");
                BindingPathSegment::Module(seg.clone())
            }
        });

        Self(path)
    }

    pub fn as_path(&self) -> &BindingPath {
        &self.0
    }

    pub fn module_path(&self) -> ModulePath {
        let segs = self.0.segments();

        let modules = segs[..segs.len() - 1]
            .iter()
            .map(|seg| match seg {
                BindingPathSegment::Module(s) => s.as_str(),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .join("::");

        ModulePath::parse(&ImmutableString::from(modules))
    }

    pub fn type_name(&self) -> &ImmutableString {
        match self.0.segments().last().unwrap() {
            BindingPathSegment::Type(s) => s,
            _ => unreachable!(),
        }
    }
}
impl Into<TypePath> for &'static str {
    fn into(self) -> TypePath {
        TypePath::parse(&ImmutableString::from(self))
    }
}
impl Into<TypeBindingModulePath> for TypePath {
    fn into(self) -> TypeBindingModulePath {
        TypeBindingModulePath(self.clone())
    }
}
impl Into<InherentImplPath> for TypePath {
    fn into(self) -> InherentImplPath {
        InherentImplPath { type_path: self }
    }
}
impl std::fmt::Debug for TypePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypePath({})", self.0.to_string())
    }
}
impl std::fmt::Display for TypePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypePath({})", self.0.to_string())
    }
}
