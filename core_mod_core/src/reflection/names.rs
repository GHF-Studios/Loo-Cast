use rhai::ImmutableString;

use crate::utils::{assert_pascal_case_clean_string, assert_snake_case_clean_string};

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