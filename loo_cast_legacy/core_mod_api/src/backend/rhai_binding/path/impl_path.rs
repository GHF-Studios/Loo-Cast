use rhai::ImmutableString;

use crate::rhai_binding::path::{trait_path::TraitPath, type_path::TypePath};

/// Format: `"Type"`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct InherentImplPath {
    pub(in crate::backend::rhai_binding) type_path: TypePath,
}
impl InherentImplPath {
    pub fn type_path(&self) -> &TypePath {
        &self.type_path
    }
}
impl Into<InherentImplPath> for &'static str {
    fn into(self) -> InherentImplPath {
        InherentImplPath {
            type_path: TypePath::parse(&ImmutableString::from(self)),
        }
    }
}
impl Into<TypePath> for InherentImplPath {
    fn into(self) -> TypePath {
        self.type_path
    }
}
impl std::fmt::Debug for InherentImplPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypePath({})", self.type_path.0.to_string())
    }
}
impl std::fmt::Display for InherentImplPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypePath({})", self.type_path.0.to_string())
    }
}

/// Format: `"<Type as Trait>"`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TraitImplPath {
    type_path: TypePath,
    trait_path: TraitPath,
}
impl TraitImplPath {
    pub fn new(type_path: TypePath, trait_path: TraitPath) -> Self {
        Self { type_path, trait_path }
    }

    pub fn parse(raw: &ImmutableString) -> Self {
        if !raw.starts_with("<") || !raw.ends_with(">") {
            panic!("TraitImplPath must be in format `<Type as Trait>`");
        }

        let inner = &raw[1..raw.len() - 1]; // strip <>
        let parts: Vec<_> = inner.split(" as ").collect();

        if parts.len() != 2 {
            panic!("TraitImplPath must be in format `<Type as Trait>`");
        }

        let type_path = TypePath::parse(&ImmutableString::from(parts[0]));
        let trait_path = TraitPath::parse(&ImmutableString::from(parts[1]));

        Self { type_path, trait_path }
    }

    pub fn type_path(&self) -> &TypePath {
        &self.type_path
    }

    pub fn trait_path(&self) -> &TraitPath {
        &self.trait_path
    }
}
impl Into<TraitImplPath> for &'static str {
    fn into(self) -> TraitImplPath {
        TraitImplPath::parse(&ImmutableString::from(self))
    }
}
impl std::fmt::Debug for TraitImplPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} as {}>", self.type_path, self.trait_path)
    }
}
impl std::fmt::Display for TraitImplPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} as {}>", self.type_path, self.trait_path)
    }
}
