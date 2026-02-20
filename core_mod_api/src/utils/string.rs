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

/// Format: "snake"
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

/// Format: "snake::snake::..."
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

/// Format: "snake::snake::Pascal"
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

/// Format: "snake::snake::Type"
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

/// Format: "snake::snake::Trait"
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

/// Format: "snake::snake::snake_function"
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

/// Format: "snake::snake::Type::snake_function"
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct TypeAssociatedFunctionPath(ReflectionPath);
impl TypeAssociatedFunctionPath {
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
impl Into<TypeAssociatedFunctionPath> for &'static str {
    fn into(self) -> TypeAssociatedFunctionPath {
        TypeAssociatedFunctionPath::parse(&ImmutableString::from(self))
    }
}
impl std::fmt::Debug for TypeAssociatedFunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeAssociatedFunctionPath({})", self.0.to_string())
    }
}
impl std::fmt::Display for TypeAssociatedFunctionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TypeAssociatedFunctionPath({})", self.0.to_string())
    }
}

/// Format: "snake::snake::Type::snake_function"
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

/// Format: "snake::snake::Type::snake_function"
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
