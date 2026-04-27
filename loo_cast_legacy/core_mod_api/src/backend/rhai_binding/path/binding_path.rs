use rhai::ImmutableString;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum BindingPathSegment {
    Module(ImmutableString),   // snake_case
    Type(ImmutableString),     // PascalCase
    Trait(ImmutableString),    // PascalCase
    Function(ImmutableString), // snake_case
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct BindingPath {
    segments: Vec<BindingPathSegment>,
}
impl BindingPath {
    pub(in crate::backend::rhai_binding) fn parse_with_classifier<F>(raw: &ImmutableString, mut classify: F) -> Self
    where
        F: FnMut(usize, usize, &ImmutableString) -> BindingPathSegment,
    {
        if raw.is_empty() {
            panic!("BindingPath must not be empty");
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

    pub fn segments(&self) -> &[BindingPathSegment] {
        &self.segments
    }

    pub fn to_string(&self) -> ImmutableString {
        let mut out = String::new();

        for (i, seg) in self.segments.iter().enumerate() {
            if i > 0 {
                out.push_str("::");
            }

            match seg {
                BindingPathSegment::Module(s) | BindingPathSegment::Type(s) | BindingPathSegment::Trait(s) | BindingPathSegment::Function(s) => out.push_str(s),
            }
        }

        ImmutableString::from(out)
    }
}
