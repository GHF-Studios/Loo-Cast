pub trait LogicSafety {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Checked;
impl LogicSafety for Checked {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Unchecked;
impl LogicSafety for Unchecked {}