use heck::{ToPascalCase, ToSnakeCase};
use rhai::ImmutableString;

pub fn assert_pascal_case_clean_string(s: &ImmutableString, string_type_name: &'static str) {
    if s.is_empty() {
        panic!("{string_type_name} strings must not be empty");
    }

    if s.chars().any(|c| c.is_whitespace()) {
        panic!("{string_type_name} strings must not contain whitespace, found '{}'", s);
    }

    if s.chars().any(|c| !c.is_ascii_alphanumeric()) {
        panic!("{string_type_name} strings must be alphanumeric ASCII, found '{}'", s);
    }

    if s.chars().next().unwrap().is_ascii_digit() {
        panic!("{string_type_name} strings must not start with a digit, found '{}'", s);
    }

    if s != s.to_pascal_case() {
        panic!("{string_type_name}s must be in 'PascalCase' format, found '{}'", s);
    }
}

pub fn assert_snake_case_clean_string(s: &ImmutableString, string_type_name: &'static str) {
    if s.is_empty() {
        panic!("{string_type_name} strings must not be empty");
    }

    if s.chars().any(|c| c.is_whitespace()) {
        panic!("{string_type_name} strings must not contain whitespace, found '{}'", s);
    }

    if s.chars().any(|c| !c.is_ascii_alphanumeric() && c != '_') {
        panic!("{string_type_name} strings must be alphanumeric ASCII or underscores, found '{}'", s);
    }

    if s.chars().next().unwrap().is_ascii_digit() {
        panic!("{string_type_name} strings must not start with a digit, found '{}'", s);
    }

    let s = if s.chars().last().unwrap() == '_' {
        s.rsplit_once('_').unwrap().0
    } else {
        s.as_str()
    };

    if s != s.to_snake_case() {
        panic!("{string_type_name}s must be in 'snake_case' format, found '{}'", s);
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum ReflectionPathSegment {
    Module(ImmutableString),      // snake_case
    Type(ImmutableString),        // PascalCase
    Trait(ImmutableString),       // PascalCase
    Function(ImmutableString),    // snake_case
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ReflectionPath {
    segments: Vec<ReflectionPathSegment>,
}
impl ReflectionPath {
    fn parse_with_classifier<F>(raw: &ImmutableString, mut classify: F) -> Self
    where
        F: FnMut(usize, usize, &ImmutableString) -> ReflectionPathSegment,
    {
        if raw.is_empty() {
            panic!("ReflectionPath must not be empty");
        }

        let raw_segments: Vec<_> = raw.split("::").collect();

        let total = raw_segments.len();

        let segments = raw_segments
            .into_iter()
            .enumerate()
            .map(|(index, seg)| {
                let seg = ImmutableString::from(seg);
                classify(index, total, &seg)
            })
            .collect();

        Self { segments }
    }

    pub fn segments(&self) -> &[ReflectionPathSegment] {
        &self.segments
    }

    pub fn to_string(&self) -> ImmutableString {
        let mut out = String::new();

        for (i, seg) in self.segments.iter().enumerate() {
            if i > 0 {
                out.push_str("::");
            }

            match seg {
                ReflectionPathSegment::Module(s)
                | ReflectionPathSegment::Type(s)
                | ReflectionPathSegment::Trait(s)
                | ReflectionPathSegment::Function(s) => out.push_str(s),
            }
        }

        ImmutableString::from(out)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ModulePath(ReflectionPath);
impl ModulePath {
    pub fn parse(raw: &ImmutableString) -> Self {
        let path = ReflectionPath::parse_with_classifier(raw, |_i, _total, seg| {
            assert_snake_case_clean_string(seg, "ModuleSegment");
            ReflectionPathSegment::Module(seg.clone())
        });

        Self(path)
    }

    pub fn as_path(&self) -> &ReflectionPath {
        &self.0
    }

    pub fn segments(&self) -> &[ReflectionPathSegment] {
        self.0.segments()
    }

    pub fn module_segments(&self) -> Vec<&ImmutableString> {
        self.0
            .segments()
            .iter()
            .map(|seg| match seg {
                ReflectionPathSegment::Module(s) => s,
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
pub struct TypeProxyModulePath(TypePath);
impl TypeProxyModulePath {
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
impl Into<TypeProxyModulePath> for &'static str {
    fn into(self) -> TypeProxyModulePath {
        TypeProxyModulePath::parse(&ImmutableString::from(self))
    }
}
impl Into<TypePath> for TypeProxyModulePath {
    fn into(self) -> TypePath {
        self.0.clone()
    }
}
impl std::fmt::Debug for TypeProxyModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeProxyModulePath({})", self.0.to_string())
    }
}
impl std::fmt::Display for TypeProxyModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeProxyModulePath({})", self.0.to_string())
    }
}

/// Format: `"snake::snake::Type"`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypePath(ReflectionPath);
impl TypePath {
    pub fn parse(raw: &ImmutableString) -> Self {
        let path = ReflectionPath::parse_with_classifier(raw, |i, total, seg| {
            if i + 1 == total {
                assert_pascal_case_clean_string(seg, "TypeName");
                ReflectionPathSegment::Type(seg.clone())
            } else {
                assert_snake_case_clean_string(seg, "ModuleSegment");
                ReflectionPathSegment::Module(seg.clone())
            }
        });

        Self(path)
    }

    pub fn as_path(&self) -> &ReflectionPath {
        &self.0
    }

    pub fn module_path(&self) -> ModulePath {
        let segs = self.0.segments();

        let modules = segs[..segs.len() - 1]
            .iter()
            .map(|seg| match seg {
                ReflectionPathSegment::Module(s) => s.as_str(),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .join("::");

        ModulePath::parse(&ImmutableString::from(modules))
    }

    pub fn type_name(&self) -> &ImmutableString {
        match self.0.segments().last().unwrap() {
            ReflectionPathSegment::Type(s) => s,
            _ => unreachable!(),
        }
    }
}
impl Into<TypePath> for &'static str {
    fn into(self) -> TypePath {
        TypePath::parse(&ImmutableString::from(self))
    }
}
impl Into<TypeProxyModulePath> for TypePath {
    fn into(self) -> TypeProxyModulePath {
        TypeProxyModulePath(self.clone())
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

/// Format: `"Type"`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct InherentImplPath {
    type_path: TypePath
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

/// Format: `"snake::snake::Trait"`
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TraitPath(ReflectionPath);
impl TraitPath {
    pub fn parse(raw: &ImmutableString) -> Self {
        let path = ReflectionPath::parse_with_classifier(raw, |i, total, seg| {
            if i + 1 == total {
                assert_pascal_case_clean_string(seg, "TraitName");
                ReflectionPathSegment::Trait(seg.clone())
            } else {
                assert_snake_case_clean_string(seg, "ModuleSegment");
                ReflectionPathSegment::Module(seg.clone())
            }
        });

        Self(path)
    }

    pub fn as_path(&self) -> &ReflectionPath {
        &self.0
    }

    pub fn module_path(&self) -> ModulePath {
        let segs = self.0.segments();

        let modules = segs[..segs.len() - 1]
            .iter()
            .map(|seg| match seg {
                ReflectionPathSegment::Module(s) => s.as_str(),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .join("::");

        ModulePath::parse(&ImmutableString::from(modules))
    }

    pub fn trait_name(&self) -> &ImmutableString {
        match self.0.segments().last().unwrap() {
            ReflectionPathSegment::Trait(s) => s,
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
pub struct ModuleAssociatedFunctionPath(ReflectionPath);
impl ModuleAssociatedFunctionPath {
    pub fn parse(raw: &ImmutableString) -> Self {
        let path = ReflectionPath::parse_with_classifier(raw, |i, total, seg| {
            if i + 1 == total {
                // last segment → function
                assert_snake_case_clean_string(seg, "ModuleAssociatedFunctionName");
                ReflectionPathSegment::Function(seg.clone())
            } else {
                assert_snake_case_clean_string(seg, "ModuleSegment");
                ReflectionPathSegment::Module(seg.clone())
            }
        });

        Self(path)
    }

    pub fn as_path(&self) -> &ReflectionPath {
        &self.0
    }

    pub fn module_path(&self) -> ModulePath {
        let segs = self.0.segments();

        let modules = segs[..segs.len() - 1]
            .iter()
            .map(|seg| match seg {
                ReflectionPathSegment::Module(s) => s.as_str(),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .join("::");

        ModulePath::parse(&ImmutableString::from(modules))
    }

    pub fn function_name(&self) -> &ImmutableString {
        match self.0.segments().last().unwrap() {
            ReflectionPathSegment::Function(s) => s,
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
pub struct ConstructorFunctionPath(ReflectionPath);
impl ConstructorFunctionPath {
    pub fn parse(raw: &ImmutableString) -> Self {
        let path = MethodFunctionPath::parse(raw);
        Self(path.0)
    }

    pub fn as_path(&self) -> &ReflectionPath {
        &self.0
    }

    pub fn module_path(&self) -> ModulePath {
        let method = MethodFunctionPath(self.0.clone());
        method.module_path()
    }

    pub fn type_name(&self) -> &ImmutableString {
        match &self.0.segments()[self.0.segments().len() - 2] {
            ReflectionPathSegment::Type(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn function_name(&self) -> &ImmutableString {
        match self.0.segments().last().unwrap() {
            ReflectionPathSegment::Function(s) => s,
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
pub struct MethodFunctionPath(ReflectionPath);
impl MethodFunctionPath {
    pub fn parse(raw: &ImmutableString) -> Self {
        let path = ReflectionPath::parse_with_classifier(raw, |i, total, seg| {
            if i + 2 == total {
                assert_pascal_case_clean_string(seg, "TypeName");
                ReflectionPathSegment::Type(seg.clone())
            } else if i + 1 == total {
                assert_snake_case_clean_string(seg, "FunctionName");
                ReflectionPathSegment::Function(seg.clone())
            } else {
                assert_snake_case_clean_string(seg, "ModuleSegment");
                ReflectionPathSegment::Module(seg.clone())
            }
        });

        Self(path)
    }

    pub fn as_path(&self) -> &ReflectionPath {
        &self.0
    }

    pub fn module_path(&self) -> ModulePath {
        let segs = self.0.segments();

        let modules = segs[..segs.len() - 2]
            .iter()
            .map(|seg| match seg {
                ReflectionPathSegment::Module(s) => s.as_str(),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .join("::");

        ModulePath::parse(&ImmutableString::from(modules))
    }

    pub fn type_name(&self) -> &ImmutableString {
        match &self.0.segments()[self.0.segments().len() - 2] {
            ReflectionPathSegment::Type(s) => s,
            _ => unreachable!(),
        }
    }

    pub fn function_name(&self) -> &ImmutableString {
        match self.0.segments().last().unwrap() {
            ReflectionPathSegment::Function(s) => s,
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
