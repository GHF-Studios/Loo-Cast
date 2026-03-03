use rhai::ImmutableString;

use crate::{
    rhai_binding::path::{
        binding_path::{BindingPath, BindingPathSegment},
        module_path::ModulePath,
        trait_path::TraitPath,
        type_path::TypePath,
        impl_path::TraitImplPath,
    },
    utils::string::{assert_snake_case_clean_string, assert_pascal_case_clean_string}
};

/// Format: `"Type::function"`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct InherentImplFunctionPath {
    type_path: TypePath,
    function_name: ImmutableString,
}
impl InherentImplFunctionPath {
    pub fn new(type_path: TypePath, function_name: ImmutableString) -> Self {
        Self { type_path, function_name }
    }

    pub fn parse(raw: &ImmutableString) -> Self {
        let parts: Vec<_> = raw.split("::").collect();

        if parts.len() < 2 {
            panic!("InherentImplFunctionPath must be `Type::function`");
        }

        let function_name = ImmutableString::from(*parts.last().unwrap());
        let type_part = parts[..parts.len() - 1].join("::");

        let type_path = TypePath::parse(&ImmutableString::from(type_part));

        Self { type_path, function_name }
    }

    pub fn type_path(&self) -> &TypePath {
        &self.type_path
    }

    pub fn function_name(&self) -> &ImmutableString {
        &self.function_name
    }

    pub fn module_path(&self) -> ModulePath {
        self.type_path.module_path()
    }
}
impl Into<InherentImplFunctionPath> for &'static str {
    fn into(self) -> InherentImplFunctionPath {
        InherentImplFunctionPath::parse(&ImmutableString::from(self))
    }
}
impl std::fmt::Debug for InherentImplFunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.type_path, self.function_name)
    }
}
impl std::fmt::Display for InherentImplFunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.type_path, self.function_name)
    }
}

/// Format: `"<Type as Trait>::function"`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TraitImplFunctionPath {
    impl_path: TraitImplPath,
    function_name: ImmutableString,
}
impl TraitImplFunctionPath {
    pub fn new(impl_path: TraitImplPath, function_name: ImmutableString) -> Self {
        Self { impl_path, function_name }
    }

    pub fn parse(raw: &ImmutableString) -> Self {
        let parts: Vec<_> = raw.split(">::").collect();

        if parts.len() != 2 {
            panic!("TraitImplFunctionPath must be in format `<Type as Trait>::function`");
        }

        let impl_part = format!("{}>", parts[0]);
        let impl_path = TraitImplPath::parse(&ImmutableString::from(impl_part));

        let function_name = ImmutableString::from(parts[1]);

        Self { impl_path, function_name }
    }

    pub fn impl_path(&self) -> &TraitImplPath {
        &self.impl_path
    }

    pub fn type_path(&self) -> &TypePath {
        self.impl_path.type_path()
    }

    pub fn trait_path(&self) -> &TraitPath {
        self.impl_path.trait_path()
    }

    pub fn function_name(&self) -> &ImmutableString {
        &self.function_name
    }
}
impl Into<TraitImplFunctionPath> for &'static str {
    fn into(self) -> TraitImplFunctionPath {
        TraitImplFunctionPath::parse(&ImmutableString::from(self))
    }
}
impl std::fmt::Debug for TraitImplFunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.impl_path, self.function_name)
    }
}
impl std::fmt::Display for TraitImplFunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.impl_path, self.function_name)
    }
}

/// Format: `"snake::snake::snake_function"`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ModuleAssociatedFunctionPath(BindingPath);
impl ModuleAssociatedFunctionPath {
    pub fn parse(raw: &ImmutableString) -> Self {
        let path = BindingPath::parse_with_classifier(raw, |i, total, seg| {
            if i + 1 == total {
                // last segment → function
                assert_snake_case_clean_string(seg, "ModuleAssociatedFunctionName");
                BindingPathSegment::Function(seg.clone())
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

    pub fn function_name(&self) -> &ImmutableString {
        match self.0.segments().last().unwrap() {
            BindingPathSegment::Function(s) => s,
            _ => unreachable!(),
        }
    }
}
impl Into<ModuleAssociatedFunctionPath> for &'static str {
    fn into(self) -> ModuleAssociatedFunctionPath {
        ModuleAssociatedFunctionPath::parse(&ImmutableString::from(self))
    }
}
impl std::fmt::Debug for ModuleAssociatedFunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModuleAssociatedFunctionPath({})", self.0.to_string())
    }
}
impl std::fmt::Display for ModuleAssociatedFunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModuleAssociatedFunctionPath({})", self.0.to_string())
    }
}

