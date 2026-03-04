use crate::bevy::prelude::Commands as BevyCommands;
use crate::rhai_binding::value_semantics::access_cell::{AccessCell, Scoped};

#[repr(transparent)]
pub struct Commands {
    pub(crate) commands: AccessCell<Scoped, BevyCommands<'static, 'static>>
}

#[repr(transparent)]
pub struct EntityCommands {
    pub(crate) entity_commands: AccessCell<Scoped, crate::bevy::prelude::EntityCommands<'static>>
}
