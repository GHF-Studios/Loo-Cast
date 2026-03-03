use rhai::ImmutableString;

use crate::{rhai_binding::path::{binding_path::{BindingPath, BindingPathSegment}, module_path::ModulePath}, utils::string::{assert_pascal_case_clean_string, assert_snake_case_clean_string}};


/// Format: `"snake::snake::Trait"`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TraitPath(BindingPath);
impl TraitPath {
    pub fn parse(raw: &ImmutableString) -> Self {
        let path = BindingPath::parse_with_classifier(raw, |i, total, seg| {
            if i + 1 == total {
                assert_pascal_case_clean_string(seg, "TraitName");
                BindingPathSegment::Trait(seg.clone())
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

    pub fn trait_name(&self) -> &ImmutableString {
        match self.0.segments().last().unwrap() {
            BindingPathSegment::Trait(s) => s,
            _ => unreachable!(),
        }
    }
}
impl Into<TraitPath> for &'static str {
    fn into(self) -> TraitPath {
        TraitPath::parse(&ImmutableString::from(self))
    }
}
impl std::fmt::Debug for TraitPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TraitPath({})", self.0.to_string())
    }
}
impl std::fmt::Display for TraitPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TraitPath({})", self.0.to_string())
    }
}
