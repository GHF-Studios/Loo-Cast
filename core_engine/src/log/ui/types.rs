#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterTreeMode {
    Span,
    Module,
    Physical,
}