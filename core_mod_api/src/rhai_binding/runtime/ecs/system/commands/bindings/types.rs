use crate::bevy::prelude::Commands as BevyCommands;
use crate::rhai_binding::value_semantics::access_cell::{AccessCell, Scoped};

#[derive(Clone)]
#[repr(transparent)]
pub struct Commands {
    pub(crate) commands: AccessCell<Scoped, BevyCommands<'static, 'static>>
}

#[derive(Clone)]
#[repr(transparent)]
pub struct EntityCommands {
    pub(crate) entity_commands: AccessCell<Scoped, crate::bevy::prelude::EntityCommands<'static>>
}
