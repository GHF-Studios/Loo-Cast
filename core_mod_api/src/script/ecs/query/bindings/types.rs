use rhai::{Array, Dynamic, ImmutableString};

#[derive(Clone, Default)]
pub struct Query {
    pub(crate) values: Vec<Dynamic>,
    pub(crate) cursor: usize,
}

impl Query {
    pub fn from_values(values: Vec<Dynamic>) -> Self {
        Self { values, cursor: 0 }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum QueryDataAccess {
    Value,
    Ref,
    Mut,
}

impl QueryDataAccess {
    fn as_str(self) -> &'static str {
        match self {
            Self::Value => "value",
            Self::Ref => "ref",
            Self::Mut => "mut",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QueryDataTerm {
    pub(crate) type_id: String,
    pub(crate) access: QueryDataAccess,
}

impl QueryDataTerm {
    pub fn value(type_id: ImmutableString) -> Self {
        Self {
            type_id: normalize_type_id(type_id, "QueryDataTerm::value"),
            access: QueryDataAccess::Value,
        }
    }

    pub fn ref_(type_id: ImmutableString) -> Self {
        Self {
            type_id: normalize_type_id(type_id, "QueryDataTerm::ref_"),
            access: QueryDataAccess::Ref,
        }
    }

    pub fn mut_(type_id: ImmutableString) -> Self {
        Self {
            type_id: normalize_type_id(type_id, "QueryDataTerm::mut_"),
            access: QueryDataAccess::Mut,
        }
    }

    pub fn type_id(&self) -> ImmutableString {
        self.type_id.clone().into()
    }

    pub fn access(&self) -> ImmutableString {
        self.access.as_str().into()
    }

    pub(crate) fn dispatch_fragment(&self) -> String {
        format!("{}:{}", self.access.as_str(), self.type_id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QueryData {
    pub(crate) terms: Vec<QueryDataTerm>,
}

impl QueryData {
    pub fn single(type_id: ImmutableString) -> Self {
        Self {
            terms: vec![QueryDataTerm::value(type_id)],
        }
    }

    pub fn tuple(type_ids: Array) -> Self {
        let terms = type_ids
            .into_iter()
            .enumerate()
            .map(|(idx, value)| {
                let Some(type_id) = value.clone().try_cast::<ImmutableString>() else {
                    panic!("QueryData::tuple expects ImmutableString at index {idx}");
                };
                QueryDataTerm::value(type_id)
            })
            .collect::<Vec<_>>();
        Self::from_term_vec(terms, "QueryData::tuple")
    }

    pub fn from_terms(terms: Array) -> Self {
        let terms = terms
            .into_iter()
            .enumerate()
            .map(|(idx, value)| {
                let Some(term) = value.clone().try_cast::<QueryDataTerm>() else {
                    panic!("QueryData::from_terms expects QueryDataTerm at index {idx}");
                };
                term
            })
            .collect::<Vec<_>>();
        Self::from_term_vec(terms, "QueryData::from_terms")
    }

    pub fn len(&self) -> i64 {
        i64::try_from(self.terms.len()).unwrap_or_else(|_| panic!("QueryData term count '{}' exceeds i64::MAX", self.terms.len()))
    }

    pub fn to_array(&self) -> Array {
        self.terms.iter().cloned().map(Dynamic::from).collect()
    }

    pub(crate) fn dispatch_key(&self) -> String {
        self.terms.iter().map(QueryDataTerm::dispatch_fragment).collect::<Vec<_>>().join("|")
    }

    fn from_term_vec(terms: Vec<QueryDataTerm>, context: &str) -> Self {
        if terms.is_empty() {
            panic!("{context} requires at least one term");
        }
        Self { terms }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct QueryFilter {
    pub(crate) with: Vec<String>,
    pub(crate) without: Vec<String>,
}

impl QueryFilter {
    pub fn none() -> Self {
        Self {
            with: Vec::new(),
            without: Vec::new(),
        }
    }

    pub fn require(type_id: ImmutableString) -> Self {
        Self::from_collections(vec![normalize_type_id(type_id, "QueryFilter::require")], Vec::new())
    }

    pub fn exclude(type_id: ImmutableString) -> Self {
        Self::from_collections(Vec::new(), vec![normalize_type_id(type_id, "QueryFilter::exclude")])
    }

    pub fn from_sets(with: Array, without: Array) -> Self {
        let with = parse_type_id_array(with, "QueryFilter::from_sets(with)");
        let without = parse_type_id_array(without, "QueryFilter::from_sets(without)");
        Self::from_collections(with, without)
    }

    pub fn with_types(&self) -> Array {
        self.with.iter().cloned().map(Dynamic::from).collect()
    }

    pub fn without_types(&self) -> Array {
        self.without.iter().cloned().map(Dynamic::from).collect()
    }

    pub(crate) fn dispatch_key(&self) -> String {
        format!("with=[{}];without=[{}]", self.with.join(","), self.without.join(","))
    }

    fn from_collections(mut with: Vec<String>, mut without: Vec<String>) -> Self {
        with.sort_unstable();
        with.dedup();
        without.sort_unstable();
        without.dedup();

        let overlap = with
            .iter()
            .filter(|type_id| without.binary_search(type_id).is_ok())
            .cloned()
            .collect::<Vec<_>>();
        if !overlap.is_empty() {
            panic!("QueryFilter cannot contain overlapping `with` and `without` ids: {:?}", overlap);
        }

        Self { with, without }
    }
}

fn normalize_type_id(type_id: ImmutableString, context: &str) -> String {
    let type_id = type_id.to_string();
    if type_id.trim().is_empty() {
        panic!("{context} requires a non-empty type id");
    }
    type_id
}

fn parse_type_id_array(values: Array, context: &str) -> Vec<String> {
    values
        .into_iter()
        .enumerate()
        .map(|(idx, value)| {
            let Some(type_id) = value.clone().try_cast::<ImmutableString>() else {
                panic!("{context} expects ImmutableString at index {idx}");
            };
            normalize_type_id(type_id, context)
        })
        .collect::<Vec<_>>()
}
