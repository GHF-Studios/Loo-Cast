use bevy::prelude::Reflect;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Reflect)]
pub enum SelectionMode {
    #[default]
    Span,
    Module,
    Physical,
}
