use rhai::ImmutableString;

use super::signatures::{FunctionOrigin, CtorSignature, MethodSignature, StaticFunctionSignature};

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