/// Format: `"Type::function"` or `"<Type as Trait>::function"`
#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ItemAssociatedFunctionPath {
    Inherent(InherentImplFunctionPath),
    ViaTrait(TraitImplFunctionPath),
}
impl ItemAssociatedFunctionPath {
    pub fn parse(raw: &ImmutableString) -> Self {
        if raw.starts_with("<") {
            Self::ViaTrait(TraitImplFunctionPath::parse(raw))
        } else {
            Self::Inherent(InherentImplFunctionPath::parse(raw))
        }
    }

    pub fn type_path(&self) -> &TypePath {
        match self {
            Self::Inherent(p) => p.type_path(),
            Self::ViaTrait(p) => p.type_path(),
        }
    }

    pub fn trait_path(&self) -> Option<&TraitPath> {
        match self {
            Self::Inherent(_) => None,
            Self::ViaTrait(p) => Some(p.trait_path()),
        }
    }

    pub fn function_name(&self) -> &ImmutableString {
        match self {
            Self::Inherent(p) => p.function_name(),
            Self::ViaTrait(p) => p.function_name(),
        }
    }

    pub fn module_path(&self) -> ModulePath {
        self.type_path().module_path()
    }
}
impl Into<ItemAssociatedFunctionPath> for &'static str {
    fn into(self) -> ItemAssociatedFunctionPath {
        ItemAssociatedFunctionPath::parse(&ImmutableString::from(self))
    }
}
impl std::fmt::Debug for ItemAssociatedFunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inherent(p) => write!(f, "{}", p),
            Self::ViaTrait(p) => write!(f, "{}", p),
        }
    }
}
impl std::fmt::Display for ItemAssociatedFunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inherent(p) => write!(f, "{}", p),
            Self::ViaTrait(p) => write!(f, "{}", p),
        }
    }
}

/// Format: `"snake::snake::Type::snake_function"`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ConstructorFunctionPath(BindingPath);
impl ConstructorFunctionPath {
    pub fn parse(raw: &ImmutableString) -> Self {
        let path = MethodFunctionPath::parse(raw);
        Self(path.0)
    }

    pub fn as_path(&self) -> &BindingPath {
        &self.0
    }

    pub fn module_path(&self) -> ModulePath {
        let method = MethodFunctionPath(self.0.clone());
        method.module_path()
    }

    pub fn type_name(&self) -> &ImmutableString {
        match &self.0.segments()[self.0.segments().len() - 2] {
            BindingPathSegment::Type(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn function_name(&self) -> &ImmutableString {
        match self.0.segments().last().unwrap() {
            BindingPathSegment::Function(s) => s,
            _ => unreachable!(),
        }
    }
}
impl Into<ConstructorFunctionPath> for &'static str {
    fn into(self) -> ConstructorFunctionPath {
        ConstructorFunctionPath::parse(&ImmutableString::from(self))
    }
}
impl std::fmt::Debug for ConstructorFunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstructorFunctionPath({})", self.0.to_string())
    }
}
impl std::fmt::Display for ConstructorFunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ConstructorFunctionPath({})", self.0.to_string())
    }
}

/// Format: `"snake::snake::Type::snake_function"`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MethodFunctionPath(BindingPath);
impl MethodFunctionPath {
    pub fn parse(raw: &ImmutableString) -> Self {
        let path = BindingPath::parse_with_classifier(raw, |i, total, seg| {
            if i + 2 == total {
                assert_pascal_case_clean_string(seg, "TypeName");
                BindingPathSegment::Type(seg.clone())
            } else if i + 1 == total {
                assert_snake_case_clean_string(seg, "FunctionName");
                BindingPathSegment::Function(seg.clone())
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

        let modules = segs[..segs.len() - 2]
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
        match &self.0.segments()[self.0.segments().len() - 2] {
            BindingPathSegment::Type(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn function_name(&self) -> &ImmutableString {
        match self.0.segments().last().unwrap() {
            BindingPathSegment::Function(s) => s,
            _ => unreachable!(),
        }
    }
}
impl Into<MethodFunctionPath> for &'static str {
    fn into(self) -> MethodFunctionPath {
        MethodFunctionPath::parse(&ImmutableString::from(self))
    }
}
impl std::fmt::Debug for MethodFunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MethodFunctionPath({})", self.0.to_string())
    }
}
impl std::fmt::Display for MethodFunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MethodFunctionPath({})", self.0.to_string())
    }
}
