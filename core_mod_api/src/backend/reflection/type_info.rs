use rhai::ImmutableString;

use crate::reflection::names::TypeName;
use crate::rhai_binding::value_semantics::ids::{DynamicTraitId, StaticTraitId, TypeId};

use super::{
    function_ids::{CtorId, MethodId, StaticFunctionId},
    layout::{FieldInfo, TypeDataInfo, TypeFormInfo, TypeLayoutInfo, VariantInfo},
    signatures::{CtorSignature, FunctionOrigin, MethodSignature, StaticFunctionSignature},
};

inventory::collect!(TypeInfo);
#[derive(Clone, PartialEq, Eq, Hash)]
// TODO: Change all Vec<SomeId> in reflection to HashSet<SomeId>
pub struct TypeInfo {
    pub type_id: TypeId,
    pub type_layout_info: TypeLayoutInfo,
    pub implemented_trait_ids: Vec<DynamicTraitId>,
    pub ctor_ids: Vec<CtorId>,
    pub method_ids: Vec<MethodId>,
    pub static_function_ids: Vec<StaticFunctionId>,
}
impl From<ImmutableString> for TypeInfo {
    fn from(input: ImmutableString) -> Self {
        fn parse_module_line(line: &str) -> &str {
            if !line.starts_with("#[module =") {
                panic!("expected #[module = \"...\"]");
            }
            line.split('"').nth(1).expect("invalid module syntax")
        }

        fn parse_type_declaration<'a>(line: &'a str, lines: &mut impl Iterator<Item = &'a str>) -> (TypeFormInfo, TypeName, Vec<DynamicTraitId>, Vec<&'a str>) {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.len() < 2 {
                panic!("invalid type declaration");
            }

            let kind = tokens[0];
            let name_and_traits = tokens[1..].join(" ");
            let name_and_traits: Vec<&str> = name_and_traits.split(':').map(str::trim).collect();
            let name = TypeName::new(ImmutableString::from(name_and_traits[0]));
            let implemented_trait_ids = if name_and_traits.len() == 2 {
                name_and_traits[1]
                    .split('+')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(|s| DynamicTraitId::from(ImmutableString::from(s)))
                    .collect()
            } else {
                Vec::new()
            };

            let mut body = Vec::new();
            for line in lines {
                if line == "}" {
                    break;
                }
                body.push(line);
            }

            match kind {
                "struct" => (TypeFormInfo::Struct, name, implemented_trait_ids, body),
                "enum" => (TypeFormInfo::Enum, name, implemented_trait_ids, body),
                _ => panic!("unknown type form: {}", kind),
            }
        }

        fn parse_fields(lines: Vec<&str>) -> Vec<FieldInfo> {
            lines
                .into_iter()
                .map(|line| {
                    let line = line.trim_end_matches(',');
                    FieldInfo::from(ImmutableString::from(line))
                })
                .collect()
        }

        fn parse_variants(lines: Vec<&str>) -> Vec<VariantInfo> {
            lines
                .into_iter()
                .map(|line| {
                    let line = line.trim_end_matches(',');
                    VariantInfo::from(ImmutableString::from(line))
                })
                .collect()
        }

        fn parse_ctor_id(line: &str) -> CtorId {
            let line = line.strip_prefix("ctor ").unwrap();
            let (sig, origin) = split_origin(line);
            CtorId {
                sig: CtorSignature::from(ImmutableString::from(sig)),
                origin,
            }
        }

        fn parse_method_id(line: &str) -> MethodId {
            let line = line.strip_prefix("fn ").unwrap();
            let (sig, origin) = split_origin(line);
            MethodId {
                sig: MethodSignature::from(ImmutableString::from(sig)),
                origin,
            }
        }

        fn parse_static_fn_id(line: &str) -> StaticFunctionId {
            let line = line.strip_prefix("static fn ").unwrap();
            let (sig, origin) = split_origin(line);
            StaticFunctionId {
                sig: StaticFunctionSignature::from(ImmutableString::from(sig)),
                origin,
            }
        }

        fn split_origin(entry: &str) -> (&str, FunctionOrigin) {
            if let Some((sig, trait_part)) = entry.split_once(" via ") {
                let trait_id = DynamicTraitId::from(ImmutableString::from(trait_part.trim()));
                (sig.trim(), FunctionOrigin::ViaTrait { trait_id })
            } else {
                (entry.trim(), FunctionOrigin::Inherent)
            }
        }

