use crate::bevy::prelude::Reflect;

#[derive(Clone, Copy, Default, Reflect, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub enum ZoomState {
    #[default]
    None,
    ZoomIn,
    ZoomOut,
}
