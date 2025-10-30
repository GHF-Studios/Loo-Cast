use bevy::prelude::*;

pub trait LogicSafety {}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct Checked;
impl LogicSafety for Checked {}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, Reflect)]
pub struct Unchecked;
impl LogicSafety for Unchecked {}