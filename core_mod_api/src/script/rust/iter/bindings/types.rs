#[derive(Clone, Default)]
pub struct StringIter {
    pub(crate) values: Vec<String>,
    pub(crate) cursor: usize,
}

impl StringIter {
    pub fn from_values(values: Vec<String>) -> Self {
        Self { values, cursor: 0 }
    }
}
