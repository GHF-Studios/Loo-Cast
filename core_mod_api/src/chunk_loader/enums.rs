use bevy::prelude::Reflect;

#[derive(Clone, Copy, Debug, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ZoomState {
    #[default]
    None,
    ZoomIn,
    ZoomOut,
}