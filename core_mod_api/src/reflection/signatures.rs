use rhai::ImmutableString;

use crate::rhai_binding::value_semantics::ids::DynamicTraitId;

use super::{
    ids::TypeId,
    names::{ArgName, CtorName, MethodName, StaticFunctionName},
};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct ArgInfo {
    pub name: ArgName,
    pub type_id: TypeId,
}
impl ArgInfo {
    pub fn new(name: impl Into<ArgName>, type_path: impl Into<TypeId>) -> Self {
        ArgInfo {
            name: name.into(),
            type_id: type_path.into(),
        }
    }
}
impl From<ImmutableString> for ArgInfo {
    fn from(arg_signature: ImmutableString) -> Self {
        let parts: Vec<&str> = arg_signature.split(": ").collect();

        ArgInfo {
            name: ArgName::from(ImmutableString::from(parts[0])),
            type_id: TypeId::from(ImmutableString::from(parts[1])),
        }
    }
}
impl From<ArgInfo> for ImmutableString {
    fn from(arg_info: ArgInfo) -> Self {
        let type_path: ImmutableString = arg_info.type_id.into();

        ImmutableString::from(format!("{}: {}", arg_info.name.name, type_path))
    }
}
impl std::fmt::Debug for ArgInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arg_signature: ImmutableString = self.clone().into();
        write!(f, "{}", arg_signature)
    }
}
impl std::fmt::Display for ArgInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let arg_signature: ImmutableString = self.clone().into();
        write!(f, "{}", arg_signature)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum FunctionOrigin {
    Inherent,
    ViaTrait { trait_id: DynamicTraitId },
}
impl From<FunctionOrigin> for ImmutableString {
    fn from(function_origin: FunctionOrigin) -> Self {
        match function_origin {
            FunctionOrigin::Inherent => ImmutableString::new(),
            FunctionOrigin::ViaTrait { trait_id } => {
                let trait_path: ImmutableString = trait_id.into();
                ImmutableString::from(format!(" via {}", trait_path))
            }
        }
    }
}
impl std::fmt::Debug for FunctionOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let origin_signature: ImmutableString = self.clone().into();
        write!(f, "{}", origin_signature)
    }
}
impl std::fmt::Display for FunctionOrigin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let origin_signature: ImmutableString = self.clone().into();
        write!(f, "{}", origin_signature)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CtorSignature {
    pub name: CtorName,
    pub arg_infos: Vec<ArgInfo>,
}
impl From<ImmutableString> for CtorSignature {
    fn from(src: ImmutableString) -> Self {
        let src = src.trim();
        let open_paren = src.find('(').expect("missing '(' in constructor signature");
        let close_paren = src.rfind(')').expect("missing ')' in constructor signature");

        let name = &src[..open_paren].trim();
        let args = &src[open_paren + 1..close_paren];

        let arg_infos = if args.is_empty() {
            Vec::new()
        } else {
            args.split(',').map(|arg| ArgInfo::from(ImmutableString::from(arg.trim()))).collect()
        };

        CtorSignature {
            name: CtorName::from(ImmutableString::from(*name)),
            arg_infos,
        }
    }
}
impl From<CtorSignature> for ImmutableString {
    fn from(ctor_sig: CtorSignature) -> Self {
        let arg_signatures: Vec<ImmutableString> = ctor_sig.arg_infos.into_iter().map(|ai| ai.into()).collect();

        ImmutableString::from(format!("ctor {}({})", ctor_sig.name.name, arg_signatures.join(", ")))
    }
}
impl std::fmt::Debug for CtorSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ctor_signature: ImmutableString = self.clone().into();
        write!(f, "{}", ctor_signature)
    }
}
impl std::fmt::Display for CtorSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ctor_signature: ImmutableString = self.clone().into();
        write!(f, "{}", ctor_signature)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MethodSignature {
    pub name: MethodName,
    pub arg_infos: Vec<ArgInfo>,
    pub return_type_id: TypeId,
}
impl From<ImmutableString> for MethodSignature {
    fn from(src: ImmutableString) -> Self {
        let src = src.trim();

        let parts: Vec<&str> = src.split("->").map(str::trim).collect();
        if parts.len() != 2 {
            panic!("method signature must contain '->'");
        }

        let decl_part = parts[0];
        let return_type_str = parts[1];

        let open_paren = decl_part.find('(').expect("missing '(' in method signature");
        let close_paren = decl_part.rfind(')').expect("missing ')' in method signature");

        let name = &decl_part[..open_paren].trim();
        let args = &decl_part[open_paren + 1..close_paren];

        let arg_infos = if args.is_empty() {
            Vec::new()
        } else {
            args.split(',').map(|arg| ArgInfo::from(ImmutableString::from(arg.trim()))).collect()
        };

        MethodSignature {
            name: MethodName::from(ImmutableString::from(*name)),
            arg_infos,
            return_type_id: TypeId::from(ImmutableString::from(return_type_str)),
        }
    }
}
impl From<MethodSignature> for ImmutableString {
    fn from(method_sig: MethodSignature) -> Self {
        let arg_signatures: Vec<ImmutableString> = method_sig.arg_infos.into_iter().map(|ai| ai.into()).collect();
        let return_type_path: ImmutableString = method_sig.return_type_id.into();

        ImmutableString::from(format!("fn {}({}) -> {}", method_sig.name.name, arg_signatures.join(", "), return_type_path))
    }
}
impl std::fmt::Debug for MethodSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_signature: ImmutableString = self.clone().into();
        write!(f, "{}", method_signature)
    }
}
impl std::fmt::Display for MethodSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let method_signature: ImmutableString = self.clone().into();
        write!(f, "{}", method_signature)
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StaticFunctionSignature {
    pub name: StaticFunctionName,
    pub arg_infos: Vec<ArgInfo>,
    pub return_type_id: TypeId,
}
impl From<ImmutableString> for StaticFunctionSignature {
    fn from(src: ImmutableString) -> Self {
        let src = src.trim();

        let parts: Vec<&str> = src.split("->").map(str::trim).collect();
        if parts.len() != 2 {
            panic!("static function signature must contain '->'");
        }

        let decl_part = parts[0];
        let return_type_str = parts[1];

        let open_paren = decl_part.find('(').expect("missing '(' in static fn signature");
        let close_paren = decl_part.rfind(')').expect("missing ')' in static fn signature");

        let name = &decl_part[..open_paren].trim();
        let args = &decl_part[open_paren + 1..close_paren];

        let arg_infos = if args.is_empty() {
            Vec::new()
        } else {
            args.split(',').map(|arg| ArgInfo::from(ImmutableString::from(arg.trim()))).collect()
        };

        StaticFunctionSignature {
            name: StaticFunctionName::from(ImmutableString::from(*name)),
            arg_infos,
            return_type_id: TypeId::from(ImmutableString::from(return_type_str)),
        }
    }
}
impl From<StaticFunctionSignature> for ImmutableString {
    fn from(static_fn_sig: StaticFunctionSignature) -> Self {
        let arg_signatures: Vec<ImmutableString> = static_fn_sig.arg_infos.into_iter().map(|ai| ai.into()).collect();
        let return_type_path: ImmutableString = static_fn_sig.return_type_id.into();

        ImmutableString::from(format!("fn {}({}) -> {}", static_fn_sig.name.name, arg_signatures.join(", "), return_type_path))
    }
}
impl std::fmt::Debug for StaticFunctionSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fn_signature: ImmutableString = self.clone().into();
        write!(f, "{}", fn_signature)
    }
}
impl std::fmt::Display for StaticFunctionSignature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fn_signature: ImmutableString = self.clone().into();
        write!(f, "{}", fn_signature)
    }
}
