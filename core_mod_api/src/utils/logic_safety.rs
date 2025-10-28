use bevy::prelude::*;

pub trait LogicSafety {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct Checked;
impl LogicSafety for Checked {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct Unchecked;
impl LogicSafety for Unchecked {}