        let input = input.as_str().replace("\r\n", "\n");
        let mut lines = input.lines().map(str::trim).peekable();

        let module_line = lines.next().expect("expected #[module = \"...\"] line");
        let module_path = parse_module_line(module_line);

        let decl_line = lines.next().expect("expected type declaration line");
        let (form_info, type_name, implemented_trait_ids, data_lines) = parse_type_declaration(decl_line, &mut lines);
        let data_info = match form_info {
            TypeFormInfo::Struct => {
                let fields = parse_fields(data_lines);
                TypeDataInfo::Struct { field_infos: fields }
            }
            TypeFormInfo::Enum => {
                let variants = parse_variants(data_lines);
                TypeDataInfo::Enum { variant_infos: variants }
            }
        };

        let type_id = TypeId::from(ImmutableString::from(format!("{}::{}", module_path, type_name)));
        let type_layout_info = TypeLayoutInfo { data_info, form_info };

        let mut ctor_ids = Vec::new();
        let mut method_ids = Vec::new();
        let mut static_function_ids = Vec::new();
        while let Some(line) = lines.next() {
            if line.starts_with("impl ") {
                // Consume "impl TypeName {"
                assert!(line.ends_with("{"));

                while let Some(entry) = lines.next() {
                    let entry = entry.trim_end_matches(';').trim();
                    if entry == "}" {
                        break;
                    }

                    if entry.starts_with("ctor ") {
                        ctor_ids.push(parse_ctor_id(entry));
                    } else if entry.starts_with("fn ") {
                        method_ids.push(parse_method_id(entry));
                    } else if entry.starts_with("static fn ") {
                        static_function_ids.push(parse_static_fn_id(entry));
                    } else {
                        panic!("unknown impl entry: {}", entry);
                    }
                }
            }
        }

        TypeInfo {
            type_id,
            type_layout_info,
            implemented_trait_ids,
            ctor_ids,
            method_ids,
            static_function_ids,
        }
    }
}
impl From<TypeInfo> for ImmutableString {
    fn from(type_info: TypeInfo) -> Self {
        let type_name: ImmutableString = type_info.type_id.type_name.clone().into();
        let module_path: ImmutableString = type_info.type_id.module_id.clone().into();

        let header = format!("#[module = \"{}\"]", module_path);

        let traits = {
            let traits = type_info
                .implemented_trait_ids
                .into_iter()
                .map(|t| format!("{}", ImmutableString::from(t)))
                .collect::<Vec<_>>()
                .join(" + ");

            if !traits.is_empty() { format!(": {traits}") } else { String::new() }
        };
        let layout = match type_info.type_layout_info.data_info {
            TypeDataInfo::Struct { field_infos } => {
                let fields = field_infos
                    .into_iter()
                    .map(|f| format!("    {},", ImmutableString::from(f)))
                    .collect::<Vec<_>>()
                    .join("\n");

                format!("struct {}{} {{\n{}\n}}", type_name, traits, fields)
            }
            TypeDataInfo::Enum { variant_infos } => {
                let variants = variant_infos
                    .into_iter()
                    .map(|v| format!("    {},", ImmutableString::from(v)))
                    .collect::<Vec<_>>()
                    .join("\n");

                format!("enum {}{} {{\n{}\n}}", type_name, traits, variants)
            }
        };

        let impl_block = if type_info.ctor_ids.is_empty() && type_info.method_ids.is_empty() && type_info.static_function_ids.is_empty() {
            String::new()
        } else {
            let ctor_lines = type_info.ctor_ids.into_iter().map(|c| format!("    ctor {};", ImmutableString::from(c)));

            let method_lines = type_info.method_ids.into_iter().map(|m| format!("    fn {};", ImmutableString::from(m)));

            let static_function_lines = type_info
                .static_function_ids
                .into_iter()
                .map(|m| format!("    static fn {};", ImmutableString::from(m)));

            let body = ctor_lines.chain(method_lines).chain(static_function_lines).collect::<Vec<_>>().join("\n");

            format!("\n\nimpl {} {{\n{}\n}}", type_name, body)
        };

        ImmutableString::from(format!("{}\n{}\n{}", header, layout, impl_block))
    }
}
impl std::fmt::Debug for TypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_signature: ImmutableString = self.clone().into();
        write!(f, "{}", type_signature)
    }
}
impl std::fmt::Display for TypeInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_signature: ImmutableString = self.clone().into();
        write!(f, "{}", type_signature)
    }
}
