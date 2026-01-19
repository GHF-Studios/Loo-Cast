use bevy::prelude::Commands as BevyCommands;
use rhai::{Dynamic, FnPtr, NativeCallContext};

use crate::script::core::internals::{traits::ScopedAccessProvider, types::ScopedAccessHandle};

#[repr(transparent)]
pub struct Commands {
    pub(crate) commands: ScopedAccessHandle<BevyCommands<'static, 'static>>
}

#[repr(transparent)]
pub struct EntityCommands {
    pub(crate) entity_commands: ScopedAccessHandle<bevy::prelude::EntityCommands<'static>>
}